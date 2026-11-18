# jt_zipper_two_arms — zipper merge of two one-lane arms into one lane

A `zipper` node nZ merges two separate one-lane incoming edges (A_in from the west, B_in from the
south) into a single one-lane OUT edge. 16 vehicles alternate arms in pairs at t=0,3,...,21 s so
both streams reach the merge together. Existing zipper cases merge two lanes of one edge; this
tests zipper right-of-way between distinct approaches.
