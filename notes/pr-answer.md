>Is there any fallout from this? It is technically correct by the rule
>that pointers are not `Send`/`Sync`, so I'm inclined to leave it.
>Unfortunately these auto traits are a bit fragile for `libc` types in
>general since the occasional addition of a pointer field does happen.
>
>> 1. Maybe a commit history rewrite would be in order.
>
>Unless some exceptional circumstance happens, there is no rewriting
>history.

Then maybe adding some comments about this to the usage guidelines would
do?
