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

  - An ```rust FfiItems``` with the parsed contents of a full crate. This will
    be referred to interchangeably as both the current module and the current
    module's ```rust FfiItems``` in the steps below.

  Outputs:

  - *Pending*.

  Steps:

  + Match on the list of child modules to the current module.

    - If there are no child modules, return unity (*Pending*.)

    - If there are any modules, extract the next module.

      + Match on the list of reexports in the extracted module.

        - If there are no reexports, run algorithm 1. Set its input to be the
          current ```rust FfiItems``` with its modules as the tail list of the
          current list of modules (i.e. discard the extracted
          ```rust FfiItems```.)

        - If there are any reexports, extract the next reexport.

          + Run algorithm 3. Set the input import to the extracted reexport.

          + Match on the result of step 1.b.1.b.1.

            - If matching against a _glob_ reexport type, proceed as follows.

              + Run algorithm 2. Set the input to algorithm 2 to be the
                extracted reexport, and the current input to algorithm 1.

            - If matching against a _specific_ reexport type, proceed as
              follows.

              + *Pending*.

          + Repeat from step 1.b.1 with the tail list of reexports.

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
            ```rust FfiItems``` to be the match found in step 1.a.1.a.

          + Return the result of step 1.a.1.a.1.

        - If the list of modules does not contain a match, return a
          single-element list. The element should consist of the value returned
          from calling the _unresolved_ data constructor.

    - If the import is an identifier or a renamed identifier, proceed as
      follows.

      + Match against the input ```rust FfiItems```'s list of items.

        - If a match is found for the identifier or original identifier (in the
          case of a rename,) proceed as follows.

          + Return a single-element list. The element should consist of a new
            ```rust FfiItems``` instance containing solely the found item,
            wrapped by a _resolved_ data constructor.

        - If no match is found, return a single-element list. The element should
          consist of the value returned from calling the _unresolved_ data
          constructor.

    - If the import is a glob, return a single-element list. The element should
      wrap the input ```rust FfiItems``` instance with a _resolved_ data
      constructor.

    - If the import is a group, proceed as follows.

      + Match on the next element of the group.

        - If there are no elements left, return the empty list.

        - If there are any elements left, extract the next element and proceed
          as follows.

          + Match against the extracted element's import type.

            - If the import is a path, extract the path and proceed as follows.

              + Match against the list of modules of the input
                ```rust FfiItems```.

                - If a match is found for the path segment or identifier,
                  proceed as follows.

                  + Run algorithm 2. Set the input import statement to be the
                    extracted path. Set the input ```rust FfiItems``` to be the
                    matched module among the current input's children.

                - If a match is not found for the path segment or identifier,
                  proceed as follows.

                  + Call the _unresolved_ data constructor.

            - If the import is an identifier (or a renamed identifier), extract
              the (original) identifier and proceed as follows.

              + *Pending*.

/ Algorithm 3: \
  Inputs:

  - An import used in a ```rust use``` statement.

  Outputs:

  - The type of reexport the input ```rust use``` statement was. This can be one
    of a _glob_ reexport or a _specific_ reexport.

  Steps:

  + Match against the type of input import path.

    - If the import is a path, run algorithm 3. Set the input path to be the
      newly-found rightmost import.

    - If the import is a glob, return a _glob_ reexport type.

    - If the import is an identifier or a renamed identifier, return a
      _specific_ reexport type.

    - If the import is a group, proceed as follows.

      + Run a list mapping algorithm over the list of elements in the group. Set
        the transform to be algorithm 3.

      + Run a list reduction algorithm over the result of step 1.d.1. Set the
        transform to be algorithm 4.

      + Match against the result of step 1.d.2.

        - If the reduction yield some value, return the value.
        - Otherwise, return a _specific_ reexport type.

/ Algorithm 4: \
  Inputs:

  - A reexport type as described in the outputs of algorithm 3.
  - A reexport type as described in the outputs of algorithm 3.

  Outputs:

  - A reexport type as described in the outputs of algorithm 3.

  Steps:

  + Match against an ordered pair of the two inputs.

    - If the leftmost element or the rightmost element are _glob_ reexport
      types, return a _glob_ reexport type.

    - Otherwise, return a _specific_ reexport type.
