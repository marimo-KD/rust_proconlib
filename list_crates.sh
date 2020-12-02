#!/bin/sh
cargo metadata --format-version 1 | jq -r '.packages | map(.id) | sort[]' | sed -e "s/(path+file:\/\/\(.*\))/\1/g" | awk '{printf "%s = { path = \"%s\" }\n", $1, $3}'
