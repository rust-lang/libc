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
because otherwise there are issues with using it in the item-list-mapping
closures. This constraint seems not feasible lest closures implement Clone.

It would seem closures do implement Clone. Constraining the closure trait any
further with an immutable and recallable trait seems neither feasible. The FnMut
trait is not required, and FnOnce can only be assumed to be a one-off thing.

The only worrying thing about the path manipulation strategy, as well as all
other propositions whose proofs use recursiveness, is that the Rust runtime
ensures not unboundedness of recursion in most cases.

This consideration will be left for later. Current endeavors are rather focused
on the renaming function to complete case handling during single-reexport
resolution, and on the merging function. These are the only two things left.

The renaming case may now more easily proceed by using the path manipulation
proposition. Looking back on it, the refined type for use statements seems like
it could benefit from making the glob injection be a const functor.

Collapsing the contents of a glob seems feasible because it contains no
information of use. Now back to the rename case. In this instance, it is known
that the rename case holds both a source identifier and a rename.

The first thing to do here would be a lookup of the source identifier. If found,
then the returned item (if not a module) would suffer in-place modification of
its path by creating a new (non-module) item.

If the returned item were to be a module, then the renamed identifier alongside
the yield module would have to be used with the path manipulation proposition.
Let the proof be written up to this point.

Further discussion is merited on the side of renaming non-module items. The path
of the item should be changed; One can be sure that post-parsing there are no
items whose paths are empty.

Beyond that, the last segment of the path can be readily removed from the
original path segments. Then the renamed identifier can be pushed anew into the
punctuated list of segments.

This process is mechanical enough for each item type that it could potentially
be automated by means of an MBE. The path field is always available, no matter
the item; All but the final operation is then item-independent.

The last required action is to reconstruct the item with the updated path, and
the updated cache for the path. This should be fairly simple to accomplish. The
only case left to address is that of modules.

When matching against a module in the rename case, the proof should instead use
the path manipulation proposition. The closure assumed true in that proposition
should capture the original path of the subtree root module.

Then for each path passed to the closure, it should extract whatever is left of
the path after the root set of segments, and prepend to that the updated path to
the root module. This goes through potentially caching the updated path.

This idea is still fuzzy; How does one find the base path of the passed root
module in the path of each item? This can be accomplished by first taking the
length of the base path, and then removing that suffix from the path segments.

The length can be obtained from the ancestor module outside of the closure. The
type in syn for representing path segments does not allow removal from the
front. One simple alternative is to perform random-access insertion.

Firstly, the length of the ancestor module's path gets stored outside the
closure. Then an iterator from this length to the length of the path passed in
the closure fetches each segment pertaining only to the item's path.

Then this gets gathered into a vector, the elements of which are iterated over
to be inserted into the updated and cached base path of the root module living
outside the closure.

An even better approach may be to get first the difference of the length of the
item's path with respect to the module's path, and then pop from the back as
many times as that difference indicates.

Then this can be gathered in a vector, to be reversed prior to being iterated
over and added to a new copy of the cached and updated version of the root
module's path (again, living outside the closure.)

Or a simpler idea would be to iterate over all segments of the item's path,
skipping first through the length of the ancestor module's path length. Then the
remaining segments can be gathered into a vector.

The vector would then be folded with a seed value consisting of a copy of the
cached new ancestor path, to which all previoulsy collected, item-specific path
segments would be appended. It would seem the proposition for handling
single-use-statement resolution has had its proof finished. The only thing left
is to provide proof for the merging proposition. In this case, this is known to
iterate through the resolution items, ignoring those that are marked unresolved.
For each resolved item, the associated use statement should be removed from the
module. Beyond that, merging should integrate the items in the container carried
for this one injection of the resolution coproduct, into the destination module.
This, as mentioned during elaboration of the path manipulation function,
requires renaming all paths in both items and module subtrees rooted at the
items to merge, such that everything but their last path segment is replaced
with the destination's module path segment. In other words, merging of resolved
items requires considering the case for non-module items, and the case for
modules. Something similar was already proofed in the rename case of the
single-use-statement resolution proposition. Maybe this is worth abstracting, or
maybe not. Either way, it seems clear items should have everything but their
trailing path segment replaced with the path of the module into which to merge.
Modules need to be fed into the path manipulation proposition, with a closure
that, again, extracts the last segment of the path, and prepends to it the path
of the module into which to merge.

= Idris community tutorial
Before starting with the Idris community tutorial, it would be best to finish
the attempt at providing a proof tha the vector splitting proposition is
correct. This was part of an initial attempt during early hours with Idris.

The proposition requires there being a vector that implies another proposition.
This other proposition assumes a natural and implies a pair of a vector with
some TBD length, and some vector with some other TBD length.

The splitting logic would dictate that the splitting index must fall between 0
and the length of the vector. Constraining that type is likely possible through
finite sets, but it may prove more instructive to do so by hand.

