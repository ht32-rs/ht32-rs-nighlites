#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADCCONF"]
    pub adcconf: ADCCONF,
    #[doc = "0x04 - ADC1RST"]
    pub adc1rst: ADC1RST,
    #[doc = "0x08 - ADC1CONV"]
    pub adc1conv: ADC1CONV,
    #[doc = "0x0c - ADC1HCONV"]
    pub adc1hconv: ADC1HCONV,
    #[doc = "0x10 - ADC1LST0"]
    pub adc1lst0: ADC1LST0,
    #[doc = "0x14 - ADC1LST1"]
    pub adc1lst1: ADC1LST1,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - ADC1HLST"]
    pub adc1hlst: ADC1HLST,
    _reserved7: [u8; 0x0c],
    #[doc = "0x30 - ADC1OFR0"]
    pub adc1ofr0: ADC1OFR0,
    #[doc = "0x34 - ADC1OFR1"]
    pub adc1ofr1: ADC1OFR1,
    #[doc = "0x38 - ADC1OFR2"]
    pub adc1ofr2: ADC1OFR2,
    #[doc = "0x3c - ADC1OFR3"]
    pub adc1ofr3: ADC1OFR3,
    #[doc = "0x40 - ADC1OFR4"]
    pub adc1ofr4: ADC1OFR4,
    #[doc = "0x44 - ADC1OFR5"]
    pub adc1ofr5: ADC1OFR5,
    #[doc = "0x48 - ADC1OFR6"]
    pub adc1ofr6: ADC1OFR6,
    #[doc = "0x4c - ADC1OFR7"]
    pub adc1ofr7: ADC1OFR7,
    _reserved15: [u8; 0x20],
    #[doc = "0x70 - ADC1STR0"]
    pub adc1str0: ADC1STR0,
    #[doc = "0x74 - ADC1STR1"]
    pub adc1str1: ADC1STR1,
    #[doc = "0x78 - ADC1STR2"]
    pub adc1str2: ADC1STR2,
    #[doc = "0x7c - ADC1STR3"]
    pub adc1str3: ADC1STR3,
    #[doc = "0x80 - ADC1STR4"]
    pub adc1str4: ADC1STR4,
    #[doc = "0x84 - ADC1STR5"]
    pub adc1str5: ADC1STR5,
    #[doc = "0x88 - ADC1STR6"]
    pub adc1str6: ADC1STR6,
    #[doc = "0x8c - ADC1STR7"]
    pub adc1str7: ADC1STR7,
    _reserved23: [u8; 0x20],
    #[doc = "0xb0 - ADC1DR0"]
    pub adc1dr0: ADC1DR0,
    #[doc = "0xb4 - ADC1DR1"]
    pub adc1dr1: ADC1DR1,
    #[doc = "0xb8 - ADC1DR2"]
    pub adc1dr2: ADC1DR2,
    #[doc = "0xbc - ADC1DR3"]
    pub adc1dr3: ADC1DR3,
    #[doc = "0xc0 - ADC1DR4"]
    pub adc1dr4: ADC1DR4,
    #[doc = "0xc4 - ADC1DR5"]
    pub adc1dr5: ADC1DR5,
    #[doc = "0xc8 - ADC1DR6"]
    pub adc1dr6: ADC1DR6,
    #[doc = "0xcc - ADC1DR7"]
    pub adc1dr7: ADC1DR7,
    _reserved31: [u8; 0x20],
    #[doc = "0xf0 - ADC1HDR0"]
    pub adc1hdr0: ADC1HDR0,
    #[doc = "0xf4 - ADC1HDR1"]
    pub adc1hdr1: ADC1HDR1,
    #[doc = "0xf8 - ADC1HDR2"]
    pub adc1hdr2: ADC1HDR2,
    #[doc = "0xfc - ADC1HDR3"]
    pub adc1hdr3: ADC1HDR3,
    #[doc = "0x100 - ADC1TCR"]
    pub adc1tcr: ADC1TCR,
    #[doc = "0x104 - ADC1TSR"]
    pub adc1tsr: ADC1TSR,
    _reserved37: [u8; 0x08],
    #[doc = "0x110 - ADC1HTCR"]
    pub adc1htcr: ADC1HTCR,
    #[doc = "0x114 - ADC1HTSR"]
    pub adc1htsr: ADC1HTSR,
    _reserved39: [u8; 0x08],
    #[doc = "0x120 - ADC1WCR"]
    pub adc1wcr: ADC1WCR,
    #[doc = "0x124 - ADC1WLTR"]
    pub adc1wltr: ADC1WLTR,
    #[doc = "0x128 - ADC1WUTR"]
    pub adc1wutr: ADC1WUTR,
    _reserved42: [u8; 0x04],
    #[doc = "0x130 - ADC1IER"]
    pub adc1ier: ADC1IER,
    #[doc = "0x134 - ADC1IRAW"]
    pub adc1iraw: ADC1IRAW,
    #[doc = "0x138 - ADC1ISR"]
    pub adc1isr: ADC1ISR,
    #[doc = "0x13c - ADC1ICLR"]
    pub adc1iclr: ADC1ICLR,
    #[doc = "0x140 - ADC1DMAR"]
    pub adc1dmar: ADC1DMAR,
    _reserved47: [u8; 0x0c],
    #[doc = "0x150 - DADCHIR"]
    pub dadchir: DADCHIR,
    #[doc = "0x154 - DADCDMAR"]
    pub dadcdmar: DADCDMAR,
}
#[doc = "ADCCONF (rw) register accessor: an alias for `Reg<ADCCONF_SPEC>`"]
pub type ADCCONF = crate::Reg<adcconf::ADCCONF_SPEC>;
#[doc = "ADCCONF"]
pub mod adcconf;
#[doc = "ADC1RST (rw) register accessor: an alias for `Reg<ADC1RST_SPEC>`"]
pub type ADC1RST = crate::Reg<adc1rst::ADC1RST_SPEC>;
#[doc = "ADC1RST"]
pub mod adc1rst;
#[doc = "ADC1CONV (rw) register accessor: an alias for `Reg<ADC1CONV_SPEC>`"]
pub type ADC1CONV = crate::Reg<adc1conv::ADC1CONV_SPEC>;
#[doc = "ADC1CONV"]
pub mod adc1conv;
#[doc = "ADC1HCONV (rw) register accessor: an alias for `Reg<ADC1HCONV_SPEC>`"]
pub type ADC1HCONV = crate::Reg<adc1hconv::ADC1HCONV_SPEC>;
#[doc = "ADC1HCONV"]
pub mod adc1hconv;
#[doc = "ADC1LST0 (rw) register accessor: an alias for `Reg<ADC1LST0_SPEC>`"]
pub type ADC1LST0 = crate::Reg<adc1lst0::ADC1LST0_SPEC>;
#[doc = "ADC1LST0"]
pub mod adc1lst0;
#[doc = "ADC1LST1 (rw) register accessor: an alias for `Reg<ADC1LST1_SPEC>`"]
pub type ADC1LST1 = crate::Reg<adc1lst1::ADC1LST1_SPEC>;
#[doc = "ADC1LST1"]
pub mod adc1lst1;
#[doc = "ADC1HLST (rw) register accessor: an alias for `Reg<ADC1HLST_SPEC>`"]
pub type ADC1HLST = crate::Reg<adc1hlst::ADC1HLST_SPEC>;
#[doc = "ADC1HLST"]
pub mod adc1hlst;
#[doc = "ADC1OFR0 (rw) register accessor: an alias for `Reg<ADC1OFR0_SPEC>`"]
pub type ADC1OFR0 = crate::Reg<adc1ofr0::ADC1OFR0_SPEC>;
#[doc = "ADC1OFR0"]
pub mod adc1ofr0;
#[doc = "ADC1OFR1 (rw) register accessor: an alias for `Reg<ADC1OFR1_SPEC>`"]
pub type ADC1OFR1 = crate::Reg<adc1ofr1::ADC1OFR1_SPEC>;
#[doc = "ADC1OFR1"]
pub mod adc1ofr1;
#[doc = "ADC1OFR2 (rw) register accessor: an alias for `Reg<ADC1OFR2_SPEC>`"]
pub type ADC1OFR2 = crate::Reg<adc1ofr2::ADC1OFR2_SPEC>;
#[doc = "ADC1OFR2"]
pub mod adc1ofr2;
#[doc = "ADC1OFR3 (rw) register accessor: an alias for `Reg<ADC1OFR3_SPEC>`"]
pub type ADC1OFR3 = crate::Reg<adc1ofr3::ADC1OFR3_SPEC>;
#[doc = "ADC1OFR3"]
pub mod adc1ofr3;
#[doc = "ADC1OFR4 (rw) register accessor: an alias for `Reg<ADC1OFR4_SPEC>`"]
pub type ADC1OFR4 = crate::Reg<adc1ofr4::ADC1OFR4_SPEC>;
#[doc = "ADC1OFR4"]
pub mod adc1ofr4;
#[doc = "ADC1OFR5 (rw) register accessor: an alias for `Reg<ADC1OFR5_SPEC>`"]
pub type ADC1OFR5 = crate::Reg<adc1ofr5::ADC1OFR5_SPEC>;
#[doc = "ADC1OFR5"]
pub mod adc1ofr5;
#[doc = "ADC1OFR6 (rw) register accessor: an alias for `Reg<ADC1OFR6_SPEC>`"]
pub type ADC1OFR6 = crate::Reg<adc1ofr6::ADC1OFR6_SPEC>;
#[doc = "ADC1OFR6"]
pub mod adc1ofr6;
#[doc = "ADC1OFR7 (rw) register accessor: an alias for `Reg<ADC1OFR7_SPEC>`"]
pub type ADC1OFR7 = crate::Reg<adc1ofr7::ADC1OFR7_SPEC>;
#[doc = "ADC1OFR7"]
pub mod adc1ofr7;
#[doc = "ADC1STR0 (rw) register accessor: an alias for `Reg<ADC1STR0_SPEC>`"]
pub type ADC1STR0 = crate::Reg<adc1str0::ADC1STR0_SPEC>;
#[doc = "ADC1STR0"]
pub mod adc1str0;
#[doc = "ADC1STR1 (rw) register accessor: an alias for `Reg<ADC1STR1_SPEC>`"]
pub type ADC1STR1 = crate::Reg<adc1str1::ADC1STR1_SPEC>;
#[doc = "ADC1STR1"]
pub mod adc1str1;
#[doc = "ADC1STR2 (rw) register accessor: an alias for `Reg<ADC1STR2_SPEC>`"]
pub type ADC1STR2 = crate::Reg<adc1str2::ADC1STR2_SPEC>;
#[doc = "ADC1STR2"]
pub mod adc1str2;
#[doc = "ADC1STR3 (rw) register accessor: an alias for `Reg<ADC1STR3_SPEC>`"]
pub type ADC1STR3 = crate::Reg<adc1str3::ADC1STR3_SPEC>;
#[doc = "ADC1STR3"]
pub mod adc1str3;
#[doc = "ADC1STR4 (rw) register accessor: an alias for `Reg<ADC1STR4_SPEC>`"]
pub type ADC1STR4 = crate::Reg<adc1str4::ADC1STR4_SPEC>;
#[doc = "ADC1STR4"]
pub mod adc1str4;
#[doc = "ADC1STR5 (rw) register accessor: an alias for `Reg<ADC1STR5_SPEC>`"]
pub type ADC1STR5 = crate::Reg<adc1str5::ADC1STR5_SPEC>;
#[doc = "ADC1STR5"]
pub mod adc1str5;
#[doc = "ADC1STR6 (rw) register accessor: an alias for `Reg<ADC1STR6_SPEC>`"]
pub type ADC1STR6 = crate::Reg<adc1str6::ADC1STR6_SPEC>;
#[doc = "ADC1STR6"]
pub mod adc1str6;
#[doc = "ADC1STR7 (rw) register accessor: an alias for `Reg<ADC1STR7_SPEC>`"]
pub type ADC1STR7 = crate::Reg<adc1str7::ADC1STR7_SPEC>;
#[doc = "ADC1STR7"]
pub mod adc1str7;
#[doc = "ADC1DR0 (rw) register accessor: an alias for `Reg<ADC1DR0_SPEC>`"]
pub type ADC1DR0 = crate::Reg<adc1dr0::ADC1DR0_SPEC>;
#[doc = "ADC1DR0"]
pub mod adc1dr0;
#[doc = "ADC1DR1 (rw) register accessor: an alias for `Reg<ADC1DR1_SPEC>`"]
pub type ADC1DR1 = crate::Reg<adc1dr1::ADC1DR1_SPEC>;
#[doc = "ADC1DR1"]
pub mod adc1dr1;
#[doc = "ADC1DR2 (rw) register accessor: an alias for `Reg<ADC1DR2_SPEC>`"]
pub type ADC1DR2 = crate::Reg<adc1dr2::ADC1DR2_SPEC>;
#[doc = "ADC1DR2"]
pub mod adc1dr2;
#[doc = "ADC1DR3 (rw) register accessor: an alias for `Reg<ADC1DR3_SPEC>`"]
pub type ADC1DR3 = crate::Reg<adc1dr3::ADC1DR3_SPEC>;
#[doc = "ADC1DR3"]
pub mod adc1dr3;
#[doc = "ADC1DR4 (rw) register accessor: an alias for `Reg<ADC1DR4_SPEC>`"]
pub type ADC1DR4 = crate::Reg<adc1dr4::ADC1DR4_SPEC>;
#[doc = "ADC1DR4"]
pub mod adc1dr4;
#[doc = "ADC1DR5 (rw) register accessor: an alias for `Reg<ADC1DR5_SPEC>`"]
pub type ADC1DR5 = crate::Reg<adc1dr5::ADC1DR5_SPEC>;
#[doc = "ADC1DR5"]
pub mod adc1dr5;
#[doc = "ADC1DR6 (rw) register accessor: an alias for `Reg<ADC1DR6_SPEC>`"]
pub type ADC1DR6 = crate::Reg<adc1dr6::ADC1DR6_SPEC>;
#[doc = "ADC1DR6"]
pub mod adc1dr6;
#[doc = "ADC1DR7 (rw) register accessor: an alias for `Reg<ADC1DR7_SPEC>`"]
pub type ADC1DR7 = crate::Reg<adc1dr7::ADC1DR7_SPEC>;
#[doc = "ADC1DR7"]
pub mod adc1dr7;
#[doc = "ADC1HDR0 (rw) register accessor: an alias for `Reg<ADC1HDR0_SPEC>`"]
pub type ADC1HDR0 = crate::Reg<adc1hdr0::ADC1HDR0_SPEC>;
#[doc = "ADC1HDR0"]
pub mod adc1hdr0;
#[doc = "ADC1HDR1 (rw) register accessor: an alias for `Reg<ADC1HDR1_SPEC>`"]
pub type ADC1HDR1 = crate::Reg<adc1hdr1::ADC1HDR1_SPEC>;
#[doc = "ADC1HDR1"]
pub mod adc1hdr1;
#[doc = "ADC1HDR2 (rw) register accessor: an alias for `Reg<ADC1HDR2_SPEC>`"]
pub type ADC1HDR2 = crate::Reg<adc1hdr2::ADC1HDR2_SPEC>;
#[doc = "ADC1HDR2"]
pub mod adc1hdr2;
#[doc = "ADC1HDR3 (rw) register accessor: an alias for `Reg<ADC1HDR3_SPEC>`"]
pub type ADC1HDR3 = crate::Reg<adc1hdr3::ADC1HDR3_SPEC>;
#[doc = "ADC1HDR3"]
pub mod adc1hdr3;
#[doc = "ADC1TCR (rw) register accessor: an alias for `Reg<ADC1TCR_SPEC>`"]
pub type ADC1TCR = crate::Reg<adc1tcr::ADC1TCR_SPEC>;
#[doc = "ADC1TCR"]
pub mod adc1tcr;
#[doc = "ADC1TSR (rw) register accessor: an alias for `Reg<ADC1TSR_SPEC>`"]
pub type ADC1TSR = crate::Reg<adc1tsr::ADC1TSR_SPEC>;
#[doc = "ADC1TSR"]
pub mod adc1tsr;
#[doc = "ADC1HTCR (rw) register accessor: an alias for `Reg<ADC1HTCR_SPEC>`"]
pub type ADC1HTCR = crate::Reg<adc1htcr::ADC1HTCR_SPEC>;
#[doc = "ADC1HTCR"]
pub mod adc1htcr;
#[doc = "ADC1HTSR (rw) register accessor: an alias for `Reg<ADC1HTSR_SPEC>`"]
pub type ADC1HTSR = crate::Reg<adc1htsr::ADC1HTSR_SPEC>;
#[doc = "ADC1HTSR"]
pub mod adc1htsr;
#[doc = "ADC1WCR (rw) register accessor: an alias for `Reg<ADC1WCR_SPEC>`"]
pub type ADC1WCR = crate::Reg<adc1wcr::ADC1WCR_SPEC>;
#[doc = "ADC1WCR"]
pub mod adc1wcr;
#[doc = "ADC1WLTR (rw) register accessor: an alias for `Reg<ADC1WLTR_SPEC>`"]
pub type ADC1WLTR = crate::Reg<adc1wltr::ADC1WLTR_SPEC>;
#[doc = "ADC1WLTR"]
pub mod adc1wltr;
#[doc = "ADC1WUTR (rw) register accessor: an alias for `Reg<ADC1WUTR_SPEC>`"]
pub type ADC1WUTR = crate::Reg<adc1wutr::ADC1WUTR_SPEC>;
#[doc = "ADC1WUTR"]
pub mod adc1wutr;
#[doc = "ADC1IER (rw) register accessor: an alias for `Reg<ADC1IER_SPEC>`"]
pub type ADC1IER = crate::Reg<adc1ier::ADC1IER_SPEC>;
#[doc = "ADC1IER"]
pub mod adc1ier;
#[doc = "ADC1IRAW (rw) register accessor: an alias for `Reg<ADC1IRAW_SPEC>`"]
pub type ADC1IRAW = crate::Reg<adc1iraw::ADC1IRAW_SPEC>;
#[doc = "ADC1IRAW"]
pub mod adc1iraw;
#[doc = "ADC1ISR (rw) register accessor: an alias for `Reg<ADC1ISR_SPEC>`"]
pub type ADC1ISR = crate::Reg<adc1isr::ADC1ISR_SPEC>;
#[doc = "ADC1ISR"]
pub mod adc1isr;
#[doc = "ADC1ICLR (rw) register accessor: an alias for `Reg<ADC1ICLR_SPEC>`"]
pub type ADC1ICLR = crate::Reg<adc1iclr::ADC1ICLR_SPEC>;
#[doc = "ADC1ICLR"]
pub mod adc1iclr;
#[doc = "ADC1DMAR (rw) register accessor: an alias for `Reg<ADC1DMAR_SPEC>`"]
pub type ADC1DMAR = crate::Reg<adc1dmar::ADC1DMAR_SPEC>;
#[doc = "ADC1DMAR"]
pub mod adc1dmar;
#[doc = "DADCHIR (rw) register accessor: an alias for `Reg<DADCHIR_SPEC>`"]
pub type DADCHIR = crate::Reg<dadchir::DADCHIR_SPEC>;
#[doc = "DADCHIR"]
pub mod dadchir;
#[doc = "DADCDMAR (rw) register accessor: an alias for `Reg<DADCDMAR_SPEC>`"]
pub type DADCDMAR = crate::Reg<dadcdmar::DADCDMAR_SPEC>;
#[doc = "DADCDMAR"]
pub mod dadcdmar;
