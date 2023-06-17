#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EXTI_CFGR0"]
    pub exti_cfgr0: EXTI_CFGR0,
    #[doc = "0x04 - EXTI_CFGR1"]
    pub exti_cfgr1: EXTI_CFGR1,
    #[doc = "0x08 - EXTI_CFGR2"]
    pub exti_cfgr2: EXTI_CFGR2,
    #[doc = "0x0c - EXTI_CFGR3"]
    pub exti_cfgr3: EXTI_CFGR3,
    #[doc = "0x10 - EXTI_CFGR4"]
    pub exti_cfgr4: EXTI_CFGR4,
    #[doc = "0x14 - EXTI_CFGR5"]
    pub exti_cfgr5: EXTI_CFGR5,
    #[doc = "0x18 - EXTI_CFGR6"]
    pub exti_cfgr6: EXTI_CFGR6,
    #[doc = "0x1c - EXTI_CFGR7"]
    pub exti_cfgr7: EXTI_CFGR7,
    #[doc = "0x20 - EXTI_CFGR8"]
    pub exti_cfgr8: EXTI_CFGR8,
    #[doc = "0x24 - EXTI_CFGR9"]
    pub exti_cfgr9: EXTI_CFGR9,
    #[doc = "0x28 - EXTI_CFGR10"]
    pub exti_cfgr10: EXTI_CFGR10,
    #[doc = "0x2c - EXTI_CFGR11"]
    pub exti_cfgr11: EXTI_CFGR11,
    #[doc = "0x30 - EXTI_CFGR12"]
    pub exti_cfgr12: EXTI_CFGR12,
    #[doc = "0x34 - EXTI_CFGR13"]
    pub exti_cfgr13: EXTI_CFGR13,
    #[doc = "0x38 - EXTI_CFGR14"]
    pub exti_cfgr14: EXTI_CFGR14,
    #[doc = "0x3c - EXTI_CFGR15"]
    pub exti_cfgr15: EXTI_CFGR15,
    #[doc = "0x40 - EXTI_CR"]
    pub exti_cr: EXTI_CR,
    #[doc = "0x44 - EXTI_EDGEFLGR"]
    pub exti_edgeflgr: EXTI_EDGEFLGR,
    #[doc = "0x48 - EXTI_EDGESR"]
    pub exti_edgesr: EXTI_EDGESR,
    #[doc = "0x4c - EXTI_SSCR"]
    pub exti_sscr: EXTI_SSCR,
    #[doc = "0x50 - EXTI_WAKUPCR"]
    pub exti_wakupcr: EXTI_WAKUPCR,
    #[doc = "0x54 - EXTI_WAKUPPOLR"]
    pub exti_wakuppolr: EXTI_WAKUPPOLR,
    #[doc = "0x58 - EXTI_WAKUPFLG"]
    pub exti_wakupflg: EXTI_WAKUPFLG,
}
#[doc = "EXTI_CFGR0 (rw) register accessor: an alias for `Reg<EXTI_CFGR0_SPEC>`"]
pub type EXTI_CFGR0 = crate::Reg<exti_cfgr0::EXTI_CFGR0_SPEC>;
#[doc = "EXTI_CFGR0"]
pub mod exti_cfgr0;
#[doc = "EXTI_CFGR1 (rw) register accessor: an alias for `Reg<EXTI_CFGR1_SPEC>`"]
pub type EXTI_CFGR1 = crate::Reg<exti_cfgr1::EXTI_CFGR1_SPEC>;
#[doc = "EXTI_CFGR1"]
pub mod exti_cfgr1;
#[doc = "EXTI_CFGR2 (rw) register accessor: an alias for `Reg<EXTI_CFGR2_SPEC>`"]
pub type EXTI_CFGR2 = crate::Reg<exti_cfgr2::EXTI_CFGR2_SPEC>;
#[doc = "EXTI_CFGR2"]
pub mod exti_cfgr2;
#[doc = "EXTI_CFGR3 (rw) register accessor: an alias for `Reg<EXTI_CFGR3_SPEC>`"]
pub type EXTI_CFGR3 = crate::Reg<exti_cfgr3::EXTI_CFGR3_SPEC>;
#[doc = "EXTI_CFGR3"]
pub mod exti_cfgr3;
#[doc = "EXTI_CFGR4 (rw) register accessor: an alias for `Reg<EXTI_CFGR4_SPEC>`"]
pub type EXTI_CFGR4 = crate::Reg<exti_cfgr4::EXTI_CFGR4_SPEC>;
#[doc = "EXTI_CFGR4"]
pub mod exti_cfgr4;
#[doc = "EXTI_CFGR5 (rw) register accessor: an alias for `Reg<EXTI_CFGR5_SPEC>`"]
pub type EXTI_CFGR5 = crate::Reg<exti_cfgr5::EXTI_CFGR5_SPEC>;
#[doc = "EXTI_CFGR5"]
pub mod exti_cfgr5;
#[doc = "EXTI_CFGR6 (rw) register accessor: an alias for `Reg<EXTI_CFGR6_SPEC>`"]
pub type EXTI_CFGR6 = crate::Reg<exti_cfgr6::EXTI_CFGR6_SPEC>;
#[doc = "EXTI_CFGR6"]
pub mod exti_cfgr6;
#[doc = "EXTI_CFGR7 (rw) register accessor: an alias for `Reg<EXTI_CFGR7_SPEC>`"]
pub type EXTI_CFGR7 = crate::Reg<exti_cfgr7::EXTI_CFGR7_SPEC>;
#[doc = "EXTI_CFGR7"]
pub mod exti_cfgr7;
#[doc = "EXTI_CFGR8 (rw) register accessor: an alias for `Reg<EXTI_CFGR8_SPEC>`"]
pub type EXTI_CFGR8 = crate::Reg<exti_cfgr8::EXTI_CFGR8_SPEC>;
#[doc = "EXTI_CFGR8"]
pub mod exti_cfgr8;
#[doc = "EXTI_CFGR9 (rw) register accessor: an alias for `Reg<EXTI_CFGR9_SPEC>`"]
pub type EXTI_CFGR9 = crate::Reg<exti_cfgr9::EXTI_CFGR9_SPEC>;
#[doc = "EXTI_CFGR9"]
pub mod exti_cfgr9;
#[doc = "EXTI_CFGR10 (rw) register accessor: an alias for `Reg<EXTI_CFGR10_SPEC>`"]
pub type EXTI_CFGR10 = crate::Reg<exti_cfgr10::EXTI_CFGR10_SPEC>;
#[doc = "EXTI_CFGR10"]
pub mod exti_cfgr10;
#[doc = "EXTI_CFGR11 (rw) register accessor: an alias for `Reg<EXTI_CFGR11_SPEC>`"]
pub type EXTI_CFGR11 = crate::Reg<exti_cfgr11::EXTI_CFGR11_SPEC>;
#[doc = "EXTI_CFGR11"]
pub mod exti_cfgr11;
#[doc = "EXTI_CFGR12 (rw) register accessor: an alias for `Reg<EXTI_CFGR12_SPEC>`"]
pub type EXTI_CFGR12 = crate::Reg<exti_cfgr12::EXTI_CFGR12_SPEC>;
#[doc = "EXTI_CFGR12"]
pub mod exti_cfgr12;
#[doc = "EXTI_CFGR13 (rw) register accessor: an alias for `Reg<EXTI_CFGR13_SPEC>`"]
pub type EXTI_CFGR13 = crate::Reg<exti_cfgr13::EXTI_CFGR13_SPEC>;
#[doc = "EXTI_CFGR13"]
pub mod exti_cfgr13;
#[doc = "EXTI_CFGR14 (rw) register accessor: an alias for `Reg<EXTI_CFGR14_SPEC>`"]
pub type EXTI_CFGR14 = crate::Reg<exti_cfgr14::EXTI_CFGR14_SPEC>;
#[doc = "EXTI_CFGR14"]
pub mod exti_cfgr14;
#[doc = "EXTI_CFGR15 (rw) register accessor: an alias for `Reg<EXTI_CFGR15_SPEC>`"]
pub type EXTI_CFGR15 = crate::Reg<exti_cfgr15::EXTI_CFGR15_SPEC>;
#[doc = "EXTI_CFGR15"]
pub mod exti_cfgr15;
#[doc = "EXTI_CR (rw) register accessor: an alias for `Reg<EXTI_CR_SPEC>`"]
pub type EXTI_CR = crate::Reg<exti_cr::EXTI_CR_SPEC>;
#[doc = "EXTI_CR"]
pub mod exti_cr;
#[doc = "EXTI_EDGEFLGR (rw) register accessor: an alias for `Reg<EXTI_EDGEFLGR_SPEC>`"]
pub type EXTI_EDGEFLGR = crate::Reg<exti_edgeflgr::EXTI_EDGEFLGR_SPEC>;
#[doc = "EXTI_EDGEFLGR"]
pub mod exti_edgeflgr;
#[doc = "EXTI_EDGESR (rw) register accessor: an alias for `Reg<EXTI_EDGESR_SPEC>`"]
pub type EXTI_EDGESR = crate::Reg<exti_edgesr::EXTI_EDGESR_SPEC>;
#[doc = "EXTI_EDGESR"]
pub mod exti_edgesr;
#[doc = "EXTI_SSCR (rw) register accessor: an alias for `Reg<EXTI_SSCR_SPEC>`"]
pub type EXTI_SSCR = crate::Reg<exti_sscr::EXTI_SSCR_SPEC>;
#[doc = "EXTI_SSCR"]
pub mod exti_sscr;
#[doc = "EXTI_WAKUPCR (rw) register accessor: an alias for `Reg<EXTI_WAKUPCR_SPEC>`"]
pub type EXTI_WAKUPCR = crate::Reg<exti_wakupcr::EXTI_WAKUPCR_SPEC>;
#[doc = "EXTI_WAKUPCR"]
pub mod exti_wakupcr;
#[doc = "EXTI_WAKUPPOLR (rw) register accessor: an alias for `Reg<EXTI_WAKUPPOLR_SPEC>`"]
pub type EXTI_WAKUPPOLR = crate::Reg<exti_wakuppolr::EXTI_WAKUPPOLR_SPEC>;
#[doc = "EXTI_WAKUPPOLR"]
pub mod exti_wakuppolr;
#[doc = "EXTI_WAKUPFLG (rw) register accessor: an alias for `Reg<EXTI_WAKUPFLG_SPEC>`"]
pub type EXTI_WAKUPFLG = crate::Reg<exti_wakupflg::EXTI_WAKUPFLG_SPEC>;
#[doc = "EXTI_WAKUPFLG"]
pub mod exti_wakupflg;
