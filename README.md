compound
----
This crate calculates the value of the initial investment based on rates of return (daily, monthly, semi-annual, annual...) :moneybag:.

## Usage

```rs
use compound;

fn main() {
    let percentages = [10., 11., 23., 34.];
    let initial_investment: f64 = 1000.00;
    let balance = compound( &initial_investment, &percentages);

    println!("Your balance is {}", balance );
    println!("The percentages were {:?}", percentages);
}
```

## Raison d'être
To make the calculation of how much your investment is worth super easy. Just pass by reference `$` the initial investment and the array of percentage returns (a slice or vector). Algorithm complexity O(n).

Super Light (5 sloc). No dependencies.

## License
MIT © [Mohamed Hayibor](https://github.com/mohamedhayibor)
