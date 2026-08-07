<!-- Thank you for submitting a PR!

We have the contribution guide, please read it if you are new here!
<https://github.com/rust-lang/libc/blob/main/CONTRIBUTING.md> -->

## Description

<!-- Add a short description about what this change does, or say "See commit
messages" if details are there. -->

## Checklist

<!-- Please make sure the following has been done before submitting a PR,
or mark it as a draft if you are not sure. See the pull request guidelines
for help
<https://github.com/rust-lang/libc/blob/main/CONTRIBUTING.md#submitting-a-pull-request>.
-->

- [ ] Relevant tests in `libc-test/semver` have been updated
- [ ] Commit messages permalink to headers for added or changed API
- [ ] Placeholder or unstable values like `*LAST` or `*MAX` have the standard
  doc comment
- [ ] Tested locally (`cargo test -p libc-test --target mytarget`);
  especially relevant for platforms that may not be checked in CI

<!-- labels: is this PR a breaking change? If it is, delete the following
line: -->
@rustbot label +stable-nominated
