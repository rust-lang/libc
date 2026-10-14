>Like what? The rules for how `Send` and `Sync` are automatically
>implemented are pretty clear, and docs already indicate whether a type
>implements these or not.

Sure it may be in the docs, but the section of rustdoc on blanket impls
seems to me to not be nearly as often looked at as other impls.

It would thus seem that libc users would likely not realize until they
got a compile error. Granted, that already serves as a "fix," so it may
just be enough.

The usage guidelines could mention that we may sweep in some such
changes across non-breaking SemVer releases. Then folks would be more
actively on the look out of types becoming `!Send`.
