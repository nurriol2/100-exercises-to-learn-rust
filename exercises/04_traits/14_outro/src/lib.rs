// TODO: Define a new `SaturatingU16` type.
//   SaturatingU16 should hold a `u16` value.
//
//   SaturatingU16 should provide conversions from `u16`
//   SaturatingU16 should provide conversions from `u8`
//
//   SaturatingU16 should provide conversions from `&u16`
//   SaturatingU16 should provide conversions from `&u8`.
//
//   Addition should saturate at the maximum value for `u16`.
//
//   SaturatingU16 should support addition, right-hand side of type SaturatingU16
//   SaturatingU16 should support addition, right-hand side of type u16
//
//   SaturatingU16 should support addition, right-hand side of type &u16
//   SaturatingU16 should support addition, right-hand side of type &SaturatingU16
//
//   It should be possible to compare it with `u16`.
//   It should be possible to compare it with another `SaturatingU16`
//
//   It should be possible to print SaturatingU16  debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SaturatingU16 {
    value: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        (*value).into()
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        (*value).into()
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: SaturatingU16) -> Self::Output {
        (self.value.saturating_add(rhs.value)).into()
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {
        (self.value.saturating_add(rhs)).into()
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {
        // `+` is the same as `Add<u16> for SaturatingU16`, above
        (self + *rhs).into()
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        // `+` is the same as `Add<SaturatingU16> for SaturatingU16`, above
        self + *rhs
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
