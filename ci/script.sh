set -ex

main() {
    for target in $(rustup target list | grep linux-gnu | cut -d' ' -f1); do
        rustup target add $target || true
        xargo check --target $target
    done
}

main
