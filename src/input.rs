use std::{
    fmt::{Binary, Display, LowerHex, Octal, UpperHex},
    ops::{Deref, DerefMut},
    str::FromStr,
};

use anyhow::anyhow;

use crate::{
    format::{Format, NumberType},
    input::Input::Generic,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input<T> {
    Generic(T),
    NegativeSignedInt(T, u8),
}

impl<T> DerefMut for Input<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Generic(n) => n,
            Self::NegativeSignedInt(n, _) => n,
        }
    }
}

impl<T> Deref for Input<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Generic(n) => n,
            Self::NegativeSignedInt(n, _) => n,
        }
    }
}

impl<T: UpperHex> UpperHex for Input<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        UpperHex::fmt(self.deref(), f)
    }
}

impl<T: LowerHex> LowerHex for Input<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(self.deref(), f)
    }
}

impl<T: Binary> Binary for Input<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Binary::fmt(self.deref(), f)
    }
}

impl<T: Octal> Octal for Input<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Octal::fmt(self.deref(), f)
    }
}

impl<T: Display> Display for Input<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.deref(), f)
    }
}
