# jm_drive_after_yellow

Signal c with the default program. vY has jmDriveAfterYellowTime=3 and may cross on yellow. vStd and vMinor use defaults. Flows every 7 s and 9 s reach yellow phases.

Refs: `bash tools/oracle/make_sim_refs.sh jm_drive_after_yellow` (C++ 1.28.0 from ~/sumo-pristine, WSL). Net: `net_ref.sh`.
