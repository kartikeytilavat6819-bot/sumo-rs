# jt_allway_stop_2lane — all-way-stop junction, two lanes per approach

Same 2-lane net and 16 per-lane connections as jt_priority_2lane, but centre node is
`allway_stop` and edge priorities are removed. Each lane of every approach stops before entering;
both lanes of the same approach can then proceed in turn. Existing all-way-stop cases use one lane.
