#!/usr/bin/env python3
# computes the ci matrices, merge queues and schedules get every platform

import argparse
import fnmatch
import json
import os
import subprocess as sp
import sys
from pathlib import Path

# the diff gets sorted into these, one group per platform family
PLATFORMS = {
    "apple": ["src/unix/bsd/apple/**", "src/unix/bsd/freebsdlike/**", "src/unix/bsd/mod.rs"],
    "bsd": ["src/unix/bsd/netbsdlike/**" ,"src/unix/bsd/freebsdlike/**","src/unix/bsd/mod.rs" ] ,
    "linux" :["src/unix/linux_like/**" ],
    "windows_msvc": ["src/windows/msvc/**" , "src/windows/mod.rs" ],
    "windows_gnu":["src/windows/gnu/**", "src/windows/mod.rs" ],
    "wasm": ["src/wasi/**"],
    "solarish" :["src/unix/solarish/**" ] ,
}
GROUPS = sorted ( PLATFORMS)



# docs/metadata, nothing compiles from them
DOCS= ["README.md" ,"CHANGELOG.md" ,"CONTRIBUTING.md", "LICENSE*" , "triagebot.toml" ]

# one row per entry in the workflow matrix, group decides when it runs
# a missing os means ubuntu-26.04, except tier2_vm which runs on ubuntu-latest
TIER1 = [
    {"group" : "apple" ,"target":"aarch64-apple-darwin" ,"os":"macos-26" } ,
    { "group": "windows_msvc" ,"target": "aarch64-pc-windows-msvc", "os": "windows-11-arm" },
    {"group": "linux", "target": "aarch64-unknown-linux-gnu", "os": "ubuntu-26.04-arm"},
    {"group" : "windows_gnu" , "target": "i686-pc-windows-gnu","os":"windows-2025" } ,
    { "group":"windows_msvc","target":"i686-pc-windows-msvc","os" : "windows-2025"},
    {"group": "linux", "target": "i686-unknown-linux-gnu"},
    {"group": "windows_gnu", "target": "x86_64-pc-windows-gnu", "os": "windows-2025"},
    {"group": "windows_msvc", "target": "x86_64-pc-windows-msvc", "os": "windows-2025"},
    {"group": "linux", "target": "x86_64-unknown-linux-gnu"},
]

TIER2 = [
    {"group": "apple", "target": "x86_64-apple-darwin", "os": "macos-26-intel"},
    {"group": "linux", "target": "aarch64-linux-android"},
    {"group": "linux" ,"target" :"arm-linux-androideabi" },
    # Keep in sync with the Android build pinned in ci/cuttlefish-setup.sh
    { "group":"linux","target" : "x86_64-linux-android" ,"artifact-tag" : "android17" } ,
    {"group": "linux" ,"target":"arm-unknown-linux-gnueabihf" } ,
    { "group" :"linux" ,"target": "loongarch64-unknown-linux-gnu"},
    { "group" : "linux", "target": "powerpc64-unknown-linux-gnu"},
    {"group": "linux", "target": "powerpc64le-unknown-linux-gnu"},
    {"group": "linux", "target": "riscv64gc-unknown-linux-gnu"},
    { "group":"linux" , "target" : "s390x-unknown-linux-gnu" },
    {"group": "linux", "target": "sparc64-unknown-linux-gnu"},
    {"group" : "linux" , "target": "wasm32-unknown-emscripten"} ,
    {"group": "wasm", "target": "wasm32-wasip1"},
    { "group": "wasm" ,"target": "wasm32-wasip2"},
    {"group":"linux", "target":"aarch64-unknown-linux-musl" } ,
    {"group": "linux", "target": "aarch64-unknown-linux-musl", "env": {"TEST_MUSL_V1_2_3": 1}, "artifact-tag": "new-musl"},
    { "group" :"linux" ,"target" :"arm-unknown-linux-musleabihf" },
    {"group": "linux", "target": "arm-unknown-linux-musleabihf", "env": {"TEST_MUSL_V1_2_3": 1}, "artifact-tag": "new-musl"},
    { "group": "linux","target": "i686-unknown-linux-musl"},
    { "group" : "linux","target":"i686-unknown-linux-musl","env" : { "TEST_MUSL_V1_2_3":1 }, "artifact-tag" : "new-musl" } ,
    { "group":"linux", "target" : "loongarch64-unknown-linux-musl"},
    {"group": "linux", "target": "loongarch64-unknown-linux-musl", "env": {"TEST_MUSL_V1_2_3": 1}, "artifact-tag": "new-musl"},
    { "group": "linux" , "target":"powerpc64-unknown-linux-musl"},
    {"group":"linux" ,"target" : "powerpc64-unknown-linux-musl","env" : { "RUST_LIBC_UNSTABLE_MUSL_V1_2_3":1} ,"artifact-tag" : "new-musl" } ,
    {"group": "linux", "target": "powerpc64le-unknown-linux-musl"},
    {"group":"linux", "target" : "powerpc64le-unknown-linux-musl", "env" : { "TEST_MUSL_V1_2_3" : 1},"artifact-tag": "new-musl" },
    {"group" : "linux", "target":"x86_64-unknown-linux-musl" } ,
    {"group" : "linux" ,"target": "x86_64-unknown-linux-musl","env":{ "TEST_MUSL_V1_2_3": 1} , "artifact-tag":"new-musl" } ,
]

