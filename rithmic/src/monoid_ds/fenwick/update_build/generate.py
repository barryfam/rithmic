#!/usr/bin/env python3

D_MAX = 16

from pathlib import Path
from string import Template
from textwrap import indent
from typing import TextIO

class AtTemplate(Template):
    delimiter = '@'

class Writer:
    def __init__(self, mapping: dict, file: TextIO):
        self.mapping = mapping
        self.file = file

    def __call__(self, template: str='', tabs=0, newlines=1):
        template = indent(template.rstrip(' ').strip('\n'), ' '*4 * tabs)
        print(AtTemplate(template).substitute(self.mapping), end='\n' * newlines, file=self.file)



with open(Path(__file__).parent / 'mod.rs', 'w') as f:
    for d in range(1, D_MAX+1):
        print(f'mod d{d};', file=f)

for d in range(1, D_MAX+1):
    with open(Path(__file__).parent / f'd{d}.rs', 'w') as f:
        writer = Writer(locals(), f)

        writer(r'''

use std::fmt::Debug;
use std::ops::{Add, Sub, Mul};

use crate::{NdVec, OdometerBE};

use super::super::super::monoid_ops::MonoidOps;
use super::super::NdFenwick;

impl<T, O: MonoidOps<T, T>> NdFenwick<@d, T, O>
where T: Default
{
    pub fn update(&mut self, index: [usize; @d], value: T)
    {
        assert!(self.inbounds(index), "index outside of shape: {:?} / {:?}", index, self.shape());

        ''', newlines=2)

        for i in range(d):
            j = d-1-i
            writer(r'''

        let mut i@j = index[@j];
        while i@j < self.shape()[@j] {

        ''', tabs=i)

        index = ', '.join(f'i{i}' for i in range(d))
        writer(r'''

            let index = [@index];

            let x;
            #[cfg(not(feature = "unsafe"))]
            { x = &mut self.0[index]; }
            #[cfg(feature = "unsafe")]
            // SAFETY: each i < shape
            unsafe { x = self.0.get_unchecked_mut(index); }
            *x = O::operator(x, &value);

        ''', tabs=d-1, newlines=2)

        for i in range(d):
            j = d-1-i
            writer(r'''

            i@i = i@i | (i@i+1);
        }

        ''', tabs=j)

        writer(r'''
    }
}

impl<T> NdFenwick<@d, T>
where
    T: Default + Copy + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    for<'a> &'a T: Add<Output=T> + Sub<Output=T> + Mul<Output=T>,
{
    pub fn build_from(ndvec: &NdVec<@d, T>) -> Self {
        let mut ft = Self::new(ndvec.shape());
        for u in OdometerBE::new(ndvec.shape()) {
            ft.update(u, ndvec[u]);
        }
        ft
    }
}

        ''')
