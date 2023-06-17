#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMP_CR0"]
    pub cmp_cr0: CMP_CR0,
    #[doc = "0x04 - CMP_VALR0"]
    pub cmp_valr0: CMP_VALR0,
    #[doc = "0x08 - CMP_IER0"]
    pub cmp_ier0: CMP_IER0,
    #[doc = "0x0c - CMP_TFR0"]
    pub cmp_tfr0: CMP_TFR0,
    _reserved4: [u8; 0xf0],
    #[doc = "0x100 - CMP_CR1"]
    pub cmp_cr1: CMP_CR1,
    #[doc = "0x104 - CMP_VALR1"]
    pub cmp_valr1: CMP_VALR1,
    #[doc = "0x108 - CMP_IER1"]
    pub cmp_ier1: CMP_IER1,
    #[doc = "0x10c - CMP_TFR1"]
    pub cmp_tfr1: CMP_TFR1,
}
#[doc = "CMP_CR0 (rw) register accessor: an alias for `Reg<CMP_CR0_SPEC>`"]
pub type CMP_CR0 = crate::Reg<cmp_cr0::CMP_CR0_SPEC>;
#[doc = "CMP_CR0"]
pub mod cmp_cr0;
#[doc = "CMP_VALR0 (rw) register accessor: an alias for `Reg<CMP_VALR0_SPEC>`"]
pub type CMP_VALR0 = crate::Reg<cmp_valr0::CMP_VALR0_SPEC>;
#[doc = "CMP_VALR0"]
pub mod cmp_valr0;
#[doc = "CMP_IER0 (rw) register accessor: an alias for `Reg<CMP_IER0_SPEC>`"]
pub type CMP_IER0 = crate::Reg<cmp_ier0::CMP_IER0_SPEC>;
#[doc = "CMP_IER0"]
pub mod cmp_ier0;
#[doc = "CMP_TFR0 (rw) register accessor: an alias for `Reg<CMP_TFR0_SPEC>`"]
pub type CMP_TFR0 = crate::Reg<cmp_tfr0::CMP_TFR0_SPEC>;
#[doc = "CMP_TFR0"]
pub mod cmp_tfr0;
#[doc = "CMP_CR1 (rw) register accessor: an alias for `Reg<CMP_CR1_SPEC>`"]
pub type CMP_CR1 = crate::Reg<cmp_cr1::CMP_CR1_SPEC>;
#[doc = "CMP_CR1"]
pub mod cmp_cr1;
#[doc = "CMP_VALR1 (rw) register accessor: an alias for `Reg<CMP_VALR1_SPEC>`"]
pub type CMP_VALR1 = crate::Reg<cmp_valr1::CMP_VALR1_SPEC>;
#[doc = "CMP_VALR1"]
pub mod cmp_valr1;
#[doc = "CMP_IER1 (rw) register accessor: an alias for `Reg<CMP_IER1_SPEC>`"]
pub type CMP_IER1 = crate::Reg<cmp_ier1::CMP_IER1_SPEC>;
#[doc = "CMP_IER1"]
pub mod cmp_ier1;
#[doc = "CMP_TFR1 (rw) register accessor: an alias for `Reg<CMP_TFR1_SPEC>`"]
pub type CMP_TFR1 = crate::Reg<cmp_tfr1::CMP_TFR1_SPEC>;
#[doc = "CMP_TFR1"]
pub mod cmp_tfr1;
