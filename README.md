Checks if allocator consumers use the allocator in a way that makes the layout's alignment statically known in the `allocate` function.

Results:
```
> just
Is the alignment statically known in the `allocate` method?
vec_with_capacity: YES
vec_push: NO
hash_set_with_capacity: YES
hash_set_insert: YES
```