The proposition constraining some natural number to exist within a range should
be indexed by a natural. If one assumes an inductive definition for the type,
there is one vacuously true case; Natural zero is always in bounds.

Thus, one case of the proposition constructs a finite set from the zeroth
natural. One other case follows then; Provided an existing finite set, there
exists another finite set representing the successor to the existing finite set.

This would dictate that some finite set indexed by 2 can be deconstructed into a
finite set that was constructed from a finite set indexed by 1, and that was
itself constructed from a finite indexed by 2.

Would this proposition be enough to reflect that there exists no natural, that
upon being dependently paired with a finite set, would not surpass its index?
This can be readily tested by producing a dependent natural with a finite set.

It would seem the above design for finite sets constrains the natural to be a
specific number, and not to fall within a specific range. Consider the case of a
natural 2, which should fall within the finite set bounded from above at 3.

The construction of the finite set follows the same construction as that of a
natural. The successor to some existing finite set builds from the prior finite
set. This allows not one to retrieve the finite set that would limit a range.

Construction of the proof requires being capable of constructing the type of the
finite set. For some natural, if a finite set represents an upper bound, the
building of the finite set must allow representing the natural.

Except it may very well be that the test case itself is flawed. The type of the
dependent pair specifically uses the dependant natural as the index to the
dependee finite set. This implies the natural must be the upper bound itself.

A better test case may be found if instead the finite set is used to constrain a
natural that exists in another context. An ideal example would seem to be a
length-indexed vector whose implicit for the length were bounded by the set.

It would seem a more precise test case may go through the proofing of an
indexing propostion for a length-indexed vector. This would consider a finite
set that represented some non-zero natural, and a vector with that length.

The finite set would have be indexed by the successor to some natural, and the
vector length would have to also be the successor the same natural. This would
ensure the finite set represents at least numbers greater than zero.

The proofing of this may not be as simple, though. The basecase would seem to
correspond with a proposition with a finite set for the successor of zero. This
would yield the only element in the vector.

This could be assumed to be the inductive hypothesis. The only other case would
be for the finite set to represent the successor of some other finite set, and
thus for the vector to hold more than one element.

In the proof, this would pattern match with the finite set as mentioned above,
and with the vector as a destructuring of its head from its tail. The only issue
to this is that this would always fetch the first element of the vector.

The only possible cases to consider would be those. It seems the finite set as
conceived represents not a bounded natural, but the idea of a bounded natural.
One can not destructure the set into any one natural in the bounded range.

For the indexing to work, the natural indicating the index of the element in the
vector would have to be bounded. The natural would thus require existing as a
dependent pair alongside a proposition indexed by the limit and natural.

This proposition to limit a natural knowing the upper bound seems expressed by
the finite set. The problem with the finite set is that the natural is not
accessbile; Only the limit is known.

A better proposition would have to build on both ideas; It would have to be
indexed by a limit and a natural. The latter would indicate the natural in
question to bound, while the former would indicate the limit of the natural.

This seems a lot like a less-than-or-equal binary relation. It is indexed by two
naturals, and indicates the lhs is strictly smaller than or equal to the rhs.
This idea seems exactly like what vector indexing needs.

One would wonder then what is the point of finite sets, then. It would seem
finite sets alone do not provide much value; std implements a bunch of utilities
for roundtripping between a finite set and a natural.

These utilites deal with integer overloads between naturals and integers, and
then between naturals and finite sets. This sometimes implies use of type system
subversion through belive_me. This is unrelated to the topic at hand.

It would seem providing a finite set then, so long as there existed a way of
converting from a natural to a finite set, is acceptable. This would mean that
the finite set exists within the limits of the vector.

The role of finite sets is then only to provide some bound in the type signature
of a proposition. The actual work of transforming the integer into a natural in
the range denoted by the finite set is delegated to some other function.

This would mean that the finite set is always useless outside of the guarantees
it provides at the type level. There's always need for a supporting
infrastructure to hold up the transformation between a natural and the set.

This then means that a custom finite set is not enough for none of the indexing
nor the vector splitting routine. There seems to be an interface for converting
between integers and some other type, which is used by std's finite sets.

Implementing integer to finite set conversion seems not to be a simple endeavor.
The integer cannot be merely any integer, and neither can it be cast first into
a natural, and pattern-matched to be used in constructing a finite set.

The finite set is already defined in the propositions for the Num interface as
providing a bound for some natural. This means integer overloading could fail.
Most notably, a negative integer would not be good enough.

One should only be capable of constructing a finite set off of an unsigned
integer literal. One could introduce such a constraint prior to the propositions
in the Num interface, and use the constrained integer for overloading.

It would seem setting up constraints prior to introducing the interface
constraint is possible. With that in mind, it then becomes possible to establish
that provided a certain integer n, this integer will only become a finite set if
another proposition determines that this is possible. This other proposition
would imply a type that would serve as proposition requirement and would either
have to be proof-searched or otherwise manually provided. In theory, a proof
search seems feasible because this exists for the sole purpose of handling
overloaded literals.
