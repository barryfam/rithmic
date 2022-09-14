use std::fmt::Display;

/**
Enable functional-style printing

# Examples
```
use itertools::Itertools;
use rithmic::PrintMethods;

let v = [2, 3, 5, 7];
v.iter().join(" ").println();  // prints "2 3 5 7\n"
```
*/
pub trait PrintMethods: Display
{
    fn format(&self) -> String {
        format!("{}", self)
    }
    fn print(&self) {
        print!("{}", self);
    }
    fn println(&self) {
        println!("{}", self);
    }
    fn eprint(&self) {
        eprint!("{}", self);
    }
    fn eprintln(&self) {
        eprintln!("{}", self);
    }
}

impl<T: Display> PrintMethods for T {}
