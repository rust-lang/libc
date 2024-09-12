#!/usr/bin/env sh
#
# Install musl and musl-sanitized linux kernel headers
# to musl-{$1} directory

set -ex

MUSL_VERSION=1.1.24
MUSL="musl-${MUSL_VERSION}"

# Download, configure, build, and install musl:
curl --retry 5 https://www.musl-libc.org/releases/${MUSL}.tar.gz | tar xzf -

cd $MUSL
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
rm -rf $MUSL

# Download, configure, build, and install musl-sanitized kernel headers.
# This routine piggybacks on: https://git.alpinelinux.org/aports/tree/main/linux-headers?h=3.20-stable
# Alpine follows stable kernel releases, 3.20 uses Linux 6.6 headers.
git clone -n --depth=1 --filter=tree:0 -b 3.20-stable https://gitlab.alpinelinux.org/alpine/aports
(
   cd aports
   git sparse-checkout set --no-cone main/linux-headers
   git checkout
   cd main/linux-headers
   cp APKBUILD APKBUILD.source
   cp APKBUILD APKBUILD.sha512
   {
   echo "printf \"\$source\""
   # shellcheck disable=SC2028
   echo "printf \"\$_kernver\n\""
   # shellcheck disable=SC2028
   echo "printf \"\$pkgver\n\""
   } >> APKBUILD.source
   echo "printf \"\$sha512sums\"" >> APKBUILD.sha512
   KERNEL_VER=$(bash APKBUILD.source | tail -2 | head -1 | tr -d "[:space:]")
   PKGVER=$(bash APKBUILD.source | tail -1 | tr -d "[:space:]")
   urls=$(bash APKBUILD.source | grep -o 'https.*')
   kernel=""
   patch=""
   for url in $urls; do
      base=$(basename "$url")
      curl --retry 5 -L "$url" > "$base"
      case $base in
         linux-*) kernel=$base;;
         patch-*) patch=$base;;
      esac
   done
   bash APKBUILD.sha512 | grep "$kernel" >> sha-check
   bash APKBUILD.sha512 | grep "$patch" >> sha-check
   sha512sum -c sha-check
   tar -xf "$kernel"
   cd "linux-$KERNEL_VER"
   if [ "$PKGVER" != "$KERNEL_VER" ]; then
      unxz -c < "../$patch" | patch -p1
   fi
   for p in ../*.patch; do
      patch -p1 < "$p"
   done
   make headers_install ARCH="${kernel_arch}" INSTALL_HDR_PATH="/musl-${musl_arch}"
)

rm -rf aports
