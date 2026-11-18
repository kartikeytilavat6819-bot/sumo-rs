#!/usr/bin/env python3
"""Import official SUMO TextTest cases as run_all oracle cases.

Usage:
    python tools/oracle/import_texttest.py basic cf_model
    python tools/oracle/import_texttest.py /path/to/tests/sumo/cf_model --dry-run

For every leaf ``sumo`` test in the given subtrees (relative to ``tests/sumo``)
one case ``tools/oracle/cases/tt_<path_with_underscores>/`` is created.  The
case contains the resolved input files (net, routes, additionals and every
``copy_test_path`` from the test's configs), a ``sim.args`` with the collated
options minus input/output-file flags, a ``CASE.md`` and a ``NEW_CASE`` marker
so ``make_sim_refs.sh`` generates the C++ references from the official sumo.

TextTest rules implemented (mirrors ``sumo-src/tools/extractTest.py``):
  * a test dir has ``options.sumo`` but no ``testsuite.*``;
  * options are the concatenation of ``options.sumo`` files from ``tests/sumo``
    down to the test dir (child appended last), with ``shlex.split``,
    ``--xml-validation`` dropped, ``{CLEAR}`` honoured and repeated options
    replaced by their last occurrence;
  * files named in options/config are looked up first in the test dir, then in
    each parent directory up to ``tests/sumo`` (most specific wins).

Tests needing the GUI, TraCI, external tools/python scripts, or with no usable
net/route are skipped.  The importer is deterministic and re-runnable: an
existing case is left untouched unless ``--force`` is given.

Standard library only (Python 3.8+), so it runs on the gate without setup.
"""
from __future__ import annotations

import argparse
import collections
import os
import shlex
import shutil
import sys
from os.path import join, realpath

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(os.path.dirname(HERE))
CASES = join(REPO, "tools", "oracle", "cases")
DEFAULT_TESTS_SUMO = r"C:\Users\karti\sumo-src\tests\sumo"

# Option names (long and short) whose value is an input file.
NET_OPTS = ("--net-file", "-n")
ROUTE_OPTS = ("-r", "--route-files", "--routes")
ADD_OPTS = ("-a", "--additional-files", "--a")
# File-valued behaviour options: their files must be copied into the case, but
# the option itself stays in sim.args (it is not an -a/--additional-files list).
EXTRA_INPUT_OPTS = ("--weight-files",)
# Output-file options.  Sub-options such as --fcd-output.attributes are *not*
# here: they stay in sim.args as pure behaviour flags.
OUTPUT_OPTS = (
    "--tripinfo-output", "--fcd-output", "--statistic-output", "--summary",
    "--summary-output", "--vehroute-output", "--vehroutes", "--stop-output",
    "--personinfo-output", "--person-summary-output", "--lanechange-output",
    "--emissions-output", "--queue-output", "--tls-state-output",
    "--tls-switch-output", "--tls-state-switch-output", "--tls-program-output",
    "--full-output", "--emergency-output", "--collision-output",
    "--safety-output", "--battery-output", "--chargingstation-output",
    "--rail-signal-output", "--person-route-output", "--link-output",
    "--weights-output", "--deadlock-output", "--ssm-output", "--bh-output",
    "--save-state.files", "--save-state.times", "--movereminder-output",
)
ALL_FILE_OPTS = (NET_OPTS, ROUTE_OPTS, ADD_OPTS, OUTPUT_OPTS)
# Sub-options of these output files are meaningful: run_all/make_sim_refs
# always pass the parent file, so keep e.g. --fcd-output.attributes.
HARNESS_OUTPUT_PREFIXES = ("--tripinfo-output.", "--fcd-output.", "--statistic-output.")

# run_all.sh / make_sim_refs.sh always pass these; a duplicate is a hard error.
HARNESS_FLAGS = ("--no-step-log",)

# A token containing any of these means the test cannot run headless with sumo.
SKIP_MARKERS = (
    ".py", "--python-script", "--traci", "--remote-port", "--gui",
    "--osg-view", "--netedit", "--duarouter", "--jtrrouter", "--marouter",
    "--od2trips", "--activitygen", "--polyconvert", "--java", "--jar",
    "--fmi", "--parquet", "--vtk", "--p2p", "--sumo-gui", "--gcf",
    "--saved-state.input", "--load-state", "--sumo-state",
    # option removed from SUMO: the official binary itself rejects it
    "--meso-lookahead",
)


