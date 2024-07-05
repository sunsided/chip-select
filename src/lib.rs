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
    /// Selects the chip, driving the line low.
    fn select(&mut self);

    /// Deselects the chip, driving the line high.
    fn deselect(&mut self);
}

/// A chip-select trait.
pub trait ChipSelectGuarded: ChipSelect {
    /// A guard that, when dropped, deselects the chip.
    type Guard<'a>
    where
        Self: 'a;

    /// Selects the device and returns a guard that, when dropped, deselects the chip.
    fn select_guard(&mut self) -> Self::Guard<'_>;
}

/// Marker trait to indicate that a pin is active low.
pub trait ActiveLow {}

/// Marker trait to indicate that a pin is active high.
pub trait ActiveHigh {}

/// A chip select pin with active-low behavior.
pub struct ChipSelectActiveLow<Pin>(Pin);

/// A chip select pin with active-high behavior.
pub struct ChipSelectActiveHigh<Pin>(Pin);

impl<Pin> ChipSelectActiveLow<Pin>
where
    Pin: OutputPin,
{
    /// Initialize the chip select.
    pub const fn new(pin: Pin) -> Self {
        Self(pin)
    }

    /// Selects the chip, driving the line low.
    pub fn select(&mut self) {
        <Pin as OutputPin>::set_low(&mut self.0).ok();
    }

    /// Deselects the chip, driving the line high.
    pub fn deselect(&mut self) {
        <Pin as OutputPin>::set_high(&mut self.0).ok();
    }

    /// Consumes self and returns the wrapped pin.
    #[must_use]
    pub fn into_inner(self) -> Pin {
        self.0
    }

    /// Selects the device and returns a guard that, when dropped, deselects the chip.
    #[must_use]
    pub fn select_guard(&mut self) -> DeselectOnDrop<Self> {
        self.select();
        DeselectOnDrop::from(self)
    }
}

impl<Pin> ChipSelectActiveHigh<Pin>
where
    Pin: OutputPin,
{
    /// Initialize the chip select.
    pub const fn new(pin: Pin) -> Self {
        Self(pin)
    }

    /// Selects the chip, driving the line high.
    pub fn select(&mut self) {
        <Pin as OutputPin>::set_high(&mut self.0).ok();
    }

    /// Deselects the chip, driving the line low.
    pub fn deselect(&mut self) {
        <Pin as OutputPin>::set_low(&mut self.0).ok();
    }

    /// Consumes self and returns the wrapped pin.
    #[must_use]
    pub fn into_inner(self) -> Pin {
        self.0
    }

    /// Selects the device and returns a guard that, when dropped, deselects the chip.
    #[must_use]
    pub fn select_guard(&mut self) -> DeselectOnDrop<Self> {
        self.select();
        DeselectOnDrop::from(self)
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
    fn select(&mut self) {
        self.select()
    }

    fn deselect(&mut self) {
        self.deselect()
    }
}

impl<Pin> ChipSelectGuarded for ChipSelectActiveLow<Pin>
where
    Pin: OutputPin,
{
    type Guard<'a> = DeselectOnDrop<'a, Self> where Pin: 'a;

    /// Selects the device and returns a guard that, when dropped, deselects the chip.
    fn select_guard(&mut self) -> DeselectOnDrop<Self> {
        self.select_guard()
    }
}

impl<Pin> ChipSelect for ChipSelectActiveHigh<Pin>
where
    Pin: OutputPin,
{
    fn select(&mut self) {
        self.select()
    }

    fn deselect(&mut self) {
        self.deselect()
    }
}

impl<Pin> ChipSelectGuarded for ChipSelectActiveHigh<Pin>
where
    Pin: OutputPin,
{
    type Guard<'a> = DeselectOnDrop<'a, Self> where Pin: 'a;

    /// Selects the device and returns a guard that, when dropped, deselects the chip.
    fn select_guard(&mut self) -> Self::Guard<'_> {
        self.select_guard()
    }
}

/// A guard that deselects the chip when it is dropped.
pub struct DeselectOnDrop<'a, T>(&'a mut T)
where
    T: ChipSelect;

impl<'a, T> From<&'a mut T> for DeselectOnDrop<'a, T>
where
    T: ChipSelect,
{
    fn from(value: &'a mut T) -> Self {
        Self(value)
    }
}

impl<'a, T> Drop for DeselectOnDrop<'a, T>
where
    T: ChipSelect,
{
    fn drop(&mut self) {
        self.0.deselect()
    }
}
