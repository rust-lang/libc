#!/bin/bash

if [[ $# < 1 ]]; then
    echo "Usage: $0 <kernel source tree>"
    exit 1
fi

src_root=$1
out_root=$(dirname $(realpath $0))

syscall_constants=$out_root/syscall_constants

truncate --size 0 "$syscall_constants"

header() {
    printf '// This file was generated via generate_syscalls.sh. Do not edit it manually.\n\n'
}

declare -A map64=(
    [fcntl]=fcntl     
    [statfs]=statfs    
    [fstatfs]=fstatfs   
    [truncate]=truncate  
    [ftruncate]=ftruncate 
    [lseek]=lseek     
    [sendfile]=sendfile  
    [fstatat]=newfstatat
    [fstat]=fstat     
    [mmap]=mmap      
    [fadvise64]=fadvise64 
    [stat]=stat      
    [lstat]=lstat     
)

declare -A map32=(
    [fcntl]=fcntl64     
    [statfs]=statfs64    
    [fstatfs]=fstatfs64   
    [truncate]=truncate64  
    [ftruncate]=ftruncate64 
    [lseek]=llseek      
    [sendfile]=sendfile64  
    [fstatat]=fstatat64   
    [fstat]=fstat64     
    [mmap]=mmap2       
    [fadvise64]=fadvise64_64
    [stat]=stat64      
    [lstat]=lstat64     
)

unistd() {
    local -n map=$1

    while read line; do
        pat='#define __NR(3264)?_([^ ]+)[[:space:]]+([0-9]+)$'
        if [[ $line =~ $pat ]]; then
            name=${BASH_REMATCH[2]}
            if [[ $name = syscalls ]] || [[ $name = arch_specific_syscall ]]; then
                continue
            fi
            if [[ -n ${BASH_REMATCH[1]} ]]; then
                name=${map[$name]}
            fi
            echo "pub const SYS_$name: ::c_long = ${BASH_REMATCH[3]};"
            echo "SYS_$name" >> "$syscall_constants"
        fi
    done <"$src_root"/include/uapi/asm-generic/unistd.h
}

unistd32_syscalls=$(unistd map32)
unistd64_syscalls=$(unistd map64)

scan_table() {
    header
    cb=$1
    abis=$2
    grep -E "^[0-9]+\s+($abis)" | \
        while read nr abi name entrypoint; do
            $cb $name $nr
            echo "SYS_$name" >> "$syscall_constants"
        done
}

generate_mips_constant() {
    echo "pub const SYS_$1: ::c_long = 4000 + $2;"
}

generate_mips64_constant() {
    echo "pub const SYS_$1: ::c_long = 5000 + $2;"
}

generate_x32_constant() {
    echo "pub const SYS_$1: ::c_long = super::__X32_SYSCALL_BIT + $2;"
}

generate_constant() {
    echo "pub const SYS_$1: ::c_long = $2;"
}

common_tbl() {
    abis=$1
    scan_table generate_constant "$abis"
}

scan_table generate_mips64_constant n64 \
    <"$src_root"/arch/mips/kernel/syscalls/syscall_n64.tbl \
    >"$out_root"/b64/mips64/syscalls.rs

common_tbl common \
    <"$src_root"/arch/arm/tools/syscall.tbl \
    >"$out_root"/b32/arm/syscalls.rs

common_tbl 'common|32' \
    <"$src_root"/arch/sparc/kernel/syscalls/syscall.tbl \
    >"$out_root"/b32/sparc/syscalls.rs

common_tbl i386 \
    <"$src_root"/arch/x86/entry/syscalls/syscall_32.tbl \
    >"$out_root"/b32/x86/syscalls.rs

common_tbl 'common|nospu|32' \
    <"$src_root"/arch/powerpc/kernel/syscalls/syscall.tbl \
    >"$out_root"/b32/powerpc/syscalls.rs

scan_table generate_mips_constant o32 \
    <"$src_root"/arch/mips/kernel/syscalls/syscall_o32.tbl \
    >"$out_root"/b32/mips/syscalls.rs

common_tbl 'common|64' \
    <"$src_root"/arch/x86/entry/syscalls/syscall_64.tbl \
    >"$out_root"/b64/x86_64/not_x32/syscalls.rs

scan_table generate_x32_constant 'common|x32' \
    <"$src_root"/arch/x86/entry/syscalls/syscall_64.tbl \
    >"$out_root"/b64/x86_64/x32/syscalls.rs

common_tbl 'common|nospu|64' \
    <"$src_root"/arch/powerpc/kernel/syscalls/syscall.tbl \
    >"$out_root"/b64/powerpc64/syscalls.rs

common_tbl 'common|64' \
    <"$src_root"/arch/s390/kernel/syscalls/syscall.tbl \
    >"$out_root"/b64/s390x/syscalls.rs

common_tbl 'common|64' \
    <"$src_root"/arch/sparc/kernel/syscalls/syscall.tbl \
    >"$out_root"/b64/sparc64/syscalls.rs

printf '%s\n' "$unistd64_syscalls" >"$out_root"/b64/riscv64/syscalls.rs

printf '%s\n' "$unistd64_syscalls" >"$out_root"/b64/aarch64/syscalls.rs

constants=$(cat "$syscall_constants" | sort -u)
printf '%s\n' "$constants" > "$syscall_constants"
