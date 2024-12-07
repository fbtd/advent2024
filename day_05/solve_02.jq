#!/usr/bin/env -S jq --slurp -Rf

def respect_rule(rule):
    [ indices(rule.from)[0], indices(rule.to)[0]]
    | .[0] and .[1] and .[0] > .[1]
    | not
;

def swap(rule):
    [ indices(rule.from)[0], indices(rule.to)[0]] as $ind
    | if $ind[0] and $ind[1] and $ind[0] > $ind[1] then
        .[$ind[0]] as $left
        | .[$ind[0]] = .[$ind[1]] 
        | .[$ind[1]] = $left
    end
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
        | not
    )
    # TODO: ugly but works
    | . as $this
    | reduce $rules.[] as $rule ($this; . | swap($rule))
    | . as $this
    | reduce ($rules | reverse | .[]) as $rule ($this; . | swap($rule))
    | . as $this
    | reduce $rules.[] as $rule ($this; . | swap($rule))
    | . as $this
    | reduce ($rules | reverse | .[]) as $rule ($this; . | swap($rule))
    | . as $this
    | reduce $rules.[] as $rule ($this; . | swap($rule))
    | . as $this
    | reduce ($rules | reverse | .[]) as $rule ($this; . | swap($rule))
    | . as $this
    | reduce $rules.[] as $rule ($this; . | swap($rule))
    | . as $this
    | reduce ($rules | reverse | .[]) as $rule ($this; . | swap($rule))
    | . as $this
    | reduce $rules.[] as $rule ($this; . | swap($rule))
    | . as $this
    | reduce ($rules | reverse | .[]) as $rule ($this; . | swap($rule))
    | . as $this
    | reduce $rules.[] as $rule ($this; . | swap($rule))
    | . as $this
    | reduce ($rules | reverse | .[]) as $rule ($this; . | swap($rule))
    | .[. | length / 2]
)
| add

