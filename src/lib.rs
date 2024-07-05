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
#[cfg_attr(docsrs, doc(cfg(feature = "hal-0_2")))]
use hal_0_2::digital::v2::OutputPin;

#[cfg(feature = "hal-1_0")]
#[cfg_attr(docsrs, doc(cfg(feature = "hal-1_0")))]
use hal_1_0::digital::OutputPin;

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

/// Marker trait to indicate that a pin is active low.
pub trait ActiveLow {}

/// Marker trait to indicate that a pin is active high.
pub trait ActiveHigh {}

/// A chip select pin with active-low behavior.
pub struct ChipSelectActiveLow<Pin>(bool, Pin);

/// A chip select pin with active-high behavior.
pub struct ChipSelectActiveHigh<Pin>(bool, Pin);

impl<Pin> ChipSelectActiveLow<Pin>
where
    Pin: OutputPin,
{
    /// Initialize the chip select.
    pub const fn new(pin: Pin) -> Self {
        Self(false, pin)
    }

    /// Enables auto-select on the chip.
    pub fn with_auto_select(mut self, enabled: bool) -> Self {
        self.0 = enabled;
        self
    }

    /// Indicates whether this instance is configured to auto-select the chip on communication.
    pub fn is_auto_select(&self) -> bool {
        self.0
    }

    /// Selects the chip if auto-select is enabled.
    pub fn auto_select(&mut self) {
        if self.0 {
            self.select()
        }
    }

    /// Selects the chip, driving the line low.
    pub fn select(&mut self) {
        <Pin as OutputPin>::set_low(&mut self.1).ok();
    }

    /// Deselects the chip, driving the line high.
    pub fn deselect(&mut self) {
        <Pin as OutputPin>::set_high(&mut self.1).ok();
    }

    /// Consumes self and returns the wrapped pin.
    #[must_use]
    pub fn into_inner(self) -> Pin {
        self.1
    }
}

impl<Pin> ChipSelectActiveHigh<Pin>
where
    Pin: OutputPin,
{
    /// Initialize the chip select.
    pub const fn new(pin: Pin) -> Self {
        Self(false, pin)
    }

    /// Enables auto-select on the chip.
    pub fn with_auto_select(mut self, enabled: bool) -> Self {
        self.0 = enabled;
        self
    }

    /// Indicates whether this instance is configured to auto-select the chip on communication.
    pub fn is_auto_select(&self) -> bool {
        self.0
    }

    /// Selects the chip if auto-select is enabled.
    pub fn auto_select(&mut self) {
        if self.0 {
            self.select()
        }
    }

    /// Selects the chip, driving the line high.
    pub fn select(&mut self) {
        <Pin as OutputPin>::set_high(&mut self.1).ok();
    }

    /// Deselects the chip, driving the line low.
    pub fn deselect(&mut self) {
        <Pin as OutputPin>::set_low(&mut self.1).ok();
    }

    /// Consumes self and returns the wrapped pin.
    #[must_use]
    pub fn into_inner(self) -> Pin {
        self.1
    }
}

impl<Pin> From<Pin> for ChipSelectActiveLow<Pin>
where
    Pin: OutputPin,
{
    fn from(value: Pin) -> Self {
        Self::new(value)
    }
}

impl<Pin> From<Pin> for ChipSelectActiveHigh<Pin>
where
    Pin: OutputPin,
{
    fn from(value: Pin) -> Self {
        Self::new(value)
    }
}

impl<Pin> ActiveLow for ChipSelectActiveLow<Pin> where Pin: OutputPin {}

impl<Pin> ActiveHigh for ChipSelectActiveHigh<Pin> where Pin: OutputPin {}

impl<Pin> ChipSelect for ChipSelectActiveLow<Pin>
where
    Pin: OutputPin,
{
    fn is_auto_select(&self) -> bool {
        self.is_auto_select()
    }

    fn select(&mut self) {
        self.select()
    }

    fn deselect(&mut self) {
        self.deselect()
    }
}

impl<Pin> ChipSelect for ChipSelectActiveHigh<Pin>
where
    Pin: OutputPin,
{
    fn is_auto_select(&self) -> bool {
        self.is_auto_select()
    }

    fn select(&mut self) {
        self.select()
    }

    fn deselect(&mut self) {
        self.deselect()
    }
}
