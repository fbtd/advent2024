#!/usr/bin/env -S jq -R --slurp -f

. | split("\n") | map(split("   ")) | map(select(length > 1)) |
{ left  : . | map(.[0] | tonumber) | sort,
  right : . | map(.[1] | tonumber) | sort,
  indexes : [range(0; . | length)]
} | . as $obj |
reduce .indexes[] as $i (0;
    . += ( $obj.left[$i] - $obj.right[$i] | abs ) 
)
