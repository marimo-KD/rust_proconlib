#!/bin/sh
cargo metadata --no-deps --format-version 1 | jq -r '.packages | map(.id) | sort[]' | grep -v "verification" | sed -e "s/(path+file:\/\/\(.*\))/\1/g" | awk '{printf "%s = { path = \"%s\" }\n", $1, $3}'
