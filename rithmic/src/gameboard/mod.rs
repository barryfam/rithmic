#[cfg(test)] mod tests;

use tinyvec::ArrayVec;

use crate::BVec;

/**
The `Gameboard` `struct` represents a rectangular grid of squares, some of which may be "blocked", and provides convenience methods for determining what squares can be reached by various movement patterns
*/
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Gameboard {
    pub h: usize,
    pub w: usize,
    pub blocked: Option<BVec>,
}

impl Gameboard
{
    /// Create a new [`Gameboard`] with `h` rows and `w` columns. No squares are blocked initially
    pub fn new(h: usize, w: usize) -> Self {
        Self { h, w, blocked: None }
    }

    /// Create a new [`Gameboard`] from a [`Vec<Vec<T>>`](std::vec::Vec). Any `blocked[i][j] == block` means `[i, j]` is blocked; any other values are ignored
    pub fn from_vec_vec<T: PartialEq>(blocked: Vec<Vec<T>>, block: T) -> Self {
        let h = blocked.len();
        let w = blocked[0].len();
        let mut b = BVec::new(h*w);
        for i in 0..h {
            assert_eq!(w, blocked[i].len());
            for j in 0..w {
                if blocked[i][j] == block {
                    b.set(i*w+j, true);
        }}}
        Self { h, w, blocked: Some(b) }
    }

    /// Check if `[i, j]` is within the board dimensions. Blocks are **not** checked
    pub fn inbounds(&self, [i, j]: [usize; 2]) -> bool {
        i < self.h && j < self.w
    }

    /// Set a block at `[i, j]`. No effect if already blocked
    pub fn block(&mut self, [i, j]: [usize; 2]) {
        self.blocked.get_or_insert_with(|| BVec::new(self.h * self.w))
            .set(i * self.w + j, true)
    }

    /// Remove any block at `[i, j]`. No effect is there is no block there
    pub fn unblock(&mut self, [i, j]: [usize; 2]) {
        self.blocked.get_or_insert_with(|| BVec::new(self.h * self.w))
            .set(i * self.w + j, false)
    }

    /// Check if there is currently a block at `[i, j]`
    pub fn is_blocked(&self, [i, j]: [usize; 2]) -> bool {
        self.blocked.as_ref().is_some_and(|b| b[i * self.w + j])
    }

    /// Move `dist` squares in a straight line. Blocks along the way are ignored, but a block at the destination, or the destination being out-of-bounds, returns `None`. `dir` must be in `0..4` and indicates direction:
    ///
    /// 0. Positive `i`
    /// 1. Positive `j`
    /// 2. Negative `i`
    /// 3. Negative `j`
    pub fn dir_jump(&self, [i, j]: [usize; 2], dir: u8, dist: usize) -> Option<[usize; 2]> {
        debug_assert!(self.inbounds([i, j]));

        match dir {
            0 => (i+dist < self.h).then(|| [i+dist, j]),
            1 => (j+dist < self.w).then(|| [i, j+dist]),
            2 => (i >= dist)      .then(|| [i-dist, j]),
            3 => (j >= dist)      .then(|| [i, j-dist]),
            _ => panic!("Invalid direction")
        }
        .filter(|&v| !self.is_blocked(v))
    }

    /// Move `dist` squares in a 45° diagonal line. Blocks along the way are ignored, but a block at the destination, or the destination being out-of-bounds, returns `None`. `dir` must be in `0..4` and indicates direction:
    ///
    /// 0. Positive `i` and `j`
    /// 1. Negative `i`, positive `j`
    /// 2. Negative `i` and `j`
    /// 3. Positive `i`, negative `j`
    pub fn diag_jump(&self, [i, j]: [usize; 2], dir: u8, dist: usize) -> Option<[usize; 2]> {
        debug_assert!(self.inbounds([i, j]));

        match dir {
            0 => (i+dist < self.h && j+dist < self.w).then(|| [i+dist, j+dist]),
            1 => (i      >= dist  && j+dist < self.w).then(|| [i-dist, j+dist]),
            2 => (i      >= dist  && j      >= dist ).then(|| [i-dist, j-dist]),
            3 => (i+dist < self.h && j      >= dist ).then(|| [i+dist, j-dist]),
            _ => panic!("Invalid direction")
        }
        .filter(|&v| !self.is_blocked(v))
    }

