#!/usr/bin/env -S jq --slurp -Rf

split("\n")
| map(
    select(length > 0)
    | split(":")
    | .[0] |= tonumber
    | .[1] |= (split(" ") | .[1:] | map(tonumber))
    | . as [$result, $digits]
    | ["+", "*"] | [combinations($digits | length -1)] as $signs
    | $signs
    | map( # ["+", "*"]
        . as $this_signs
        | reduce range(. | length) as $i ($digits[0];
            if $this_signs[$i] == "+" then
                . += $digits[$i+1]
            elif $this_signs[$i] == "*" then
                . *= $digits[$i+1]
            else
                error("sign not found")
            end
        )
        | select(. == $result)
    )
    | select(length > 0)
    | .[0]
)
| add

