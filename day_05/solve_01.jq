#!/usr/bin/env -S jq --slurp -Rf

def respect_rule(rule):
    [ indices(rule.from)[0], indices(rule.to)[0]]
    | .[0] and .[1] and .[0] > .[1]
    | not
;

( split("\n\n")
| .[0] |= (split("\n") | map(select(length > 0) | split("|") | map(tonumber) | {from: .[0], to: .[1]}))
| .[1] |= (split("\n") | map(select(length > 0) | split(",") | map(tonumber)))
) as [$rules, $updates]

| $updates
| map(
    select(
        . as $this
        | reduce $rules.[] as $rule (true; . and ($this | respect_rule($rule)))
    )
    | .[. | length / 2]
)
| add

