# ffor
a lightning fast, alternative to the builtin ```for``` in rust

- has 5 versions: ffor8, ffor16, ffor32, ffor64, ffor128
- uses ```#![no_std]```
 

## sometimes even the defaults are bloated
**use:**
\
in ```Cargo.toml```
```rust
[dependencies]
ffor = "0.0.5"
```
in ```main.rs```
```rust
let mut i = 0;
while i != -1 {
    // Your speed-demon logic here
    i = ffor32(10, i);
}
```
