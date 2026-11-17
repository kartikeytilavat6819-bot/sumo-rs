# jt_priority_2lane — 4-arm priority junction, two lanes per approach

Centre node C (`priority`), four unregulated arms. Incoming edges have 2 lanes; outgoing 1 lane.
Right turns and straight movements use lane 0, left turns use lane 1 (16 explicit connections).
N/S arms have priority 2, E/W priority 1. 20 vehicles depart in perpendicular pairs at 0,6,...,54 s;
left-turners start on lane 1, right/straight on lane 0. Tests right-of-way resolution across two lanes,
which no single-lane jt_* case covers.
