// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Std method aliases (f300+). f374 = iter.

/// f374 = iter
pub trait F374 {
    type Item;
    fn f374(&self) -> impl Iterator<Item = &Self::Item> + '_;
}

impl<T> F374 for [T] {
    type Item = T;
    fn f374(&self) -> impl Iterator<Item = &Self::Item> + '_ {
        self.iter()
    }
}

impl<T> F374 for Vec<T> {
    type Item = T;
    fn f374(&self) -> impl Iterator<Item = &Self::Item> + '_ {
        self.iter()
    }
}
