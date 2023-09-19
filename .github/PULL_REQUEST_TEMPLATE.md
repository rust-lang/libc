Thanks for considering submitting a PR!

Here's a checklist for things that will be checked during review or continuous integration.

- \[ ] Edit corresponding file(s) under `libc-test/semver` when you add/remove item(s)
- \[ ] Your PR doesn't contain any *unstable* values like `*LAST` or `*MAX` (see [#3131](https://github.com/rust-lang/libc/issues/3131))
- \[ ] If your PR increments version number, it must not contain any other changes
- \[ ] `rustc ci/style.rs && ./style src`
- \[ ] `cd libc-test && cargo test`
  - (this might fail on your env due to environment difference between your env and CI. Ignore failures if you are not sure)

Delete this line and everything above before opening your PR.
