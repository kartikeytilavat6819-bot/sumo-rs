# jt_unreg_2lane — unregulated 4-arm junction, two lanes per approach

Same 2-lane net and 16 per-lane connections as jt_priority_2lane, but the centre node is
`unregulated` and the priority attributes on the incoming edges are removed: every driver uses
gap acceptance against traffic on both lanes of the conflicting approaches. jt_unregulated uses
a single lane.
