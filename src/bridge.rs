// Copyright 2022 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::pci_config::PciSubclass;

// PCI-to-PCI Bridge constants.
/// Byte offset of the start of the BARs in the PCI configuration space.
pub const BARS_START_OFFSET: usize = 0x10;
/// Number of BARs in a generic PCI device configuration space.
pub const NUM_BARS: usize = 2;
/// Offset of Primary Bus Number in the PCI configuration space.
pub const PRIMARY_BUS_NUMBER_OFFSET: usize = 0x18;
/// Offset of Secondary Bus Number in the PCI configuration space.
pub const SECONDARY_BUS_NUMBER_OFFSET: usize = 0x19;
/// Offset of Subordinate Bus Number in the PCI configuration space.
pub const SUBORDINATE_BUS_NUMBER: usize = 0x1a;
/// Offset of Secondary Latency Timer in the PCI configuration space.
pub const SECONDARY_LATENCY_TIMER: usize = 0x1b;
/// Offset of IO Base in the PCI configuration space.
pub const IO_BASE_OFFSET: usize = 0x1c;
/// Offset of IO Limit in the PCI configuration space.
pub const IO_LIMIT_OFFSET: usize = 0x1d;
/// Offset of Secondary Status in the PCI configuration space.
pub const SECONDARY_STATUS_OFFSET: usize = 0x1e;
/// Offset of Memory Base in the PCI configuration space.
pub const MEMORY_BASE_OFFSET: usize = 0x20;
/// Offset of Memory Limit in the PCI configuration space.
pub const MEMORY_LIMIT_OFFSET: usize = 0x22;
/// Offset of Prefetchable Memory Base in the PCI configuration space.
pub const PREFETCH_MEMORY_BASE_OFFSET: usize = 0x24;
/// Offset of Prefetchable Memory Limit in the PCI configuration space.
pub const PREFETCH_MEMORY_LIMIT_OFFSET: usize = 0x26;
/// Offset of Prefetchable Base Upper 32 bits in the PCI configuration space.
pub const PREFETCH_BASE_UPPER_OFFSET: usize = 0x28;
/// Offset of Prefetchable Limit Upper 32 bits in the PCI configuration space.
pub const PREFETCH_LIMIT_UPPER_OFFSET: usize = 0x2c;
/// Offset of IO Base Upper 32 bits in the PCI configuration space.
pub const IO_BASE_UPPER_OFFSET: usize = 0x30;
/// Offset of IO Base Lower 32 bits in the PCI configuration space.
pub const IO_BASE_LOWER_OFFSET: usize = 0x32;
/// Offset of Capabilities Pointer in the PCI configuration space.
pub const CAPABILITIES_POINTER_OFFSET: usize = 0x34;
/// Offset of the ROM BAR in the PCI configuration space.
pub const ROM_BAR_OFFSET: usize = 0x38;
/// Offset of Interrupt Line in the PCI configuration space.
pub const INTERRUPT_LINE_OFFSET: usize = 0x3c;
/// Offset of Interrupt Pin in the PCI configuration space.
pub const INTERRUPT_PIN_OFFSET: usize = 0x3d;
/// Offset of Bridge Control in the PCI configuration space.
pub const BRIDGE_CONTROL_OFFSET: usize = 0x3e;

#[derive(Copy, Clone)]
/// Enum representing PCI bridge subclasses as presented in the PCI
/// specification.
pub enum PciBridgeSubclass {
    /// Host bridge.
    HostBridge = 0x00,
    /// PCI-to-ISA bridge.
    IsaBridge = 0x01,
    /// PCI-to-PCI bridge.
    PciToPciBridge = 0x04,
    /// Unknown/unimplemented bridge types.
    OtherBridgeDevice = 0x80,
}

impl From<u8> for PciBridgeSubclass {
    fn from(item: u8) -> Self {
        match item {
            0x00 => Self::HostBridge,
            0x01 => Self::IsaBridge,
            0x04 => Self::PciToPciBridge,
            0x80 | _ => Self::OtherBridgeDevice,
        }
    }
}

impl PciSubclass for PciBridgeSubclass {
    fn value(&self) -> u8 {
        *self as u8
    }
}
