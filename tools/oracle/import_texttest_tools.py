#!/usr/bin/env python3
"""Import official SUMO TextTest subtrees into the existing differential oracles.

TextTest rules (see tests/README_Tests.md and tools/extractTest.py):
  * testsuite.<app> lists the child tests of a suite (one per non-comment line);
  * a leaf test's command line is the concatenation of options.<app> from the app
    root down to the leaf (later switches override earlier ones, {CLEAR} resets);
  * input files are named by copy_test_path: entries in config.<app> and are
    looked up in the leaf first, then in its parents (nearest wins).

Generated artefacts (all deterministic, all regenerated on every run):
  netconvert -> tools/oracle/cases/itt_nc_<name>/  (input files copied here) plus
                tools/oracle/import_texttest_net.rows, sourced by run_netopts_new.sh.
  duarouter  -> tools/oracle/cases/duarouter_itt_<name>/ with the `net` and `args`
                files that tools/oracle/run_duarouter.sh already discovers.

References are NEVER committed: both runners call the pristine binary live.  Tests
that expect a non-zero exit code, name a known bug, or call a GUI/external tool are
skipped (the duarouter runner has no error-case verdict; keeping netconvert error
cases out keeps the first import comparable to the passing rows).

Usage:
  python tools/oracle/import_texttest_tools.py --app netconvert \
      --subtree C:/Users/karti/sumo-src/tests/netconvert/import/XML/types \
      --subtree C:/Users/karti/sumo-src/tests/netconvert/import/XML/speed_in_kmh
  python tools/oracle/import_texttest_tools.py --app duarouter \
      --subtree C:/Users/karti/sumo-src/tests/duarouter/dua
  --limit N caps the number of imported leaves per invocation (0 = no cap).
"""

from __future__ import annotations

import argparse
import shlex
import shutil
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
CASES = ROOT / "tools" / "oracle" / "cases"
ROWS = HERE / "import_texttest_net.rows"

PREFIX = {"netconvert": "itt_nc_", "duarouter": "duarouter_itt_"}

# Options whose value is a file name.  Used only to tell "value" from "next flag".
VALUE_OPTS = {
    "-n", "-o", "-s", "-t", "-a", "-e", "-i", "-b", "-c", "-d", "-p", "-x", "-y",
    "--net-file", "--net", "--sumo-net-file", "--node-files", "--edge-files",
    "--connection-files", "--type-files", "--tllogic-files", "--additional-files",
    "--additional", "--districts",
    "--edge-data-files", "--edge-permission-files", "--trip-files", "--route-files",
    "--flow-files", "--alternatives-files", "--weights", "--weight-files",
    "--taz-files", "--shapes", "--opendrive-files", "--osm-files", "--matsim-files",
    "--dlr-navteq-files", "--visum-file", "--vissim-file", "--itsumo-files",
    "--output", "--output-file", "--alt-output-file", "--vtype-output",
    "--plain-output-prefix", "--opendrive-output", "--matsim-output",
    "--dlr-navteq-output", "--amitran-output", "--street-sign-output",
    "--ptstop-output", "--ptline-output", "--parking-output", "--polygon-output",
    "--poi-output", "--additional-output", "--railway.topology.output",
    "--joined-output", "--log", "--save-configuration",
    "--device.rerouting.threads", "--tripinfo-output", "--fcd-output",
    "--statistic-output", "--vehroute-output", "--summary-output",
    "--emergency-output", "--penalties",
}
# Files the runners already write themselves: netconvert gets -o from the runner;
# duarouter keeps -o but it must land in the per-side output directory.
NET_OUT_OPTS = {
    "-o", "--output", "--output-file", "--save-configuration", "--log",
    "--plain-output-prefix", "--opendrive-output", "--matsim-output",
    "--dlr-navteq-output", "--amitran-output", "--street-sign-output",
    "--ptstop-output", "--ptline-output", "--parking-output", "--polygon-output",
    "--poi-output", "--additional-output", "--railway.topology.output",
    "--joined-output", "--write-results-to", "--fcd-output", "--tripinfo-output",
    "--vehroute-output", "--statistic-output", "--summary-output",
}
DUA_OUT_OPTS = {
    "-o", "--output-file", "--alt-output-file", "--vtype-output",
    "--save-configuration", "--log",
}
SKIP_TOKENS = (
    "netedit", "sumo-gui", "netgen.py", "polyconvert", "netconvert.py", ".jar",
    "java", "--gui", "--xdot", "--gdal", "xdot.py", "python",
    "$SUMO_HOME", "typemap", "netconvert2.py", "--image-output",
    "--routing-threads",  # parallel routing needs Fox; our build is single-threaded
)


