<!-- Thank you for submitting a PR!

We have the contribution guide, please read it if you are new here!
<https://github.com/rust-lang/libc/blob/main/CONTRIBUTING.md>

Please fill out the below template.
-->

# Description

<!-- Add a short description about what this change does -->

# Sources

<!-- All API changes must have links to headers and/or documentation,
preferably both -->

# Checklist

<!-- Please make sure the following has been done before submitting a PR,
or mark it as a draft if you are not sure. -->

- [ ] Relevant tests in `libc-test/semver` have been updated
- [ ] No placeholder or unstable values like `*LAST` or `*MAX` are
  included (see [#3131](https://github.com/rust-lang/libc/issues/3131))
- [ ] Tested locally (`cd libc-test && cargo test --target mytarget`);
  especially relevant for platforms that may not be checked in CI
