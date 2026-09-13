#import "@local/scratchpad:0.1.4": *

#show: template.with(title: [Notes on extending `ctest`])

#title()

Extending `ctest` requires keeping track of the ```rust use``` statements while
parsing to get right the item visibility across modules. We could very well be
producing tests for an item without it being used in its "source" module.

This, though, shouldn't matter much. The module from which we parsed the item
should be enough to test it; The problem comes when filtering. Folks expect to
filter items that are potentially reexported.

Suppose somebody sets up a skip for a record `Foo`, and trusts that there will
be an item path `bar::Foo` that will match this item. The item is defined in
module `crate::bar::foo::Foo`, but reexported in module `bar`.

While going through the skips in the `ctest` internal logic, we find that this
item will never get a hit. The path associated with it is `bar::foo::Foo`, and
not `bar::Foo`.

Solving this particular need could go through parsing as well reexports, and
scanning them afterwards. This second pass would add "synonyms" to the item
paths of each parsed item. Skips would run against each of those paths.

This is assuming our only needs are concerned with filtering. This is not
necessarily the case. Or maybe it is. `ctest` parses all items in a crate. Then
it generates tests that refer to items on the Rust side of things.

There is one more usecase for ```rust use```-statement parsing. Suppose somebody
sets up a record that is not public under path `crate::foo::Bar`. Then they
reexport it under path `crate::Bar`.

Then suppose they call ```rust TestGenerator::skip_private```. We are again
screwed big time if the Rust tests refer to the item through the private item
path. Or are we, now? How does that option work when not set?

Apparently, I do not have to worry about item resolution. We can refer to both
public and private items in the generated tests. This means the only thing that
needs to work with reexports are filters.

This means that function ```rust TranslateHelper::filter_ffi_items``` may be the
only thing that needs changing. That function filters out all items that will be
tested, so we can also recursively filter items there.

The simplest approach that comes to mind to have multiple identifiers assigned
to a given symbol is to perform multiple passes. Once all symbols are parsed
into `FfiItems`, we make one full traversal per parsed item.

Each full traversal of the same `FfiItems` ensures for each given item, we keep
all paths that could be used to refer to that item in scope. Once `Ffiitems` is
parsed, it will not change anymore until we filter out elements.

This is not the best approach when it comes to performance, but it will do for
an initial implementation. In terms of memory storage, to avoid funny allocation
issues, we can clone `FfiItems` initially.

The cloned `FfiItems` is then used for the passes; The original `FfiItems` gets
its parsed items modified with alternative paths. Seems fair enough. We fully
own the items within it at filter-time, so the plan seems feasible.

The problem comes when you think about item resolution in those secondary
passes. It is non-trivial to implement because at module
```rust crate::bar::foo``` you can have the following code:

```rust
pub struct Bar;
```

Then back at module ```rust crate::bar```, you can have the following code.

```rust
mod foo;

use foo::*;
```

But you could also have this:

```rust
mod foo;

use self::foo::*;
```

And for that matter, you could have instead module
```rust crate::bar::barfoo::foo```, with the following layout:

/ Module ```rust crate::bar::barfoo::foo```:
  ```rust
  pub struct Bar;
  ```

/ Module ```rust crate::bar::barfoo```:
  ```rust
  mod foo:
  use foo::*;
  ```

/ Module ```rust crate::bar```:
  ```rust
  use foo::*;
  mod barfoo;
  ```

The above situation is one of a number of potentially complex item resolution
scenarios that I would have to deal with. Maybe there is some library that does
this for me, or maybe I can use some library straight from `rustc`.

It seems like `rustc_resolve` does just this, but it is not made for use outside
the compiler, so it is not available in `crates.io`. The next best thing would
be to inline each of these, and perform multiple passes per module.

Inlining is not enitrely clear to me right now, but the multi-passes would work
a lot like Typst's convergence algorithm. One pass yields modules we know are
child modules to the current one, and modules we don't know about.

Then we can assume all imports that have not been resolved yet need special
attention in subsequent passes. After the first pass, we have expanded the
"importable" items, so we should be capable of resolving all "delayed" imports.

This is just theory, though.
