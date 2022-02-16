// Copyright 2022 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::fmt;

type Result<T> = std::result::Result<T, Error>;

/// Represents (byte offset, len) for a capability in the config space.
pub type CapabilityRegion = (usize, usize);

// PCI common configuration constants.
/// Length of PCI configuration space, in bytes.
pub const PCI_CONFIGURATION_SPACE_SIZE: usize = 256;
/// Length of PCIe configuration space, in bytes.
pub const PCIE_CONFIGURATION_SPACE_SIZE: usize = 4096;
/// Number of PCIe configuration space registers.
pub const NUM_CONFIGURATION_REGISTERS: usize = 1024;
/// Offset of vendor ID in the PCI configuration space.
pub const VENDOR_ID_OFFSET: usize = 0x00;
/// Offset of device ID in the PCI configuration space.
pub const DEVICE_ID_OFFSET: usize = 0x02;
/// Offset of command in the PCI configuration space.
pub const COMMAND_OFFSET: usize = 0x04;
/// Offset of status in the PCI configuration space.
pub const STATUS_OFFSET: usize = 0x06;
/// Offset of revision ID in the PCI configuration space.
pub const REVISION_ID_OFFSET: usize = 0x08;
/// Offset of programming interface in the PCI configuration space.
pub const PROG_IF_OFFSET: usize = 0x09;
/// Offset of subclass in the PCI configuration space.
pub const SUBCLASS_OFFSET: usize = 0x0A;
/// Offset of class code in the PCI configuration space.
pub const CLASS_CODE_OFFSET: usize = 0x0B;
/// Offset of cache line size in the PCI configuration space.
pub const CACHE_LINE_SIZE_OFFSET: usize = 0x0C;
/// Offset of latency timer in the PCI configuration space.
pub const LATENCY_TIMER_OFFSET: usize = 0x0D;
/// Offset of header type in the PCI configuration space.
pub const HEADER_TYPE_OFFSET: usize = 0x0E;
/// Offset of BIST in the PCI configuration space.
pub const BIST_OFFSET: usize = 0x0F;

// Check to ensure all address accesses to the configuration space are 32bit
// aligned.
pub(crate) fn validate_dword_alignment(addr: usize) -> Result<()> {
    if addr % 4 != 0 {
        return Err(Error::UnalignedAccess);
    }
    Ok(())
}

#[derive(Debug)]
/// Error type for PCI configuration space accesses.
pub enum Error {
    /// Offset of the access to the configuration space has improper alignment.
    UnalignedAccess,
    /// Offset of the access to the configuration space is out of bounds.
    OffsetOutOfBounds,
    // Length of the buffer in the access to the configuration space is
    // invalid.
    // Only used if `read` and `write` methods use buffers and not `u32`s.

    // InvalidDataLen,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match *self {
            UnalignedAccess => write!(f, "unaligned access"),
            OffsetOutOfBounds => write!(f, "out of bounds access"),
        }
    }
}

/// Enum representing PCI header types as presented in the PCI specification.
#[derive(Copy, Clone)]
pub enum PciHeaderType {
    /// Generic PCI device.
    Device = 0x00,
    /// PCI-to-PCI bridge.
    PciToPciBridge = 0x01,
    /// PCI-to-Cardbus bridge.
    PciToCardbusBridge = 0x02,
    /// Unknown header type.
    Unknown = 0xff,
}

impl From<u8> for PciHeaderType {
    fn from(item: u8) -> Self {
        match item {
            0x00 => Self::Device,
            0x01 => Self::PciToPciBridge,
            0x02 => Self::PciToCardbusBridge,
            _ => Self::Unknown,
        }
    }
}

