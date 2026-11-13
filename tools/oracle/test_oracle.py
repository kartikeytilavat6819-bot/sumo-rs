"""Tests for compare_net.py and compare_sim.py. Stdlib unittest only.

Run: python -m unittest discover -s tools/oracle -p 'test_*.py'
"""

import io
import os
import sys
import tempfile
import unittest
from contextlib import redirect_stderr, redirect_stdout
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))

import compare_net  # noqa: E402
import compare_sim  # noqa: E402


def write_pair(ref_xml, cand_xml):
    d = tempfile.TemporaryDirectory()
    ref = os.path.join(d.name, "ref.xml")
    cand = os.path.join(d.name, "cand.xml")
    for path, text in ((ref, ref_xml), (cand, cand_xml)):
        with open(path, "w", encoding="utf-8") as f:
            f.write(text)
    return d, ref, cand


def run_net(ref_xml, cand_xml):
    d, ref, cand = write_pair(ref_xml, cand_xml)
    with d:
        out, err = io.StringIO(), io.StringIO()
        with patch.object(sys, "argv", ["compare_net.py", ref, cand]), \
                redirect_stdout(out), redirect_stderr(err):
            try:
                rc = compare_net.main()
            except SystemExit as e:
                rc = e.code
    return rc, out.getvalue()


def run_sim(mode, ref_xml, cand_xml):
    d, ref, cand = write_pair(ref_xml, cand_xml)
    with d:
        out = io.StringIO()
        with redirect_stdout(out):
            rc = compare_sim.main(["--" + mode, ref, cand])
    return rc, out.getvalue()


NET = """<net>
  <edge id="e1" from="a" to="b" numLanes="1" speed="13.89">
    <lane id="e1_0" index="0" speed="13.89" length="100.0"/>
  </edge>
</net>
"""

TWO_EDGE_NET = """<net>
  <edge id="e1" from="a" to="b" numLanes="1" speed="13.89"/>
  <edge id="e2" from="b" to="c" numLanes="1" speed="13.89"/>
</net>
"""

ONE_EDGE_NET = """<net>
  <edge id="e1" from="a" to="b" numLanes="1" speed="13.89"/>
</net>
"""


def tripinfo(ids):
    rows = "\n".join(
        f'  <tripinfo id="{v}" depart="0.0" departLane="e1_0" departSpeed="0.0" arrival="10.0" '
        f'arrivalLane="e1_0" arrivalSpeed="13.89" duration="10.0" routeLength="100.0" '
        f'waitingTime="0.0" timeLoss="0.0"/>'
        for v in ids)
    return f"<tripinfos>\n{rows}\n</tripinfos>\n"


def fcd(steps):
    rows = []
    for t, x, speed in steps:
        rows.append(
            f'  <timestep time="{t}">\n'
            f'    <vehicle id="v1" x="{x}" y="0.0" angle="0.0" speed="{speed}" lane="e1_0" pos="{x}"/>\n'
            f'  </timestep>')
    return "<fcd_export>\n" + "\n".join(rows) + "\n</fcd_export>\n"


class TestCompareNet(unittest.TestCase):
    def test_identical(self):
        rc, out = run_net(NET, NET)
        self.assertEqual(rc, 0)
        self.assertIn("0 differences", out)

    def test_float_within_tolerance(self):
        rc, _ = run_net(NET, NET.replace('speed="13.89"', 'speed="13.8900005"', 1))
        self.assertEqual(rc, 0)

    def test_float_outside_tolerance_names_attribute(self):
        rc, out = run_net(NET, NET.replace('speed="13.89"', 'speed="13.9"', 1))
        self.assertEqual(rc, 1)
        self.assertIn("speed", out)
        self.assertIn("e1", out)

    def test_lane_speed_changed(self):
        rc, out = run_net(NET, NET.replace('speed="13.89" length', 'speed="10.0" length'))
        self.assertEqual(rc, 1)
        self.assertIn("e1_0", out)
        self.assertIn("speed", out)

    def test_missing_edge(self):
        rc, out = run_net(TWO_EDGE_NET, ONE_EDGE_NET)
        self.assertEqual(rc, 1)
        self.assertIn("e2", out)

    def test_malformed_xml(self):
        rc, _ = run_net("<net><edge id='e1'>", NET)
        self.assertEqual(rc, 2)


class TestCompareSim(unittest.TestCase):
    def test_identical_tripinfo(self):
        rc, out = run_sim("tripinfo", tripinfo(["v1"]), tripinfo(["v1"]))
        self.assertEqual(rc, 0)
        self.assertIn("MATCH", out)

    def test_duration_changed_names_vehicle(self):
        rc, out = run_sim("tripinfo", tripinfo(["v1"]),
                          tripinfo(["v1"]).replace('duration="10.0"', 'duration="11.5"'))
        self.assertEqual(rc, 1)
        self.assertIn("v1", out)
        self.assertIn("duration", out)

    def test_vehicle_only_in_reference(self):
        rc, out = run_sim("tripinfo", tripinfo(["v1", "v2"]), tripinfo(["v1"]))
        self.assertEqual(rc, 1)
        self.assertIn("v2", out)
        self.assertIn("missing", out)

    def test_fcd_first_diverging_timestep(self):
        ref_steps = [(t, float(t - 1), 1.0) for t in range(1, 7)]
        cand_steps = [(t, 99.0 if t == 5 else float(t - 1), 99.0 if t == 6 else 1.0)
                      for t in range(1, 7)]
        rc, out = run_sim("fcd", fcd(ref_steps), fcd(cand_steps))
        self.assertEqual(rc, 1)
        self.assertIn("t=5", out)
        self.assertNotIn("t=6", out)


if __name__ == "__main__":
    unittest.main()
