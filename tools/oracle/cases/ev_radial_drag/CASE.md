# Case `ev_radial_drag`: radialDragCoefficient and maximumPower on an Energy/unknown battery EV

Energy/unknown vType with `radialDragCoefficient="0.05"`, `maximumPower="80000"` plus mass,
frontSurfaceArea, airDragCoefficient, rollDragCoefficient and recuperationEfficiency. These feed the
Energy consumption model used by the battery device. `has.battery.device=true`, 30 kWh capacity,
20 kWh initial charge. Battery output compared at precision 2 (distinct from the existing precision 4 and 6 cases).
