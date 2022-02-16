// Copyright 2022 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! This module provides an abstraction over a subset of the PCI configuration
//! space functionality necessary for emulating a PCI device.
//!
//! The PCI specification states that every PCI compatible device must provide
//! a standardized configuration space of 256 bytes, organized into registers
//! of 4 bytes. All devices share the layout of the first 4 registers, defined
//! in the PCI specification.
//!
//! Abstractions for the common PCI configuration space are present in the
//! `pci_config` module.
//!
//! The header type determines what kind of device that is configured and the
//! layout of the next 12 registers. Header type 0x00 is for generic PCI
//! devices, present in the `device` module. Header type 0x01 is for PCI
//! bridges, present in the `bridge` module. Header type 0x02 is for PCI
//! Cardbus bridges and is not implemented in this crate.
//!
//! Though not present right now, the aim of this crate is to provide a
//! Host Bridge / PCI Bus abstraction which would then be integrated with the
//! `Device` and `MutDevice` traits in `vm-device`.

#![deny(missing_docs)]

/// PCI Base Address Register addressing and configuration.
pub mod bar;
/// PCI Bridge device (header type 0x01) configuration.
pub mod bridge;
/// PCI generic device (header type 0x00) configuration.
pub mod device;
/// Common PCI configuration.
pub mod pci_config;
