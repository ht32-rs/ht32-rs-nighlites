#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWRCU_PWRSR"]
    pub pwrcu_pwrsr: PWRCU_PWRSR,
    #[doc = "0x04 - PWRCU_PWRCR"]
    pub pwrcu_pwrcr: PWRCU_PWRCR,
    #[doc = "0x08 - PWRCU_PWRTEST"]
    pub pwrcu_pwrtest: PWRCU_PWRTEST,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - PWRCU_LVDCSR"]
    pub pwrcu_lvdcsr: PWRCU_LVDCSR,
    _reserved4: [u8; 0x08],
    #[doc = "0x1c - PWRCU_PWRLDOSR"]
    pub pwrcu_pwrldosr: PWRCU_PWRLDOSR,
}
#[doc = "PWRCU_PWRSR (rw) register accessor: an alias for `Reg<PWRCU_PWRSR_SPEC>`"]
pub type PWRCU_PWRSR = crate::Reg<pwrcu_pwrsr::PWRCU_PWRSR_SPEC>;
#[doc = "PWRCU_PWRSR"]
pub mod pwrcu_pwrsr;
#[doc = "PWRCU_PWRCR (rw) register accessor: an alias for `Reg<PWRCU_PWRCR_SPEC>`"]
pub type PWRCU_PWRCR = crate::Reg<pwrcu_pwrcr::PWRCU_PWRCR_SPEC>;
#[doc = "PWRCU_PWRCR"]
pub mod pwrcu_pwrcr;
#[doc = "PWRCU_PWRTEST (rw) register accessor: an alias for `Reg<PWRCU_PWRTEST_SPEC>`"]
pub type PWRCU_PWRTEST = crate::Reg<pwrcu_pwrtest::PWRCU_PWRTEST_SPEC>;
#[doc = "PWRCU_PWRTEST"]
pub mod pwrcu_pwrtest;
#[doc = "PWRCU_LVDCSR (rw) register accessor: an alias for `Reg<PWRCU_LVDCSR_SPEC>`"]
pub type PWRCU_LVDCSR = crate::Reg<pwrcu_lvdcsr::PWRCU_LVDCSR_SPEC>;
#[doc = "PWRCU_LVDCSR"]
pub mod pwrcu_lvdcsr;
#[doc = "PWRCU_PWRLDOSR (rw) register accessor: an alias for `Reg<PWRCU_PWRLDOSR_SPEC>`"]
pub type PWRCU_PWRLDOSR = crate::Reg<pwrcu_pwrldosr::PWRCU_PWRLDOSR_SPEC>;
#[doc = "PWRCU_PWRLDOSR"]
pub mod pwrcu_pwrldosr;
