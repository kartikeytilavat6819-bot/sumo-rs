#!/usr/bin/env python3
"""Manage TextTest rows on which the official SUMO exits non-zero (expected-error tests).

Three modes:

  python park_tt_errors.py <case>...          legacy: park a case (kept for provenance)
  python park_tt_errors.py --unpark <case>...  generate ref.errors + EXPECT_EXIT from the
      official TextTest expectation (errors.sumo / output.sumo at the case's Source: path)
      and remove the PENDING marker so run_all.sh scores the case.
  python park_tt_errors.py --compare <ref.errors> <our-log>
      compare normalised error lines; exit 0 on match, 1 on first difference.

The expectation is never derived from our binary: ref.errors holds the verbatim official
error/warning lines; normalisation (times, paths, SUMO banners, Rust panic noise) is applied
identically to both sides by --compare.
"""
import os
import re
import sys

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
CASES = os.path.join(REPO, "tools", "oracle", "cases")
DEFAULT_TESTS_SUMO = r"C:\Users\karti\sumo-src\release-1.28.0\tests\sumo"
PARK_REASON = ("PARKED: official SUMO 1.28.0 exits non-zero for this TextTest row "
               "(it validates an erroneous additional-file attribute and checks the error message).\n"
               "The run_all harness compares tripinfo/fcd/statistic outputs only, so this row cannot be\n"
               "scored meaningfully. Parked rather than deleted to keep the import provenance.\n")
PENDING_TEXT = "error-message test: official SUMO exits non-zero; harness compares sim outputs only\n"


def source_of(case):
    path = os.path.join(CASES, case, "CASE.md")
    if not os.path.isfile(path):
        return None
    with open(path, encoding="utf-8", errors="replace") as fh:
        for line in fh:
            m = re.match(r"Source: `([^`]+)`", line)
            if m:
                return m.group(1).strip()
    return None


def park(case):
    d = os.path.join(CASES, case)
    with open(os.path.join(d, "CASE.md"), "a", encoding="utf-8", newline="\n") as fh:
        fh.write("\n" + PARK_REASON)
    with open(os.path.join(d, "PENDING"), "w", encoding="utf-8", newline="\n") as fh:
        fh.write(PENDING_TEXT)
    for f in ("NEW_CASE", "ref.tripinfo.xml", "ref.fcd.xml", "ref.stat.xml"):
        try:
            os.remove(os.path.join(d, f))
        except OSError:
            pass
    print("PARKED", case)


def unpark(case, tests_sumo):
    d = os.path.join(CASES, case)
    src = source_of(case)
    if not src:
        print("UNPARK SKIP (no Source:):", case)
        return False
    srdir = os.path.join(tests_sumo, src.replace("/", os.sep))
    # texttest compares the two streams separately, each in order: errors.sumo against
    # stderr, output.sumo against stdout. Keep them in separate reference files.
    expectation = {}
    for name in ("errors.sumo", "output.sumo"):
        p = os.path.join(srdir, name)
        with open(p, encoding="utf-8", errors="replace") as fh:
            expectation[name] = fh.read().splitlines()
    def _write(fname, lines):
        with open(os.path.join(d, fname), "w", encoding="utf-8", newline="\n") as fh:
            for ln in lines:
                fh.write(ln.rstrip("\r") + "\n")
    # always write both files: an empty official stream is still an expectation (our
    # corresponding stream must be empty too), so the harness compares both unconditionally.
    _write("ref.errors", expectation["errors.sumo"])
    _write("ref.output", expectation["output.sumo"])
    with open(os.path.join(d, "EXPECT_EXIT"), "w", encoding="utf-8", newline="\n") as fh:
        fh.write("official SUMO 1.28.0 exits non-zero (expected-error TextTest row)\n")
        fh.write("Official expectation: " + src.replace(os.sep, "/") + "/{errors,output}.sumo\n")
    try:
        os.remove(os.path.join(d, "PENDING"))
    except OSError:
        pass
    print("UNPARKED", case, "(%d stderr, %d stdout official lines)" %
          (len(expectation["errors.sumo"]), len(expectation["output.sumo"])))
    return True


