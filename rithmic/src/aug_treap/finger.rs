use std::ptr;

use super::Node;

impl<K, V, A> Node<K, V, A>
{
    pub fn finger_last_true<'me:'n, 'n>
        (&'me self, mut predicate: impl FnMut(&Self) -> bool, stack: &mut Vec<&'n Self>)
        -> bool
    {
        stack.push(self);
        if predicate(self) {
            self.right.as_ref().map(|u| u.finger_last_true(predicate, stack));
            true
        }
        else {
            if self.left.as_ref().map_or(false, |u| u.finger_last_true(predicate, stack)) {
                true
            } else {
                stack.pop();
                false
            }
        }
    }

    pub fn finger_first_false<'me:'n, 'n>
        (&'me self, mut predicate: impl FnMut(&Self) -> bool, stack: &mut Vec<&'n Self>)
        -> bool
    {
        stack.push(self);
        if !predicate(self) {
            self.left.as_ref().map(|u| u.finger_first_false(predicate, stack));
            true
        }
        else {
            if self.right.as_ref().map_or(false, |u| u.finger_first_false(predicate, stack)) {
                true
            } else {
                stack.pop();
                false
            }
        }
    }
}

pub trait Finger<'n, K, V, A>
{
    fn tip(&self) -> &'n Node<K, V, A>;
    fn successor(self) -> Self;
    fn predecessor(self) -> Self;
}

impl<'n, K, V, A> Finger<'n, K, V, A> for Vec<&'n Node<K, V, A>>
{
    #[inline]
    fn tip(&self) -> &'n Node<K, V, A> {
        self.last().unwrap()
    }

    #[inline]
    fn successor(mut self) -> Self {
        debug_assert!(!self.is_empty());

        if let Some(ref u) = self.last().unwrap().right {
            self.push(u);
            let mut u = u;
            while let Some(ref v) = u.left {
                u = v;
                self.push(u);
            }
        }
        else {
            while let Some(u) = self.pop() && let Some(p) = self.last()
            {
                if let Some(pl) = &p.left && ptr::eq(u, pl.as_ref()) {
                    break
                }
            }
        }
        self
    }

    #[inline]
    fn predecessor(mut self) -> Self {
        debug_assert!(!self.is_empty());

        if let Some(ref u) = self.last().unwrap().left {
            self.push(u);
            let mut u = u;
            while let Some(ref v) = u.right {
                u = v;
                self.push(u);
            }
        }
        else {
            while let Some(u) = self.pop() && let Some(p) = self.last()
            {
                if let Some(pr) = &p.right && ptr::eq(u, pr.as_ref()) {
                    break
                }
            }
        }
        self
    }
}
