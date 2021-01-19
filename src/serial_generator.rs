use crate::Serial;

use std::{
    fmt,
    fmt::{Display, Formatter},
};

/// A utility for generating instances of a given [Serial] type.
///
/// See the [crate] documentation for more information.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SerialGenerator<T: Serial = u32> {
    value: T,
}

impl<T: Serial> SerialGenerator<T> {
    /// Create a new generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new generator with the given value.
    ///
    /// [Serial::START] is always considered the first value. If this method is
    /// used with a greater value, [previous](Self::previous) may give an
    /// unexpected answer because the "previous" value is calculated, not
    /// recorded.
    pub fn with_init_value(value: T) -> Self {
        SerialGenerator { value }
    }

    /// Generate a new instance of the generator's [Serial] type.
    pub fn generate(&mut self) -> T {
        let current = self.value.clone();
        let next = current.next_increment();
        self.value = next;

        current
    }

    /// Return the previously generated value.
    ///
    /// This method will return None if the current value is [Serial::START].
    ///
    /// The return value is calculated, not recorded. If the highest possible
    /// value has been reached, this method will still return one less than that
    /// value. To check that unique values can still be generated, use
    /// [is_at_max](Self::is_at_max).
    pub fn previous(&self) -> Option<T> {
        if self.value == T::START {
            None
        } else {
            Some(self.value.prev_increment())
        }
    }

    /// Return the number of unique values that can still be generated by this
    /// generator.
    pub fn remaining_increments(&self) -> T {
        self.value.remaining_increments()
    }

    /// Return a boolean representing whether unique values can still be
    /// generated.
    pub fn is_at_max(&self) -> bool {
        self.value.is_max_value()
    }

    /// Alias of [Self::is_at_max].
    #[deprecated(
        since = "0.2.2",
        note = "Please use is_at_max instead."
    )]
    pub fn has_remaining_increments(&self) -> bool {
        self.is_at_max()
    }
}

impl <T: Serial, U: Serial + From<T>> From<T> for SerialGenerator<U> {
    fn from(other: T) -> Self {
        SerialGenerator::with_init_value(other.into())
    }
}

impl<T: Serial> Default for SerialGenerator<T> {
    fn default() -> Self {
        SerialGenerator { value: T::START }
    }
}

impl<T: fmt::Debug + Display + Serial> Display for SerialGenerator<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T: Serial> Iterator for SerialGenerator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_max() {
            None
        } else {
            let next_value = self.generate();

            Some(next_value)
        }
    }
}
