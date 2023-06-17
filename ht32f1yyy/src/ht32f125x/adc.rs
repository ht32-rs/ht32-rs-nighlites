#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - ADC_RST"]
    pub adc_rst: ADC_RST,
    #[doc = "0x08 - ADC_CONV"]
    pub adc_conv: ADC_CONV,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - ADC_LST0"]
    pub adc_lst0: ADC_LST0,
    #[doc = "0x14 - ADC_LST1"]
    pub adc_lst1: ADC_LST1,
    _reserved4: [u8; 0x58],
    #[doc = "0x70 - ADC_STR0"]
    pub adc_str0: ADC_STR0,
    #[doc = "0x74 - ADC_STR1"]
    pub adc_str1: ADC_STR1,
    #[doc = "0x78 - ADC_STR2"]
    pub adc_str2: ADC_STR2,
    #[doc = "0x7c - ADC_STR3"]
    pub adc_str3: ADC_STR3,
    #[doc = "0x80 - ADC_STR4"]
    pub adc_str4: ADC_STR4,
    #[doc = "0x84 - ADC_STR5"]
    pub adc_str5: ADC_STR5,
    #[doc = "0x88 - ADC_STR6"]
    pub adc_str6: ADC_STR6,
    #[doc = "0x8c - ADC_STR7"]
    pub adc_str7: ADC_STR7,
    _reserved12: [u8; 0x20],
    #[doc = "0xb0 - ADC_DR0"]
    pub adc_dr0: ADC_DR0,
    #[doc = "0xb4 - ADC_DR1"]
    pub adc_dr1: ADC_DR1,
    #[doc = "0xb8 - ADC_DR2"]
    pub adc_dr2: ADC_DR2,
    #[doc = "0xbc - ADC_DR3"]
    pub adc_dr3: ADC_DR3,
    #[doc = "0xc0 - ADC_DR4"]
    pub adc_dr4: ADC_DR4,
    #[doc = "0xc4 - ADC_DR5"]
    pub adc_dr5: ADC_DR5,
    #[doc = "0xc8 - ADC_DR6"]
    pub adc_dr6: ADC_DR6,
    #[doc = "0xcc - ADC_DR7"]
    pub adc_dr7: ADC_DR7,
    _reserved20: [u8; 0x30],
    #[doc = "0x100 - ADC_TCR"]
    pub adc_tcr: ADC_TCR,
    #[doc = "0x104 - ADC_TSR"]
    pub adc_tsr: ADC_TSR,
    _reserved22: [u8; 0x18],
    #[doc = "0x120 - ADC_WCR"]
    pub adc_wcr: ADC_WCR,
    #[doc = "0x124 - ADC_LTR"]
    pub adc_ltr: ADC_LTR,
    #[doc = "0x128 - ADC_UTR"]
    pub adc_utr: ADC_UTR,
    _reserved25: [u8; 0x04],
    #[doc = "0x130 - ADC_IMR"]
    pub adc_imr: ADC_IMR,
    #[doc = "0x134 - ADC_IRAW"]
    pub adc_iraw: ADC_IRAW,
    #[doc = "0x138 - ADC_IMASK"]
    pub adc_imask: ADC_IMASK,
    #[doc = "0x13c - ADC_ICLR"]
    pub adc_iclr: ADC_ICLR,
}
#[doc = "ADC_RST (rw) register accessor: an alias for `Reg<ADC_RST_SPEC>`"]
pub type ADC_RST = crate::Reg<adc_rst::ADC_RST_SPEC>;
#[doc = "ADC_RST"]
pub mod adc_rst;
#[doc = "ADC_CONV (rw) register accessor: an alias for `Reg<ADC_CONV_SPEC>`"]
pub type ADC_CONV = crate::Reg<adc_conv::ADC_CONV_SPEC>;
#[doc = "ADC_CONV"]
pub mod adc_conv;
#[doc = "ADC_LST0 (rw) register accessor: an alias for `Reg<ADC_LST0_SPEC>`"]
pub type ADC_LST0 = crate::Reg<adc_lst0::ADC_LST0_SPEC>;
#[doc = "ADC_LST0"]
pub mod adc_lst0;
#[doc = "ADC_LST1 (rw) register accessor: an alias for `Reg<ADC_LST1_SPEC>`"]
pub type ADC_LST1 = crate::Reg<adc_lst1::ADC_LST1_SPEC>;
#[doc = "ADC_LST1"]
pub mod adc_lst1;
#[doc = "ADC_STR0 (rw) register accessor: an alias for `Reg<ADC_STR0_SPEC>`"]
pub type ADC_STR0 = crate::Reg<adc_str0::ADC_STR0_SPEC>;
#[doc = "ADC_STR0"]
pub mod adc_str0;
#[doc = "ADC_STR1 (rw) register accessor: an alias for `Reg<ADC_STR1_SPEC>`"]
pub type ADC_STR1 = crate::Reg<adc_str1::ADC_STR1_SPEC>;
#[doc = "ADC_STR1"]
pub mod adc_str1;
#[doc = "ADC_STR2 (rw) register accessor: an alias for `Reg<ADC_STR2_SPEC>`"]
pub type ADC_STR2 = crate::Reg<adc_str2::ADC_STR2_SPEC>;
#[doc = "ADC_STR2"]
pub mod adc_str2;
#[doc = "ADC_STR3 (rw) register accessor: an alias for `Reg<ADC_STR3_SPEC>`"]
pub type ADC_STR3 = crate::Reg<adc_str3::ADC_STR3_SPEC>;
#[doc = "ADC_STR3"]
pub mod adc_str3;
#[doc = "ADC_STR4 (rw) register accessor: an alias for `Reg<ADC_STR4_SPEC>`"]
pub type ADC_STR4 = crate::Reg<adc_str4::ADC_STR4_SPEC>;
#[doc = "ADC_STR4"]
pub mod adc_str4;
#[doc = "ADC_STR5 (rw) register accessor: an alias for `Reg<ADC_STR5_SPEC>`"]
pub type ADC_STR5 = crate::Reg<adc_str5::ADC_STR5_SPEC>;
#[doc = "ADC_STR5"]
pub mod adc_str5;
#[doc = "ADC_STR6 (rw) register accessor: an alias for `Reg<ADC_STR6_SPEC>`"]
pub type ADC_STR6 = crate::Reg<adc_str6::ADC_STR6_SPEC>;
#[doc = "ADC_STR6"]
pub mod adc_str6;
#[doc = "ADC_STR7 (rw) register accessor: an alias for `Reg<ADC_STR7_SPEC>`"]
pub type ADC_STR7 = crate::Reg<adc_str7::ADC_STR7_SPEC>;
#[doc = "ADC_STR7"]
pub mod adc_str7;
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
#[doc = "ADC_LTR (rw) register accessor: an alias for `Reg<ADC_LTR_SPEC>`"]
pub type ADC_LTR = crate::Reg<adc_ltr::ADC_LTR_SPEC>;
#[doc = "ADC_LTR"]
pub mod adc_ltr;
#[doc = "ADC_UTR (rw) register accessor: an alias for `Reg<ADC_UTR_SPEC>`"]
pub type ADC_UTR = crate::Reg<adc_utr::ADC_UTR_SPEC>;
#[doc = "ADC_UTR"]
pub mod adc_utr;
#[doc = "ADC_IMR (rw) register accessor: an alias for `Reg<ADC_IMR_SPEC>`"]
pub type ADC_IMR = crate::Reg<adc_imr::ADC_IMR_SPEC>;
#[doc = "ADC_IMR"]
pub mod adc_imr;
#[doc = "ADC_IRAW (rw) register accessor: an alias for `Reg<ADC_IRAW_SPEC>`"]
pub type ADC_IRAW = crate::Reg<adc_iraw::ADC_IRAW_SPEC>;
#[doc = "ADC_IRAW"]
pub mod adc_iraw;
#[doc = "ADC_IMASK (rw) register accessor: an alias for `Reg<ADC_IMASK_SPEC>`"]
pub type ADC_IMASK = crate::Reg<adc_imask::ADC_IMASK_SPEC>;
#[doc = "ADC_IMASK"]
pub mod adc_imask;
#[doc = "ADC_ICLR (rw) register accessor: an alias for `Reg<ADC_ICLR_SPEC>`"]
pub type ADC_ICLR = crate::Reg<adc_iclr::ADC_ICLR_SPEC>;
#[doc = "ADC_ICLR"]
pub mod adc_iclr;
