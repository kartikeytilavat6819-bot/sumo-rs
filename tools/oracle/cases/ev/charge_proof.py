"""Print battery level of each vehicle over time and the charging-station totals.

Usage: python charge_proof.py BATTERY.xml [CHARGINGSTATIONS.xml]
Charging is proven when actualBatteryCapacity of ev_charge rises while chargingStationId is set.
"""
import sys
import xml.etree.ElementTree as ET


def battery_rows(path):
    rows = []
    for ts in ET.parse(path).getroot().iter("timestep"):
        for v in ts.iter("vehicle"):
            rows.append((float(ts.get("time")), v.get("id"), float(v.get("actualBatteryCapacity")),
                         v.get("chargingStationId"), float(v.get("energyCharged"))))
    return rows


def main():
    rows = battery_rows(sys.argv[1])
    charge = [r for r in rows if r[1] == "ev_charge"]
    stop = [r for r in charge if r[3] not in (None, "NULL")]
    gain = (charge[-1][2] - charge[0][2]) if charge else 0.0
    print(f"ev_charge samples={len(charge)} at_station_samples={len(stop)}")
    if stop:
        print(f"ev_charge first_at_station t={stop[0][0]:.2f} level={stop[0][2]:.2f}")
        print(f"ev_charge last_at_station  t={stop[-1][0]:.2f} level={stop[-1][2]:.2f}")
    print(f"ev_charge level_start={charge[0][2]:.2f} level_end={charge[-1][2]:.2f} delta={gain:.2f}")
    print(f"ev_charge max_energyCharged={max(r[4] for r in charge):.2f}")
    drive = [r for r in rows if r[1] == "ev_drive"]
    print(f"ev_drive samples={len(drive)} charged_anywhere={any(r[4] > 0 for r in drive)}")
    if len(sys.argv) > 2:
        root = ET.parse(sys.argv[2]).getroot()
        for cs in root.iter("chargingStation"):
            print("chargingStation", cs.attrib)
            for t in cs:
                print(" ", t.tag, t.attrib)


if __name__ == "__main__":
    main()
