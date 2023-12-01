Thanks for considering submitting a PR!

We have the [contribution guide](https://github.com/rust-lang/libc/blob/main/CONTRIBUTING.md). Please read it if you're new here!

Here's a checklist for things that will be checked during review or continuous integration.

- \[ ] Edit corresponding file(s) under `libc-test/semver` when you add/remove item(s), e.g. edit `linux.txt` if you add an item to `src/unix/linux_like/linux/mod.rs`
- \[ ] Your PR doesn't contain any private or *unstable* values like `*LAST` or `*MAX` (see [#3131](https://github.com/rust-lang/libc/issues/3131))
- \[ ] If your PR has a breaking change, please clarify it
- \[ ] If your PR increments version number, it must NOT contain any other changes (otherwise a release could be delayed)
- \[ ] Make sure `ci/style.sh` passes
- \[ ] `cd libc-test && cargo test`
  - (this might fail on your env due to environment difference between your env and CI. Ignore failures if you are not sure)

Delete this line and everything above before opening your PR.
