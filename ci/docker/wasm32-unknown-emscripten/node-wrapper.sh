#!/bin/sh

set -eux

me="$1"
shift
dir=$(dirname "$me")
file=$(basename "$me")

cd "$dir"
exec node "$file" "$@"
