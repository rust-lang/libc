set -ex

# shellcheck disable=SC1091
source /emsdk-portable/emsdk_env.sh &> /dev/null

# emsdk-portable provides a node binary, but we need version 8 to run wasm
export PATH="/node-v12.3.1-linux-x64/bin:$PATH"

exec "$@"
