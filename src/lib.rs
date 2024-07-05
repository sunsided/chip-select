//! Chip-Select GPIO support traits.

#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(all(feature = "hal-0_2", feature = "hal-1_0"))]
compile_error!(
    "HAL feature \"hal-0_2\" and feature \"hal-1_0\" cannot be enabled at the same time"
);

#[cfg(not(any(feature = "hal-0_2", feature = "hal-1_0")))]
compile_error!("A HAL feature (\"hal-0_2\" or \"hal-1_0\") must be enabled");

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
