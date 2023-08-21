#!/usr/bin/env bash

set -ex

# shellcheck disable=SC1091
source /emsdk-portable/emsdk_env.sh &> /dev/null

exec "$@"
