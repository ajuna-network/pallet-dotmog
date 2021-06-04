# DOTMog Pallet

This is the DOTMog pallet which lives as its own crate so it can be imported into multiple runtimes.

## Purpose

This pallet acts as the main pallet for the game DOTMog.

## Dependencies

### Traits

This pallet does not depend on any externally defined traits.

### Pallets

This pallet does not depend on any other FRAME pallet or externally developed modules.

## Installation

### Runtime `Cargo.toml`

To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
# external pallets
pallet-dotmog = {default-features = false, version = '0.1.0', git = 'https://github.com/dotmog/pallet-dotmog.git'}
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
    'pallet-dotmog/std',
]
```

### Runtime `lib.rs`

You should implement it's trait like so:

```rust
TODO
/// Used for test_module
impl pallet_dotmog::Config for Runtime {
	type Event = Event;
}
```

and include it in your `construct_runtime!` macro:

```rust
DotMogModule: pallet_dotmog::{Pallet, Call, Storage, Event<T>},
```

### Genesis Configuration

This dotmog pallet does have a genesis configuration.

```rust
TODO
```

## Reference Docs

You can view the reference docs for this pallet by running:

```
cargo doc --open
```
