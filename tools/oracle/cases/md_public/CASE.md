# md_public

`detectPersons="public"` (and a combined `"walk public"` mask) on edgeData/laneData, using the PT net, bus line L1 and persons with `personTrip modes="public"` from `ped_persontrip_public`. This is the one person mode not covered by `md_persons`.

Sim: `--end 400 --persontrip.walkfactor 0.75 --additional-files tools/oracle/cases/md_public/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_pub.xml`, `ref.ed_pubwalk.xml`, `ref.ld_pub.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_public; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
