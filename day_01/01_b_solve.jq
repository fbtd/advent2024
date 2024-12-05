#!/usr/bin/env -S jq -R --slurp -f

. | split("\n") | map(split("   ")) | map(select(length > 1)) |
{ left  : . | map(.[0]),
  right : . | map(.[1]),
} | . as $obj |
reduce .left[] as $l (0;
    . += ($obj.right | indices($l) | length) * ($l | tonumber)
)
