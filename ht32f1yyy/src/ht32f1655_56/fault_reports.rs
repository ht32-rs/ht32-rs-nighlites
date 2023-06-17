#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_bfsr: [u8; 0x04],
    #[doc = "0x04 - HFSR"]
    pub hfsr: HFSR,
    #[doc = "0x08 - DFSR"]
    pub dfsr: DFSR,
    #[doc = "0x0c - MMFAR"]
    pub mmfar: MMFAR,
    #[doc = "0x10 - BFAR"]
    pub bfar: BFAR,
    #[doc = "0x14 - AFSR"]
    pub afsr: AFSR,
}
impl RegisterBlock {
    #[doc = "0x00 - MFSR"]
    #[inline(always)]
    pub const fn fault_reports_mfsr(&self) -> &FAULT_REPORTS_MFSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - CFSR"]
    #[inline(always)]
    pub const fn cfsr(&self) -> &CFSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x01 - BFSR"]
    #[inline(always)]
    pub const fn bfsr(&self) -> &BFSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(1usize).cast() }
    }
    #[doc = "0x02 - UFSR"]
    #[inline(always)]
    pub const fn ufsr(&self) -> &UFSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
}
#[doc = "CFSR (rw) register accessor: an alias for `Reg<CFSR_SPEC>`"]
pub type CFSR = crate::Reg<cfsr::CFSR_SPEC>;
#[doc = "CFSR"]
pub mod cfsr;
#[doc = "Fault_Reports_MFSR (rw) register accessor: an alias for `Reg<FAULT_REPORTS_MFSR_SPEC>`"]
pub type FAULT_REPORTS_MFSR = crate::Reg<fault_reports_mfsr::FAULT_REPORTS_MFSR_SPEC>;
#[doc = "MFSR"]
pub mod fault_reports_mfsr;
#[doc = "MMFAR (rw) register accessor: an alias for `Reg<MMFAR_SPEC>`"]
pub type MMFAR = crate::Reg<mmfar::MMFAR_SPEC>;
#[doc = "MMFAR"]
pub mod mmfar;
#[doc = "BFSR (rw) register accessor: an alias for `Reg<BFSR_SPEC>`"]
pub type BFSR = crate::Reg<bfsr::BFSR_SPEC>;
#[doc = "BFSR"]
pub mod bfsr;
#[doc = "BFAR (rw) register accessor: an alias for `Reg<BFAR_SPEC>`"]
pub type BFAR = crate::Reg<bfar::BFAR_SPEC>;
#[doc = "BFAR"]
pub mod bfar;
#[doc = "UFSR (rw) register accessor: an alias for `Reg<UFSR_SPEC>`"]
pub type UFSR = crate::Reg<ufsr::UFSR_SPEC>;
#[doc = "UFSR"]
pub mod ufsr;
#[doc = "HFSR (rw) register accessor: an alias for `Reg<HFSR_SPEC>`"]
pub type HFSR = crate::Reg<hfsr::HFSR_SPEC>;
#[doc = "HFSR"]
pub mod hfsr;
#[doc = "DFSR (rw) register accessor: an alias for `Reg<DFSR_SPEC>`"]
pub type DFSR = crate::Reg<dfsr::DFSR_SPEC>;
#[doc = "DFSR"]
pub mod dfsr;
#[doc = "AFSR (rw) register accessor: an alias for `Reg<AFSR_SPEC>`"]
pub type AFSR = crate::Reg<afsr::AFSR_SPEC>;
#[doc = "AFSR"]
pub mod afsr;
