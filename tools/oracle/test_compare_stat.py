import os
import subprocess
import sys
import tempfile
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
SCRIPT = os.path.join(HERE, "compare_stat.py")

HEAD = '<?xml version="1.0" encoding="UTF-8"?>\n<statistics>\n'
PERF = '<performance clockBegin="1.00" duration="24.00"/>\n'
TRIP = ('<vehicleTripStatistics count="5" routeLength="274.81" speed="16.22" duration="16.80" '
        'waitingTime="0.00" timeLoss="4.38" departDelay="0.00" departDelayWaiting="-1.00" '
        'totalTravelTime="84.00" totalDepartDelay="0.00"/>\n')
PED = '<pedestrianStatistics number="0" routeLength="0.00" duration="0.00" timeLoss="0.00"/>\n'
TAIL = '</statistics>\n'


def doc(trip=TRIP, perf=PERF):
    return HEAD + perf + '<vehicles loaded="5" inserted="5" running="0" waiting="0"/>\n' + trip + PED + TAIL


class CompareStatTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)

    def write(self, name, text):
        path = os.path.join(self.tmp.name, name)
        with open(path, "w", encoding="utf-8") as f:
            f.write(text)
        return path

    def run_cmp(self, ref_text, cand_text):
        ref = self.write("ref.xml", ref_text)
        cand = self.write("cand.xml", cand_text)
        return subprocess.run([sys.executable, SCRIPT, ref, cand],
                              capture_output=True, text=True)

    def test_identical_exits_0(self):
        r = self.run_cmp(doc(), doc())
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)
        self.assertIn("SUMMARY: 0 differences", r.stdout)

    def test_performance_is_ignored(self):
        r = self.run_cmp(doc(perf=PERF), doc(perf='<performance clockBegin="999.00" duration="1.00"/>\n'))
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)

    def test_within_tolerance_is_identical(self):
        cand = TRIP.replace('speed="16.22"', 'speed="16.2200000000001"')
        r = self.run_cmp(doc(), doc(trip=cand))
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)

    def test_changed_attribute_exits_1_and_names_it(self):
        cand = TRIP.replace('timeLoss="4.38"', 'timeLoss="4.5"')
        r = self.run_cmp(doc(), doc(trip=cand))
        self.assertEqual(r.returncode, 1, r.stdout + r.stderr)
        self.assertIn("vehicleTripStatistics", r.stdout)
        self.assertIn("timeLoss", r.stdout)
        self.assertIn("ref='4.38'", r.stdout)
        self.assertIn("cand='4.5'", r.stdout)
        self.assertIn("SUMMARY: 1 differences in 1 groups", r.stdout)

    def test_missing_element_exits_1(self):
        cand = HEAD + PERF + '<vehicles loaded="5" inserted="5" running="0" waiting="0"/>\n' + PED + TAIL
        r = self.run_cmp(doc(), cand)
        self.assertEqual(r.returncode, 1, r.stdout + r.stderr)
        self.assertIn("vehicleTripStatistics", r.stdout)

    def test_malformed_xml_exits_2(self):
        r = self.run_cmp(doc(), "<statistics><vehicles")
        self.assertEqual(r.returncode, 2, r.stdout + r.stderr)
        self.assertIn("parse error", r.stderr)

    def test_wrong_root_exits_2(self):
        r = self.run_cmp(doc(), "<net></net>\n")
        self.assertEqual(r.returncode, 2, r.stdout + r.stderr)
        self.assertIn("expected <statistics>", r.stderr)


if __name__ == "__main__":
    unittest.main()
