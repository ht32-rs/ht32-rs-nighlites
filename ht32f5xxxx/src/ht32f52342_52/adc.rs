#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC_CR"]
    pub adc_cr: ADC_CR,
    #[doc = "0x04 - ADC_LST0"]
    pub adc_lst0: ADC_LST0,
    #[doc = "0x08 - ADC_LST1"]
    pub adc_lst1: ADC_LST1,
    _reserved3: [u8; 0x14],
    #[doc = "0x20 - ADC_STR"]
    pub adc_str: ADC_STR,
    _reserved4: [u8; 0x0c],
    #[doc = "0x30 - ADC_DR0"]
    pub adc_dr0: ADC_DR0,
    #[doc = "0x34 - ADC_DR1"]
    pub adc_dr1: ADC_DR1,
    #[doc = "0x38 - ADC_DR2"]
    pub adc_dr2: ADC_DR2,
    #[doc = "0x3c - ADC_DR3"]
    pub adc_dr3: ADC_DR3,
    #[doc = "0x40 - ADC_DR4"]
    pub adc_dr4: ADC_DR4,
    #[doc = "0x44 - ADC_DR5"]
    pub adc_dr5: ADC_DR5,
    #[doc = "0x48 - ADC_DR6"]
    pub adc_dr6: ADC_DR6,
    #[doc = "0x4c - ADC_DR7"]
    pub adc_dr7: ADC_DR7,
    _reserved12: [u8; 0x20],
    #[doc = "0x70 - ADC_TCR"]
    pub adc_tcr: ADC_TCR,
    #[doc = "0x74 - ADC_TSR"]
    pub adc_tsr: ADC_TSR,
    #[doc = "0x78 - ADC_WCR"]
    pub adc_wcr: ADC_WCR,
    #[doc = "0x7c - ADC_TR"]
    pub adc_tr: ADC_TR,
    #[doc = "0x80 - ADC_IMR"]
    pub adc_ier: ADC_IER,
    #[doc = "0x84 - ADC_IRAW"]
    pub adc_iraw: ADC_IRAW,
    #[doc = "0x88 - ADC_ISR"]
    pub adc_isr: ADC_ISR,
    #[doc = "0x8c - ADC_ICLR"]
    pub adc_iclr: ADC_ICLR,
    #[doc = "0x90 - ADC_DMAR"]
    pub adc_dmar: ADC_DMAR,
}
#[doc = "ADC_CR (rw) register accessor: an alias for `Reg<ADC_CR_SPEC>`"]
pub type ADC_CR = crate::Reg<adc_cr::ADC_CR_SPEC>;
#[doc = "ADC_CR"]
pub mod adc_cr;
#[doc = "ADC_LST0 (rw) register accessor: an alias for `Reg<ADC_LST0_SPEC>`"]
pub type ADC_LST0 = crate::Reg<adc_lst0::ADC_LST0_SPEC>;
#[doc = "ADC_LST0"]
pub mod adc_lst0;
#[doc = "ADC_LST1 (rw) register accessor: an alias for `Reg<ADC_LST1_SPEC>`"]
pub type ADC_LST1 = crate::Reg<adc_lst1::ADC_LST1_SPEC>;
#[doc = "ADC_LST1"]
pub mod adc_lst1;
#[doc = "ADC_STR (rw) register accessor: an alias for `Reg<ADC_STR_SPEC>`"]
pub type ADC_STR = crate::Reg<adc_str::ADC_STR_SPEC>;
#[doc = "ADC_STR"]
pub mod adc_str;
#[doc = "ADC_DR0 (rw) register accessor: an alias for `Reg<ADC_DR0_SPEC>`"]
pub type ADC_DR0 = crate::Reg<adc_dr0::ADC_DR0_SPEC>;
#[doc = "ADC_DR0"]
pub mod adc_dr0;
#[doc = "ADC_DR1 (rw) register accessor: an alias for `Reg<ADC_DR1_SPEC>`"]
pub type ADC_DR1 = crate::Reg<adc_dr1::ADC_DR1_SPEC>;
#[doc = "ADC_DR1"]
pub mod adc_dr1;
#[doc = "ADC_DR2 (rw) register accessor: an alias for `Reg<ADC_DR2_SPEC>`"]
pub type ADC_DR2 = crate::Reg<adc_dr2::ADC_DR2_SPEC>;
#[doc = "ADC_DR2"]
pub mod adc_dr2;
#[doc = "ADC_DR3 (rw) register accessor: an alias for `Reg<ADC_DR3_SPEC>`"]
pub type ADC_DR3 = crate::Reg<adc_dr3::ADC_DR3_SPEC>;
#[doc = "ADC_DR3"]
pub mod adc_dr3;
#[doc = "ADC_DR4 (rw) register accessor: an alias for `Reg<ADC_DR4_SPEC>`"]
pub type ADC_DR4 = crate::Reg<adc_dr4::ADC_DR4_SPEC>;
#[doc = "ADC_DR4"]
pub mod adc_dr4;
#[doc = "ADC_DR5 (rw) register accessor: an alias for `Reg<ADC_DR5_SPEC>`"]
pub type ADC_DR5 = crate::Reg<adc_dr5::ADC_DR5_SPEC>;
#[doc = "ADC_DR5"]
pub mod adc_dr5;
#[doc = "ADC_DR6 (rw) register accessor: an alias for `Reg<ADC_DR6_SPEC>`"]
pub type ADC_DR6 = crate::Reg<adc_dr6::ADC_DR6_SPEC>;
#[doc = "ADC_DR6"]
pub mod adc_dr6;
#[doc = "ADC_DR7 (rw) register accessor: an alias for `Reg<ADC_DR7_SPEC>`"]
pub type ADC_DR7 = crate::Reg<adc_dr7::ADC_DR7_SPEC>;
#[doc = "ADC_DR7"]
pub mod adc_dr7;
#[doc = "ADC_TCR (rw) register accessor: an alias for `Reg<ADC_TCR_SPEC>`"]
pub type ADC_TCR = crate::Reg<adc_tcr::ADC_TCR_SPEC>;
#[doc = "ADC_TCR"]
pub mod adc_tcr;
#[doc = "ADC_TSR (rw) register accessor: an alias for `Reg<ADC_TSR_SPEC>`"]
pub type ADC_TSR = crate::Reg<adc_tsr::ADC_TSR_SPEC>;
#[doc = "ADC_TSR"]
pub mod adc_tsr;
#[doc = "ADC_WCR (rw) register accessor: an alias for `Reg<ADC_WCR_SPEC>`"]
pub type ADC_WCR = crate::Reg<adc_wcr::ADC_WCR_SPEC>;
#[doc = "ADC_WCR"]
pub mod adc_wcr;
#[doc = "ADC_TR (rw) register accessor: an alias for `Reg<ADC_TR_SPEC>`"]
pub type ADC_TR = crate::Reg<adc_tr::ADC_TR_SPEC>;
#[doc = "ADC_TR"]
pub mod adc_tr;
#[doc = "ADC_IER (rw) register accessor: an alias for `Reg<ADC_IER_SPEC>`"]
pub type ADC_IER = crate::Reg<adc_ier::ADC_IER_SPEC>;
#[doc = "ADC_IMR"]
pub mod adc_ier;
#[doc = "ADC_IRAW (rw) register accessor: an alias for `Reg<ADC_IRAW_SPEC>`"]
pub type ADC_IRAW = crate::Reg<adc_iraw::ADC_IRAW_SPEC>;
#[doc = "ADC_IRAW"]
pub mod adc_iraw;
#[doc = "ADC_ISR (rw) register accessor: an alias for `Reg<ADC_ISR_SPEC>`"]
pub type ADC_ISR = crate::Reg<adc_isr::ADC_ISR_SPEC>;
#[doc = "ADC_ISR"]
pub mod adc_isr;
#[doc = "ADC_ICLR (rw) register accessor: an alias for `Reg<ADC_ICLR_SPEC>`"]
pub type ADC_ICLR = crate::Reg<adc_iclr::ADC_ICLR_SPEC>;
#[doc = "ADC_ICLR"]
pub mod adc_iclr;
#[doc = "ADC_DMAR (rw) register accessor: an alias for `Reg<ADC_DMAR_SPEC>`"]
pub type ADC_DMAR = crate::Reg<adc_dmar::ADC_DMAR_SPEC>;
#[doc = "ADC_DMAR"]
pub mod adc_dmar;
