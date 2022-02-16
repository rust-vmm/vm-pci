# vm-pci

## Design

This crate provides an abstraction over a subset of the PCI configuration
space functionality necessary for emulating a PCI device.

The PCI specification states that every PCI compatible device must provide
a standardized configuration space of 256 bytes, organized into registers
of 4 bytes. All devices share the layout of the first 4 registers, defined
below

```
bit 31                                             bit 0
|------------------------------------------------------|
|        Device ID         |         Vendor ID         |
|--------------------------+---------------------------|
|          Status          |         Command           |
|--------------------------+------------+--------------|
| Class code | Subclass    |    Prog IF | Revision ID  |
|------------+-------------+------------+--------------|
| BIST       | Header Type | Lat Timer  | Cache Line   |
|--------------------------+---------------------------|
```

Abstractions for the common PCI configuration space are present in the
`pci_config` module.

The header type determines what kind of device that is configured and the
layout of the next 12 registers. Header type 0x00 is for generic PCI
devices, present in the `device` module. Header type 0x01 is for PCI
bridges, present in the `bridge` module. Header type 0x02 is for PCI
Cardbus bridges and is not implemented in this crate.

Though not present right now, the aim of this crate is to provide a
Host Bridge PCI Bus abstraction which would then be integrated with the
`Device` and `MutDevice` traits in `vm-device`.

## Usage

Add vm-pci as a dependency in Cargo.toml

```toml
[dependencies]
vm-pci = "*"
````

Import the `PciConfig` trait and implement it to emulate a simple host bridge.
Then import the `PciDeviceConfig` trait and implement it for your devices.
The device implementations need to be added to a PCI bus, which needs to be
exposed in the Port IO or MMIO Device Managers in order to be visible in the
guest. Additionally, you can add any desired PCI capabilities to your devices'
configuration spaces and emulate them separately.

## Examples

Basic implementation of a common PCI configuration space.

```rust
use vm_pci;

struct DummyConfig {
    registers: [u32; 64],
}

impl PciConfig for DummyConfig {
    fn read_register(&self, idx: usize) -> std::result::Result<u32, ConfigSpaceAccessError> {
        if idx >= 64 {
            return Err(ConfigSpaceAccessError::OffsetOutOfBounds);
        }
        Ok(self.registers[idx])
    }

    fn write_register(
        &mut self,
        data: u32,
        idx: usize,
   ) -> std::result::Result<(), ConfigSpaceAccessError> {
        if idx >= 64 {
            return Err(ConfigSpaceAccessError::OffsetOutOfBounds);
        }
        self.registers[idx] = data;
        Ok(())
    }
}
```

The struct above can have further trait implementations (e.g.
`PciDeviceConfig`) if one needs to access to PCI device functionality, like
adding and retrieving BARs.

## License

This project is licensed under either of

- [Apache License](http://www.apache.org/licenses/LICENSE-2.0), Version 2.0
- [BSD-3-Clause License](https://opensource.org/licenses/BSD-3-Clause)