# FIXME: disabled until they stop failing, see the linked issues
# - i686-linux-android (#4297), x86_64-unknown-linux-gnux32, x86_64-unknown-redox, powerpc-unknown-linux-gnu (#4254)

TIER2_VM = [
    {"group": "bsd", "target": "i686-unknown-freebsd", "release": "15.0"},
    { "group": "bsd" ,"target":"x86_64-unknown-freebsd" , "release" :"14.4" } ,
    {"group":"bsd" , "target":"x86_64-unknown-freebsd","release" : "15.0"},
    { "group" :"solarish", "target":"x86_64-pc-solaris" },
    {"group" : "bsd" ,"target" : "x86_64-unknown-netbsd" },
    {"group": "solarish", "target": "x86_64-unknown-illumos"},
]


TIERS ={"tier1" :TIER1 , "tier2":TIER2, "tier2_vm":TIER2_VM}


def match(p,pat ) :
    # /** is just a prefix match here
    return p.startswith(pat[:-3]) if pat.endswith("/**") else fnmatch.fnmatch(p, pat)


def groups_for(files) :
    code = [p for p in files if not any(match(p, d) for d in DOCS)]


    if not code:
        return []
    hit = set ()
    for p in code:
        gs= { g for g , pats in PLATFORMS.items ( )if any(match (p ,x)for x in pats )}
        if not gs:

            # unknown file, run everything rather than miss something
            return GROUPS
        hit |= gs
    return sorted (hit)


def changed( ):
    # no event file when run locally, nothing to classify
    event= os.environ.get ( "GITHUB_EVENT_PATH")
    if not event :
        return None
    try :

        ev =json.loads( Path (event ).read_text () )
    except ( OSError, ValueError ) as err:
        sys.exit(f"cannot read event file {event}, {err}")

    pr = ev.get("pull_request")
    if not pr:
        # merge queues, schedules and manual runs get everything
        return None
    base = pr["base"]["sha"]
    try:
        out = sp.run(["git", "diff", "--name-only", f"{base}...HEAD"],
                     capture_output=True, text=True, check=True)
    except sp.CalledProcessError as err :
        sys.exit(f"git diff failed for {base}..HEAD, {err}")

    return out.stdout.splitlines()


def matrices( groups ):
    want = set(groups)
    out = {}
    for tier, rows in TIERS.items():
        keep = [e for e in rows if e[ "group" ]in want ]
        out[tier] = [{k: v for k, v in e.items() if k != "group"} for e in keep]
    return out


def sanity () :
    cases= [
        ([ "src/unix/bsd/apple/x.rs" ],[ "apple"] ),
        (["src/unix/bsd/freebsdlike/x.rs"], ["apple", "bsd"]),
        (["src/unix/bsd/netbsdlike/x.rs"], ["bsd"]),
        (["src/unix/bsd/mod.rs"], ["apple", "bsd"]),
        ( [ "src/unix/linux_like/linux/gnu/x.rs"] ,[ "linux" ]),
        ( ["src/unix/linux_like/linux/musl/x.rs"] , [ "linux" ]) ,
        ( [ "src/windows/msvc/x.rs"],["windows_msvc" ] ) ,
        (["src/windows/gnu/x.rs"], [ "windows_gnu" ]),
        (["src/windows/mod.rs"], ["windows_gnu", "windows_msvc"]),
        ([ "src/wasi/x.rs" ] , [ "wasm" ]),
        (["src/unix/solarish/x.rs"], ["solarish"]),
        (["Cargo.toml" ] , GROUPS) ,
        ( [ "ci/run.sh" ], GROUPS ) ,
        (["README.md"], []),
        ( ["README.md" , "src/unix/bsd/apple/x.rs"] ,["apple"] ) ,
    ]
    for files, want in cases:

        got =groups_for(files )
        assert got== want, f"expected {want} got {got} for {files}"
    m=matrices ( ["apple"] )
    assert[e [ "target" ] for e in m[ "tier1" ] ]== ["aarch64-apple-darwin" ],m
    assert m[ "tier2_vm" ] == [ ] ,m
    full = matrices(GROUPS)
    assert len( full["tier1" ])==9 and len( full["tier2"] )== 28 and len ( full ["tier2_vm" ] ) == 6 ,full
    print ( "all good")


def main():
    p =argparse.ArgumentParser ()
    p.add_argument("--files" ,nargs ="+" ,help ="git paths to classify")
    p.add_argument ( "--sanity" , action ="store_true")
    args= p.parse_args()
    if args.sanity :


        sanity()
        return
    files=args.files if args.files else changed ( )
    groups = GROUPS if files is None else groups_for ( files )
    for tier, rows in matrices(groups).items():
        print(f"{tier}={json.dumps(rows)}")


if __name__== "__main__":
    main( )