/// Enum representing PCI classes as presented in the PCI specification.
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum PciClassCode {
    /// Precursor to class codes (introduced in PCI rev 2.0)
    TooOld,
    /// Mass storage controller.
    MassStorage,
    /// Network controller.
    NetworkController,
    /// Display controller.
    DisplayController,
    /// Media controller.
    MultimediaController,
    /// Memory controller.
    MemoryController,
    /// Bridge device.
    BridgeDevice,
    /// Simple communications controller.
    SimpleCommunicationController,
    /// Base system peripheral.
    BaseSystemPeripheral,
    /// Input device.
    InputDevice,
    /// Docking station.
    DockingStation,
    /// Processor.
    Processor,
    /// Serial bus controller.
    SerialBusController,
    /// Wireless controller.
    WirelessController,
    /// Intelligent IO controller.
    IntelligentIoController,
    /// Encryption controller.
    EncryptionController,
    /// Signal processing.
    SignalProcessing,
    /// Processing accelerator.
    ProcessingAccelerator,
    /// Non-essential instrumentation.
    NonEssentialInstrumentation,
    /// Unknown class.
    Other = 0xff,
}

impl PciClassCode {
    /// Represent the class as a byte to be used in the PCI configuration
    /// space.
    pub fn value(self) -> u8 {
        self as u8
    }
}

impl From<u8> for PciClassCode {
    fn from(item: u8) -> Self {
        match item {
            0x00 => Self::TooOld,
            0x01 => Self::MassStorage,
            0x02 => Self::NetworkController,
            0x03 => Self::DisplayController,
            0x04 => Self::MultimediaController,
            0x05 => Self::MemoryController,
            0x06 => Self::BridgeDevice,
            0x07 => Self::SimpleCommunicationController,
            0x08 => Self::BaseSystemPeripheral,
            0x09 => Self::InputDevice,
            0x0a => Self::DockingStation,
            0x0b => Self::Processor,
            0x0c => Self::SerialBusController,
            0x0d => Self::WirelessController,
            0x0e => Self::IntelligentIoController,
            0x0f => Self::EncryptionController,
            0x10 => Self::SignalProcessing,
            0x11 => Self::ProcessingAccelerator,
            0x12 => Self::NonEssentialInstrumentation,
            0xff | _ => Self::Other,
        }
    }
}

/// A PCI subclass abstraction.
///
/// Each class in `PciClassCode` can specify a unique set of subclasses. This
/// trait is implemented by each subclass. It allows use of a trait object to
/// generate configurations.
pub trait PciSubclass: From<u8> {
    /// Represent the subclass as a byte to be used in the PCI configuration
    /// space.
    fn value(&self) -> u8;
}

/// A PCI programming interface abstraction.
///
/// Each combination of `PciClassCode` and `PciSubclass` can specify a set of
/// register-level programming interfaces. This trait is implemented by each
/// programming interface. It allows use of a trait object to generate
/// configurations.
pub trait PciProgrammingInterface: From<u8> {
    /// Represent the programming interface as a byte to be used in the PCI
    /// configuration space.
    fn value(&self) -> u8;
}

/// Enum representing PCI capability IDs as presented in the PCI specification.
#[derive(PartialEq, Copy, Clone)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum PciCapabilityId {
    /// Reserved.
    ListId = 0,
    /// Power Management Interface.
    PowerManagement = 0x01,
    /// Accelerated Graphics Port.
    AcceleratedGraphicsPort = 0x02,
    /// Vital Product Data.
    VitalProductData = 0x03,
    /// Slot Identification.
    SlotIdentification = 0x04,
    /// Message Signaled Interrupts.
    MessageSignalledInterrupts = 0x05,
    /// CompactPCI Hot Swap.
    CompactPciHotSwap = 0x06,
    /// PCI-X.
    PciX = 0x07,
    /// HyperTransport.
    HyperTransport = 0x08,
    /// Vendor specific.
    VendorSpecific = 0x09,
    /// Debug port.
    DebugPort = 0x0A,
    /// CompactPCI central resource control.
    CompactPciCentralResourceControl = 0x0B,
    /// PCI Hot-Plug.
    PciHotPlug = 0x0C,
    /// PCI Bridge Subsystem Vendor ID.
    BridgeSubsystemVendorDeviceId = 0x0D,
    /// AGP 8x.
    Agp8X = 0x0E,
    /// Secure Device.
    SecureDevice = 0x0F,
    /// PCI Express.
    PciExpress = 0x10,
    /// MSI-X.
    MsiX = 0x11,
    /// SATA Data Index.
    SataDataIndex = 0x12,
    /// PCI Advanced Features.
    PciAdvancedFeatures = 0x13,
    /// PCI Enhanced Allocation.
    PciEnhancedAllocation = 0x14,
}