def ancestors(source: str, tests_sumo: str):
    """Directory chain from the test dir up to and including tests/sumo."""
    chain = []
    cur = realpath(source)
    limit = realpath(tests_sumo)
    while True:
        chain.append(cur)
        if cur == limit or os.path.dirname(cur) == cur:
            break
        cur = os.path.dirname(cur)
    # most specific first
    return chain


def collect_files_and_config(source: str, tests_sumo: str):
    """Return (option files, config files, potentials) walking up the tree."""
    chain = ancestors(source, tests_sumo)
    option_files = collections.defaultdict(list)
    config_files = collections.defaultdict(list)
    potentials = collections.defaultdict(list)
    for d in chain:  # most specific first
        try:
            names = sorted(os.listdir(d))
        except OSError:
            continue
        for f in names:
            path = join(d, f)
            if f not in potentials or os.path.isdir(path):
                potentials[f].append(path)
            if f.startswith("options."):
                option_files[f[8:]].append(path)
            if f.startswith("config."):
                config_files[f[7:]].append(path)
    return option_files, config_files, potentials


def collate_options(option_files, app="sumo"):
    """Concatenate options.<app> parent-first with extractTest's dedup rules."""
    files = sorted(option_files.get(app, []), key=lambda p: p.count(os.sep))
    app_options = []
    skip = False
    for f in files:
        with open(f, encoding="utf-8", errors="replace") as fh:
            text = fh.read()
        for o in shlex.split(text):
            if skip:
                skip = False
                continue
            if o == "--xml-validation":
                skip = True
                continue
            if o == "{CLEAR}":
                app_options = []
                continue
            if o == "-" and o in app_options:  # never happens, kept for parity
                continue
            if o.startswith("-") and o in app_options:
                idx = app_options.index(o)
                if idx < len(app_options) - 1 and not app_options[idx + 1].startswith("-"):
                    del app_options[idx:idx + 2]
                else:
                    del app_options[idx:idx + 1]
            app_options.append(o)
    return app_options


def parse_file_options(tokens):
    """Split collated tokens into (behaviour tokens, net, routes, additionals, outputs)."""
    sim = []
    net = None
    routes = []
    additions = []
    outputs = []
    i = 0
    while i < len(tokens):
        t = tokens[i]
        matched = None
        for group in ALL_FILE_OPTS:
            if t in group:
                val = tokens[i + 1] if i + 1 < len(tokens) else ""
                i += 2
                matched = (t, group, val)
                break
            hit = next((o for o in group if t.startswith(o + "=")), None)
            if hit is not None:
                val = t[len(hit) + 1:]
                i += 1
                matched = (hit, group, val)
                break
        if matched is None:
            # An empty string value (e.g. '--opt ""' in options.sumo) cannot
            # survive the shell expansion of sim.args.  Fold it into '--opt='
            # so both the ref run and run_all pass an explicit empty value.
            if t == "" and sim and sim[-1].startswith("-") and "=" not in sim[-1]:
                sim[-1] += "="
                i += 1
                continue
            if t in HARNESS_FLAGS or any(t.startswith(f + "=") for f in HARNESS_FLAGS):
                i += 1
                if i < len(tokens) and tokens[i] in ("true", "false", "1", "0"):
                    i += 1
                continue
            sim.append(t)
            i += 1
            continue
        name, group, val = matched
        if group is NET_OPTS:
            net = val
        elif group is ROUTE_OPTS:
            routes.extend(x for x in val.split(",") if x)
        elif group is ADD_OPTS:
            additions.extend(x for x in val.split(",") if x)
        else:
            outputs.append((name, val))
    return sim, net, routes, additions, outputs


def resolve(fname: str, potentials):
    """Resolve a file name against the test dir first, then its parents."""
    fname = fname.strip().strip('"').strip("'")
    if not fname:
        return None
    hits = potentials.get(fname)
    if hits:
        return hits[0]
    base = os.path.basename(fname)
    hits = potentials.get(base)
    if hits:
        return hits[0]
    return None


