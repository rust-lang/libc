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
  mod foo;
  mod test;
  use foo::*;
  ```

/ Module ```rust crate::bar```:
  ```rust
  // use foo::*; // `foo` comes from `barfoo`'s reexport below
  mod barfoo;
  use barfoo::foo::{test, Bar};
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

Following up from the prior example, we can say that the first pass would yield
the following information:

```rust
// module `crate::bar`
use foo::*; // [UNRESOLVED]
mod barfoo;
use barfoo::*; // [RESOLVED]
```

The resulting ```rust FfiItems``` for module ```rust crate::bar``` would contain
one module, and all items from that one child module.

That would leave us with an ```rust FfiItems``` for module ```rust crate::bar```
consisting of two modules; Module `barfoo` and module `barfoo::foo`. But
resolving solely modules this way is useless.

Instead, the algorithm could be something along the lines of (a bit like
Bellman-Ford except without proof of correctness:)

/ Algorithm 1: \
  Inputs:

  - An ```rust FfiItems``` instance with the parsed contents of a full crate.
    The parsed contents must contain both currently parsed items and import
    statements (so just items.)

  Outputs:

  - *Pending*.

  Steps:

  + Match on the list of child modules to the input ```rust FfiItems```.

    - If there are no child modules, return unity (*Pending*.)

    - If there are any modules, extract the next module.

      + Match on the list of reexports in the extracted module.

        - If there are no reexports, proceed as follows.

          + Run algorithm 1. Set its input to be a new ```rust FfiItems``` with
            its modules as the tail list of the current list of modules, barring
            the extracted module.

        - If there are any reexports, extract the next reexport.

          + Run algorithm 2. Set the input import statement to be the extracted
            reexport. Set the input ```rust FfiItems``` to be the current
            input's ```rust FfiItems```.

/ Algorithm 2: \
  Inputs:

  - An import ```rust use``` statement.

  - A base ```rust FfiItems``` corresponding to the module where the above
    import statement lives at.

  Outputs:

  - A list of a coproduct type. The type considers two data constructors; One
    for _resolved_ modules, and another for _unresolved_ modules. The former
    takes a single parameter of type ```rust FfiItems```.

    This returns a list instead of a single ```rust FfiItems``` instance because
    a given ```rust use``` statement could refer to a group in its tail segment.
    Each element of the group could itself expand to an arbitrary reexport.

  Steps:

  + Match against the type of input import.

    - If the import is a path, proceed as follows.

      + Match against the list of modules in the input ```rust FfiItems```,
        searching for the leftmost extracted segment of the input import's path.

        - If the list of modules contains a match against the path, proceed as
          follows.

          + Run algorithm 2. Set the input ```rust use``` statement to be the
            rhs of the current import statement. Set the input
            ```rust FfiItems``` to be the match module.

          + Return the result of step 1.a.1.a.1.

        - If the list of modules does not contain a match, proceed as follows.

          + Return a single-element list. Its one element should consist of the
            value returned from calling the _unresolved_ data constructor.

    - If the import is an identifier or a renamed identifier, proceed as
      follows.

      + Match against the input ```rust FfiItems```'s list of items.

        - If a match is found for the identifier or original identifier (in the
          case of a rename,) proceed as follows.

          + Return a singleton list. Its one element should consist of a new
            ```rust FfiItems``` instance containing solely the found item,
            wrapped by a _resolved_ data constructor.

        - If no match is found, proceed as follows.

          + Return a singleton list. The element should consist of the value
            returned from calling the _unresolved_ data constructor.

    - If the import is a glob, proceed as follows.

      + Return a single-element list. Its one element should wrap the input
        ```rust FfiItems``` instance with a _resolved_ data constructor.

    - If the import is a group, proceed as follows.

      + Run algorithm 3. Set the input list to be the matched group. Set the
        input ```rust FfiItems``` to be the current input's ```rust FfiItems```.

      + Return the result of step 1.d.1.

/ Algorithm 3: \
  Inputs:

  - A list of grouped elements in the tail of an import statement.

  - An ```rust FfiItems``` instance where the reexport from which the above
    input is sourced (i.e. the instance containing the whole ```rust use```
    statement.)

  Outputs:

  - A list of ```rust FfiItems``` wrapped with the same sum type as outlined in
    the outputs of algorithm 2.

  Steps:

  + Match against the input list.

    - If there are no elements left in the group, return the empty list.

    - If there are any elements left, extract the next element and proceed as
      follows.

      + Run algorithm 2. Set the input import statement to be the extracted
        element. Set the input ```rust FfiItems``` to be the current input
        ```rust FfiItems```.

      + Run algorithm 3. Set te input list to be the tail list of elements after
        extracting the above element. Set the ```rust FfiItems``` instance to be
        the same input instance as we currently have as input.

      + Return the resulting list from appending the lists from step 1.b.1 to
        the lists from step 1.b.2.
