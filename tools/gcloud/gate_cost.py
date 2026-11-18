#!/usr/bin/env python3
"""Live Google Cloud spend of the gate VMs: catalog list prices x uptime measured by Cloud Monitoring. Read-only.
   python tools/gcloud/gate_cost.py        (gate_progress.ps1 calls it on every refresh)"""
import datetime as dt, json, os, subprocess, tempfile, time, urllib.parse, urllib.request

PROJ = "project-b17b0335-3081-4c61-89b"
ACCT = "kartikeytilavat6819@gmail.com"  # user's order: only the personal account, never the work account
VCPU, RAM_GIB = 16, 40  # e2-custom-16-40960 (both VMs)
SKU = {"cpu": "1B35-7EB3-EEE9", "ram": "D45F-2D53-1247", "ip": "4AF8-7C1F-39C4", "disk": "6AE1-525F-8B80"}
FMT = "%Y-%m-%dT%H:%M:%SZ"


def token():
    env = dict(os.environ, CLOUDSDK_CORE_ACCOUNT=ACCT, CLOUDSDK_CORE_PROJECT=PROJ)
    return subprocess.run("gcloud auth print-access-token", shell=True, env=env,
                          capture_output=True, text=True, check=True).stdout.strip()


def get(url, tok, params=None):
    if params:
        url += "?" + urllib.parse.urlencode(params)
    req = urllib.request.Request(url, headers={"Authorization": f"Bearer {tok}", "x-goog-user-project": PROJ})
    with urllib.request.urlopen(req, timeout=60) as r:
        return json.load(r)


def prices(tok):
    # the catalog is ~35k SKUs in 7 pages: fetch once a day
    cache = os.path.join(tempfile.gettempdir(), "gate_prices.json")
    if os.path.exists(cache) and time.time() - os.path.getmtime(cache) < 86400:
        return json.load(open(cache))
    want = {v: k for k, v in SKU.items()}
    out, page = {}, ""
    while True:
        p = {"pageSize": 5000, "currencyCode": "USD"} | ({"pageToken": page} if page else {})
        d = get("https://cloudbilling.googleapis.com/v1/services/6F81-5844-456A/skus", tok, p)
        for s in d.get("skus", []):
            if s["skuId"] in want:
                u = s["pricingInfo"][0]["pricingExpression"]["tieredRates"][-1]["unitPrice"]
                out[want[s["skuId"]]] = int(u.get("units", "0")) + u.get("nanos", 0) / 1e9
        page = d.get("nextPageToken")
        if not page:
            break
    assert len(out) == len(SKU), f"price catalog lacks SKUs {set(SKU) - set(out)}"
    json.dump(out, open(cache, "w"))
    return out


def uptime_hours(tok, start, end):
    secs = max(int((end - start).total_seconds()), 60)
    d = get(f"https://monitoring.googleapis.com/v3/projects/{PROJ}/timeSeries", tok, {
        "filter": 'metric.type="compute.googleapis.com/instance/uptime"',
        "interval.startTime": start.strftime(FMT), "interval.endTime": end.strftime(FMT),
        "aggregation.alignmentPeriod": f"{secs}s", "aggregation.perSeriesAligner": "ALIGN_SUM"})
    return sum(p["value"].get("doubleValue", 0) for ts in d.get("timeSeries", []) for p in ts["points"]) / 3600


def main():
    tok = token()
    pr = prices(tok)
    vm_h = VCPU * pr["cpu"] + RAM_GIB * pr["ram"] + pr["ip"]
    inst = get(f"https://compute.googleapis.com/compute/v1/projects/{PROJ}/aggregated/instances", tok)
    disks = get(f"https://compute.googleapis.com/compute/v1/projects/{PROJ}/aggregated/disks", tok)
    running = sum(i["status"] == "RUNNING" for z in inst["items"].values() for i in z.get("instances", []))
    dl = [(int(k["sizeGb"]), dt.datetime.fromisoformat(k["creationTimestamp"]))
          for z in disks["items"].values() for k in z.get("disks", [])]
    disk_gib = sum(g for g, _ in dl)
    disk_h = disk_gib * pr["disk"] / 730  # GiB-month price, Google's 730 h month
    now = dt.datetime.now().astimezone()
    # disk GiB-hours since `start`, each disk counted from its creation
    disk_cost = lambda start: sum(g * pr["disk"] / 730 * max((now - max(start, c)).total_seconds(), 0) / 3600 for g, c in dl)
    midnight = now.replace(hour=0, minute=0, second=0, microsecond=0)
    first = midnight.replace(day=1)
    utc = lambda t: t.astimezone(dt.timezone.utc)
    print("cloud cost (estimate: list prices x measured uptime, before credits; uptime lags ~3 min)")
    print(f"  now:   {running} VM running -> ${running * vm_h + disk_h:.2f}/h   (${vm_h:.3f} per VM-h, disks {disk_gib} GiB ${disk_h:.3f}/h)")
    for name, start in (("today", midnight), ("month", first)):
        h = uptime_hours(tok, utc(start), utc(now))
        disk = disk_cost(start)
        print(f"  {name}: {h:6.1f} VM-h -> ${h * vm_h + disk:6.2f}  (VMs ${h * vm_h:.2f} + disks ${disk:.2f})")


if __name__ == "__main__":
    main()
