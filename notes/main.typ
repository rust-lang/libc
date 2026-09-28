#import "@local/scratchpad:0.1.4": *

#show: template.with(title: [Notes on extending `ctest`])

#title()

= On the design and implementation of the rename proposition
The function is known to be needed both in the merging function and in the
rename case of the single-reexport resolution proposition. The proposition can
assume true a closure, and imply another proposition from a module to a module.

The closure would be passed the path to some item, and it would be expected to
return another path. The new path would then be used to replace that of the item
while performing a traversal of the assumed module tree.

Both values may be passed by value to the proposition, and the closure may take
as well a fully-owned path, and return a fully-owned path. The proof would be
shaped as a single-run of the closure over the root, and a map of the children.

The path of the module would first be passed to the closure. Then a new module
would be produced, where the path would correspond with that returned by the
clsoure. The map over the child items would repeat the same things.

Note the traversal would be required to go through all items in the subtree
rooted at the passed module. This implies that there ought be multiple maps for
each of the containers used for each type of item.

Recusion would take place when the container being handled would correspond with
that of child modules to the currently traversed module. For each of those, the
same logic would follow, but additionally the mapping would take place anew.

This hints at the possibility to abstract away the module-pass in a single inner
function that would take the module and the closure, and yield the updated
module tree rooted at the passed module.

This is exactly what the whole renaming proposition would do. This implies that
the logic should only consist of performing the update over the module path,
then repeating the same thing over the child items, recursing with the modules.

The only detail of the proof that remains to be specified is the behavior post
recursing with each of the child modules. The returned module would be assigned
instead to the prior module. This means there is no need to rename the child.

The renaming of the module's path will already take place within the next call
stack frame. There is one thing that remains to be specified; The module path
exists not only in the corresponding field, but also in a cached form.

The cached form would also need updating after the path were changed. This would
seem to be readily possible with the proposition that is also used at parse-time
to yield the same paths serving as a stringified cache of the data structure.

It would seem the proof is complete; There is something odd with it, though. Let
there be a review of the path manipulation strategy that would be used when
handling renames in the single-reexport resolution proposition.

Renaming the last segment of all paths appearing in a given module subtree would
need to change the last segment of that tree's root, followed by a change of all
subsequent child items' path segments corresponding to the ancestor's old ident.

This could be accomplished by capturing in the closure the original path of the
ancestor module. Then whatever followed from the path passed to the clsoure post
extracting this path would be reappended to the modified ancestor's path.

So to picture an example; Consider path foo::bar. The module tree rooted at
module bar should be renamed to baz. This would mean that all items whose paths
started with foo::baz in that tree would first extract the path after this.

For example, if there is some item with path foo::bar::foobar::Foo, the closure
would extract the path after the original ancestor's foo::bar path. This would
correspond with foobar::Foo.

Then this would be appended to the result of modifying the root module's path to
foo::baz. The end result would be path foo::baz::foobar::Foo. Of course, it
could be that there is no path after the root path, like the root module itself.

In those cases, the append opeeration would yield only the modified root path.
Notably, the Rust runtime has no way of knowing that the path being extracted
from the path passed to the closure is prepended by the root module's path.

This will require some potential unreachable! calls, but that should be
acceptable. Once the proposition for handing use-trees with renames in them is
discussed, the details of the proof will be provided.

Something that remains to be discussed from the proofing of the path
manipulation proposition is that there exists a potential issue with the way it
handles the traits implemented by the passed closure.

The trat implementation of the passed closure is also required to be Clone
because otherwise there are issues with using it in the item list mappings
closures. This constraint seems not feasible lest closures implement Clone.

It would seem closures do implement Clone. Constraining the closure trait any
further than with an immutable and recallable trait seems neither feasible. The
FnMut trait is not required, and FnOnce is only known to be a one-off thing.
