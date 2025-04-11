<!-- Thank you for submitting a PR!

We have the contribution guide, please read it if you are new here!
<https://github.com/rust-lang/libc/blob/main/CONTRIBUTING.md>

Please fill out the below template.
-->

# Description

<!-- Add a short description about what this change does -->

# Sources

<!-- All API changes must have permalinks to headers. Common sources:

* Linux uapi https://github.com/torvalds/linux/tree/master/include/uapi
* Glibc https://github.com/bminor/glibc
* Musl https://github.com/bminor/musl
* Apple XNU https://github.com/apple-oss-distributions/xnu
* Android https://cs.android.com/android/platform/superproject/main

After navigating to the relevant file, click the triple dots and select "copy
permalink" if on GitHub, or l-r (links->commit) for the Android source to get a
link to the current version of the header.

If sources are closed, link to documentation or paste relevant C definitions.
-->

# Checklist

<!-- Please make sure the following has been done before submitting a PR,
or mark it as a draft if you are not sure. -->

- [ ] Relevant tests in `libc-test/semver` have been updated
- [ ] No placeholder or unstable values like `*LAST` or `*MAX` are
  included (see [#3131](https://github.com/rust-lang/libc/issues/3131))
- [ ] Tested locally (`cd libc-test && cargo test --target mytarget`);
  especially relevant for platforms that may not be checked in CI
