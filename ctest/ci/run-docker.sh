# Small script to run tests for a target (or all targets) inside all the
# respective docker images.

set -ex

run() {
    echo "Building docker container for TARGET=${1}"
    docker build -t ctest -f ci/docker/$1/Dockerfile ci/
    mkdir -p target
    target=$1
    echo "Running docker"
    docker run \
           --user `id -u`:`id -g` \
           --rm \
           --init \
           --volume $HOME/.cargo:/cargo-h \
           --env CARGO_HOME=/cargo-h \
           --volume `rustc --print sysroot`:/rust:ro \
           --env TARGET=$target \
           --volume `pwd`:/checkout:ro \
           --volume `pwd`/target:/checkout/target \
           --workdir /checkout \
           --privileged \
           ctest \
           bash \
           -c 'PATH=/rust/bin:$PATH exec ci/run.sh'
}

if [ -z "$1" ]; then
    for d in `ls ci/docker/`; do
        run $d
    done
else
    run $1
fi
