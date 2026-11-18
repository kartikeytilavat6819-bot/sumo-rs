# jt_keepclear_pass — `keepClear="false"` combined with `pass="true"`

Priority 4-arm junction: the two major through connections carry
`pass="true" keepClear="false"`; the two minor through connections carry
`keepClear="false"` only (serialized as `keepClear="0"` / `pass="1"`).
A blocker stops on `S_out_0` for 100 s and 16 other vehicles approach, so
both connection flags matter. No committed case combines these two flags.

Net: C++ netconvert 1.28.0; sim refs: C++ sumo 1.28.0, `--end 200`.
