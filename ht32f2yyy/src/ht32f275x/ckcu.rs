#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CKCU_GCFGR"]
    pub ckcu_gcfgr: CKCU_GCFGR,
    #[doc = "0x04 - CKCU_GCCR"]
    pub ckcu_gccr: CKCU_GCCR,
    #[doc = "0x08 - CKCU_GCSR"]
    pub ckcu_gcsr: CKCU_GCSR,
    #[doc = "0x0c - CKCU_GCIR"]
    pub ckcu_gcir: CKCU_GCIR,
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - CKCU_PLLCFGR"]
    pub ckcu_pllcfgr: CKCU_PLLCFGR,
    #[doc = "0x1c - CKCU_PLLCR"]
    pub ckcu_pllcr: CKCU_PLLCR,
    #[doc = "0x20 - CKCU_AHBCFGR"]
    pub ckcu_ahbcfgr: CKCU_AHBCFGR,
    #[doc = "0x24 - CKCU_AHBCCR"]
    pub ckcu_ahbccr: CKCU_AHBCCR,
    #[doc = "0x28 - CKCU_APBCFGR"]
    pub ckcu_apbcfgr: CKCU_APBCFGR,
    #[doc = "0x2c - CKCU_APBCCR0"]
    pub ckcu_apbccr0: CKCU_APBCCR0,
    #[doc = "0x30 - CKCU_APBCCR1"]
    pub ckcu_apbccr1: CKCU_APBCCR1,
    #[doc = "0x34 - CKCU_CKST"]
    pub ckcu_ckst: CKCU_CKST,
    _reserved12: [u8; 0x02c8],
    #[doc = "0x300 - CKCU_LPCR"]
    pub ckcu_lpcr: CKCU_LPCR,
    #[doc = "0x304 - CKCU_MCUDBGCR"]
    pub ckcu_mcudbgcr: CKCU_MCUDBGCR,
}
#[doc = "CKCU_GCFGR (rw) register accessor: an alias for `Reg<CKCU_GCFGR_SPEC>`"]
pub type CKCU_GCFGR = crate::Reg<ckcu_gcfgr::CKCU_GCFGR_SPEC>;
#[doc = "CKCU_GCFGR"]
pub mod ckcu_gcfgr;
#[doc = "CKCU_GCCR (rw) register accessor: an alias for `Reg<CKCU_GCCR_SPEC>`"]
pub type CKCU_GCCR = crate::Reg<ckcu_gccr::CKCU_GCCR_SPEC>;
#[doc = "CKCU_GCCR"]
pub mod ckcu_gccr;
#[doc = "CKCU_GCSR (rw) register accessor: an alias for `Reg<CKCU_GCSR_SPEC>`"]
pub type CKCU_GCSR = crate::Reg<ckcu_gcsr::CKCU_GCSR_SPEC>;
#[doc = "CKCU_GCSR"]
pub mod ckcu_gcsr;
#[doc = "CKCU_GCIR (rw) register accessor: an alias for `Reg<CKCU_GCIR_SPEC>`"]
pub type CKCU_GCIR = crate::Reg<ckcu_gcir::CKCU_GCIR_SPEC>;
#[doc = "CKCU_GCIR"]
pub mod ckcu_gcir;
#[doc = "CKCU_PLLCFGR (rw) register accessor: an alias for `Reg<CKCU_PLLCFGR_SPEC>`"]
pub type CKCU_PLLCFGR = crate::Reg<ckcu_pllcfgr::CKCU_PLLCFGR_SPEC>;
#[doc = "CKCU_PLLCFGR"]
pub mod ckcu_pllcfgr;
#[doc = "CKCU_PLLCR (rw) register accessor: an alias for `Reg<CKCU_PLLCR_SPEC>`"]
pub type CKCU_PLLCR = crate::Reg<ckcu_pllcr::CKCU_PLLCR_SPEC>;
#[doc = "CKCU_PLLCR"]
pub mod ckcu_pllcr;
#[doc = "CKCU_AHBCFGR (rw) register accessor: an alias for `Reg<CKCU_AHBCFGR_SPEC>`"]
pub type CKCU_AHBCFGR = crate::Reg<ckcu_ahbcfgr::CKCU_AHBCFGR_SPEC>;
#[doc = "CKCU_AHBCFGR"]
pub mod ckcu_ahbcfgr;
#[doc = "CKCU_AHBCCR (rw) register accessor: an alias for `Reg<CKCU_AHBCCR_SPEC>`"]
pub type CKCU_AHBCCR = crate::Reg<ckcu_ahbccr::CKCU_AHBCCR_SPEC>;
#[doc = "CKCU_AHBCCR"]
pub mod ckcu_ahbccr;
#[doc = "CKCU_APBCFGR (rw) register accessor: an alias for `Reg<CKCU_APBCFGR_SPEC>`"]
pub type CKCU_APBCFGR = crate::Reg<ckcu_apbcfgr::CKCU_APBCFGR_SPEC>;
#[doc = "CKCU_APBCFGR"]
pub mod ckcu_apbcfgr;
#[doc = "CKCU_APBCCR0 (rw) register accessor: an alias for `Reg<CKCU_APBCCR0_SPEC>`"]
pub type CKCU_APBCCR0 = crate::Reg<ckcu_apbccr0::CKCU_APBCCR0_SPEC>;
#[doc = "CKCU_APBCCR0"]
pub mod ckcu_apbccr0;
#[doc = "CKCU_APBCCR1 (rw) register accessor: an alias for `Reg<CKCU_APBCCR1_SPEC>`"]
pub type CKCU_APBCCR1 = crate::Reg<ckcu_apbccr1::CKCU_APBCCR1_SPEC>;
#[doc = "CKCU_APBCCR1"]
pub mod ckcu_apbccr1;
#[doc = "CKCU_CKST (rw) register accessor: an alias for `Reg<CKCU_CKST_SPEC>`"]
pub type CKCU_CKST = crate::Reg<ckcu_ckst::CKCU_CKST_SPEC>;
#[doc = "CKCU_CKST"]
pub mod ckcu_ckst;
#[doc = "CKCU_LPCR (rw) register accessor: an alias for `Reg<CKCU_LPCR_SPEC>`"]
pub type CKCU_LPCR = crate::Reg<ckcu_lpcr::CKCU_LPCR_SPEC>;
#[doc = "CKCU_LPCR"]
pub mod ckcu_lpcr;
#[doc = "CKCU_MCUDBGCR (rw) register accessor: an alias for `Reg<CKCU_MCUDBGCR_SPEC>`"]
pub type CKCU_MCUDBGCR = crate::Reg<ckcu_mcudbgcr::CKCU_MCUDBGCR_SPEC>;
#[doc = "CKCU_MCUDBGCR"]
pub mod ckcu_mcudbgcr;
