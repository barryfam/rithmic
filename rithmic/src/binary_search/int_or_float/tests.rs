use super::*;

#[test]
fn test_midpoint_basic() {
    assert_eq!(1.midpoint(4), 2);
    assert_eq!(4.midpoint(1), 3);
    assert_eq!((-3).midpoint(-4), -3);
    assert_eq!((-4).midpoint(-3), -4);
    assert_eq!((-3).midpoint(4), 0);
    assert_eq!(4.midpoint(-3), 1);
}

#[test]
fn test_midpoint_overflow() {
    assert_eq!(1_000_000_000_i32.midpoint(1_500_000_000), 1_250_000_000);
    assert_eq!((-1_000_000_000_i32).midpoint(1_500_000_000), 250_000_000);

    assert_eq!(f64::MAX.midpoint(f64::MAX/4.*3.), f64::MAX/8.*7.);
    assert_eq!(f64::MIN.midpoint(f64::MIN/4.*3.), f64::MIN/8.*7.);
}

macro test_midpoint_brute_int($name:ident, $type:ty) {
    #[test]
    fn $name()
    {
        for i in <$type>::MIN..<$type>::MAX
        {
            let mut m_floor = i;
            let mut m_ceil = i;
            let mut odd_step = true;

            for j in i+1..=<$type>::MAX
            {
                if odd_step { m_ceil += 1; }
                else { m_floor += 1; }
                odd_step ^= true;

                assert_eq!(i.midpoint(j), m_floor);
                assert_eq!(j.midpoint(i), m_ceil);
            }
        }
    }
}

test_midpoint_brute_int!(test_midpoint_brute_u8, u8);
test_midpoint_brute_int!(test_midpoint_brute_i8, i8);
