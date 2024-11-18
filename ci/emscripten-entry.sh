#!/usr/bin/env bash

set -eux

# shellcheck disable=SC1091
source /emsdk-portable/emsdk_env.sh &> /dev/null

exec "$@"
