#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - ADC_RST"]
    pub adc_rst: ADC_RST,
    #[doc = "0x08 - ADC_CONV"]
    pub adc_conv: ADC_CONV,
    #[doc = "0x0c - ADC_HCONV"]
    pub adc_hconv: ADC_HCONV,
    #[doc = "0x10 - ADC_LST0"]
    pub adc_lst0: ADC_LST0,
    #[doc = "0x14 - ADC_LST1"]
    pub adc_lst1: ADC_LST1,
    #[doc = "0x18 - ADC_LST2"]
    pub adc_lst2: ADC_LST2,
    #[doc = "0x1c - ADC_LST3"]
    pub adc_lst3: ADC_LST3,
    #[doc = "0x20 - ADC_HLST"]
    pub adc_hlst: ADC_HLST,
    _reserved8: [u8; 0x0c],
    #[doc = "0x30 - ADC_OFR0"]
    pub adc_ofr0: ADC_OFR0,
    #[doc = "0x34 - ADC_OFR1"]
    pub adc_ofr1: ADC_OFR1,
    #[doc = "0x38 - ADC_OFR2"]
    pub adc_ofr2: ADC_OFR2,
    #[doc = "0x3c - ADC_OFR3"]
    pub adc_ofr3: ADC_OFR3,
    #[doc = "0x40 - ADC_OFR4"]
    pub adc_ofr4: ADC_OFR4,
    #[doc = "0x44 - ADC_OFR5"]
    pub adc_ofr5: ADC_OFR5,
    #[doc = "0x48 - ADC_OFR6"]
    pub adc_ofr6: ADC_OFR6,
    #[doc = "0x4c - ADC_OFR7"]
    pub adc_ofr7: ADC_OFR7,
    #[doc = "0x50 - ADC_OFR8"]
    pub adc_ofr8: ADC_OFR8,
    #[doc = "0x54 - ADC_OFR9"]
    pub adc_ofr9: ADC_OFR9,
    #[doc = "0x58 - ADC_OFR10"]
    pub adc_ofr10: ADC_OFR10,
    #[doc = "0x5c - ADC_OFR11"]
    pub adc_ofr11: ADC_OFR11,
    #[doc = "0x60 - ADC_OFR12"]
    pub adc_ofr12: ADC_OFR12,
    #[doc = "0x64 - ADC_OFR13"]
    pub adc_ofr13: ADC_OFR13,
    #[doc = "0x68 - ADC_OFR14"]
    pub adc_ofr14: ADC_OFR14,
    #[doc = "0x6c - ADC_OFR15"]
    pub adc_ofr15: ADC_OFR15,
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
    #[doc = "0x90 - ADC_STR8"]
    pub adc_str8: ADC_STR8,
    #[doc = "0x94 - ADC_STR9"]
    pub adc_str9: ADC_STR9,
    #[doc = "0x98 - ADC_STR10"]
    pub adc_str10: ADC_STR10,
    #[doc = "0x9c - ADC_STR11"]
    pub adc_str11: ADC_STR11,
    #[doc = "0xa0 - ADC_STR12"]
    pub adc_str12: ADC_STR12,
    #[doc = "0xa4 - ADC_STR13"]
    pub adc_str13: ADC_STR13,
    #[doc = "0xa8 - ADC_STR14"]
    pub adc_str14: ADC_STR14,
    #[doc = "0xac - ADC_STR15"]
    pub adc_str15: ADC_STR15,
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
    #[doc = "0xd0 - ADC_DR8"]
    pub adc_dr8: ADC_DR8,
    #[doc = "0xd4 - ADC_DR9"]
    pub adc_dr9: ADC_DR9,
    #[doc = "0xd8 - ADC_DR10"]
    pub adc_dr10: ADC_DR10,
    #[doc = "0xdc - ADC_DR11"]
    pub adc_dr11: ADC_DR11,
    #[doc = "0xe0 - ADC_DR12"]
    pub adc_dr12: ADC_DR12,
    #[doc = "0xe4 - ADC_DR13"]
    pub adc_dr13: ADC_DR13,
    #[doc = "0xe8 - ADC_DR14"]
    pub adc_dr14: ADC_DR14,
    #[doc = "0xec - ADC_DR15"]
    pub adc_dr15: ADC_DR15,
    #[doc = "0xf0 - ADC_HDR0"]
    pub adc_hdr0: ADC_HDR0,
    #[doc = "0xf4 - ADC_HDR1"]
    pub adc_hdr1: ADC_HDR1,
    #[doc = "0xf8 - ADC_HDR2"]
    pub adc_hdr2: ADC_HDR2,
    #[doc = "0xfc - ADC_HDR3"]
    pub adc_hdr3: ADC_HDR3,
    #[doc = "0x100 - ADC_TCR"]
    pub adc_tcr: ADC_TCR,
    #[doc = "0x104 - ADC_TSR"]
    pub adc_tsr: ADC_TSR,
    _reserved62: [u8; 0x08],
    #[doc = "0x110 - ADC_HTCR"]
    pub adc_htcr: ADC_HTCR,
    #[doc = "0x114 - ADC_HTSR"]
    pub adc_htsr: ADC_HTSR,
    _reserved64: [u8; 0x08],
    #[doc = "0x120 - ADC_WCR"]
    pub adc_wcr: ADC_WCR,
    #[doc = "0x124 - ADC_LTR"]
    pub adc_ltr: ADC_LTR,
    #[doc = "0x128 - ADC_UTR"]
    pub adc_utr: ADC_UTR,
    _reserved67: [u8; 0x04],
    #[doc = "0x130 - ADC_IMR"]
    pub adc_imr: ADC_IMR,
    #[doc = "0x134 - ADC_IRAW"]
    pub adc_iraw: ADC_IRAW,
    #[doc = "0x138 - ADC_IMASK"]
    pub adc_imask: ADC_IMASK,
    #[doc = "0x13c - ADC_ICLR"]
    pub adc_iclr: ADC_ICLR,
    #[doc = "0x140 - ADC_DMAR"]
    pub adc_dmar: ADC_DMAR,
}
#[doc = "ADC_RST (rw) register accessor: an alias for `Reg<ADC_RST_SPEC>`"]
pub type ADC_RST = crate::Reg<adc_rst::ADC_RST_SPEC>;
#[doc = "ADC_RST"]
pub mod adc_rst;
#[doc = "ADC_CONV (rw) register accessor: an alias for `Reg<ADC_CONV_SPEC>`"]
pub type ADC_CONV = crate::Reg<adc_conv::ADC_CONV_SPEC>;
#[doc = "ADC_CONV"]
pub mod adc_conv;
#[doc = "ADC_HCONV (rw) register accessor: an alias for `Reg<ADC_HCONV_SPEC>`"]
pub type ADC_HCONV = crate::Reg<adc_hconv::ADC_HCONV_SPEC>;
#[doc = "ADC_HCONV"]
pub mod adc_hconv;
#[doc = "ADC_LST0 (rw) register accessor: an alias for `Reg<ADC_LST0_SPEC>`"]
pub type ADC_LST0 = crate::Reg<adc_lst0::ADC_LST0_SPEC>;
#[doc = "ADC_LST0"]
pub mod adc_lst0;
#[doc = "ADC_LST1 (rw) register accessor: an alias for `Reg<ADC_LST1_SPEC>`"]
pub type ADC_LST1 = crate::Reg<adc_lst1::ADC_LST1_SPEC>;
#[doc = "ADC_LST1"]
pub mod adc_lst1;
#[doc = "ADC_LST2 (rw) register accessor: an alias for `Reg<ADC_LST2_SPEC>`"]
pub type ADC_LST2 = crate::Reg<adc_lst2::ADC_LST2_SPEC>;
#[doc = "ADC_LST2"]
pub mod adc_lst2;
#[doc = "ADC_LST3 (rw) register accessor: an alias for `Reg<ADC_LST3_SPEC>`"]
pub type ADC_LST3 = crate::Reg<adc_lst3::ADC_LST3_SPEC>;
#[doc = "ADC_LST3"]
pub mod adc_lst3;
#[doc = "ADC_HLST (rw) register accessor: an alias for `Reg<ADC_HLST_SPEC>`"]
pub type ADC_HLST = crate::Reg<adc_hlst::ADC_HLST_SPEC>;
#[doc = "ADC_HLST"]
pub mod adc_hlst;
#[doc = "ADC_OFR0 (rw) register accessor: an alias for `Reg<ADC_OFR0_SPEC>`"]
pub type ADC_OFR0 = crate::Reg<adc_ofr0::ADC_OFR0_SPEC>;
#[doc = "ADC_OFR0"]
pub mod adc_ofr0;
#[doc = "ADC_OFR1 (rw) register accessor: an alias for `Reg<ADC_OFR1_SPEC>`"]
pub type ADC_OFR1 = crate::Reg<adc_ofr1::ADC_OFR1_SPEC>;
#[doc = "ADC_OFR1"]
pub mod adc_ofr1;
#[doc = "ADC_OFR2 (rw) register accessor: an alias for `Reg<ADC_OFR2_SPEC>`"]
pub type ADC_OFR2 = crate::Reg<adc_ofr2::ADC_OFR2_SPEC>;
#[doc = "ADC_OFR2"]
pub mod adc_ofr2;
#[doc = "ADC_OFR3 (rw) register accessor: an alias for `Reg<ADC_OFR3_SPEC>`"]
pub type ADC_OFR3 = crate::Reg<adc_ofr3::ADC_OFR3_SPEC>;
#[doc = "ADC_OFR3"]
pub mod adc_ofr3;
#[doc = "ADC_OFR4 (rw) register accessor: an alias for `Reg<ADC_OFR4_SPEC>`"]
pub type ADC_OFR4 = crate::Reg<adc_ofr4::ADC_OFR4_SPEC>;
#[doc = "ADC_OFR4"]
pub mod adc_ofr4;
#[doc = "ADC_OFR5 (rw) register accessor: an alias for `Reg<ADC_OFR5_SPEC>`"]
pub type ADC_OFR5 = crate::Reg<adc_ofr5::ADC_OFR5_SPEC>;
#[doc = "ADC_OFR5"]
pub mod adc_ofr5;
#[doc = "ADC_OFR6 (rw) register accessor: an alias for `Reg<ADC_OFR6_SPEC>`"]
pub type ADC_OFR6 = crate::Reg<adc_ofr6::ADC_OFR6_SPEC>;
#[doc = "ADC_OFR6"]
pub mod adc_ofr6;
#[doc = "ADC_OFR7 (rw) register accessor: an alias for `Reg<ADC_OFR7_SPEC>`"]
pub type ADC_OFR7 = crate::Reg<adc_ofr7::ADC_OFR7_SPEC>;
#[doc = "ADC_OFR7"]
pub mod adc_ofr7;
#[doc = "ADC_OFR8 (rw) register accessor: an alias for `Reg<ADC_OFR8_SPEC>`"]
pub type ADC_OFR8 = crate::Reg<adc_ofr8::ADC_OFR8_SPEC>;
#[doc = "ADC_OFR8"]
pub mod adc_ofr8;
#[doc = "ADC_OFR9 (rw) register accessor: an alias for `Reg<ADC_OFR9_SPEC>`"]
pub type ADC_OFR9 = crate::Reg<adc_ofr9::ADC_OFR9_SPEC>;
#[doc = "ADC_OFR9"]
pub mod adc_ofr9;
#[doc = "ADC_OFR10 (rw) register accessor: an alias for `Reg<ADC_OFR10_SPEC>`"]
pub type ADC_OFR10 = crate::Reg<adc_ofr10::ADC_OFR10_SPEC>;
#[doc = "ADC_OFR10"]
pub mod adc_ofr10;
#[doc = "ADC_OFR11 (rw) register accessor: an alias for `Reg<ADC_OFR11_SPEC>`"]
pub type ADC_OFR11 = crate::Reg<adc_ofr11::ADC_OFR11_SPEC>;
#[doc = "ADC_OFR11"]
pub mod adc_ofr11;
#[doc = "ADC_OFR12 (rw) register accessor: an alias for `Reg<ADC_OFR12_SPEC>`"]
pub type ADC_OFR12 = crate::Reg<adc_ofr12::ADC_OFR12_SPEC>;
#[doc = "ADC_OFR12"]
pub mod adc_ofr12;
#[doc = "ADC_OFR13 (rw) register accessor: an alias for `Reg<ADC_OFR13_SPEC>`"]
pub type ADC_OFR13 = crate::Reg<adc_ofr13::ADC_OFR13_SPEC>;
#[doc = "ADC_OFR13"]
pub mod adc_ofr13;
#[doc = "ADC_OFR14 (rw) register accessor: an alias for `Reg<ADC_OFR14_SPEC>`"]
pub type ADC_OFR14 = crate::Reg<adc_ofr14::ADC_OFR14_SPEC>;
#[doc = "ADC_OFR14"]
pub mod adc_ofr14;
#[doc = "ADC_OFR15 (rw) register accessor: an alias for `Reg<ADC_OFR15_SPEC>`"]
pub type ADC_OFR15 = crate::Reg<adc_ofr15::ADC_OFR15_SPEC>;
#[doc = "ADC_OFR15"]
pub mod adc_ofr15;
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
#[doc = "ADC_STR8 (rw) register accessor: an alias for `Reg<ADC_STR8_SPEC>`"]
pub type ADC_STR8 = crate::Reg<adc_str8::ADC_STR8_SPEC>;
#[doc = "ADC_STR8"]
pub mod adc_str8;
#[doc = "ADC_STR9 (rw) register accessor: an alias for `Reg<ADC_STR9_SPEC>`"]
pub type ADC_STR9 = crate::Reg<adc_str9::ADC_STR9_SPEC>;
#[doc = "ADC_STR9"]
pub mod adc_str9;
#[doc = "ADC_STR10 (rw) register accessor: an alias for `Reg<ADC_STR10_SPEC>`"]
pub type ADC_STR10 = crate::Reg<adc_str10::ADC_STR10_SPEC>;
#[doc = "ADC_STR10"]
pub mod adc_str10;
#[doc = "ADC_STR11 (rw) register accessor: an alias for `Reg<ADC_STR11_SPEC>`"]
pub type ADC_STR11 = crate::Reg<adc_str11::ADC_STR11_SPEC>;
#[doc = "ADC_STR11"]
pub mod adc_str11;
#[doc = "ADC_STR12 (rw) register accessor: an alias for `Reg<ADC_STR12_SPEC>`"]
pub type ADC_STR12 = crate::Reg<adc_str12::ADC_STR12_SPEC>;
#[doc = "ADC_STR12"]
pub mod adc_str12;
#[doc = "ADC_STR13 (rw) register accessor: an alias for `Reg<ADC_STR13_SPEC>`"]
pub type ADC_STR13 = crate::Reg<adc_str13::ADC_STR13_SPEC>;
#[doc = "ADC_STR13"]
pub mod adc_str13;
#[doc = "ADC_STR14 (rw) register accessor: an alias for `Reg<ADC_STR14_SPEC>`"]
pub type ADC_STR14 = crate::Reg<adc_str14::ADC_STR14_SPEC>;
#[doc = "ADC_STR14"]
pub mod adc_str14;
#[doc = "ADC_STR15 (rw) register accessor: an alias for `Reg<ADC_STR15_SPEC>`"]
pub type ADC_STR15 = crate::Reg<adc_str15::ADC_STR15_SPEC>;
#[doc = "ADC_STR15"]
pub mod adc_str15;
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
#[doc = "ADC_DR8 (rw) register accessor: an alias for `Reg<ADC_DR8_SPEC>`"]
pub type ADC_DR8 = crate::Reg<adc_dr8::ADC_DR8_SPEC>;
#[doc = "ADC_DR8"]
pub mod adc_dr8;
#[doc = "ADC_DR9 (rw) register accessor: an alias for `Reg<ADC_DR9_SPEC>`"]
pub type ADC_DR9 = crate::Reg<adc_dr9::ADC_DR9_SPEC>;
#[doc = "ADC_DR9"]
pub mod adc_dr9;
#[doc = "ADC_DR10 (rw) register accessor: an alias for `Reg<ADC_DR10_SPEC>`"]
pub type ADC_DR10 = crate::Reg<adc_dr10::ADC_DR10_SPEC>;
#[doc = "ADC_DR10"]
pub mod adc_dr10;
#[doc = "ADC_DR11 (rw) register accessor: an alias for `Reg<ADC_DR11_SPEC>`"]
pub type ADC_DR11 = crate::Reg<adc_dr11::ADC_DR11_SPEC>;
#[doc = "ADC_DR11"]
pub mod adc_dr11;
#[doc = "ADC_DR12 (rw) register accessor: an alias for `Reg<ADC_DR12_SPEC>`"]
pub type ADC_DR12 = crate::Reg<adc_dr12::ADC_DR12_SPEC>;
#[doc = "ADC_DR12"]
pub mod adc_dr12;
#[doc = "ADC_DR13 (rw) register accessor: an alias for `Reg<ADC_DR13_SPEC>`"]
pub type ADC_DR13 = crate::Reg<adc_dr13::ADC_DR13_SPEC>;
#[doc = "ADC_DR13"]
pub mod adc_dr13;
#[doc = "ADC_DR14 (rw) register accessor: an alias for `Reg<ADC_DR14_SPEC>`"]
pub type ADC_DR14 = crate::Reg<adc_dr14::ADC_DR14_SPEC>;
#[doc = "ADC_DR14"]
pub mod adc_dr14;
#[doc = "ADC_DR15 (rw) register accessor: an alias for `Reg<ADC_DR15_SPEC>`"]
pub type ADC_DR15 = crate::Reg<adc_dr15::ADC_DR15_SPEC>;
#[doc = "ADC_DR15"]
pub mod adc_dr15;
#[doc = "ADC_HDR0 (rw) register accessor: an alias for `Reg<ADC_HDR0_SPEC>`"]
pub type ADC_HDR0 = crate::Reg<adc_hdr0::ADC_HDR0_SPEC>;
#[doc = "ADC_HDR0"]
pub mod adc_hdr0;
#[doc = "ADC_HDR1 (rw) register accessor: an alias for `Reg<ADC_HDR1_SPEC>`"]
pub type ADC_HDR1 = crate::Reg<adc_hdr1::ADC_HDR1_SPEC>;
#[doc = "ADC_HDR1"]
pub mod adc_hdr1;
#[doc = "ADC_HDR2 (rw) register accessor: an alias for `Reg<ADC_HDR2_SPEC>`"]
pub type ADC_HDR2 = crate::Reg<adc_hdr2::ADC_HDR2_SPEC>;
#[doc = "ADC_HDR2"]
pub mod adc_hdr2;
#[doc = "ADC_HDR3 (rw) register accessor: an alias for `Reg<ADC_HDR3_SPEC>`"]
pub type ADC_HDR3 = crate::Reg<adc_hdr3::ADC_HDR3_SPEC>;
#[doc = "ADC_HDR3"]
pub mod adc_hdr3;
#[doc = "ADC_TCR (rw) register accessor: an alias for `Reg<ADC_TCR_SPEC>`"]
pub type ADC_TCR = crate::Reg<adc_tcr::ADC_TCR_SPEC>;
#[doc = "ADC_TCR"]
pub mod adc_tcr;
#[doc = "ADC_TSR (rw) register accessor: an alias for `Reg<ADC_TSR_SPEC>`"]
pub type ADC_TSR = crate::Reg<adc_tsr::ADC_TSR_SPEC>;
#[doc = "ADC_TSR"]
pub mod adc_tsr;
#[doc = "ADC_HTCR (rw) register accessor: an alias for `Reg<ADC_HTCR_SPEC>`"]
pub type ADC_HTCR = crate::Reg<adc_htcr::ADC_HTCR_SPEC>;
#[doc = "ADC_HTCR"]
pub mod adc_htcr;
#[doc = "ADC_HTSR (rw) register accessor: an alias for `Reg<ADC_HTSR_SPEC>`"]
pub type ADC_HTSR = crate::Reg<adc_htsr::ADC_HTSR_SPEC>;
#[doc = "ADC_HTSR"]
pub mod adc_htsr;
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
#[doc = "ADC_DMAR (rw) register accessor: an alias for `Reg<ADC_DMAR_SPEC>`"]
pub type ADC_DMAR = crate::Reg<adc_dmar::ADC_DMAR_SPEC>;
#[doc = "ADC_DMAR"]
pub mod adc_dmar;