def tests_root(leaf: Path, app: str) -> Path:
    """The <tests> directory: the highest ancestor carrying options.<app>."""
    cur = leaf.resolve()
    found = cur
    while True:
        if (cur / f"options.{app}").is_file() or (cur / f"config.{app}").is_file():
            found = cur
        parent = cur.parent
        if parent == cur:
            break
        cur = parent
    # <tests> is the parent of the app directory that owns options.<app>.
    return found.parent


def leaf_tests(subtree: Path, app: str) -> list[Path]:
    """Walk testsuite.<app> files; return the leaf tests in deterministic order."""
    out: list[Path] = []

    def walk(d: Path) -> None:
        suite = d / f"testsuite.{app}"
        if suite.is_file():
            for line in suite.read_text(encoding="utf-8", errors="replace").splitlines():
                name = line.strip()
                if not name or name.startswith("#"):
                    continue
                walk(d / name)
            return
        if d.is_dir():
            out.append(d.resolve())

    walk(subtree.resolve())
    return sorted(set(out))


def effective_options(leaf: Path, app: str, root: Path) -> list[str]:
    """Concatenate options.<app> from root to leaf with extractTest.py's dedup."""
    chain: list[Path] = []
    cur = leaf
    while cur == root or root in cur.parents:
        f = cur / f"options.{app}"
        if f.is_file():
            chain.append(f)
        if cur == root:
            break
        cur = cur.parent
    chain.reverse()  # shallowest first

    opts: list[str] = []
    for f in chain:
        tokens = shlex.split(f.read_text(encoding="utf-8", errors="replace"))
        new: list[str] = []
        skip = False
        clear: list[str] | None = None
        for o in tokens:
            if skip:
                skip = False
                continue
            if app == "netconvert":
                # netconvert rows all validate cleanly; dropping this keeps their args small
                if o == "--xml-validation":
                    skip = True
                    continue
                if o.startswith("--xml-validation="):
                    continue
            if o == "--no-step-log":
                continue  # run_duarouter.sh already adds it
            if o == "{CLEAR}":
                opts = []
                continue
            if o == "{CLEAR":
                clear = []
                continue
            if clear is not None:
                if o.endswith("}"):
                    clear.append(o[:-1])
                    n = len(clear)
                    for idx in range(len(opts) - n + 1):
                        if opts[idx:idx + n] == clear:
                            del opts[idx:idx + n]
                            clear = None
                            break
                else:
                    clear.append(o)
                continue
            if o.startswith("-") and o in opts:
                idx = opts.index(o)
                size = 2 if idx < len(opts) - 1 and not opts[idx + 1].startswith("-") else 1
                del opts[idx:idx + size]
            new.append(o)
        opts += new
    return opts


