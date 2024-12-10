#!/usr/bin/env -S jq --slurp -Rf

def tovalue:
    .id * ( .first * .len + (.len * (.len-1) / 2))
;


. | rtrimstr("\n")
| split("")
| map(tonumber)
| . as $digits
| [foreach range(length) as $i ({last_free: -1, total_len: 0};
    .last_free += $digits[$i] |
    if $i % 2 == 0 then
        .total_len += $digits[$i] 
    end
    ;
    if $i % 2 == 0 then
        {
            id: ($i/2),
            first: (.last_free-$digits[$i]+1),
            last: .last_free,
            len: ($digits[$i]),
            total_len: .total_len
        }
    elif $digits[$i] > 0 then
        {
            id: "gap",
            first: (.last_free-$digits[$i]+1),
            last: .last_free,
            len: ($digits[$i]),
            total_len: .total_len
        }
    else
        empty
    end
)]
#| as $data # [{id: 1, first: 3, last: 5} ... ]
| .[-1].total_len as $total_len
| map(select(.id=="gap" and .first < $total_len)) as $gaps
| map(select(.id!="gap")) as $data
| {gaps: $gaps, data: $data, more_data: []}
| until(.gaps | length == 0;
    if .data[-1].len > .gaps[0].len then   # can fill the gap
        .data[-1].len -= .gaps[0].len
        | .more_data += [{id: .data[-1].id, len: .gaps[0].len, first:.gaps[0].first}] 
        | .gaps[0] |= empty
    elif .data[-1].len < .gaps[0].len then # cannot fill the gap
        .gaps[0].len -= .data[-1].len
        | .more_data += [{id: .data[-1].id, len: .data[-1].len, first:.gaps[0].first}] 
        | .gaps[0].first += .data[-1].len
        | .data[-1] |= empty
    else                                   # is the gap
        .more_data += [{id: .data[-1].id, len: .gaps[0].len, first:.gaps[0].first}] 
        | .data[-1] |= empty
        | .gaps[0] |= empty
    end
)

| .data + .more_data
| map(tovalue)
| add
