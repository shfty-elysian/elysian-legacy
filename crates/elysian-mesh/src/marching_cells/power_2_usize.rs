use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InvalidPowerOf2;

impl Display for InvalidPowerOf2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid power of 2")
    }
}

impl Error for InvalidPowerOf2 {}

macro_rules! impl_power_2_num {
    ($ident:ident, $ty:ty) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        pub struct $ident($ty);

        impl $ident {
            /// # Safety
            ///
            /// The value must be a power of two.
            pub const unsafe fn new_unchecked(n: $ty) -> Self {
                assert!(n.is_power_of_two());
                Self(n)
            }

            pub const fn new(n: $ty) -> Option<Self> {
                if n.is_power_of_two() {
                    // Safety: we just checked that n is a power of two
                    Some(unsafe { Self::new_unchecked(n) })
                } else {
                    None
                }
            }

            pub const fn get(self) -> $ty {
                self.0
            }
        }

        impl From<$ident> for $ty {
            fn from(power2: $ident) -> $ty {
                power2.get()
            }
        }

        impl TryFrom<$ty> for $ident {
            type Error = InvalidPowerOf2;

            fn try_from(n: $ty) -> Result<$ident, InvalidPowerOf2> {
                $ident::new(n).ok_or(InvalidPowerOf2)
            }
        }

        impl core::ops::BitAnd<$ty> for $ident {
            type Output = $ty;

            fn bitand(self, rhs: $ty) -> $ty {
                self.0 & rhs
            }
        }

        impl core::ops::BitOr<$ty> for $ident {
            type Output = $ty;

            fn bitor(self, rhs: $ty) -> $ty {
                self.0 | rhs
            }
        }

        impl core::ops::BitXor<$ty> for $ident {
            type Output = $ty;

            fn bitxor(self, rhs: $ty) -> $ty {
                self.0 ^ rhs
            }
        }

        impl core::ops::Not for $ident {
            type Output = $ty;

            fn not(self) -> $ty {
                !self.0
            }
        }

        impl core::ops::BitAnd<Self> for $ident {
            type Output = $ty;

            fn bitand(self, rhs: Self) -> $ty {
                self.0 & rhs.0
            }
        }

        impl core::ops::BitOr<Self> for $ident {
            type Output = $ty;

            fn bitor(self, rhs: Self) -> $ty {
                self.0 | rhs.0
            }
        }

        impl core::ops::BitXor<Self> for $ident {
            type Output = $ty;

            fn bitxor(self, rhs: Self) -> $ty {
                self.0 ^ rhs.0
            }
        }
    };
}

impl_power_2_num!(Power2U8, u8);
impl_power_2_num!(Power2U16, u16);
impl_power_2_num!(Power2U32, u32);
impl_power_2_num!(Power2U64, u64);
impl_power_2_num!(Power2U128, u128);
impl_power_2_num!(Power2Usize, usize);
