/// A trait that defines how a type is used by SerialGenerator.
pub trait Serial
where
    Self: Clone + Ord,
{
    /// The default initial and lowest possible value.
    const START: Self;

    /// Return the next sequential value. Return an equal value if it is at its
    /// maximum.
    fn next_increment(&self) -> Self;

    /// Return the previous sequential value. Return an equal value if it is at
    /// its minimum.
    fn prev_increment(&self) -> Self;

    /// Return a boolean representing whether the value is equal to its maximum.
    fn is_max_value(&self) -> bool;
}

macro_rules! impl_serial {
    ($($t:ty),+ $(,)?) => {
        $(
            impl Serial for $t {
                const START: Self = Self::MIN;

                fn next_increment(&self) -> Self {
                    self.saturating_add(1)
                }

                fn prev_increment(&self) -> Self {
                    self.saturating_sub(1)
                }

                fn is_max_value(&self) -> bool {
                    self == &Self::MAX
                }
            }
        )+
    }
}

impl_serial!(u8, u16, u32, u64, u128, usize);
