Thanks for considering submitting a PR!

Here's a checklist for things that will be checked during review or continuous integration.

- \[ ] Edit corresponding file(s) under `libc-test/semver` when you add/remove item(s)
- \[ ] `rustc ci/style.rs && ./style src`
- \[ ] `cd libc-test && cargo test` (This might fail on your env due to environment difference between your env and CI. Ignore failures if you are not sure.)
- \[ ] Your PR that bumps up the crate version doesn't contain any other changes

Delete this line and everything above before opening your PR.
