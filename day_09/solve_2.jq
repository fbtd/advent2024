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
            len: ($digits[$i]),
        }
    else
        empty
    end
)]
#| as $data # [{id: 1, first: 3, last: 5} ... ]
| .[-1].total_len as $total_len
| map(select(.id=="gap")) as $gaps
| map(select(.id!="gap")) as $data
| {gaps: $gaps, data: $data, more_data: []}
| until((.data | length == 0);
    .data[-1].len as $min_len
    | .data[-1].first as $max_first
    | ([.gaps.[] | select(.len >= $min_len and .first <= $max_first)] | sort_by(.first)[0]) as $gap
    | if isempty($gap) or $gap == null then
        .more_data += [.data[-1]]
    else
        .more_data += [{id: (.data[-1].id), len:(.data[-1].len), first: ($gap.first)}]
        | .gaps -= [$gap]
        | .gaps += [{id: $gap.id, len: ($gap.len - .data[-1].len), first: ($gap.first + .data[-1].len)}]
    end
    | .data[-1] |= empty
)
| debug
| .data + .more_data
| map(tovalue)
| add
