use std::ops::{Deref, DerefMut};

use serde::{Deserialize, de::DeserializeOwned};

#[derive(Debug, Deserialize)]
pub struct Args<T>(pub T);
impl<T> Args<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Args<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Args<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: DeserializeOwned> From<T> for Args<T> {
    fn from(t: T) -> Args<T> {
        Args(t)
    }
}
