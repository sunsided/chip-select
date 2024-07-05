//! Chip-Select GPIO support traits.

/// A chip-select trait.
pub trait ChipSelect {
    /// Indicates whether this instance is configured to auto-select the chip on communication.
    #[must_use]
    fn is_auto_select(&self) -> bool;

    /// Selects the chip if auto-select is enabled.
    fn auto_select(&mut self) {
        if self.is_auto_select() {
            self.select()
        }
    }

    /// Selects the chip, driving the line low.
    fn select(&mut self);

    /// Deselects the chip, driving the line high.
    fn deselect(&mut self);
}

