from pathlib import Path
cases = Path("tools/oracle/cases")
roots = {
    "ref.tripinfo.xml": "tripinfos",
    "ref.fcd.xml": "fcd-export",
    "ref.stat.xml": "statistics",
}
n = 0
for d in sorted(cases.glob("tt_errors_*")):
    if not (d / "EXPECT_EXIT").exists():
        continue
    for name, root in roots.items():
        p = d / name
        if not p.exists():
            p.write_text(f'<?xml version="1.0" encoding="UTF-8"?>\n<{root}/>\n', newline="")
    n += 1
print("added refs to", n, "error cases")
