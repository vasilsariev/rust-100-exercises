use std::ops::Add;

// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should be possible to print its debug representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SaturatingU16 {
    value: u16,
}

//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        Self {
            value: value as u16,
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        Self { value: *value }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        Self {
            value: *value as u16,
        }
    }
}

impl From<&SaturatingU16> for SaturatingU16 {
    fn from(value: &SaturatingU16) -> Self {
        *value
    }
}

// impl From<SaturatingU16> for u16 {
//     fn from(value: SaturatingU16) -> Self {
//         value.value
//     }
// }

//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.

impl<Rhs> Add<Rhs> for SaturatingU16
where
    Rhs: Into<SaturatingU16>,
{
    type Output = Self;

    fn add(self, rhs: Rhs) -> Self::Output {
        let rhs = rhs.into();
        Self {
            value: self.value.saturating_add(rhs.value),
        }
    }
}

//   It should be possible to compare it with another `SaturatingU16` or a `u16`.

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
