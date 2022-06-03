use itertools::Itertools;

use super::*;

#[test]
fn test_moves() {
    let mut b = Gameboard::new(4, 5);
    let u = [1, 0];

    fn assert_eq_unordered(a: impl IntoIterator<Item=[usize; 2]>, b: impl IntoIterator<Item=[usize; 2]>) {
        assert!(a.into_iter().sorted().eq(b.into_iter().sorted()));
    }

    assert_eq_unordered(b.dpad4(u), [
        [0, 0],
        [1, 1],
        [2, 0],
    ]);

    assert_eq_unordered(b.dpad8(u), [
        [0, 0],
        [0, 1],
        [1, 1],
        [2, 0],
        [2, 1],
    ]);

    assert_eq_unordered(b.knight_jumps(u), [
        [0, 2],
        [2, 2],
        [3, 1],
    ]);

    assert_eq_unordered(b.rook_moves(u), [
        [0, 0],
        [1, 1],
        [1, 2],
        [1, 3],
        [1, 4],
        [2, 0],
        [3, 0],
    ]);

    assert_eq_unordered(b.bishop_moves(u), [
        [0, 1],
        [2, 1],
        [3, 2],
    ]);

    assert_eq_unordered(b.manhat_jumps(u, 3), [
        [0, 2],
        [1, 3],
        [2, 2],
        [3, 1],
    ]);

    b.block([1, 2]);
    b.block([2, 0]);
    b.block([3, 1]);

    assert_eq_unordered(b.dpad4(u), [
        [0, 0],
        [1, 1],
    ]);

    assert_eq_unordered(b.dpad8(u), [
        [0, 0],
        [0, 1],
        [1, 1],
        [2, 1],
    ]);

    assert_eq_unordered(b.knight_jumps(u), [
        [0, 2],
        [2, 2],
    ]);

    assert_eq_unordered(b.rook_moves(u), [
        [0, 0],
        [1, 1],
    ]);

    assert_eq_unordered(b.bishop_moves(u), [
        [0, 1],
        [2, 1],
        [3, 2],
    ]);

    assert_eq_unordered(b.manhat_jumps(u, 3), [
        [0, 2],
        [1, 3],
        [2, 2],
    ]);
}