    /// Of the four squares orthogonally adjacent to `[i, j]`, return those that are within the board bounds and not blocked
    pub fn dpad4(&self, [i, j]: [usize; 2]) -> ArrayVec::<[[usize; 2]; 4]> {
        let mut v = ArrayVec::new();
        for dir in 0..4 {
            if let Some(vv) = self.dir_jump([i, j], dir, 1) {
                v.push(vv);
            }
        }
        v
    }

    /// Of the eight squares surrounding `[i, j]` (orthogonally and diagonally), return those that are within the board bounds and not blocked
    pub fn dpad8(&self, [i, j]: [usize; 2]) -> ArrayVec::<[[usize; 2]; 8]> {
        let mut v = ArrayVec::new();
        for dir in 0..4 {
            if let Some(vv) = self.dir_jump([i, j], dir, 1) {
                v.push(vv);
            }
            if let Some(vv) = self.diag_jump([i, j], dir, 1) {
                v.push(vv);
            }
        }
        v
    }

    /// Of the squares reachable by ["knight's move"](https://en.wikipedia.org/wiki/Knight_(chess)#Movement) from `[i, j]`, return those that are within the board bounds and not blocked
    pub fn knight_jumps(&self, [i, j]: [usize; 2]) -> ArrayVec::<[[usize; 2]; 8]> {
        debug_assert!(self.inbounds([i, j]));

        let [i, j] = [i as isize, j as isize];

        let mut v = ArrayVec::new();
        for vv in [
            [i+2, j+1],
            [i+2, j-1],
            [i-2, j+1],
            [i-2, j-1],
            [i+1, j+2],
            [i+1, j-2],
            [i-1, j+2],
            [i-1, j-2],
        ] {
            if (0..self.h as isize).contains(&vv[0]) &&
                (0..self.w as isize).contains(&vv[1])
            {
                let vv = [vv[0] as usize, vv[1] as usize];
                if !self.is_blocked(vv) {
                    v.push(vv);
                }
            }
        }
        v
    }

    /// Of the squares reachable by ["rook's move"](https://en.wikipedia.org/wiki/Rook_(chess)#Placement_and_movement) from `[i, j]`, return those that are within the board bounds and not blocked
    pub fn rook_moves(&self, [i, j]: [usize; 2]) -> Vec<[usize; 2]>
    {
        let mut v = if self.blocked.is_none() {
            Vec::with_capacity(self.h + self.w - 2)
        } else {
            vec![]
        };

        for dir in 0..4 {
            let mut u = [i, j];
            while let Some(vv) = self.dir_jump(u, dir, 1) {
                v.push(vv);
                u = vv;
            }
        }
        v
    }

    /// Of the squares reachable by ["bishop's move"](https://en.wikipedia.org/wiki/Bishop_(chess)#Placement_and_movement) from `[i, j]`, return those that are within the board bounds and not blocked
    pub fn bishop_moves(&self, [i, j]: [usize; 2]) -> Vec<[usize; 2]> {
        let mut v = vec![];
        for dir in 0..4 {
            let mut u = [i, j];
            while let Some(vv) = self.diag_jump(u, dir, 1) {
                v.push(vv);
                u = vv;
            }
        }
        v
    }

    /// Of the squares reachable by ["queen's move"](https://en.wikipedia.org/wiki/Queen_(chess)#Placement_and_movement) from `[i, j]`, return those that are within the board bounds and not blocked
    pub fn queen_moves(&self, [i, j]: [usize; 2]) -> Vec<[usize; 2]> {
        let mut v = self.rook_moves([i, j]);
        v.append(&mut self.bishop_moves([i, j]));
        v
    }

    /// Of the squares that are `dist` [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) away from `[i, j]`, return those that are within the board bounds and not blocked
    pub fn manhat_jumps(&self, [i, j]: [usize; 2], dist: usize) -> Vec<[usize; 2]> {
        debug_assert!(self.inbounds([i, j]));

        if dist == 0 {
            return if !self.is_blocked([i, j]) { vec![[i, j]] } else { vec![] }
        }

        let mut v = vec![];
        for ii in i.saturating_sub(dist) ..= (i+dist).min(self.h-1) {
            let di = i.abs_diff(ii);
            let dj = dist - di;
            if let Some(vv) = self.dir_jump([ii, j], 1, dj) {
                v.push(vv);
            }
            if let Some(vv) = self.dir_jump([ii, j], 3, dj) {
                v.push(vv);
            }
        }
        v
    }
}
