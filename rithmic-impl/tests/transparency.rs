#![feature(decl_macro, rustc_attrs)]

use rithmic_impl::transparent;

#[test]
fn test_transparent() {
    #[transparent]
    macro f($x:expr) {
        $x + y
    }

    let y = 3;
    assert_eq!(f!(2), 5);
}
