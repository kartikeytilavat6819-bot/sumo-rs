# Case `ev_depleted`: vehicle running out of battery

Energy/unknown vType with a very low initial charge (`device.battery.chargeLevel="5"` Wh) and high consumption
parameters (mass 2000, frontSurfaceArea 3.0, airDragCoefficient 0.5, rollDragCoefficient 0.02). The battery
device emits a "Battery of vehicle is depleted" warning and the battery output shows the capacity dropping
below zero/being clamped. No charging station is present. Covers the depleted-battery path.
