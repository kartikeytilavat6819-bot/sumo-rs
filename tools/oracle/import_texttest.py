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
ADD_OPTS = ("-a", "--additional-files", "--additional", "--a")
# Input files named by a behaviour option rather than -n/-r/-a.  These stay in
# sim.args but their value must be rewritten to the copied case-relative path.
STATE_OPTS = ("--device.fcd-replay.file",)
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
ALL_FILE_OPTS = (NET_OPTS, ROUTE_OPTS, ADD_OPTS, STATE_OPTS, OUTPUT_OPTS)

# run_all.sh / make_sim_refs.sh always pass these; a duplicate is a hard error.
HARNESS_FLAGS = ("--no-step-log",)

# A token containing any of these means the test cannot run headless with sumo.
SKIP_MARKERS = (
    ".py", "--python-script", "--traci", "--remote-port", "--gui",
    "--osg-view", "--netedit", "--duarouter", "--jtrrouter", "--marouter",
    "--od2trips", "--activitygen", "--polyconvert", "--java", "--jar",
    "--fmi", "--parquet", "--vtk", "--p2p", "--sumo-gui", "--gcf",
    "--saved-state.input", "--load-state", "--sumo-state",
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
    state_files = []
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
        elif group is STATE_OPTS:
            state_files.append((name, val))
        else:
            outputs.append((name, val))
    return sim, net, routes, additions, state_files, outputs


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
    case_dir = join(CASES, name)

    option_files, config_files, potentials = collect_files_and_config(source, tests_sumo)
    tokens = collate_options(option_files, "sumo")
    if not tokens:
        return None, "no options"

    joined = " ".join(tokens)
    for marker in SKIP_MARKERS:
        if marker in joined:
            return None, "needs external tool/GUI/TraCI: " + marker

    sim, net, routes, additions, state_files, outputs = parse_file_options(tokens)

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
        if route_path is None and is_route_file(p) and not state_files:
            route_path = p
        else:
            # run_all.sh and make_sim_refs.sh both run from the repo root, so
            # additional files must be addressed relative to it (existing cases
            # use "tools/oracle/cases/<case>/<file>" for the same reason).
            kept_additions.append(os.path.relpath(
                join(case_dir, os.path.basename(p)), REPO).replace(os.sep, "/"))
    if not route_path and not state_files:
        return None, "no route input"

    if os.path.exists(case_dir) and not force:
        return name, {"skipped_existing": True, "source": rel}

    kept_state = []
    for opt_name, val in state_files:
        p = resolve(val, potentials)
        if not p:
            continue
        kept_state.append((opt_name, os.path.relpath(
            join(case_dir, os.path.basename(p)), REPO).replace(os.sep, "/")))

    chosen = [net] + ([os.path.basename(route_path)] if route_path else [])
    chosen += [os.path.basename(resolve(a, potentials)) for a in additions if resolve(a, potentials)]
    chosen += [os.path.basename(resolve(val, potentials)) for _, val in state_files
               if resolve(val, potentials)]
    chosen += [v for _, v in outputs]
    chosen = [c for c in chosen if c]

    if os.path.isdir(case_dir):
        shutil.rmtree(case_dir)
    os.makedirs(case_dir)

    copy_inputs(case_dir, potentials, config_files, chosen)

    # Normalised, stable inputs for run_all.sh.
    shutil.copy2(net_path, join(case_dir, "ref_simple.net.xml"))
    if route_path:
        shutil.copy2(route_path, join(case_dir, "routes.rou"))
    else:
        # FCD-replay builds its vehicles from the state file, not from -r.
        with open(join(case_dir, "routes.rou"), "w", encoding="utf-8", newline="\n") as fh:
            fh.write("<routes/>\n")

    sim_args = list(sim)
    if kept_additions:
        sim_args += ["-a", ",".join(sorted(set(kept_additions)))]
    for opt_name, path in kept_state:
        sim_args += [opt_name, path]

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
        "route": os.path.basename(route_path) if route_path else "(empty)",
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
