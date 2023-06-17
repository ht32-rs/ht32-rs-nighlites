#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - ADC0RST"]
    pub adc0rst: ADC0RST,
    #[doc = "0x08 - ADC0CONV"]
    pub adc0conv: ADC0CONV,
    #[doc = "0x0c - ADC0HCONV"]
    pub adc0hconv: ADC0HCONV,
    #[doc = "0x10 - ADC0LST0"]
    pub adc0lst0: ADC0LST0,
    #[doc = "0x14 - ADC0LST1"]
    pub adc0lst1: ADC0LST1,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - ADC0HLST"]
    pub adc0hlst: ADC0HLST,
    _reserved6: [u8; 0x0c],
    #[doc = "0x30 - ADC0OFR0"]
    pub adc0ofr0: ADC0OFR0,
    _reserved7: [u8; 0x08],
    #[doc = "0x3c - ADC0OFR3"]
    pub adc0ofr3: ADC0OFR3,
    #[doc = "0x40 - ADC0OFR4"]
    pub adc0ofr4: ADC0OFR4,
    #[doc = "0x44 - ADC0OFR5"]
    pub adc0ofr5: ADC0OFR5,
    #[doc = "0x48 - ADC0OFR6"]
    pub adc0ofr6: ADC0OFR6,
    #[doc = "0x4c - ADC0OFR7"]
    pub adc0ofr7: ADC0OFR7,
    _reserved12: [u8; 0x20],
    #[doc = "0x70 - ADC0STR0"]
    pub adc0str0: ADC0STR0,
    #[doc = "0x74 - ADC0STR1"]
    pub adc0str1: ADC0STR1,
    #[doc = "0x78 - ADC0STR2"]
    pub adc0str2: ADC0STR2,
    #[doc = "0x7c - ADC0STR3"]
    pub adc0str3: ADC0STR3,
    #[doc = "0x80 - ADC0STR4"]
    pub adc0str4: ADC0STR4,
    #[doc = "0x84 - ADC0STR5"]
    pub adc0str5: ADC0STR5,
    #[doc = "0x88 - ADC0STR6"]
    pub adc0str6: ADC0STR6,
    #[doc = "0x8c - ADC0STR7"]
    pub adc0str7: ADC0STR7,
    _reserved20: [u8; 0x20],
    #[doc = "0xb0 - ADC0DR0"]
    pub adc0dr0: ADC0DR0,
    #[doc = "0xb4 - ADC0DR1"]
    pub adc0dr1: ADC0DR1,
    #[doc = "0xb8 - ADC0DR2"]
    pub adc0dr2: ADC0DR2,
    #[doc = "0xbc - ADC0DR3"]
    pub adc0dr3: ADC0DR3,
    #[doc = "0xc0 - ADC0DR4"]
    pub adc0dr4: ADC0DR4,
    #[doc = "0xc4 - ADC0DR5"]
    pub adc0dr5: ADC0DR5,
    #[doc = "0xc8 - ADC0DR6"]
    pub adc0dr6: ADC0DR6,
    #[doc = "0xcc - ADC0DR7"]
    pub adc0dr7: ADC0DR7,
    _reserved28: [u8; 0x20],
    #[doc = "0xf0 - ADC0HDR0"]
    pub adc0hdr0: ADC0HDR0,
    #[doc = "0xf4 - ADC0HDR1"]
    pub adc0hdr1: ADC0HDR1,
    #[doc = "0xf8 - ADC0HDR2"]
    pub adc0hdr2: ADC0HDR2,
    #[doc = "0xfc - ADC0HDR3"]
    pub adc0hdr3: ADC0HDR3,
    #[doc = "0x100 - ADC0TCR"]
    pub adc0tcr: ADC0TCR,
    #[doc = "0x104 - ADC0TSR"]
    pub adc0tsr: ADC0TSR,
    _reserved34: [u8; 0x08],
    #[doc = "0x110 - ADC0HTCR"]
    pub adc0htcr: ADC0HTCR,
    #[doc = "0x114 - ADC0HTSR"]
    pub adc0htsr: ADC0HTSR,
    _reserved36: [u8; 0x08],
    #[doc = "0x120 - ADC0WCR"]
    pub adc0wcr: ADC0WCR,
    #[doc = "0x124 - ADC0WLTR"]
    pub adc0wltr: ADC0WLTR,
    #[doc = "0x128 - ADC0WUTR"]
    pub adc0wutr: ADC0WUTR,
    _reserved39: [u8; 0x04],
    #[doc = "0x130 - ADC0IER"]
    pub adc0ier: ADC0IER,
    #[doc = "0x134 - ADC0IRAW"]
    pub adc0iraw: ADC0IRAW,
    #[doc = "0x138 - ADC0ISR"]
    pub adc0isr: ADC0ISR,
    #[doc = "0x13c - ADC0ICLR"]
    pub adc0iclr: ADC0ICLR,
    #[doc = "0x140 - ADC0DMAR"]
    pub adc0dmar: ADC0DMAR,
}
#[doc = "ADC0RST (rw) register accessor: an alias for `Reg<ADC0RST_SPEC>`"]
pub type ADC0RST = crate::Reg<adc0rst::ADC0RST_SPEC>;
#[doc = "ADC0RST"]
pub mod adc0rst;
#[doc = "ADC0CONV (rw) register accessor: an alias for `Reg<ADC0CONV_SPEC>`"]
pub type ADC0CONV = crate::Reg<adc0conv::ADC0CONV_SPEC>;
#[doc = "ADC0CONV"]
pub mod adc0conv;
#[doc = "ADC0HCONV (rw) register accessor: an alias for `Reg<ADC0HCONV_SPEC>`"]
pub type ADC0HCONV = crate::Reg<adc0hconv::ADC0HCONV_SPEC>;
#[doc = "ADC0HCONV"]
pub mod adc0hconv;
#[doc = "ADC0LST0 (rw) register accessor: an alias for `Reg<ADC0LST0_SPEC>`"]
pub type ADC0LST0 = crate::Reg<adc0lst0::ADC0LST0_SPEC>;
#[doc = "ADC0LST0"]
pub mod adc0lst0;
#[doc = "ADC0LST1 (rw) register accessor: an alias for `Reg<ADC0LST1_SPEC>`"]
pub type ADC0LST1 = crate::Reg<adc0lst1::ADC0LST1_SPEC>;
#[doc = "ADC0LST1"]
pub mod adc0lst1;
#[doc = "ADC0HLST (rw) register accessor: an alias for `Reg<ADC0HLST_SPEC>`"]
pub type ADC0HLST = crate::Reg<adc0hlst::ADC0HLST_SPEC>;
#[doc = "ADC0HLST"]
pub mod adc0hlst;
#[doc = "ADC0OFR0 (rw) register accessor: an alias for `Reg<ADC0OFR0_SPEC>`"]
pub type ADC0OFR0 = crate::Reg<adc0ofr0::ADC0OFR0_SPEC>;
#[doc = "ADC0OFR0"]
pub mod adc0ofr0;
#[doc = "ADC0OFR3 (rw) register accessor: an alias for `Reg<ADC0OFR3_SPEC>`"]
pub type ADC0OFR3 = crate::Reg<adc0ofr3::ADC0OFR3_SPEC>;
#[doc = "ADC0OFR3"]
pub mod adc0ofr3;
#[doc = "ADC0OFR4 (rw) register accessor: an alias for `Reg<ADC0OFR4_SPEC>`"]
pub type ADC0OFR4 = crate::Reg<adc0ofr4::ADC0OFR4_SPEC>;
#[doc = "ADC0OFR4"]
pub mod adc0ofr4;
#[doc = "ADC0OFR5 (rw) register accessor: an alias for `Reg<ADC0OFR5_SPEC>`"]
pub type ADC0OFR5 = crate::Reg<adc0ofr5::ADC0OFR5_SPEC>;
#[doc = "ADC0OFR5"]
pub mod adc0ofr5;
#[doc = "ADC0OFR6 (rw) register accessor: an alias for `Reg<ADC0OFR6_SPEC>`"]
pub type ADC0OFR6 = crate::Reg<adc0ofr6::ADC0OFR6_SPEC>;
#[doc = "ADC0OFR6"]
pub mod adc0ofr6;
#[doc = "ADC0OFR7 (rw) register accessor: an alias for `Reg<ADC0OFR7_SPEC>`"]
pub type ADC0OFR7 = crate::Reg<adc0ofr7::ADC0OFR7_SPEC>;
#[doc = "ADC0OFR7"]
pub mod adc0ofr7;
#[doc = "ADC0STR0 (rw) register accessor: an alias for `Reg<ADC0STR0_SPEC>`"]
pub type ADC0STR0 = crate::Reg<adc0str0::ADC0STR0_SPEC>;
#[doc = "ADC0STR0"]
pub mod adc0str0;
#[doc = "ADC0STR1 (rw) register accessor: an alias for `Reg<ADC0STR1_SPEC>`"]
pub type ADC0STR1 = crate::Reg<adc0str1::ADC0STR1_SPEC>;
#[doc = "ADC0STR1"]
pub mod adc0str1;
#[doc = "ADC0STR2 (rw) register accessor: an alias for `Reg<ADC0STR2_SPEC>`"]
pub type ADC0STR2 = crate::Reg<adc0str2::ADC0STR2_SPEC>;
#[doc = "ADC0STR2"]
pub mod adc0str2;
#[doc = "ADC0STR3 (rw) register accessor: an alias for `Reg<ADC0STR3_SPEC>`"]
pub type ADC0STR3 = crate::Reg<adc0str3::ADC0STR3_SPEC>;
#[doc = "ADC0STR3"]
pub mod adc0str3;
#[doc = "ADC0STR4 (rw) register accessor: an alias for `Reg<ADC0STR4_SPEC>`"]
pub type ADC0STR4 = crate::Reg<adc0str4::ADC0STR4_SPEC>;
#[doc = "ADC0STR4"]
pub mod adc0str4;
#[doc = "ADC0STR5 (rw) register accessor: an alias for `Reg<ADC0STR5_SPEC>`"]
pub type ADC0STR5 = crate::Reg<adc0str5::ADC0STR5_SPEC>;
#[doc = "ADC0STR5"]
pub mod adc0str5;
#[doc = "ADC0STR6 (rw) register accessor: an alias for `Reg<ADC0STR6_SPEC>`"]
pub type ADC0STR6 = crate::Reg<adc0str6::ADC0STR6_SPEC>;
#[doc = "ADC0STR6"]
pub mod adc0str6;
#[doc = "ADC0STR7 (rw) register accessor: an alias for `Reg<ADC0STR7_SPEC>`"]
pub type ADC0STR7 = crate::Reg<adc0str7::ADC0STR7_SPEC>;
#[doc = "ADC0STR7"]
pub mod adc0str7;
#[doc = "ADC0DR0 (rw) register accessor: an alias for `Reg<ADC0DR0_SPEC>`"]
pub type ADC0DR0 = crate::Reg<adc0dr0::ADC0DR0_SPEC>;
#[doc = "ADC0DR0"]
pub mod adc0dr0;
#[doc = "ADC0DR1 (rw) register accessor: an alias for `Reg<ADC0DR1_SPEC>`"]
pub type ADC0DR1 = crate::Reg<adc0dr1::ADC0DR1_SPEC>;
#[doc = "ADC0DR1"]
pub mod adc0dr1;
#[doc = "ADC0DR2 (rw) register accessor: an alias for `Reg<ADC0DR2_SPEC>`"]
pub type ADC0DR2 = crate::Reg<adc0dr2::ADC0DR2_SPEC>;
#[doc = "ADC0DR2"]
pub mod adc0dr2;
#[doc = "ADC0DR3 (rw) register accessor: an alias for `Reg<ADC0DR3_SPEC>`"]
pub type ADC0DR3 = crate::Reg<adc0dr3::ADC0DR3_SPEC>;
#[doc = "ADC0DR3"]
pub mod adc0dr3;
#[doc = "ADC0DR4 (rw) register accessor: an alias for `Reg<ADC0DR4_SPEC>`"]
pub type ADC0DR4 = crate::Reg<adc0dr4::ADC0DR4_SPEC>;
#[doc = "ADC0DR4"]
pub mod adc0dr4;
#[doc = "ADC0DR5 (rw) register accessor: an alias for `Reg<ADC0DR5_SPEC>`"]
pub type ADC0DR5 = crate::Reg<adc0dr5::ADC0DR5_SPEC>;
#[doc = "ADC0DR5"]
pub mod adc0dr5;
#[doc = "ADC0DR6 (rw) register accessor: an alias for `Reg<ADC0DR6_SPEC>`"]
pub type ADC0DR6 = crate::Reg<adc0dr6::ADC0DR6_SPEC>;
#[doc = "ADC0DR6"]
pub mod adc0dr6;
#[doc = "ADC0DR7 (rw) register accessor: an alias for `Reg<ADC0DR7_SPEC>`"]
pub type ADC0DR7 = crate::Reg<adc0dr7::ADC0DR7_SPEC>;
#[doc = "ADC0DR7"]
pub mod adc0dr7;
#[doc = "ADC0HDR0 (rw) register accessor: an alias for `Reg<ADC0HDR0_SPEC>`"]
pub type ADC0HDR0 = crate::Reg<adc0hdr0::ADC0HDR0_SPEC>;
#[doc = "ADC0HDR0"]
pub mod adc0hdr0;
#[doc = "ADC0HDR1 (rw) register accessor: an alias for `Reg<ADC0HDR1_SPEC>`"]
pub type ADC0HDR1 = crate::Reg<adc0hdr1::ADC0HDR1_SPEC>;
#[doc = "ADC0HDR1"]
pub mod adc0hdr1;
#[doc = "ADC0HDR2 (rw) register accessor: an alias for `Reg<ADC0HDR2_SPEC>`"]
pub type ADC0HDR2 = crate::Reg<adc0hdr2::ADC0HDR2_SPEC>;
#[doc = "ADC0HDR2"]
pub mod adc0hdr2;
#[doc = "ADC0HDR3 (rw) register accessor: an alias for `Reg<ADC0HDR3_SPEC>`"]
pub type ADC0HDR3 = crate::Reg<adc0hdr3::ADC0HDR3_SPEC>;
#[doc = "ADC0HDR3"]
pub mod adc0hdr3;
#[doc = "ADC0TCR (rw) register accessor: an alias for `Reg<ADC0TCR_SPEC>`"]
pub type ADC0TCR = crate::Reg<adc0tcr::ADC0TCR_SPEC>;
#[doc = "ADC0TCR"]
pub mod adc0tcr;
#[doc = "ADC0TSR (rw) register accessor: an alias for `Reg<ADC0TSR_SPEC>`"]
pub type ADC0TSR = crate::Reg<adc0tsr::ADC0TSR_SPEC>;
#[doc = "ADC0TSR"]
pub mod adc0tsr;
#[doc = "ADC0HTCR (rw) register accessor: an alias for `Reg<ADC0HTCR_SPEC>`"]
pub type ADC0HTCR = crate::Reg<adc0htcr::ADC0HTCR_SPEC>;
#[doc = "ADC0HTCR"]
pub mod adc0htcr;
#[doc = "ADC0HTSR (rw) register accessor: an alias for `Reg<ADC0HTSR_SPEC>`"]
pub type ADC0HTSR = crate::Reg<adc0htsr::ADC0HTSR_SPEC>;
#[doc = "ADC0HTSR"]
pub mod adc0htsr;
#[doc = "ADC0WCR (rw) register accessor: an alias for `Reg<ADC0WCR_SPEC>`"]
pub type ADC0WCR = crate::Reg<adc0wcr::ADC0WCR_SPEC>;
#[doc = "ADC0WCR"]
pub mod adc0wcr;
#[doc = "ADC0WLTR (rw) register accessor: an alias for `Reg<ADC0WLTR_SPEC>`"]
pub type ADC0WLTR = crate::Reg<adc0wltr::ADC0WLTR_SPEC>;
#[doc = "ADC0WLTR"]
pub mod adc0wltr;
#[doc = "ADC0WUTR (rw) register accessor: an alias for `Reg<ADC0WUTR_SPEC>`"]
pub type ADC0WUTR = crate::Reg<adc0wutr::ADC0WUTR_SPEC>;
#[doc = "ADC0WUTR"]
pub mod adc0wutr;
#[doc = "ADC0IER (rw) register accessor: an alias for `Reg<ADC0IER_SPEC>`"]
pub type ADC0IER = crate::Reg<adc0ier::ADC0IER_SPEC>;
#[doc = "ADC0IER"]
pub mod adc0ier;
#[doc = "ADC0IRAW (rw) register accessor: an alias for `Reg<ADC0IRAW_SPEC>`"]
pub type ADC0IRAW = crate::Reg<adc0iraw::ADC0IRAW_SPEC>;
#[doc = "ADC0IRAW"]
pub mod adc0iraw;
#[doc = "ADC0ISR (rw) register accessor: an alias for `Reg<ADC0ISR_SPEC>`"]
pub type ADC0ISR = crate::Reg<adc0isr::ADC0ISR_SPEC>;
#[doc = "ADC0ISR"]
pub mod adc0isr;
#[doc = "ADC0ICLR (rw) register accessor: an alias for `Reg<ADC0ICLR_SPEC>`"]
pub type ADC0ICLR = crate::Reg<adc0iclr::ADC0ICLR_SPEC>;
#[doc = "ADC0ICLR"]
pub mod adc0iclr;
#[doc = "ADC0DMAR (rw) register accessor: an alias for `Reg<ADC0DMAR_SPEC>`"]
pub type ADC0DMAR = crate::Reg<adc0dmar::ADC0DMAR_SPEC>;
#[doc = "ADC0DMAR"]
pub mod adc0dmar;
