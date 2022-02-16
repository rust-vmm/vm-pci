// Copyright 2022 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::fmt;

use crate::pci_config::{validate_dword_alignment, Error as ConfigSpaceAccessError};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
/// Error type for PCI Base Address Register accesses.
pub enum Error {
    /// Invalid access in the configuration space.
    Access(ConfigSpaceAccessError),
    /// Memory region defined by the BAR address and lenght is invalid.
    BarAddressInvalid(u64, u64),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            Access(e) => write!(f, "config space access: {}", e),
            BarAddressInvalid(addr, len) => {
                write!(f, "out of bounds BAR addr {} len {}", addr, len)
            }
        }
    }
}

impl From<ConfigSpaceAccessError> for Error {
    fn from(e: ConfigSpaceAccessError) -> Self {
        Error::Access(e)
    }
}

#[derive(Copy, Clone)]
/// Enum representing the "prefetchable" state masks of BARs (bit 3 in the register).
pub enum PciBarPrefetchable {
    /// Non-prefetchable BARs have bit 3 set to 0.
    NotPrefetchable = 0,
    /// Prefetchable BARs have bit 3 set to 1.
    Prefetchable = 0x08,
}

#[derive(Copy, Clone, PartialEq)]
/// PCI BAR memory region abstraction.
pub enum PciBarRegion {
    /// Represents a 32-bit IO space region.
    Io {
        /// Start address of the 32-bit IO space region.
        addr: u32,
        /// Length of the region, in bytes.
        len: u32,
    },
    /// Represents a 32-bit memory mapped region.
    Memory32Bit {
        /// Start address of the 32-bit memory mapped region.
        addr: u32,
        /// Length of the region, in bytes.
        len: u32,
    },
    /// Represents a 64-bit memory mapped region.
    Memory64Bit {
        /// Start address of the 64-bit memory mapped region.
        addr: u64,
        /// Length of the region, in bytes.
        len: u64,
    },
}

impl PciBarRegion {
    /// Describe an IO space BAR region.
    pub fn new_io_region(addr: u32, len: u32) -> Result<Self> {
        addr.checked_add(len)
            .ok_or(Error::BarAddressInvalid(addr.into(), len.into()))?;
        validate_dword_alignment(addr as usize)?;
        Ok(PciBarRegion::Io { addr, len })
    }

    /// Describe a memory mapped 32bit BAR region.
    pub fn new_32bit_mem_region(addr: u32, len: u32) -> Result<Self> {
        addr.checked_add(len)
            .ok_or(Error::BarAddressInvalid(addr.into(), len.into()))?;
        validate_dword_alignment(addr as usize)?;
        Ok(PciBarRegion::Memory32Bit { addr, len })
    }

    /// Describe a memory mapped 64bit BAR region.
    pub fn new_64bit_mem_region(addr: u64, len: u64) -> Result<Self> {
        addr.checked_add(len)
            .ok_or(Error::BarAddressInvalid(addr.into(), len.into()))?;
        validate_dword_alignment(addr as usize)?;
        Ok(PciBarRegion::Memory64Bit { addr, len })
    }
}

#[derive(Copy, Clone)]
/// Configuration of a Base Address Register.
pub struct PciBarConfig {
    /// Index of the BAR.
    pub index: usize,
    /// Region referenced by BAR.
    pub region: PciBarRegion,
    /// Prefetchable status.
    pub prefetchable: PciBarPrefetchable,
}

#[derive(Copy, Clone)]
/// Configuration of a ROM Base Address Register.
pub struct PciRomBarConfig {
    /// Region referenced by BAR.
    pub region: PciBarRegion,
    /// Enable bit.
    pub enable: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_bar_region() {
        let r = PciBarRegion::new_io_region(0x1000, 4096).unwrap();
        assert!(
            r == PciBarRegion::Io {
                addr: 0x1000,
                len: 4096
            }
        );

        let err_len = u32::MAX - 512;
        match PciBarRegion::new_io_region(0x1000, err_len) {
            Err(Error::BarAddressInvalid(0x1000, len)) => {
                assert_eq!(len, err_len as u64);
            }
            _ => assert!(false),
        }

        match PciBarRegion::new_io_region(0x1003, 4096) {
            Err(Error::Access(ConfigSpaceAccessError::UnalignedAccess)) => {}
            _ => assert!(false),
        }
    }

    #[test]
    fn test_32bit_mem_bar_region() {
        let r = PciBarRegion::new_32bit_mem_region(0x1000, 4096).unwrap();
        assert!(
            r == PciBarRegion::Memory32Bit {
                addr: 0x1000,
                len: 4096
            }
        );

        let err_len = u32::MAX - 512;
        match PciBarRegion::new_32bit_mem_region(0x1000, err_len) {
            Err(Error::BarAddressInvalid(0x1000, len)) => {
                assert_eq!(len, err_len as u64);
            }
            _ => assert!(false),
        }

        match PciBarRegion::new_32bit_mem_region(0x1003, 4096) {
            Err(Error::Access(ConfigSpaceAccessError::UnalignedAccess)) => {}
            _ => assert!(false),
        }
    }

    #[test]
    fn test_64bit_mem_bar_region() {
        let r = PciBarRegion::new_64bit_mem_region(0x1000, 4096).unwrap();
        assert!(
            r == PciBarRegion::Memory64Bit {
                addr: 0x1000,
                len: 4096
            }
        );

        let big_len = u64::MAX / 2;
        let r = PciBarRegion::new_64bit_mem_region(0x1000, big_len).unwrap();
        assert!(
            r == PciBarRegion::Memory64Bit {
                addr: 0x1000,
                len: big_len
            }
        );

        let err_len = u64::MAX - 512;
        match PciBarRegion::new_64bit_mem_region(0x1000, err_len) {
            Err(Error::BarAddressInvalid(0x1000, len)) => {
                assert_eq!(len, err_len as u64);
            }
            _ => assert!(false),
        }

        match PciBarRegion::new_64bit_mem_region(0x1003, 4096) {
            Err(Error::Access(ConfigSpaceAccessError::UnalignedAccess)) => {}
            _ => assert!(false),
        }
    }
}
