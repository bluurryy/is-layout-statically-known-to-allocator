default:
    @ cargo asm -V > /dev/null 2> /dev/null || { echo "\`cargo-show-asm\` must be installed to run these tests: https://crates.io/crates/cargo-show-asm"; exit 1; }
    @ echo "Is the alignment statically known in the \`allocate\` method?"
    @ just check vec_with_capacity
    @ just check vec_push
    @ just check hash_set_with_capacity 
    @ just check hash_set_insert 

check function:
    @ echo -n "{{function}}: "; asm=$(cargo asm --simplify --context=1000 {{function}} 2> /dev/null) && { echo "$asm" | grep DOES_NOT_INLINE_ALLOCATE_FUNCTION > /dev/null && echo "NO" || echo "YES"; } || echo "FAILED TO PRODUCE ASM"