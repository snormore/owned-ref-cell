# OwnedRefCell

`OwnedRefCell` is a custom implementation of Rust's [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html) that allows for a different borrowing mechanism. Unlike `RefCell` which grants references tied to the lifetimes of borrow scopes, `OwnedRefCell` returns special owned references. These references maintain their borrowed state until they are explicitly dropped, offering more flexibility in managing lifetimes in complex or dynamic application structures.

![Tests](https://github.com/snormore/owned-ref-cell/actions/workflows/tests.yml/badge.svg)
![Lints](https://github.com/snormore/owned-ref-cell/actions/workflows/lints.yml/badge.svg)
![Docs](https://github.com/snormore/owned-ref-cell/actions/workflows/docs.yml/badge.svg)
[![codecov](https://codecov.io/gh/snormore/owned-ref-cell/graph/badge.svg?token=TGH857JV5B)](https://codecov.io/gh/snormore/owned-ref-cell)

## Features

- **Owned References**: Provides `OwnedRef` and `OwnedRefMut`, which manage the borrow state internally and allow for dynamic and flexible lifetimes.
- **Safe Borrowing**: Ensures safe borrowing at runtime, preventing data races and enabling mutable or immutable access as needed.
- **Easy Integration**: Designed to be a drop-in replacement for scenarios where `RefCell`'s lifetime management is too restrictive.

## Getting Started

Ensure you have Rust installed on your machine. If not, you can install Rust by following the instructions on the [official Rust site](https://www.rust-lang.org/).

### Installation

Add `OwnedRefCell` to your Rust project by including it in your `Cargo.toml` dependencies:

```toml
[dependencies]
owned_ref_cell = { git = "https://github.com/snormore/owned_ref_cell.git" }
```

### Usage

Here is a simple example of how to use `OwnedRefCell`:

```rust
use owned_ref_cell::OwnedRefCell;

fn main() {
    let cell = OwnedRefCell::new(42);

    {
        let value = cell.borrow();
        assert_eq!(*value, 42);
    }

    {
        let mut value = cell.borrow_mut();
        *value = 45;
    }

    {
        let value = cell.borrow();
        assert_eq!(*value, 45);
    }
}
```

## Documentation

Find more detailed documentation [here](https://snormore.github.io/owned-ref-cell), or run the following command to generate docs and view them in your browser:

```bash
cargo doc --open
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests, create issues for bugs and feature requests, and contribute to improving the documentation.

## Support

If you encounter any issues or require assistance, please file an issue on the GitHub repository.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
