#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RSTCU_GRSR"]
    pub rstcu_grsr: RSTCU_GRSR,
    #[doc = "0x04 - RSTCU_AHBPRSTR"]
    pub rstcu_ahbprstr: RSTCU_AHBPRSTR,
    #[doc = "0x08 - RSTCU_APBPRSTR0"]
    pub rstcu_apbprstr0: RSTCU_APBPRSTR0,
    #[doc = "0x0c - RSTCU_APBPRSTR1"]
    pub rstcu_apbprstr1: RSTCU_APBPRSTR1,
}
#[doc = "RSTCU_GRSR (rw) register accessor: an alias for `Reg<RSTCU_GRSR_SPEC>`"]
pub type RSTCU_GRSR = crate::Reg<rstcu_grsr::RSTCU_GRSR_SPEC>;
#[doc = "RSTCU_GRSR"]
pub mod rstcu_grsr;
#[doc = "RSTCU_AHBPRSTR (rw) register accessor: an alias for `Reg<RSTCU_AHBPRSTR_SPEC>`"]
pub type RSTCU_AHBPRSTR = crate::Reg<rstcu_ahbprstr::RSTCU_AHBPRSTR_SPEC>;
#[doc = "RSTCU_AHBPRSTR"]
pub mod rstcu_ahbprstr;
#[doc = "RSTCU_APBPRSTR0 (rw) register accessor: an alias for `Reg<RSTCU_APBPRSTR0_SPEC>`"]
pub type RSTCU_APBPRSTR0 = crate::Reg<rstcu_apbprstr0::RSTCU_APBPRSTR0_SPEC>;
#[doc = "RSTCU_APBPRSTR0"]
pub mod rstcu_apbprstr0;
#[doc = "RSTCU_APBPRSTR1 (rw) register accessor: an alias for `Reg<RSTCU_APBPRSTR1_SPEC>`"]
pub type RSTCU_APBPRSTR1 = crate::Reg<rstcu_apbprstr1::RSTCU_APBPRSTR1_SPEC>;
#[doc = "RSTCU_APBPRSTR1"]
pub mod rstcu_apbprstr1;
