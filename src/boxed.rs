use std::{
    borrow::{Borrow, BorrowMut},
    fmt,
};

use crate::value::Value;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Boxed(pub Value);

impl Boxed {
    pub fn as_value(&self) -> &Value {
        &self.0
    }
    pub fn as_value_mut(&mut self) -> &mut Value {
        &mut self.0
    }
}

impl fmt::Debug for Boxed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for Boxed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Value> for Boxed {
    fn from(v: Value) -> Self {
        Self(v)
    }
}

impl AsRef<Value> for Boxed {
    fn as_ref(&self) -> &Value {
        &self.0
    }
}

impl AsMut<Value> for Boxed {
    fn as_mut(&mut self) -> &mut Value {
        &mut self.0
    }
}

impl Borrow<Value> for Boxed {
    fn borrow(&self) -> &Value {
        &self.0
    }
}

impl BorrowMut<Value> for Boxed {
    fn borrow_mut(&mut self) -> &mut Value {
        &mut self.0
    }
}