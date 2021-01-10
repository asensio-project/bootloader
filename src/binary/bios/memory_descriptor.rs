use crate::{binary::legacy_memory_region::LegacyMemoryRegion, memory_region::MemoryRegionKind};
use x86_64::PhysAddr;

impl LegacyMemoryRegion for E820MemoryRegion {
    fn start(&self) -> PhysAddr {
        PhysAddr::new(self.start_addr)
    }

    fn len(&self) -> u64 {
        self.len
    }

    fn kind(&self) -> MemoryRegionKind {
        match self.region_type {
            1 => MemoryRegionKind::Usable,
            other => MemoryRegionKind::UnknownBios(other),
        }
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct E820MemoryRegion {
    pub start_addr: u64,
    pub len: u64,
    pub region_type: u32,
    pub acpi_extended_attributes: u32,
}
