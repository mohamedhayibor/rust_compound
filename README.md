# compound ![](https://travis-ci.org/mohamedhayibor/rust_compound.svg?branch=master)

This crate calculates the value of the initial investment based on rates of return (daily, monthly, semi-annual, annual...) :moneybag:.

## Usage

```rust
extern crate compound;
use compound::compound;

fn main() {
    let percentages = [10., 11., 23., 34.];
    let initial_investment: f64 = 1000.00;
    let balance = compound( initial_investment, &percentages);
    // alternative > let balance  = compound(1000., &percentages);

    println!("Your balance is {}", balance );  // Your balance is 2012.45
    println!("The percentages were {:?}", percentages);  // The percentages were [10, 11, 23, 34]
}
```

## How to include it in your project

In your project's `Cargo.toml` include the package name and version under like so:
```rust
[dependencies]
compound = "0.1.0"
```

## Raison d'être
To make the calculation of how much your investment is worth super easy. Just pass in the initial investment and the array of percentage returns by (a slice or vector) by reference Algorithm complexity O(n).

> Super important: all values inside the vector or slice need to be floats (f64) otherwise your operation will fail.

Super Light (5 sloc). No dependencies.

## License
MIT © [Mohamed Hayibor](https://github.com/mohamedhayibor)