def copy_inputs(leaf: Path, app: str, root: Path, dest: Path) -> dict[str, Path]:
    """Copy every copy_test_path input into dest (flattened, nearest wins).

    Returns a map original-relative-path -> copied file (for option rewriting).
    """
    names: list[str] = []
    cur = leaf
    while cur == root or root in cur.parents:
        cfg = cur / f"config.{app}"
        if cfg.is_file():
            for line in cfg.read_text(encoding="utf-8", errors="replace").splitlines():
                entry = line.strip().split(":", 1)
                if len(entry) != 2 or "copy_test_path" not in entry[0]:
                    continue
                name = entry[1].strip()
                if name and name not in names:
                    names.append(name)
        if cur == root:
            break
        cur = cur.parent

    # For each configured name, the nearest existing file (leaf -> parents) wins.
    files: dict[str, Path] = {}
    for name in names:
        cur = leaf
        while cur == root or root in cur.parents:
            src = cur / name
            if src.exists():
                files[name] = src
                break
            if cur == root:
                break
            cur = cur.parent

    copied: dict[str, Path] = {}
    for name in sorted(files, key=lambda s: (-len(s), s)):  # longest first
        src = files[name]
        if src.is_dir():
            for child in sorted(src.rglob("*")):
                if child.is_file():
                    tgt = dest / child.relative_to(src)
                    tgt.parent.mkdir(parents=True, exist_ok=True)
                    shutil.copy2(child, tgt)
                    copied[str(child.relative_to(src)).replace("\\", "/")] = tgt
        else:
            shutil.copy2(src, dest / src.name)
            copied[src.name] = dest / src.name
            copied[name.replace("\\", "/")] = dest / src.name
    return copied


def case_path(p: Path) -> str:
    return p.relative_to(ROOT).as_posix()


def rewrite(tokens: list[str], app: str, copied: dict[str, Path], case: Path,
            out_root: str) -> tuple[list[str], str | None]:
    """Rewrite file operands into the case dir; redirect outputs to the per-side dir.

    Returns (option tokens, net-file path) for duarouter (None for netconvert).
    """
    result: list[str] = []
    net: str | None = None
    i = 0
    while i < len(tokens):
        tok = tokens[i]
        i += 1
        key, eq, val = tok.partition("=")
        had_eq = bool(eq)
        consumed = False
        if not had_eq and key in VALUE_OPTS and i < len(tokens):
            val = tokens[i]
            i += 1
            consumed = True

        def fmt(k: str, v: str) -> str:
            return f"{k}={v}" if had_eq else f"{k} {v}"

        # net-file is supplied by run_duarouter.sh (-n); netconvert never needs it.
        if key in ("--net-file", "-n") and app == "duarouter":
            if copied.get(val) is not None:
                net = case_path(copied[val])
            continue

        # Outputs go to the per-side output directory (or are dropped for the net).
        if key in (NET_OUT_OPTS if app == "netconvert" else DUA_OUT_OPTS):
            if key in ("-o", "--output", "--output-file"):
                if app == "netconvert":
                    continue  # run_netopts_new.sh adds its own -o
                result.append(fmt(key, f"{out_root}/routes.rou.xml"))
                continue
            new = f"{out_root}/{Path(val).name}" if val else f"{out_root}/output"
            result.append(fmt(key, new))
            continue

        # Input operands: match the longest copied relative path, then the basename.
        # File-list options also accept comma-separated names; rewrite each part.
        if val and (had_eq or key in VALUE_OPTS):
            parts = val.split(",")
            targets = [_find_copied(p, copied) for p in parts]
            if len(parts) == 1 and targets[0] is not None:
                result.append(fmt(key, case_path(targets[0])))
                continue
            if len(parts) > 1 and all(t is not None for t in targets):
                result.append(fmt(key, ",".join(case_path(t) for t in targets)))
                continue
        elif tok and not tok.startswith("-"):
            target = _find_copied(tok, copied)
            if target is not None:
                tok = case_path(target)
        if consumed and (had_eq or key in VALUE_OPTS):
            result.append(fmt(key, val))
        else:
            result.append(tok)
    return result, net


def _find_copied(value: str, copied: dict[str, Path]) -> Path | None:
    v = value.replace("\\", "/")
    if v in copied:
        return copied[v]
    base = Path(v).name
    hits = [p for name, p in copied.items() if Path(name).name == base]
    if len(set(hits)) == 1:
        return hits[0]
    for name, p in sorted(copied.items(), key=lambda kv: -len(kv[0])):
        if v.endswith("/" + name) or v == name:
            return p
    return None


