#!/usr/bin/env python3
# tier1 tests skip platforms the diff doesnt touch, this decides which ones run

import argparse
import fnmatch
import json
import os
import subprocess as sp
import sys
from pathlib import Path

#touching any of these runs everything
CORE = [
    "src/lib.rs", "src/types.rs", "src/macros.rs", "src/primitives.rs",
    "src/unix/mod.rs","build.rs","Cargo.toml", "Cargo.lock" ,"ci/**",
    ".github/**" , "ctest/**" , "ctest-test/**" ,"libc-test/**" ,"semver/**" ,
    "etc/**" ,
]

GROUPS={
    "core": CORE,
    "apple": ["src/unix/bsd/apple/**", "src/unix/bsd/freebsdlike/**", "src/unix/bsd/mod.rs" ],
    "linux_gnu":["src/unix/linux_like/**"],
    "windows_msvc":["src/windows/msvc/**", "src/windows/mod.rs"] ,
    "windows_gnu": ["src/windows/gnu/**", "src/windows/mod.rs"],
}



# docs/metadata, nothing compiles from them
DOCS=["README.md","CHANGELOG.md", "CONTRIBUTING.md","LICENSE*", "triagebot.toml"]


def match (p,pat ) :
    #/** is just a prefix match here
    return p.startswith(pat[:-3]) if pat.endswith("/**") else fnmatch.fnmatch(p, pat)


def groups_for( files):

    if all (any ( match (p,pat) for pat in DOCS ) for p in files) :
        return []
    hit = {g for p in files for g, pats in GROUPS.items() if any(match(p, x) for x in pats)}
    # no match, run everything rather than miss something
    return sorted(GROUPS) if not hit else sorted(hit)


def changed():


    #no event file when run locally, nothing to classify
    event = os.environ.get ( "GITHUB_EVENT_PATH" )
    if not event:
        return None
    try:
        ev=json.loads (Path (event ).read_text() )
    except ( OSError ,ValueError ) as err :
        sys.exit(f"cannot read event file {event}, {err}")

    pr,mq =ev.get("pull_request" ) , ev.get( "merge_group" )
    if pr:
        base, head = pr["base"]["sha"], "HEAD"
    elif mq:
        base, head= mq[ "base_sha" ],mq [ "head_sha" ]
    else:  # schedule/dispatch, run everything
        return None
    try:
        out = sp.run(["git", "diff", "--name-only", f"{base}...{head}"],
                     capture_output =True,text= True , check=True)
    except sp.CalledProcessError as err:
        sys.exit(f"git diff failed for {base}..{head}, {err}")

    return out.stdout.splitlines()


def sanity():
    cases=[
        ( [ "src/unix/bsd/apple/x.rs"], [ "apple"]) ,
        ( ["src/unix/bsd/freebsdlike/x.rs" ], ["apple"] ),
        (["src/unix/bsd/mod.rs"], ["apple"]),
        (["src/unix/linux_like/linux/gnu/x.rs"] , [ "linux_gnu"] ) ,
        ( [ "src/windows/msvc/x.rs"], [ "windows_msvc"]),
        (["src/windows/gnu/x.rs"], ["windows_gnu"]),
        ( [ "src/windows/mod.rs" ] ,[ "windows_gnu","windows_msvc" ]),
        ( [ "Cargo.toml" ], ["core" ]),
        ( ["ci/run.sh"] , [ "core" ] ),
        (["src/types.rs"], ["core"]),
        (["README.md"], []),
        #newlib has no tier1 target, the fail-safe kicks in
        ( [ "src/newlib/mod.rs" ] ,sorted (GROUPS ) ),
        ( [ "README.md","src/newlib/mod.rs" ],sorted(GROUPS)) ,
    ]
    for files,want in cases :
        got = groups_for (files )
        assert got == want, f"expected {want} got {got} for {files}"
    print( "all good")


def main () :
    p = argparse.ArgumentParser()
    p.add_argument( "--files",nargs= "+",help = "git paths to classify" )
    p.add_argument("--sanity", action="store_true")
    args=p.parse_args()
    if args.sanity:
        sanity()
        return
    files = args.files if args.files else changed ()
    groups= list( GROUPS ) if files is None else groups_for ( files )
    print(json.dumps( groups ) )
    out=os.environ.get ("GITHUB_OUTPUT" )
    if out :

        with open (out,"a")as f:
            f.write(f"changes={json.dumps(groups)}\n")


if __name__ =="__main__" :
    main ()
