# Changelog

## Unreleased

## 0.4.4

* Specify linkage for `__test_fn...()` [#33]
* Remove the use of `mem::zeroed()` from generated code [#35]

[#33]: https://github.com/JohnTitor/ctest2/pull/33
[#35]: https://github.com/JohnTitor/ctest2/pull/35

## 0.4.3

* Add support for long double [#30]

[#30]: https://github.com/JohnTitor/ctest2/pull/30

## 0.4.2

* Allow to subtract in constants [#28]
* Update `rustc_version` to 0.4.

[#28]: https://github.com/JohnTitor/ctest2/pull/28

## 0.4.1

* Fix the `deref_nullptr` warning. [#24]
* Fix unaligned_references warning. [#25]

[#24]: https://github.com/JohnTitor/ctest2/pull/24
[#25]: https://github.com/JohnTitor/ctest2/pull/25

## 0.4.0

* Add support for Haiku
* Update `rustc_version` to 0.3.2.
* MSRV is now 1.34.
* Update crates to edition 2018.

## 0.3.0

* Initial release as `ctest2`.
