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

#[cfg(feature = "hal-0_2")]
extern crate hal_0_2 as hal;

#[cfg(feature = "hal-1_0")]
extern crate hal_1_0 as hal;

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

/// A chip select pin with active-low behavior.
pub struct ChipSelectActiveLow<Pin>(bool, Pin);

/// A chip select pin with active-high behavior.
pub struct ChipSelectActiveHigh<Pin>(bool, Pin);

impl<Pin> ChipSelectActiveLow<Pin>
where
    Pin: hal::digital::v2::OutputPin,
{
    /// Initialize the chip select.
    pub fn from<P>(pin: Pin) -> Self {
        Self(false, pin)
    }

    /// Enables auto-select on the chip.
    pub fn with_auto_select(mut self, enabled: bool) -> Self {
        self.0 = enabled;
        self
    }

    /// Selects the chip if auto-select is enabled.
    pub fn auto_select(&mut self) {
        if self.0 {
            self.select()
        }
    }

    /// Selects the chip, driving the line low.
    pub fn select(&mut self) {
        <Pin as hal::digital::v2::OutputPin>::set_low(&mut self.1).ok();
    }

    /// Deselects the chip, driving the line high.
    pub fn deselect(&mut self) {
        <Pin as hal::digital::v2::OutputPin>::set_high(&mut self.1).ok();
    }
}

impl<Pin> ChipSelectActiveHigh<Pin>
where
    Pin: hal::digital::v2::OutputPin,
{
    /// Initialize the chip select.
    pub fn from<P>(pin: Pin) -> Self {
        Self(false, pin)
    }

    /// Enables auto-select on the chip.
    pub fn with_auto_select(mut self, enabled: bool) -> Self {
        self.0 = enabled;
        self
    }

    /// Selects the chip if auto-select is enabled.
    pub fn auto_select(&mut self) {
        if self.0 {
            self.select()
        }
    }

    /// Selects the chip, driving the line high.
    pub fn select(&mut self) {
        <Pin as hal::digital::v2::OutputPin>::set_high(&mut self.1).ok();
    }

    /// Deselects the chip, driving the line low.
    pub fn deselect(&mut self) {
        <Pin as hal::digital::v2::OutputPin>::set_low(&mut self.1).ok();
    }
}
