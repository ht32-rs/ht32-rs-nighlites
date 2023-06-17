#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWRSR"]
    pub pwrsr: PWRSR,
    #[doc = "0x04 - PWRCR"]
    pub pwrcr: PWRCR,
    #[doc = "0x08 - PWRTEST"]
    pub pwrtest: PWRTEST,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - LVDCSR"]
    pub lvdcsr: LVDCSR,
}
#[doc = "PWRSR (rw) register accessor: an alias for `Reg<PWRSR_SPEC>`"]
pub type PWRSR = crate::Reg<pwrsr::PWRSR_SPEC>;
#[doc = "PWRSR"]
pub mod pwrsr;
#[doc = "PWRCR (rw) register accessor: an alias for `Reg<PWRCR_SPEC>`"]
pub type PWRCR = crate::Reg<pwrcr::PWRCR_SPEC>;
#[doc = "PWRCR"]
pub mod pwrcr;
#[doc = "PWRTEST (rw) register accessor: an alias for `Reg<PWRTEST_SPEC>`"]
pub type PWRTEST = crate::Reg<pwrtest::PWRTEST_SPEC>;
#[doc = "PWRTEST"]
pub mod pwrtest;
#[doc = "LVDCSR (rw) register accessor: an alias for `Reg<LVDCSR_SPEC>`"]
pub type LVDCSR = crate::Reg<lvdcsr::LVDCSR_SPEC>;
#[doc = "LVDCSR"]
pub mod lvdcsr;
