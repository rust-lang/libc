libc
====

A Rust library with native bindings to the types and functions commonly found on
various systems, including libc.

[![Build Status](https://travis-ci.org/alexcrichton/libc.svg?branch=master)](https://travis-ci.org/alexcrichton/libc)
[![Build status](https://ci.appveyor.com/api/projects/status/v0414slj8y8nga0p?svg=true)](https://ci.appveyor.com/project/alexcrichton/libc)

[Documentation](http://alexcrichton.com/libc)

## Platform Support

Tested:
  * `{i686,x86_64}-pc-windows-{msvc,gnu}`
  *  `{i686,x86_64,mips,aarch64}-unknown-linux-gnu`
  *  `x86_64-unknown-linux-musl`
  *  `arm-unknown-linux-gnueabihf`
  *  `arm-linux-androideabi`
  *  `{i686,x86_64}-apple-{darwin,ios}`

Untested:
  * `{i686,x86_64}-unknown-freebsd`
  * `x86_64-unknown-{bitrig,dragonfly,openbsd,netbsd}`
