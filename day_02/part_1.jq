#!/usr/bin/env -S jq -nRf

[inputs] | map(
    split(" ") |
    map(tonumber) |
    reduce .[] as $n ({last_n: 0, direction: "FIRST"};
        if .direction == "FIRST" then
            .last_n = $n | .direction = "FLAT"
        elif ( .last_n > $n) and ( .direction == "DECREASING" ) and ((.last_n - $n) | abs) <= 3 then
            .last_n = $n
        elif ( .last_n > $n) and ( .direction == "FLAT" ) and ((.last_n - $n) | abs) <= 3 then
            .last_n = $n | .direction = "DECREASING"
        elif ( .last_n < $n) and ( .direction == "INCREASING" ) and ((.last_n - $n) | abs) <= 3 then
            .last_n = $n
        elif ( .last_n < $n) and ( .direction == "FLAT" ) and ((.last_n - $n) | abs) <= 3 then
            .last_n = $n | .direction = "INCREASING"
        else
            .direction = ":("
        end
    ) |
    select(.direction != ":(") 
) |
length
