# REPORT: city_munich (run 2026-11-16, ours = debug build in sumo-rs-target-x07)

| stage | verdict | first difference / message |
|---|---|---|
| net (netconvert, `--osm-files input.osm --simple-projection`) | **PANIC** | `crates\sumo_core\src\netbuild\nbpt_line_cont.rs:307:9`: `Error: NBPTLineCont::fixBidiStops: shortcut, needs NBPTStop bidi links and router operations`. Exit 1, no net written. C++ builds the net. |
| sim tripinfo / fcd / stat | **PANIC** | `crates\sumo_core\src\microsim\traffic_lights\ms_traffic_light_logic.rs:68:5`: `MSTrafficLightLogic::getCurrentPhaseDef is pure virtual; call it on a concrete traffic light logic`. Exit 1, no output. C++ on the same net exits 0 with 94 tripinfo entries. |

Not compared: nothing, because no output was produced on either side.

Notes:
- The sim panic is on the committed C++ net (`ref_simple.net.xml`), so it is not caused by our netconvert.
- Cause not diagnosed. The most likely lead is the three traffic lights netconvert drops because they control no
  links (`1955028`, `278690117`, `293544170`).
- Both failures are real product gaps for this city shape, and the first is a netconvert PANIC on a normal
  OSM extract.
