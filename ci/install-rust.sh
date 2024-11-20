#!/usr/bin/env sh
# This is intended to be used in CI only.

set -eux

echo "Setup toolchain"

toolchain="${TOOLCHAIN:-nightly}"
os="${OS:-}"

case "$(uname -s)" in
    Linux*)     os=linux ;;
    Darwin*)    os=macos ;;
    MINGW*)     os=windows ;;
    *)
        echo "Unknown system $(uname -s)"
        exit 1
        ;;
esac

if [ "$os" = "windows" ] && [ -n "${TARGET:-}" ]; then
    toolchain="$toolchain-$TARGET"
    rustup set profile minimal
fi

rustup set profile minimal
rustup update --force "$toolchain"
rustup default "$toolchain"

if [ -n "${TARGET:-}" ]; then
    echo "Install target"
    rustup target add "$TARGET"
fi

if [ -n "${INSTALL_RUST_SRC:-}" ]; then
    echo "Install rust-src"
    rustup component add rust-src
fi

if [ "$os" = "windows" ]; then
    if [ "${ARCH_BITS:-}" = "i686" ]; then
        echo "Install MinGW32"
        choco install mingw --x86 --force
    fi

    echo "Find GCC libraries"
    gcc -print-search-dirs
    /usr/bin/find "C:\ProgramData\Chocolatey" -name "crt2*"
    /usr/bin/find "C:\ProgramData\Chocolatey" -name "dllcrt2*"
    /usr/bin/find "C:\ProgramData\Chocolatey" -name "libmsvcrt*"

    if [ -n "${ARCH_BITS:-}" ]; then
        echo "Fix MinGW"
        for i in crt2.o dllcrt2.o libmingwex.a libmsvcrt.a ; do
            cp -f "/C/ProgramData/Chocolatey/lib/mingw/tools/install/mingw$ARCH_BITS/$ARCH-w64-mingw32/lib/$i" "$(rustc --print sysroot)/lib/rustlib/$TARGET/lib"
        done
    fi
fi

echo "Query rust and cargo versions"
rustc -Vv
cargo -V
rustup -Vv
rustup show

echo "Generate lockfile"
N=5
n=0
until [ $n -ge $N ]; do
    if cargo generate-lockfile; then
        break
    fi

    n=$((n+1))
    sleep 1
done