impl From<u8> for PciCapabilityId {
    fn from(item: u8) -> Self {
        match item {
            0x00 => PciCapabilityId::ListId,
            0x01 => PciCapabilityId::PowerManagement,
            0x02 => PciCapabilityId::AcceleratedGraphicsPort,
            0x03 => PciCapabilityId::VitalProductData,
            0x04 => PciCapabilityId::SlotIdentification,
            0x05 => PciCapabilityId::MessageSignalledInterrupts,
            0x06 => PciCapabilityId::CompactPciHotSwap,
            0x07 => PciCapabilityId::PciX,
            0x08 => PciCapabilityId::HyperTransport,
            0x09 => PciCapabilityId::VendorSpecific,
            0x0A => PciCapabilityId::DebugPort,
            0x0B => PciCapabilityId::CompactPciCentralResourceControl,
            0x0C => PciCapabilityId::PciHotPlug,
            0x0D => PciCapabilityId::BridgeSubsystemVendorDeviceId,
            0x0E => PciCapabilityId::Agp8X,
            0x0F => PciCapabilityId::SecureDevice,
            0x10 => PciCapabilityId::PciExpress,
            0x11 => PciCapabilityId::MsiX,
            0x12 => PciCapabilityId::SataDataIndex,
            0x13 => PciCapabilityId::PciAdvancedFeatures,
            0x14 => PciCapabilityId::PciEnhancedAllocation,
            _ => PciCapabilityId::ListId,
        }
    }
}

/// A PCI capability abstraction.
///
/// For the purposes of representing capabilities in the PCI configuration
/// space, an entity which implements `PciCapability` must be able to present
/// itself as a byte slice and provide its unique capability ID.
///
/// # Example
/// ```
/// use vm_pci::pci_config::{PciCapability, PciCapabilityId};
///
/// #[repr(packed)]
/// struct PcieCap {
///     ptype: u16,
///     data: [u8; 59],
/// }
///
/// impl PciCapability for PcieCap {
///     fn bytes(&self) -> &[u8] {
///         // Safe because struct is packed.
///         unsafe {
///             std::slice::from_raw_parts(
///                 self as *const PcieCap as *const u8,
///                 std::mem::size_of::<PcieCap>()
///             )
///         }
///     }
///
///     fn id(&self) -> PciCapabilityId {
///         PciCapabilityId::PciExpress
///     }
/// }
/// ```
pub trait PciCapability {
    /// Returns the byte representation of the capability in the configuration
    /// space.
    fn bytes(&self) -> &[u8];

    /// Returns the ID of the capability, as presented in the PCI
    /// specification.
    fn id(&self) -> PciCapabilityId;
}

/// Allows access to a PCI configuration space.
///
/// # Example
/// ```
/// use vm_pci::pci_config::{Error as ConfigSpaceAccessError, PciConfig};
///
/// struct DummyConfig {
///     registers: [u32; 64],
/// }
///
/// impl PciConfig for DummyConfig {
///     fn read_register(&self, idx: usize) -> std::result::Result<u32, ConfigSpaceAccessError> {
///         if idx >= 64 {
///             return Err(ConfigSpaceAccessError::OffsetOutOfBounds);
///         }
///         Ok(self.registers[idx])
///     }
///
///     fn write_register(
///         &mut self,
///         data: u32,
///         idx: usize,
///    ) -> std::result::Result<(), ConfigSpaceAccessError> {
///         if idx >= 64 {
///             return Err(ConfigSpaceAccessError::OffsetOutOfBounds);
///         }
///         self.registers[idx] = data;
///         Ok(())
///     }
/// }
/// ```
pub trait PciConfig {
    // Should this be done with registers or bytes??
    //fn read_data(&self, data: &mut[u8], offset: usize) -> Result<()>;
    //fn write_data(&mut self, data: &[u8], offset: usize) -> Result<()>;

    /// Read a register from the configuration space.
    ///
    /// # Arguments
    ///
    /// * `idx`:   index of the register
    fn read_register(&self, idx: usize) -> Result<u32>;

