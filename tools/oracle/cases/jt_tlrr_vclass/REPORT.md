# jt_tlrr_vclass — traffic-light right-on-red with mixed vehicle classes

Same 4-arm `traffic_light_right_on_red` net as jt_traffic_light_right_on_red, but the 20 vehicles
cycle through passenger, bicycle, motorcycle and truck vTypes. Right-on-red becomes class-dependent
when the following vehicle is a slow/long truck versus a bicycle. jt_traffic_light_right_on_red
uses cars only; jt_vclass_mix covers classes on a different junction type.
