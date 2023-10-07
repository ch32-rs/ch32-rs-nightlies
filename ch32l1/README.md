# ch32l1
This crate provides an autogenerated API for access to CH32L1 peripherals.
The API is generated using [svd2rust] with patched svd files containing
extensive type-safe support. For more information please see the [main repo].

Refer to the [documentation] for full details.

[svd2rust]: https://github.com/rust-embedded/svd2rust
[main repo]: https://github.com/ch32-rs/ch32-rs
[documentation]: https://docs.rs/ch32l1/latest/ch32l1/

## Usage
Each device supported by this crate is behind a feature gate so that you only
compile the device(s) you want. To use, in your Cargo.toml:

```toml
[dependencies.ch32l1]
version = "0.1.7"
features = ["ch32l103", "critical-section"]

[dependencies.riscv]
version = "0.10.1"
features = ["critical-section-single-hart"]
```

In your code:

```rust
use ch32l1::ch32l103;

let mut peripherals = ch32l103::Peripherals::take().unwrap();
let gpioa = &peripherals.GPIOA;
gpioa.odr.modify(|_, w| w.odr0().set_bit());
```

For full details on the autogenerated API, please see:
https://docs.rs/svd2rust/0.29.0/svd2rust/#peripheral-api

## Supported Devices

| Module | Devices | Links |
|:------:|:-------:|:-----:|