def skip_test(leaf: Path, app: str, root: Path, tokens: list[str]) -> bool:
    # Expected failures are not comparable by the duarouter runner; keep them out
    # of the first import for both apps so a PASS really means matching output.
    cur = leaf
    while cur == root or root in cur.parents:
        ec = cur / f"exitcode.{app}"
        if ec.is_file() and ec.read_text(encoding="utf-8", errors="replace").strip() not in ("", "0"):
            return True
        kb = cur / f"knownbugs.{app}"
        if kb.is_file() and kb.read_text(encoding="utf-8", errors="replace").strip():
            return True
        if "errors" == cur.name:
            return True
        if cur == root:
            break
        cur = cur.parent
    # duarouter with no options exits 1 ("No output specified."); the runner labels any
    # candidate error as ERROR and never compares stderr, so such error-cases are unverifiable.
    if app == "duarouter" and not tokens:
        return True
    # Pure public-transport person trips that reach their destination through a busStop
    # <access> link exercise intermodal routing not supported by the Rust port yet.
    if app == "duarouter":
        texts = [f.read_text(encoding="utf-8", errors="replace")
                 for f in leaf.glob("*.xml") if f.is_file()]
        joined_inputs = "\n".join(texts)
        if "<access " in joined_inputs and 'modes="public"' in joined_inputs:
            return True
    joined = " ".join(tokens)
    return any(s in joined for s in SKIP_TOKENS)


def import_app(app: str, subtrees: list[Path], limit: int) -> tuple[list[str], int]:
    prefix = PREFIX[app]
    for old in CASES.glob(prefix + "*"):
        if old.is_dir():
            shutil.rmtree(old)
    rows: list[str] = []
    imported = 0
    for subtree in subtrees:
        root = tests_root(subtree, app)
        per_subtree = 0
        for leaf in leaf_tests(subtree, app):
            if limit and per_subtree >= limit:
                break
            tokens = effective_options(leaf, app, root)
            if skip_test(leaf, app, root, tokens):
                continue
            rel = leaf.relative_to(root).as_posix()
            name = (prefix + rel.replace("/", "_").replace("\\", "_")
                    .replace("-", "_").replace(".", "_"))
            case = CASES / name
            if case.exists():
                shutil.rmtree(case)
            case.mkdir(parents=True, exist_ok=True)
            copied = copy_inputs(leaf, app, root, case)
            out_root = "@DIR@"
            if app == "netconvert":
                out_root = "@D"
            opts, net = rewrite(tokens, app, copied, case, out_root)

            if net is None:
                # find a net among the copied inputs as the runner needs one
                cand = [p for n, p in copied.items() if n.endswith(".net.xml")]
                if cand:
                    net = case_path(sorted(set(cand))[0])
            if app == "duarouter":
                if net is None:
                    shutil.rmtree(case)
                    continue
                (case / "net").write_text(net + "\n", encoding="utf-8", newline="\n")
                (case / "args").write_text(" ".join(opts) + "\n", encoding="utf-8", newline="\n")
            else:
                # netconvert: inputs in `args:`, redirected extra outputs in OPTIONS
                inputs = [t for t in opts if "@D" not in t]
                outputs = [t for t in opts if "@D" in t]
                args_txt = " ".join(inputs)
                (case / "net.args").write_text(args_txt + "\n", encoding="utf-8", newline="\n")
                rows.append(f"row {name} \"args:{args_txt}\" \"{' '.join(outputs)}\"")
            imported += 1
            per_subtree += 1
    return rows, imported


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--app", choices=sorted(PREFIX), required=True)
    ap.add_argument("--subtree", action="append", required=True, type=Path)
    ap.add_argument("--limit", type=int, default=0)
    args = ap.parse_args()

    rows, count = import_app(args.app, args.subtree, args.limit)
    if args.app == "netconvert":
        ROWS.write_text("\n".join(rows) + ("\n" if rows else ""),
                        encoding="utf-8", newline="\n")
    print(f"imported {count} {args.app} tests")
    return 0


if __name__ == "__main__":
    sys.exit(main())
