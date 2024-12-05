#!/usr/bin/env -S jq -nRf

def deltas:
    . as $original
    | reduce $original[1:].[] as $item({last_item: $original[0], dt: []};
        .dt += [ $item - .last_item | abs ]
        | .last_item = $item 
    )
    | .dt
;

def is_safe:
    . as $original
    | [ 
        sort == $original,
        (sort | reverse) == $original
    ]
    | any
        and ($original | unique | length) == ($original | length)
        and ($original | deltas | max <= 3 )
;

def permutations:
    length as $len
    | . as $obj 
    | [range(1;$len+1)]
    | map($obj[0:.-1] + $obj[.:]) + [$obj]
;

[inputs] | map(
    split(" ")
    | map(tonumber)
    | permutations
    | map(is_safe)
    #| map(deltas)
    | any
    | select(.)
)
| length
