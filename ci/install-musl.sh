#!/bin/sh
#
# Install musl and musl-sanitized linux kernel headers
# to musl-{$1} directory

set -eux

musl_version=1.1.24
musl="musl-${musl_version}"

# Download, configure, build, and install musl:
curl --retry 5 https://www.musl-libc.org/releases/${musl}.tar.gz | tar xzf -

cd "$musl"
case ${1} in
    aarch64)
        musl_arch=aarch64
        kernel_arch=arm64
        CC=aarch64-linux-gnu-gcc \
          ./configure --prefix="/musl-${musl_arch}" --enable-wrapper=yes
        make install -j4
        ;;
    arm)
        musl_arch=arm
        kernel_arch=arm
        CC=arm-linux-gnueabihf-gcc CFLAGS="-march=armv6 -marm -mfpu=vfp" \
          ./configure --prefix="/musl-${musl_arch}" --enable-wrapper=yes
        make install -j4
        ;;
    i686)
        # cross-compile musl for i686 using the system compiler on an x86_64
        # system.
        musl_arch=i686
        kernel_arch=i386
        # Specifically pass -m32 in CFLAGS and override CC when running
        # ./configure, since otherwise the script will fail to find a compiler.
        CC=gcc CFLAGS="-m32" \
          ./configure --prefix="/musl-${musl_arch}" --disable-shared --target=i686
        # unset CROSS_COMPILE when running make; otherwise the makefile will
        # call the non-existent binary 'i686-ar'.
        make CROSS_COMPILE= install -j4
        ;;
    x86_64)
        musl_arch=x86_64
        kernel_arch=x86_64
        ./configure --prefix="/musl-${musl_arch}"
        make install -j4
        ;;
    s390x)
        musl_arch=s390x
        kernel_arch=s390
        CC=s390x-linux-gnu-gcc \
          ./configure --prefix="/musl-${musl_arch}" --enable-wrapper=yes
        make install -j4
        ;;
    *)
        echo "Unknown target arch: \"${1}\""
        exit 1
        ;;
esac


# shellcheck disable=SC2103
cd ..
rm -rf "$musl"

# Download, configure, build, and install musl-sanitized kernel headers.

# Alpine follows stable kernel releases, 3.20 uses Linux 6.6 headers.
alpine_version=3.20
alpine_git=https://gitlab.alpinelinux.org/alpine/aports

# This routine piggybacks on: https://git.alpinelinux.org/aports/tree/main/linux-headers?h=3.20-stable
git clone -n --depth=1 --filter=tree:0 -b "${alpine_version}-stable" "$alpine_git"
(
    cd aports
    git sparse-checkout set --no-cone main/linux-headers
    git checkout

    cd main/linux-headers
    cp APKBUILD APKBUILD.vars
    cat <<- EOF >> APKBUILD.vars
        echo "\$source" > alpine-source
        echo "\$_kernver" > alpine-kernver
        echo "\$pkgver" > alpine-pkgver
        echo "\$sha512sums" > alpine-sha512sums
EOF

    # Retrieve all the variables
    sh APKBUILD.vars

    cat APKBUILD.vars

    kernel_version=$(tr -d "[:space:]" < alpine-kernver)
    pkg_version=$(tr -d "[:space:]" < alpine-pkgver)

    urls=$(grep -o 'https.*' alpine-source)
    kernel=""
    patch=""
    for url in $urls; do
        base=$(basename "$url")
        curl --retry 5 -L "$url" > "$base"
        case $base in
            linux-*) kernel=$base;;
            patch-*) patch=$base;;
        esac
        # Check if file is known
        grep -o "$base" alpine-sha512sums
    done

    # Double check checksums
    sha512sum -c alpine-sha512sums

    # Extract, apply patches, compile and install headers
    tar -xf "$kernel"
    cd "linux-$kernel_version"
    if [ "$pkg_version" != "$kernel_version" ]; then
        unxz -c < "../$patch" | patch -p1
    fi
    for p in ../*.patch; do
        patch -p1 < "$p"
    done
    make headers_install ARCH="${kernel_arch}" INSTALL_HDR_PATH="/musl-${musl_arch}"
)

rm -rf aports
