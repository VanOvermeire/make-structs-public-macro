# Make Public Macro

Rust macro that makes all the fields of a struct public.

I did not publish this code as a crate because I noticed something [like this already exists](https://github.com/yuchunzhou/public).

```rust
// somewhere in your `main.rs`:
#[macro_use]
extern crate make_fields_public;

// ...

#[public]
struct Example {
    first: i32,
    second: i32,
}
```

Will change the given struct into:

```rust
pub struct Example {
    pub first: i32,
    pub second: i32,
}
```

Also see `test_structs.rs`.

## Nested structs

If you want nested structs to also be visible, make sure to add the macro to these nested structs as well. Again, see the examples in `test_structs.rs`.
