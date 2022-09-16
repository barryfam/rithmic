/**
Unwrap a [`Result<T, T>`](Result), ignoring the `Ok`/`Err` status

Primarily for use with [`slice::binary_search`], to use the index regardless of whether the value is found or not

# Examples
```
use rithmic::UnwrapAny;

let a = [2, 3, 5, 7, 11];
assert_eq!(a.binary_search(&6).unwrap_any(), 3);
assert_eq!(a.binary_search(&7).unwrap_any(), 3);
```
*/
pub trait UnwrapAny {
    type T;
    fn unwrap_any(self) -> Self::T;
}

impl<T> UnwrapAny for Result<T, T> {
    type T = T;
    fn unwrap_any(self) -> T {
        match self {
            Ok(x) => x,
            Err(x) => x
        }
    }
}
