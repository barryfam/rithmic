use std::mem;

use crate::OptionMerge;

#[derive(Clone)]
struct Node<T> {
    item: T,
    left: OptNode<T>,
    right: OptNode<T>,
}
type OptNode<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    pub fn new(item: T) -> Self {
        Self {
            item,
            left: None,
            right: None
        }
    }
}

fn link<T: Ord>(u: OptNode<T>, v: OptNode<T>) -> OptNode<T> {
    u.merge(v, |mut u, mut v| {
        if u.item < v.item {
            mem::swap(&mut u, &mut v)
        }
        if rand::random() {
            u.left = link(u.left, Some(v));
        } else {
            u.right = link(u.right, Some(v));
        }
        u
    })
}

/**
A [priority max-queue](https://en.wikipedia.org/wiki/Priority_queue)

`MeldHeap`'s interface is similar to [`std::collections::BinaryHeap`], but [`MeldHeap::meld`] runs in *O*(log *n*) time versus [`BinaryHeap::append`](`std::collections::BinaryHeap::append)'s *O*(*n*)
```
# use rithmic::MeldHeap;
let q0 = MeldHeap::from_iter([2, 3, 5]);
let q1 = MeldHeap::from_iter([1, 4, 6]);
let mut q = q0.meld(q1);

assert_eq!(q.pop(), Some(6));
assert_eq!(q.pop(), Some(5));
assert_eq!(q.pop(), Some(4));
```
*/
#[derive(Clone)]
pub struct MeldHeap<T> {
    root: OptNode<T>,
    len: usize,
}

impl<T: Ord> MeldHeap<T>
{
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Add an item to the heap
    pub fn push(&mut self, item: T) {
        let r = self.root.take();
        self.root = link(r, Some(box Node::new(item)));
        self.len += 1;
    }

    /// Remove and return the maximum item in the heap. See [`MeldHeap`] for an example
    pub fn pop(&mut self) -> Option<T> {
        let r = self.root.take()?;
        self.root = link(r.left, r.right);
        self.len -= 1;
        Some(r.item)
    }

    /// Return but do not remove the maximum item in the heap
    pub fn peek(&self) -> Option<&T> {
        self.root.as_ref().map(|r| &r.item)
    }

    /// Combine two heaps. See [`MeldHeap`] for an example
    pub fn meld(self, other: Self) -> Self {
        Self {
            root: link(self.root, other.root),
            len: self.len + other.len
        }
    }
}

impl<T: Ord> Default for MeldHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> FromIterator<T> for MeldHeap<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut heap = Self::new();
        for item in iter {
            heap.push(item);
        }
        heap
    }
}
