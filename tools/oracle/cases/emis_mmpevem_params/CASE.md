# Case `emis_mmpevem_params`: MMPEVEM with explicit energy parameters

DEFAULT_VEHTYPE: `MMPEVEM/unknown` with vType attributes mass, frontSurfaceArea, airDragCoefficient,
rollDragCoefficient, wheelRadius, gearRatio, gearEfficiency (read by HelpersMMPEVEM::compute). `--emission-output.precision 6`.
Note: `MMPEVEM/VW_ID3` is rejected by C++ ("emissionClass ... doesn't exist"), so only `MMPEVEM/unknown` is valid.
