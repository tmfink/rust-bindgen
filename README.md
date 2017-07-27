# `bindgen`

**`bindgen` automatically generates Rust FFI bindings to C and C++ libraries.**

For example, given the C header `doggo.h`:

```c
typedef struct Doggo {
    int many;
    char wow;
} Doggo;

void eleven_out_of_ten_majestic_af(Doggo* pupper);
```

`bindgen` produces Rust FFI code allowing you to call into the `doggo` library's
functions and use its types:

```rust
/* automatically generated by rust-bindgen */

#[repr(C)]
pub struct Doggo {
    pub many: ::std::os::raw::c_int,
    pub wow: ::std::os::raw::c_char,
}

extern "C" {
    pub fn eleven_out_of_ten_majestic_af(pupper: *mut Doggo);
}
```

## Users Guide

[📚 Read the `bindgen` users guide here! 📚](https://rust-lang-nursery.github.io/rust-bindgen)

## API Reference

[API reference documentation is on docs.rs](https://docs.rs/bindgen)

## Contributing

[See `CONTRIBUTING.md` for hacking on `bindgen`!](./CONTRIBUTING.md)