    /// Write to a register in the configuration space.
    ///
    /// # Arguments
    ///
    /// * `data`:   data to be written
    /// * `idx`:    index of the register
    fn write_register(&mut self, data: u32, idx: usize) -> Result<()>;

    /// Aligned read of a dword from the given offset in configuration space.
    ///
    /// # Arguments
    ///
    /// * `offset`:   offset of the dword to be read
    fn read_dword(&self, offset: usize) -> Result<u32> {
        validate_dword_alignment(offset)?;
        let reg_offset = offset / 4;
        self.read_register(reg_offset)
    }

    /// Aligned read of a word from the given offset in configuration space.
    ///
    /// # Arguments
    ///
    /// * `offset`:   offset of the word to be read
    fn read_word(&self, offset: usize) -> Result<u16> {
        let reg_offset = offset / 4;
        let byte_idx = offset % 4;
        let res = match byte_idx {
            0 => self.read_register(reg_offset)? & 0x0000_ffff,
            2 => (self.read_register(reg_offset)? & 0xffff_0000) >> 16,
            1 | 3 => {
                return Err(Error::UnalignedAccess);
            }
            _ => unreachable!(),
        };
        // Or u16::try_from(res).unwrap()
        Ok(res as u16)
    }

    /// Aligned read of a byte from the given offset in configuration space.
    ///
    /// # Arguments
    ///
    /// * `offset`:   offset of the byte to be read
    fn read_byte(&self, offset: usize) -> Result<u8> {
        let reg_offset = offset / 4;
        let byte_idx = offset % 4;
        let res = match byte_idx {
            0 => self.read_register(reg_offset)? & 0x0000_00ff,
            1 => (self.read_register(reg_offset)? & 0x0000_ff00) >> 8,
            2 => (self.read_register(reg_offset)? & 0x00ff_0000) >> 16,
            3 => (self.read_register(reg_offset)? & 0xff00_0000) >> 24,
            _ => unreachable!(),
        };
        // Or u8::try_from(res).unwrap()
        Ok(res as u8)
    }

    /// Aligned write of a dword at the given offset in configuration space.
    ///
    /// # Arguments
    ///
    /// * `offset`:   offset of the dword to be written
    fn write_dword(&mut self, value: u32, offset: usize) -> Result<()> {
        validate_dword_alignment(offset)?;
        let reg_offset = offset / 4;
        self.write_register(value, reg_offset)
    }

    /// Aligned write of a word at the given offset in configuration space.
    ///
    /// # Arguments
    ///
    /// * `offset`:   offset of the word to be written
    fn write_word(&mut self, value: u16, offset: usize) -> Result<()> {
        let reg_offset = offset / 4;
        let byte_idx = offset % 4;
        let res = match byte_idx {
            0 => (self.read_register(reg_offset)? & 0xffff_0000) | u32::from(value),
            2 => (self.read_register(reg_offset)? & 0x0000_ffff) | (u32::from(value) << 16),
            1 | 3 => {
                return Err(Error::UnalignedAccess);
            }
            _ => unreachable!(),
        };
        self.write_register(res, reg_offset)
    }

    /// Aligned write of a byte at the given offset in configuration space.
    ///
    /// # Arguments
    ///
    /// * `offset`:   offset of the byte to be written
    fn write_byte(&mut self, value: u8, offset: usize) -> Result<()> {
        let reg_offset = offset / 4;
        let byte_idx = offset % 4;
        let res = match byte_idx {
            0 => (self.read_register(reg_offset)? & 0xffff_ff00) | u32::from(value),
            1 => (self.read_register(reg_offset)? & 0xffff_00ff) | (u32::from(value) << 8),
            2 => (self.read_register(reg_offset)? & 0xff00_ffff) | (u32::from(value) << 16),
            3 => (self.read_register(reg_offset)? & 0x00ff_ffff) | (u32::from(value) << 24),
            _ => unreachable!(),
        };
        self.write_register(res, reg_offset)
    }

    /// Reads the vendor ID from the configuration space.
    fn vendor_id(&self) -> Result<u16> {
        self.read_word(VENDOR_ID_OFFSET)
    }

