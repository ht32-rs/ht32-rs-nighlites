#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BAKSR"]
    pub baksr: BAKSR,
    #[doc = "0x04 - BAKCR"]
    pub bakcr: BAKCR,
    #[doc = "0x08 - BAKTEST"]
    pub baktest: BAKTEST,
    #[doc = "0x0c - BAKLDOSR"]
    pub bakldosr: BAKLDOSR,
    #[doc = "0x10 - LVDCSR"]
    pub lvdcsr: LVDCSR,
    _reserved5: [u8; 0xec],
    #[doc = "0x100 - BAKREG0"]
    pub bakreg0: BAKREG0,
    #[doc = "0x104 - BAKREG1"]
    pub bakreg1: BAKREG1,
    #[doc = "0x108 - BAKREG2"]
    pub bakreg2: BAKREG2,
    #[doc = "0x10c - BAKREG3"]
    pub bakreg3: BAKREG3,
    #[doc = "0x110 - BAKREG4"]
    pub bakreg4: BAKREG4,
    #[doc = "0x114 - BAKREG5"]
    pub bakreg5: BAKREG5,
    #[doc = "0x118 - BAKREG6"]
    pub bakreg6: BAKREG6,
    #[doc = "0x11c - BAKREG7"]
    pub bakreg7: BAKREG7,
    #[doc = "0x120 - BAKREG8"]
    pub bakreg8: BAKREG8,
    #[doc = "0x124 - BAKREG9"]
    pub bakreg9: BAKREG9,
}
#[doc = "BAKSR (rw) register accessor: an alias for `Reg<BAKSR_SPEC>`"]
pub type BAKSR = crate::Reg<baksr::BAKSR_SPEC>;
#[doc = "BAKSR"]
pub mod baksr;
#[doc = "BAKCR (rw) register accessor: an alias for `Reg<BAKCR_SPEC>`"]
pub type BAKCR = crate::Reg<bakcr::BAKCR_SPEC>;
#[doc = "BAKCR"]
pub mod bakcr;
#[doc = "BAKTEST (rw) register accessor: an alias for `Reg<BAKTEST_SPEC>`"]
pub type BAKTEST = crate::Reg<baktest::BAKTEST_SPEC>;
#[doc = "BAKTEST"]
pub mod baktest;
#[doc = "BAKLDOSR (rw) register accessor: an alias for `Reg<BAKLDOSR_SPEC>`"]
pub type BAKLDOSR = crate::Reg<bakldosr::BAKLDOSR_SPEC>;
#[doc = "BAKLDOSR"]
pub mod bakldosr;
#[doc = "LVDCSR (rw) register accessor: an alias for `Reg<LVDCSR_SPEC>`"]
pub type LVDCSR = crate::Reg<lvdcsr::LVDCSR_SPEC>;
#[doc = "LVDCSR"]
pub mod lvdcsr;
#[doc = "BAKREG0 (rw) register accessor: an alias for `Reg<BAKREG0_SPEC>`"]
pub type BAKREG0 = crate::Reg<bakreg0::BAKREG0_SPEC>;
#[doc = "BAKREG0"]
pub mod bakreg0;
#[doc = "BAKREG1 (rw) register accessor: an alias for `Reg<BAKREG1_SPEC>`"]
pub type BAKREG1 = crate::Reg<bakreg1::BAKREG1_SPEC>;
#[doc = "BAKREG1"]
pub mod bakreg1;
#[doc = "BAKREG2 (rw) register accessor: an alias for `Reg<BAKREG2_SPEC>`"]
pub type BAKREG2 = crate::Reg<bakreg2::BAKREG2_SPEC>;
#[doc = "BAKREG2"]
pub mod bakreg2;
#[doc = "BAKREG3 (rw) register accessor: an alias for `Reg<BAKREG3_SPEC>`"]
pub type BAKREG3 = crate::Reg<bakreg3::BAKREG3_SPEC>;
#[doc = "BAKREG3"]
pub mod bakreg3;
#[doc = "BAKREG4 (rw) register accessor: an alias for `Reg<BAKREG4_SPEC>`"]
pub type BAKREG4 = crate::Reg<bakreg4::BAKREG4_SPEC>;
#[doc = "BAKREG4"]
pub mod bakreg4;
#[doc = "BAKREG5 (rw) register accessor: an alias for `Reg<BAKREG5_SPEC>`"]
pub type BAKREG5 = crate::Reg<bakreg5::BAKREG5_SPEC>;
#[doc = "BAKREG5"]
pub mod bakreg5;
#[doc = "BAKREG6 (rw) register accessor: an alias for `Reg<BAKREG6_SPEC>`"]
pub type BAKREG6 = crate::Reg<bakreg6::BAKREG6_SPEC>;
#[doc = "BAKREG6"]
pub mod bakreg6;
#[doc = "BAKREG7 (rw) register accessor: an alias for `Reg<BAKREG7_SPEC>`"]
pub type BAKREG7 = crate::Reg<bakreg7::BAKREG7_SPEC>;
#[doc = "BAKREG7"]
pub mod bakreg7;
#[doc = "BAKREG8 (rw) register accessor: an alias for `Reg<BAKREG8_SPEC>`"]
pub type BAKREG8 = crate::Reg<bakreg8::BAKREG8_SPEC>;
#[doc = "BAKREG8"]
pub mod bakreg8;
#[doc = "BAKREG9 (rw) register accessor: an alias for `Reg<BAKREG9_SPEC>`"]
pub type BAKREG9 = crate::Reg<bakreg9::BAKREG9_SPEC>;
#[doc = "BAKREG9"]
pub mod bakreg9;