def normalise(line):
    """Apply the same texttest normalisation to the official text and to our output."""
    s = line.rstrip("\r\n")
    if not s.strip():
        return None
    # local-only / Rust-only noise that the official binary never emits
    if s.startswith("Warning: Environment variable SUMO_HOME is not set properly"):
        return None
    if s.startswith("thread '") and " panicked at " in s:
        return None
    if s.startswith("note: run with `RUST_BACKTRACE=1`"):
        return None
    if s.startswith("Eclipse SUMO ") or s.startswith("Build features"):
        return None
    # times (texttest config_all: [0-9.]+ms / spent Xs / Duration Xs)
    s = re.sub(r"[0-9.]+ms", "(TIME)", s)
    s = re.sub(r"spent [0-9.]+s", "spent (TIME)", s)
    s = re.sub(r"Duration [0-9.]+s", "Duration (TIME)", s)
    # version-dependent banner (texttest config.sumo: Simulation.*started with)
    s = re.sub(r"Simulation.*?started with", "Simulation started with", s)
    # texttest error normalisations (config.sumo / config_all)
    s = re.sub(r"Cannot find local schema '[^']*'", "Cannot find local schema", s)
    s = re.sub(r"Cannot read local schema '[^']*'", "Cannot find local schema", s)
    s = re.sub(r"primary document entity.*", "primary document entity <path_to_file>", s)
    s = s.replace("(Datei oder Verzeichnis nicht gefunden)", "(No such file or directory)")
    # paths: a quoted path (slash + dot) becomes its basename, so 'tools/oracle/cases/.../x.net.xml'
    # matches the official 'x.net.xml'; quoted edge/lane ids such as '4/1to3/1' are left alone.
    def shorten(m):
        q = m.group(1)
        if "/" in q and "." in q:
            return "'" + q.replace("\\", "/").rstrip("/").split("/")[-1] + "'"
        return m.group(0)
    s = re.sub(r"'([^']*)'", shorten, s)
    # the harness rewrites --device.ssm.filter-edges.input-file to a repo-relative path so it is
    # findable from the repo root; TextTest ran it as a bare filename. Compare by basename there.
    s = re.sub(r"(SSM device edge filter \()([^)]*)(\))",
               lambda m: m.group(1) + m.group(2).replace("\\", "/").rstrip("/").split("/")[-1] + m.group(3), s)
    # the harness renames inputs (net.net.xml -> ref_simple.net.xml, routes -> routes.rou),
    # so the file named by a "Loading ... from '...'" line is a path, not a semantic value.
    s = re.sub(r"(Loading [^']*? from )'[^']*'", r"\1'(FILE)'", s)
    return s


def normalised_lines(path):
    # keep file order: texttest diffs the normalised lines in order, so a wrong
    # message order is a real difference and must not be masked by sorting.
    with open(path, encoding="utf-8", errors="replace") as fh:
        return [x for x in (normalise(ln) for ln in fh) if x is not None]


def compare(ref, log):
    want = normalised_lines(ref)
    got = normalised_lines(log)
    if want == got:
        return 0
    # report the first difference for run_all.sh's FAIL detail
    for i in range(max(len(want), len(got))):
        a = want[i] if i < len(want) else "<none>"
        b = got[i] if i < len(got) else "<none>"
        if a != b:
            print("ref: " + a[:150])
            print("got: " + b[:150])
            break
    return 1


def main(argv):
    if argv and argv[0] == "--compare":
        if len(argv) != 3:
            print("usage: park_tt_errors.py --compare <ref.errors> <our-log>", file=sys.stderr)
            return 2
        return compare(argv[1], argv[2])
    if argv and argv[0] == "--unpark":
        tests_sumo = os.path.realpath(os.environ.get("SUMO_TESTS", DEFAULT_TESTS_SUMO))
        ok = True
        for case in argv[1:]:
            ok = unpark(case, tests_sumo) and ok
        return 0 if ok else 2
    if not argv:
        print(__doc__)
        return 2
    for case in argv:
        if case.startswith("-"):
            print("unknown option: " + case, file=sys.stderr)
            return 2
        park(case)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