    /// Writes the vendor ID to the configuration space.
    fn write_vendor_id(&mut self, device_id: u16) -> Result<()> {
        self.write_word(device_id, VENDOR_ID_OFFSET)
    }

    /// Reads the device ID from the configuration space.
    fn device_id(&self) -> Result<u16> {
        self.read_word(DEVICE_ID_OFFSET)
    }

    /// Writes the device ID to the configuration space.
    fn write_device_id(&mut self, device_id: u16) -> Result<()> {
        self.write_word(device_id, DEVICE_ID_OFFSET)
    }

    /// Reads the command from the configuration space.
    fn command(&self) -> Result<u16> {
        self.read_word(COMMAND_OFFSET)
    }

    /// Writes the command to the configuration space.
    fn write_command(&mut self, command: u16) -> Result<()> {
        self.write_word(command, COMMAND_OFFSET)
    }

    /// Reads the status from the configuration space.
    fn status(&self) -> Result<u16> {
        self.read_word(STATUS_OFFSET)
    }

    /// Writes the status to the configuration space.
    fn write_status(&mut self, status: u16) -> Result<()> {
        self.write_word(status, STATUS_OFFSET)
    }

    /// Reads the revision ID from the configuration space.
    fn revision_id(&self) -> Result<u8> {
        self.read_byte(REVISION_ID_OFFSET)
    }

    /// Writes the revision ID to the configuration space.
    fn write_revision_id(&mut self, revision_id: u8) -> Result<()> {
        self.write_byte(revision_id, REVISION_ID_OFFSET)
    }

    /// Reads the programming interface from the configuration space.
    fn prog_if(&self) -> Result<u8> {
        self.read_byte(PROG_IF_OFFSET)
    }

    /// Writes the programming interface to the configuration space.
    fn write_prog_if(&mut self, prog_if: u8) -> Result<()> {
        self.write_byte(prog_if, PROG_IF_OFFSET)
    }

    /// Reads the subclass from the configuration space.
    fn subclass(&self) -> Result<u8> {
        self.read_byte(SUBCLASS_OFFSET)
    }

    /// Writes the subclass to the configuration space.
    fn write_subclass(&mut self, subclass: u8) -> Result<()> {
        self.write_byte(subclass, SUBCLASS_OFFSET)
    }

    /// Reads the class code from the configuration space.
    fn class_code(&self) -> Result<u8> {
        self.read_byte(CLASS_CODE_OFFSET)
    }

    /// Writes the class code to the configuration space.
    fn write_class_code(&mut self, class_code: u8) -> Result<()> {
        self.write_byte(class_code, CLASS_CODE_OFFSET)
    }

    /// Reads the cache line size from the configuration space.
    fn cache_line_size(&self) -> Result<u8> {
        self.read_byte(CACHE_LINE_SIZE_OFFSET)
    }

    /// Writes the cache line size to the configuration space.
    fn write_cache_line_size(&mut self, cache_line_size: u8) -> Result<()> {
        self.write_byte(cache_line_size, CACHE_LINE_SIZE_OFFSET)
    }

    /// Reads the latency timer from the configuration space.
    fn latency_timer(&self) -> Result<u8> {
        self.read_byte(LATENCY_TIMER_OFFSET)
    }

    /// Writes the latency timer to the configuration space.
    fn write_latency_timer(&mut self, latency_timer: u8) -> Result<()> {
        self.write_byte(latency_timer, LATENCY_TIMER_OFFSET)
    }

    /// Reads the header type from the configuration space.
    fn header_type(&self) -> Result<u8> {
        self.read_byte(HEADER_TYPE_OFFSET)
    }

    /// Writes the header type to the configuration space.
    fn write_header_type(&mut self, header_type: u8) -> Result<()> {
        self.write_byte(header_type, HEADER_TYPE_OFFSET)
    }

    /// Reads the BIST from the configuration space.
    fn bist(&self) -> Result<u8> {
        self.read_byte(BIST_OFFSET)
    }

