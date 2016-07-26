compound
----
This crate calculates the value of the initial investment based on rates of return (daily, monthly, semi-annual, annual...) :moneybag:.

## Usage

```rust
extern crate compound;
use compound::compound;

fn main() {
    let percentages = [10., 11., 23., 34.];
    let initial_investment: f64 = 1000.00;
    let balance = compound( &initial_investment, &percentages);

    println!("Your balance is {}", balance );  // Your balance is 2012.45
    println!("The percentages were {:?}", percentages);  // The percentages were [10, 11, 23, 34]
}
```

## Raison d'être
To make the calculation of how much your investment is worth super easy. Just pass by reference `$` the initial investment and the array of percentage returns (a slice or vector). Algorithm complexity O(n).

> Super important: all values inside the vector or slice need to be floats (f64) otherwise your operation will fail.

Super Light (5 sloc). No dependencies.

## License
MIT © [Mohamed Hayibor](https://github.com/mohamedhayibor)
