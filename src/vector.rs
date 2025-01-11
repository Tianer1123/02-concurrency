pub use std::ops::Deref;

use anyhow::{anyhow, Result};
use std::ops::{Add, AddAssign, Mul};

pub struct Vector<T> {
    data: Vec<T>,
}

// pretend this is a heavy operation,CPU intensive
pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Mul<Output = T> + Add<Output = T> + Default + AddAssign + Copy,
{
    // a.len() ==> a.data.len() 实现了Deref trait
    if a.len() != b.len() {
        return Err(anyhow!("Matrix dot product error: a.len() != b.len()"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }

    Ok(sum)
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}