    /// Writes the BIST to the configuration space.
    fn write_bist(&mut self, bist: u8) -> Result<()> {
        self.write_byte(bist, BIST_OFFSET)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_dword_alignment() {
        match validate_dword_alignment(0x1000) {
            Ok(()) => {}
            _ => assert!(false),
        }

        match validate_dword_alignment(0x1003) {
            Err(Error::UnalignedAccess) => {}
            _ => assert!(false),
        }
    }

    struct DummyConfig {
        registers: [u32; 64],
    }

    impl PciConfig for DummyConfig {
        fn read_register(&self, idx: usize) -> std::result::Result<u32, Error> {
            if idx >= 64 {
                return Err(Error::OffsetOutOfBounds);
            }
            Ok(self.registers[idx])
        }

        fn write_register(&mut self, data: u32, idx: usize) -> std::result::Result<(), Error> {
            if idx >= 64 {
                return Err(Error::OffsetOutOfBounds);
            }
            self.registers[idx] = data;
            Ok(())
        }
    }

    #[test]
    fn test_config_read() {
        let mut config = DummyConfig {
            registers: [0u32; 64],
        };
        config.registers[0] = 0xdead_beef;
        config.registers[4] = 0xddcc_bbaa;

        // Byte reads. All byte reads are aligned.
        assert_eq!(config.read_byte(0).unwrap(), 0xef);
        assert_eq!(config.read_byte(1).unwrap(), 0xbe);
        assert_eq!(config.read_byte(2).unwrap(), 0xad);
        assert_eq!(config.read_byte(3).unwrap(), 0xde);
        assert_eq!(config.read_byte(16).unwrap(), 0xaa);
        assert_eq!(config.read_byte(17).unwrap(), 0xbb);
        assert_eq!(config.read_byte(18).unwrap(), 0xcc);
        assert_eq!(config.read_byte(19).unwrap(), 0xdd);

        // Word reads.
        assert_eq!(config.read_word(0).unwrap(), 0xbeef);
        assert_eq!(config.read_word(2).unwrap(), 0xdead);
        assert!(config.read_word(1).is_err());
        assert!(config.read_word(3).is_err());
        assert_eq!(config.read_word(16).unwrap(), 0xbbaa);
        assert_eq!(config.read_word(18).unwrap(), 0xddcc);
        assert!(config.read_word(17).is_err());
        assert!(config.read_word(19).is_err());

        // Dword reads.
        assert_eq!(config.read_dword(0).unwrap(), 0xdead_beef);
        assert!(config.read_dword(1).is_err());
        assert!(config.read_dword(2).is_err());
        assert!(config.read_dword(3).is_err());
        assert_eq!(config.read_dword(16).unwrap(), 0xddcc_bbaa);
        assert!(config.read_dword(17).is_err());
        assert!(config.read_dword(18).is_err());
        assert!(config.read_dword(19).is_err());
    }

    #[test]
    fn test_config_write() {
        let mut config = DummyConfig {
            registers: [0u32; 64],
        };

        // Byte writes. All byte accesses are aligned.
        assert!(config.write_byte(0xef, 0).is_ok());
        assert!(config.write_byte(0xbe, 1).is_ok());
        assert!(config.write_byte(0xad, 2).is_ok());
        assert!(config.write_byte(0xde, 3).is_ok());
        // Assert writes are visible in the register.
        assert_eq!(config.registers[0], 0xdead_beef);

        // Word writes.
        assert!(config.write_word(0xbeef, 4).is_ok());
        assert!(config.write_word(0xdead, 6).is_ok());
        // Assert writes are visible in the register.
        assert_eq!(config.registers[1], 0xdead_beef);

        // Unaligned word accesses.
        assert!(config.write_word(0, 5).is_err());
        assert!(config.write_word(0, 7).is_err());
        // Assert state hasn't changed.
        assert_eq!(config.registers[1], 0xdead_beef);

        assert_eq!(config.registers[2], 0);
        // Dword write.
        assert!(config.write_dword(0xdead_beef, 8).is_ok());
        // Assert write are visible in the register.
        assert_eq!(config.registers[2], 0xdead_beef);

        // Unaligned dword accesses.
        assert!(config.write_dword(0, 9).is_err());
        assert!(config.write_dword(0, 10).is_err());
        assert!(config.write_dword(0, 11).is_err());
        // Assert state hasn't changed.
        assert_eq!(config.registers[2], 0xdead_beef);
    }
}
