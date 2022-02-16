// Copyright 2022 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::bar::{Error as BarAddressError, PciBarConfig, PciBarRegion, PciRomBarConfig};
use crate::pci_config::{Error as ConfigSpaceAccessError, PciConfig};
use std::fmt;

type Result<T> = std::result::Result<T, Error>;

// Generic device PCI device constants.
/// Byte offset of the start of the BARs in the PCI configuration space.
pub const BARS_START_OFFSET: usize = 0x10;
/// Number of BARs in a generic PCI device configuration space.
pub const NUM_BARS: usize = 6;
/// Offset of Cardbus CIS pointer in the PCI configuration space.
pub const CARDBUS_CIS_POINTER_OFFSET: usize = 0x28;
/// Offset of Subsystem Vendor ID in the PCI configuration space.
pub const SUBSYSTEM_VENDOR_ID_OFFSET: usize = 0x2c;
/// Offset of Subsystem ID in the PCI configuration space.
pub const SUBSYSTEM_ID_OFFSET: usize = 0x2e;
/// Offset of the ROM BAR in the PCI configuration space.
pub const ROM_BAR_OFFSET: usize = 0x30;
/// Register number of the ROM BAR in the PCI configuration space.
pub const ROM_BAR_REG_IDX: usize = 0xc;
/// Offset of Capabilities Pointer in the PCI configuration space.
pub const CAPABILITIES_POINTER_OFFSET: usize = 0x34;
/// Offset of Interrupt Line in the PCI configuration space.
pub const INTERRUPT_LINE_OFFSET: usize = 0x3c;
/// Offset of Interrupt Pin in the PCI configuration space.
pub const INTERRUPT_PIN_OFFSET: usize = 0x3d;
/// Offset of Min Grant in the PCI configuration space.
pub const MIN_GRANT_OFFSET: usize = 0x3e;
/// Offset of Max Latency in the PCI configuration space.
pub const MAX_LATENCY_OFFSET: usize = 0x3f;

#[derive(Debug)]
/// Error type for PCI Device configuration space accesses.
pub enum Error {
    /// PCI common configuration space access error.
    Access(ConfigSpaceAccessError),
    /// BAR addressing error.
    BarAddress(BarAddressError),
    /// Invalid BAR slot.
    BarIndex,
    /// BAR slot is already used.
    BarInUse(usize),
    /// BAR slot is already used by a 64-bit BAR.
    BarInUse64(usize),
    /// Requested BAR is unavailable.
    BarInvalid(usize),
    /// Requested 64-bit BAR is unavailable.
    BarInvalid64(usize),
    /// ROM BAR slot is already used.
    RomBarInUse,
    /// ROM BAR is unavailable.
    RomBarInvalid,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            Access(e) => write!(f, "access error: {}", e),
            BarAddress(e) => write!(f, "BAR region: {}", e),
            BarIndex => write!(f, "invalid BAR slot"),
            BarInUse(slot) => write!(f, "BAR slot {} already used", slot),
            BarInUse64(slot) => write!(f, "BAR slot {} already used by 64bit BAR", slot),
            BarInvalid(slot) => write!(f, "requested BAR {} unavailable", slot),
            BarInvalid64(slot) => write!(f, "requested 64bit BAR {} unavailable", slot),
            RomBarInUse => write!(f, "ROM BAR slot already used"),
            RomBarInvalid => write!(f, "ROM BAR unavailable"),
        }
    }
}

impl From<ConfigSpaceAccessError> for Error {
    fn from(e: ConfigSpaceAccessError) -> Self {
        Error::Access(e)
    }
}

impl From<BarAddressError> for Error {
    fn from(e: BarAddressError) -> Self {
        Error::BarAddress(e)
    }
}

/// A PCI Device (header type 0x00) configuration space abstraction.
///
/// The most important feature of the PCI device configuration space is
/// the Base Address Register representation in memory. A PCI Device
/// configuration space must be able to:
/// - add BARs, through `add_bar()`
/// - retrieve BAR addresses, through `bar_address()`
/// - add the ROM BAR, through `add_rom_bar()`
/// - retrieve the ROM BAR address, through `rom_bar_address()`
pub trait PciDeviceConfig: PciConfig {
    /// Add a Base Address Register to the configuration space.
    ///
    /// # Arguments
    ///
    /// * `config`:   the configuration of the BAR to be added
    fn add_bar(&mut self, config: PciBarConfig) -> Result<()>;

    /// Add a ROM Base Address Register to the configuration space.
    ///
    /// # Arguments
    ///
    /// * `config`:   the configuration of the ROM BAR to be added
    fn add_rom_bar(&mut self, config: PciRomBarConfig) -> Result<()>;

    /// Retrieves the address in a Base Address Register from the configuration
    /// space.
    ///
    /// # Arguments
    ///
    /// * `idx`:   the index of the requested BAR
    fn bar_address(&self, idx: usize) -> Result<PciBarRegion>;

    /// Retrieves the address in the ROM Base Address Register from the
    /// configuration space.
    fn rom_bar_address(&self) -> Result<PciBarRegion>;
}