def copy_inputs(test_dir, potentials, config_files, chosen):
    """Copy every input file named by config copy_test_path or by an option."""
    # Safety: never write into the read-only C++ source tree.
    if os.path.commonpath([realpath(test_dir), realpath(CASES)]) != realpath(CASES):
        raise RuntimeError("refusing to write outside " + CASES + ": " + test_dir)
    copied = set()
    # Only the files actually named by the collated options are inputs we need;
    # copying the full config copy_test_path set duplicates large shared nets
    # into every case.  Option values resolve through `potentials` (test dir
    # first, then parents), so indirect config entries are not required.
    entries = [("copy_test_path", fname) for fname in sorted(set(chosen))]
    for key, val in entries:
        src = resolve(val, potentials)
        if not src or not os.path.exists(src):
            continue
        dst = join(test_dir, os.path.basename(val))
        if os.path.isdir(src):
            if key == "copy_test_path_merge" and os.path.isdir(dst):
                shutil.copytree(src, dst, dirs_exist_ok=True)
            else:
                if os.path.isdir(dst):
                    shutil.rmtree(dst)
                shutil.copytree(src, dst)
        else:
            shutil.copy2(src, dst)
        copied.add(os.path.basename(val))
    return copied


def is_route_file(path: str) -> bool:
    """Cheap check: a route file has a <routes>/<trip> root or .rou/.rou.xml suffix."""
    name = os.path.basename(path).lower()
    if name.endswith(".rou.xml") or name.endswith(".rou") or ".rou." in name:
        return True
    try:
        with open(path, encoding="utf-8", errors="replace") as fh:
            head = fh.read(2048)
        return "<routes" in head or "<trip" in head or "<vehicle" in head
    except OSError:
        return False


def import_one(source, tests_sumo, force=False):
    """Import a single TextTest leaf. Returns (name, info-dict) or (None, skip-reason)."""
    source = realpath(source)
    rel = os.path.relpath(source, realpath(tests_sumo))
    name = "tt_" + rel.replace(os.sep, "_")
    case_dir = join(CASES, name)  # needed by additionals resolution below

    option_files, config_files, potentials = collect_files_and_config(source, tests_sumo)
    tokens = collate_options(option_files, "sumo")
    if not tokens:
        return None, "no options"

    joined = " ".join(tokens)
    for marker in SKIP_MARKERS:
        if marker in joined:
            return None, "needs external tool/GUI/TraCI: " + marker

    # Error-exit tests cannot become sim-output oracle rows: the C++ itself
    # quits with a nonzero exit code (run_all marks any nonzero exit as FAIL).
    for d in ancestors(source, tests_sumo):
        ef = join(d, "errors.sumo")
        if os.path.isfile(ef):
            with open(ef, encoding="utf-8", errors="replace") as fh:
                if "Quitting (on error)" in fh.read():
                    return None, "error-exit test (errors.sumo Quitting on error)"

    sim, net, routes, additions, outputs = parse_file_options(tokens)

    net_path = resolve(net, potentials) if net else None
    if not net_path or not net_path.lower().endswith(".net.xml"):
        return None, "no plain .net.xml input"

    route_path = None
    for r in routes:
        p = resolve(r, potentials)
        if p:
            route_path = p
            break
    kept_additions = []
    for a in additions:
        p = resolve(a, potentials)
        if not p:
            continue
        if route_path is None and is_route_file(p):
            route_path = p
        else:
            # run_all.sh and make_sim_refs.sh both run from the repo root, so
            # additional files must be addressed relative to it (existing cases
            # use "tools/oracle/cases/<case>/<file>" for the same reason).
            kept_additions.append(os.path.relpath(
                join(case_dir, os.path.basename(p)), REPO).replace(os.sep, "/"))
    if not route_path:
        return None, "no route input"

    # File-valued behaviour options (e.g. --weight-files): copy their inputs
    # and rewrite the option value to a repo-relative path, since run_all and
    # make_sim_refs.sh both run from the repo root.
    extra_inputs = []
    def _extra_value(val):
        kept = []
        for v in val.split(","):
            if not v:
                continue
            src = resolve(v, potentials)
            if src:
                extra_inputs.append(os.path.basename(src))
                kept.append(os.path.relpath(
                    join(case_dir, os.path.basename(src)), REPO).replace(os.sep, "/"))
            else:
                kept.append(v)
        return ",".join(kept)
    for i, t in enumerate(list(sim)):
        for o in EXTRA_INPUT_OPTS:
            if t == o and i + 1 < len(sim):
                sim[i + 1] = _extra_value(sim[i + 1])
            elif t.startswith(o + "="):
                sim[i] = o + "=" + _extra_value(t[len(o) + 1:])

    if os.path.exists(case_dir) and not force:
        return name, {"skipped_existing": True, "source": rel}

    chosen = [net, os.path.basename(route_path)]
    chosen += [os.path.basename(resolve(a, potentials)) for a in additions if resolve(a, potentials)]
    chosen += extra_inputs
    chosen += [v for _, v in outputs]
    chosen = [c for c in chosen if c]

    if os.path.isdir(case_dir):
        shutil.rmtree(case_dir)
    os.makedirs(case_dir)

    copy_inputs(case_dir, potentials, config_files, chosen)

    # Normalised, stable inputs for run_all.sh.
    shutil.copy2(net_path, join(case_dir, "ref_simple.net.xml"))
    shutil.copy2(route_path, join(case_dir, "routes.rou"))

    # Drop sub-options of output files the harness does not pass (e.g.
    # --vehroute-output.exit-times): without the parent file the C++ rejects
    # them, and the harness never compares vehroute output.
    sim_args = [t for t in sim if not (
        any(t.startswith(o + ".") for o in OUTPUT_OPTS)
        and not any(t.startswith(p) for p in HARNESS_OUTPUT_PREFIXES))]
    if kept_additions:
        sim_args += ["-a", ",".join(sorted(set(kept_additions)))]

    with open(join(case_dir, "sim.args"), "w", encoding="utf-8", newline="\n") as fh:
        fh.write(" ".join(sim_args) + "\n")
    with open(join(case_dir, "NEW_CASE"), "w", encoding="utf-8", newline="\n") as fh:
        fh.write("Generated by tools/oracle/import_texttest.py from "
                 + rel.replace(os.sep, "/") + "\n")
    with open(join(case_dir, "CASE.md"), "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# Imported TextTest case\n\n")
        fh.write(f"Source: `{rel.replace(os.sep, '/')}` (SUMO 1.28.0 official TextTest suite).\n\n")
        fh.write("Collated options (`options.sumo`, parent first):\n\n")
        fh.write("```\n" + " ".join(tokens) + "\n```\n\n")
        fh.write("`sim.args` keeps the behaviour options; net/route/additional/output files\n")
        fh.write("are copied into this case and the net/route are exposed as\n")
        fh.write("`ref_simple.net.xml` / `routes.rou`. TextTest-specific output files are not\n")
        fh.write("imported; run_all compares tripinfo/fcd/statistics regenerated by the\n")
        fh.write("official sumo via `make_sim_refs.sh`.\n")

    return name, {
        "source": rel,
        "net": os.path.basename(net_path),
        "route": os.path.basename(route_path),
        "additions": sorted(set(kept_additions)),
        "outputs": [n for n, _ in outputs],
        "sim_args": sim_args,
    }


