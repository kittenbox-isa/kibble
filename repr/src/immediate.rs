use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// An immediate, guaranteed to be representable within BITS bits.
pub struct Immediate<const BITS: u8> {
    value: u32,
}

impl<const BITS: u8> Immediate<BITS> {
    pub fn new(n: u32) -> Self {
        Self::new_checked(n).expect(
            "attempt to initialize a {BITS}-long \
                immediate with oversized value {n}",
        )
    }

    pub fn new_checked(n: u32) -> Option<Self> {
        let mask = Self::mask();

        if n != n & mask {
            None
        }
        else {
            Some(Self { value: n })
        }
    }

    pub fn upgrade<const NEW_BITS: u8>(self) -> Immediate<NEW_BITS> {
        if NEW_BITS < BITS {
            panic!(
                "cannot upgrade immediate from length {BITS} to the \
                    smaller length {NEW_BITS}"
            );
        }

        Immediate { value: self.value }
    }

    pub fn downgrade<const NEW_BITS: u8>(
        self,
    ) -> Option<Immediate<NEW_BITS>> {
        if NEW_BITS >= BITS {
            Some(self.upgrade())
        }
        else {
            Immediate::new_checked(self.value)
        }
    }

    const fn mask() -> u32 {
        let mask_bits = 32 - BITS;
        u32::MAX << mask_bits >> mask_bits
    }
}

impl<const BITS: u8> From<Immediate<BITS>> for u32 {
    fn from(imm: Immediate<BITS>) -> u32 {
        *imm
    }
}

impl<const BITS: u8> ops::Deref for Immediate<BITS> {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.value
    }
}
