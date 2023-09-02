# resonata
A music theory library for Rust

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
resonata = "0.4.0"
```

## Example

```rust
use resonata::notes::*;

let note = note!("C").unwrap();
let scale = Scale::major(&note);

assert_eq!(scale.to_notes(), vec![
    note!("C").unwrap(),
    note!("D").unwrap(),
    note!("E").unwrap(),
    note!("F").unwrap(),
    note!("G").unwrap(),
    note!("A").unwrap(),
    note!("B").unwrap(),
]);
```

## Documentation

Documentation is available [here](https://docs.rs/resonata).

## Contributing

Contributions are welcome! Please open an issue if you have any questions or
suggestions. Pull requests are welcome too. I also have no idea what I'm doing,
so if you see something that could be done better, please let me know!

## License

MIT