def find_leaves(subtree_root):
    out = []
    for root, dirs, files in os.walk(subtree_root):
        dirs.sort()
        if any(f.startswith("testsuite.") for f in files):
            continue  # suite directory, not a test
        if "options.sumo" in files:
            out.append(root)
    return out


def main(argv=None):
    p = argparse.ArgumentParser(description=__doc__,
                                formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("subtrees", nargs="+",
                   help="subtree under tests/sumo (e.g. basic, cf_model) or an absolute path")
    p.add_argument("--tests-sumo", default=os.environ.get("SUMO_TESTS", DEFAULT_TESTS_SUMO),
                   help="path to tests/sumo (default: %(default)s)")
    p.add_argument("--dry-run", action="store_true", help="list cases without writing")
    p.add_argument("--force", action="store_true", help="overwrite existing cases")
    args = p.parse_args(argv)

    tests_sumo = realpath(args.tests_sumo)
    imported = skipped = 0
    reasons = collections.Counter()
    for subtree in args.subtrees:
        root = subtree if os.path.isabs(subtree) else join(tests_sumo, subtree)
        root = realpath(root)
        if not os.path.isdir(root):
            print(f"ERROR: no such subtree: {root}", file=sys.stderr)
            return 2
        leaves = find_leaves(root)
        print(f"# {os.path.relpath(root, tests_sumo)}: {len(leaves)} leaf tests")
        for leaf in leaves:
            rel = os.path.relpath(leaf, tests_sumo)
            if args.dry_run:
                name = "tt_" + rel.replace(os.sep, "_")
                print("WOULD IMPORT", name)
                imported += 1
                continue
            name, info = import_one(leaf, tests_sumo, args.force)
            if name is None:
                skipped += 1
                reasons[info.split(":")[0]] += 1
            elif info.get("skipped_existing"):
                skipped += 1
                reasons["existing"] += 1
            else:
                imported += 1
                print("IMPORTED", name)
    print(f"# imported={imported} skipped={skipped}")
    for reason, n in reasons.most_common():
        print(f"# skip {reason}: {n}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
