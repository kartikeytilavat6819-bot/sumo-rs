# calibrator

Calibrator `cal0` on E1 at 50 m, with an inline `<flow>` of 720 veh/h at 10 m/s on route `r0` (E0 E1), plus four background cars.
No type attribute on the inline flow: the additional file loads before `routes.rou`, so a `type` vType would be unknown (C++ error).
Sim: `--end 200 --additional-files tools/oracle/cases/calibrator/calibrator.add.xml`.
