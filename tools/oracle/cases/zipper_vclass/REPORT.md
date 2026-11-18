# zipper_vclass — zipper merge with mixed vehicle classes

Same 2-lane-to-1-lane zipper net as zipper, but the 16 vehicles cycle through passenger, bicycle and
truck vTypes across departLane 0/1. The alternating merge now has to interleave vehicles with very
different lengths and approach speeds. The plain zipper case covers cars only.
