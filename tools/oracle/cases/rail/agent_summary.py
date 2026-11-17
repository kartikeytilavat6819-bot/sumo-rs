import sys
import xml.etree.ElementTree as ET

ti, fcd = sys.argv[1], sys.argv[2]
for t in ET.parse(ti).getroot():
    print(f"  {t.get('id')} depart={t.get('depart')} departDelay={t.get('departDelay')} arrival={t.get('arrival')} departSpeed={t.get('departSpeed')}")
speeds = {}
for step in ET.parse(fcd).getroot():
    time = float(step.get("time"))
    if time > 40:
        continue
    for v in step:
        if v.get("id") == "trainEast":
            speeds[time] = v.get("speed")
print("  trainEast speed t=0..40:", " ".join(f"{int(t)}:{speeds[t]}" for t in sorted(speeds)))
