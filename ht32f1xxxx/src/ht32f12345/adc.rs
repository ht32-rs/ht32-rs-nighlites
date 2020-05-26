#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
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
    _reserved8: [u8; 12usize],
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
    _reserved62: [u8; 8usize],
    #[doc = "0x110 - ADC_HTCR"]
    pub adc_htcr: ADC_HTCR,
    #[doc = "0x114 - ADC_HTSR"]
    pub adc_htsr: ADC_HTSR,
    _reserved64: [u8; 8usize],
    #[doc = "0x120 - ADC_WCR"]
    pub adc_wcr: ADC_WCR,
    #[doc = "0x124 - ADC_LTR"]
    pub adc_ltr: ADC_LTR,
    #[doc = "0x128 - ADC_UTR"]
    pub adc_utr: ADC_UTR,
    _reserved67: [u8; 4usize],
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
#[doc = "ADC_RST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_rst](adc_rst) module"]
pub type ADC_RST = crate::Reg<u32, _ADC_RST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_RST;
#[doc = "`read()` method returns [adc_rst::R](adc_rst::R) reader structure"]
impl crate::Readable for ADC_RST {}
#[doc = "`write(|w| ..)` method takes [adc_rst::W](adc_rst::W) writer structure"]
impl crate::Writable for ADC_RST {}
#[doc = "ADC_RST"]
pub mod adc_rst;
#[doc = "ADC_CONV\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_conv](adc_conv) module"]
pub type ADC_CONV = crate::Reg<u32, _ADC_CONV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CONV;
#[doc = "`read()` method returns [adc_conv::R](adc_conv::R) reader structure"]
impl crate::Readable for ADC_CONV {}
#[doc = "`write(|w| ..)` method takes [adc_conv::W](adc_conv::W) writer structure"]
impl crate::Writable for ADC_CONV {}
#[doc = "ADC_CONV"]
pub mod adc_conv;
#[doc = "ADC_HCONV\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_hconv](adc_hconv) module"]
pub type ADC_HCONV = crate::Reg<u32, _ADC_HCONV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_HCONV;
#[doc = "`read()` method returns [adc_hconv::R](adc_hconv::R) reader structure"]
impl crate::Readable for ADC_HCONV {}
#[doc = "`write(|w| ..)` method takes [adc_hconv::W](adc_hconv::W) writer structure"]
impl crate::Writable for ADC_HCONV {}
#[doc = "ADC_HCONV"]
pub mod adc_hconv;
#[doc = "ADC_LST0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_lst0](adc_lst0) module"]
pub type ADC_LST0 = crate::Reg<u32, _ADC_LST0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_LST0;
#[doc = "`read()` method returns [adc_lst0::R](adc_lst0::R) reader structure"]
impl crate::Readable for ADC_LST0 {}
#[doc = "`write(|w| ..)` method takes [adc_lst0::W](adc_lst0::W) writer structure"]
impl crate::Writable for ADC_LST0 {}
#[doc = "ADC_LST0"]
pub mod adc_lst0;
#[doc = "ADC_LST1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_lst1](adc_lst1) module"]
pub type ADC_LST1 = crate::Reg<u32, _ADC_LST1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_LST1;
#[doc = "`read()` method returns [adc_lst1::R](adc_lst1::R) reader structure"]
impl crate::Readable for ADC_LST1 {}
#[doc = "`write(|w| ..)` method takes [adc_lst1::W](adc_lst1::W) writer structure"]
impl crate::Writable for ADC_LST1 {}
#[doc = "ADC_LST1"]
pub mod adc_lst1;
#[doc = "ADC_LST2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_lst2](adc_lst2) module"]
pub type ADC_LST2 = crate::Reg<u32, _ADC_LST2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_LST2;
#[doc = "`read()` method returns [adc_lst2::R](adc_lst2::R) reader structure"]
impl crate::Readable for ADC_LST2 {}
#[doc = "`write(|w| ..)` method takes [adc_lst2::W](adc_lst2::W) writer structure"]
impl crate::Writable for ADC_LST2 {}
#[doc = "ADC_LST2"]
pub mod adc_lst2;
#[doc = "ADC_LST3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_lst3](adc_lst3) module"]
pub type ADC_LST3 = crate::Reg<u32, _ADC_LST3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_LST3;
#[doc = "`read()` method returns [adc_lst3::R](adc_lst3::R) reader structure"]
impl crate::Readable for ADC_LST3 {}
#[doc = "`write(|w| ..)` method takes [adc_lst3::W](adc_lst3::W) writer structure"]
impl crate::Writable for ADC_LST3 {}
#[doc = "ADC_LST3"]
pub mod adc_lst3;
#[doc = "ADC_HLST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_hlst](adc_hlst) module"]
pub type ADC_HLST = crate::Reg<u32, _ADC_HLST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_HLST;
#[doc = "`read()` method returns [adc_hlst::R](adc_hlst::R) reader structure"]
impl crate::Readable for ADC_HLST {}
#[doc = "`write(|w| ..)` method takes [adc_hlst::W](adc_hlst::W) writer structure"]
impl crate::Writable for ADC_HLST {}
#[doc = "ADC_HLST"]
pub mod adc_hlst;
#[doc = "ADC_OFR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr0](adc_ofr0) module"]
pub type ADC_OFR0 = crate::Reg<u32, _ADC_OFR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR0;
#[doc = "`read()` method returns [adc_ofr0::R](adc_ofr0::R) reader structure"]
impl crate::Readable for ADC_OFR0 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr0::W](adc_ofr0::W) writer structure"]
impl crate::Writable for ADC_OFR0 {}
#[doc = "ADC_OFR0"]
pub mod adc_ofr0;
#[doc = "ADC_OFR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr1](adc_ofr1) module"]
pub type ADC_OFR1 = crate::Reg<u32, _ADC_OFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR1;
#[doc = "`read()` method returns [adc_ofr1::R](adc_ofr1::R) reader structure"]
impl crate::Readable for ADC_OFR1 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr1::W](adc_ofr1::W) writer structure"]
impl crate::Writable for ADC_OFR1 {}
#[doc = "ADC_OFR1"]
pub mod adc_ofr1;
#[doc = "ADC_OFR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr2](adc_ofr2) module"]
pub type ADC_OFR2 = crate::Reg<u32, _ADC_OFR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR2;
#[doc = "`read()` method returns [adc_ofr2::R](adc_ofr2::R) reader structure"]
impl crate::Readable for ADC_OFR2 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr2::W](adc_ofr2::W) writer structure"]
impl crate::Writable for ADC_OFR2 {}
#[doc = "ADC_OFR2"]
pub mod adc_ofr2;
#[doc = "ADC_OFR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr3](adc_ofr3) module"]
pub type ADC_OFR3 = crate::Reg<u32, _ADC_OFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR3;
#[doc = "`read()` method returns [adc_ofr3::R](adc_ofr3::R) reader structure"]
impl crate::Readable for ADC_OFR3 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr3::W](adc_ofr3::W) writer structure"]
impl crate::Writable for ADC_OFR3 {}
#[doc = "ADC_OFR3"]
pub mod adc_ofr3;
#[doc = "ADC_OFR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr4](adc_ofr4) module"]
pub type ADC_OFR4 = crate::Reg<u32, _ADC_OFR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR4;
#[doc = "`read()` method returns [adc_ofr4::R](adc_ofr4::R) reader structure"]
impl crate::Readable for ADC_OFR4 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr4::W](adc_ofr4::W) writer structure"]
impl crate::Writable for ADC_OFR4 {}
#[doc = "ADC_OFR4"]
pub mod adc_ofr4;
#[doc = "ADC_OFR5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr5](adc_ofr5) module"]
pub type ADC_OFR5 = crate::Reg<u32, _ADC_OFR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR5;
#[doc = "`read()` method returns [adc_ofr5::R](adc_ofr5::R) reader structure"]
impl crate::Readable for ADC_OFR5 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr5::W](adc_ofr5::W) writer structure"]
impl crate::Writable for ADC_OFR5 {}
#[doc = "ADC_OFR5"]
pub mod adc_ofr5;
#[doc = "ADC_OFR6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr6](adc_ofr6) module"]
pub type ADC_OFR6 = crate::Reg<u32, _ADC_OFR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR6;
#[doc = "`read()` method returns [adc_ofr6::R](adc_ofr6::R) reader structure"]
impl crate::Readable for ADC_OFR6 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr6::W](adc_ofr6::W) writer structure"]
impl crate::Writable for ADC_OFR6 {}
#[doc = "ADC_OFR6"]
pub mod adc_ofr6;
#[doc = "ADC_OFR7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr7](adc_ofr7) module"]
pub type ADC_OFR7 = crate::Reg<u32, _ADC_OFR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR7;
#[doc = "`read()` method returns [adc_ofr7::R](adc_ofr7::R) reader structure"]
impl crate::Readable for ADC_OFR7 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr7::W](adc_ofr7::W) writer structure"]
impl crate::Writable for ADC_OFR7 {}
#[doc = "ADC_OFR7"]
pub mod adc_ofr7;
#[doc = "ADC_OFR8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr8](adc_ofr8) module"]
pub type ADC_OFR8 = crate::Reg<u32, _ADC_OFR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR8;
#[doc = "`read()` method returns [adc_ofr8::R](adc_ofr8::R) reader structure"]
impl crate::Readable for ADC_OFR8 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr8::W](adc_ofr8::W) writer structure"]
impl crate::Writable for ADC_OFR8 {}
#[doc = "ADC_OFR8"]
pub mod adc_ofr8;
#[doc = "ADC_OFR9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr9](adc_ofr9) module"]
pub type ADC_OFR9 = crate::Reg<u32, _ADC_OFR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR9;
#[doc = "`read()` method returns [adc_ofr9::R](adc_ofr9::R) reader structure"]
impl crate::Readable for ADC_OFR9 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr9::W](adc_ofr9::W) writer structure"]
impl crate::Writable for ADC_OFR9 {}
#[doc = "ADC_OFR9"]
pub mod adc_ofr9;
#[doc = "ADC_OFR10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr10](adc_ofr10) module"]
pub type ADC_OFR10 = crate::Reg<u32, _ADC_OFR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR10;
#[doc = "`read()` method returns [adc_ofr10::R](adc_ofr10::R) reader structure"]
impl crate::Readable for ADC_OFR10 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr10::W](adc_ofr10::W) writer structure"]
impl crate::Writable for ADC_OFR10 {}
#[doc = "ADC_OFR10"]
pub mod adc_ofr10;
#[doc = "ADC_OFR11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr11](adc_ofr11) module"]
pub type ADC_OFR11 = crate::Reg<u32, _ADC_OFR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR11;
#[doc = "`read()` method returns [adc_ofr11::R](adc_ofr11::R) reader structure"]
impl crate::Readable for ADC_OFR11 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr11::W](adc_ofr11::W) writer structure"]
impl crate::Writable for ADC_OFR11 {}
#[doc = "ADC_OFR11"]
pub mod adc_ofr11;
#[doc = "ADC_OFR12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr12](adc_ofr12) module"]
pub type ADC_OFR12 = crate::Reg<u32, _ADC_OFR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR12;
#[doc = "`read()` method returns [adc_ofr12::R](adc_ofr12::R) reader structure"]
impl crate::Readable for ADC_OFR12 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr12::W](adc_ofr12::W) writer structure"]
impl crate::Writable for ADC_OFR12 {}
#[doc = "ADC_OFR12"]
pub mod adc_ofr12;
#[doc = "ADC_OFR13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr13](adc_ofr13) module"]
pub type ADC_OFR13 = crate::Reg<u32, _ADC_OFR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR13;
#[doc = "`read()` method returns [adc_ofr13::R](adc_ofr13::R) reader structure"]
impl crate::Readable for ADC_OFR13 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr13::W](adc_ofr13::W) writer structure"]
impl crate::Writable for ADC_OFR13 {}
#[doc = "ADC_OFR13"]
pub mod adc_ofr13;
#[doc = "ADC_OFR14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr14](adc_ofr14) module"]
pub type ADC_OFR14 = crate::Reg<u32, _ADC_OFR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR14;
#[doc = "`read()` method returns [adc_ofr14::R](adc_ofr14::R) reader structure"]
impl crate::Readable for ADC_OFR14 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr14::W](adc_ofr14::W) writer structure"]
impl crate::Writable for ADC_OFR14 {}
#[doc = "ADC_OFR14"]
pub mod adc_ofr14;
#[doc = "ADC_OFR15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr15](adc_ofr15) module"]
pub type ADC_OFR15 = crate::Reg<u32, _ADC_OFR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR15;
#[doc = "`read()` method returns [adc_ofr15::R](adc_ofr15::R) reader structure"]
impl crate::Readable for ADC_OFR15 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr15::W](adc_ofr15::W) writer structure"]
impl crate::Writable for ADC_OFR15 {}
#[doc = "ADC_OFR15"]
pub mod adc_ofr15;
#[doc = "ADC_STR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str0](adc_str0) module"]
pub type ADC_STR0 = crate::Reg<u32, _ADC_STR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR0;
#[doc = "`read()` method returns [adc_str0::R](adc_str0::R) reader structure"]
impl crate::Readable for ADC_STR0 {}
#[doc = "`write(|w| ..)` method takes [adc_str0::W](adc_str0::W) writer structure"]
impl crate::Writable for ADC_STR0 {}
#[doc = "ADC_STR0"]
pub mod adc_str0;
#[doc = "ADC_STR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str1](adc_str1) module"]
pub type ADC_STR1 = crate::Reg<u32, _ADC_STR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR1;
#[doc = "`read()` method returns [adc_str1::R](adc_str1::R) reader structure"]
impl crate::Readable for ADC_STR1 {}
#[doc = "`write(|w| ..)` method takes [adc_str1::W](adc_str1::W) writer structure"]
impl crate::Writable for ADC_STR1 {}
#[doc = "ADC_STR1"]
pub mod adc_str1;
#[doc = "ADC_STR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str2](adc_str2) module"]
pub type ADC_STR2 = crate::Reg<u32, _ADC_STR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR2;
#[doc = "`read()` method returns [adc_str2::R](adc_str2::R) reader structure"]
impl crate::Readable for ADC_STR2 {}
#[doc = "`write(|w| ..)` method takes [adc_str2::W](adc_str2::W) writer structure"]
impl crate::Writable for ADC_STR2 {}
#[doc = "ADC_STR2"]
pub mod adc_str2;
#[doc = "ADC_STR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str3](adc_str3) module"]
pub type ADC_STR3 = crate::Reg<u32, _ADC_STR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR3;
#[doc = "`read()` method returns [adc_str3::R](adc_str3::R) reader structure"]
impl crate::Readable for ADC_STR3 {}
#[doc = "`write(|w| ..)` method takes [adc_str3::W](adc_str3::W) writer structure"]
impl crate::Writable for ADC_STR3 {}
#[doc = "ADC_STR3"]
pub mod adc_str3;
#[doc = "ADC_STR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str4](adc_str4) module"]
pub type ADC_STR4 = crate::Reg<u32, _ADC_STR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR4;
#[doc = "`read()` method returns [adc_str4::R](adc_str4::R) reader structure"]
impl crate::Readable for ADC_STR4 {}
#[doc = "`write(|w| ..)` method takes [adc_str4::W](adc_str4::W) writer structure"]
impl crate::Writable for ADC_STR4 {}
#[doc = "ADC_STR4"]
pub mod adc_str4;
#[doc = "ADC_STR5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str5](adc_str5) module"]
pub type ADC_STR5 = crate::Reg<u32, _ADC_STR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR5;
#[doc = "`read()` method returns [adc_str5::R](adc_str5::R) reader structure"]
impl crate::Readable for ADC_STR5 {}
#[doc = "`write(|w| ..)` method takes [adc_str5::W](adc_str5::W) writer structure"]
impl crate::Writable for ADC_STR5 {}
#[doc = "ADC_STR5"]
pub mod adc_str5;
#[doc = "ADC_STR6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str6](adc_str6) module"]
pub type ADC_STR6 = crate::Reg<u32, _ADC_STR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR6;
#[doc = "`read()` method returns [adc_str6::R](adc_str6::R) reader structure"]
impl crate::Readable for ADC_STR6 {}
#[doc = "`write(|w| ..)` method takes [adc_str6::W](adc_str6::W) writer structure"]
impl crate::Writable for ADC_STR6 {}
#[doc = "ADC_STR6"]
pub mod adc_str6;
#[doc = "ADC_STR7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str7](adc_str7) module"]
pub type ADC_STR7 = crate::Reg<u32, _ADC_STR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR7;
#[doc = "`read()` method returns [adc_str7::R](adc_str7::R) reader structure"]
impl crate::Readable for ADC_STR7 {}
#[doc = "`write(|w| ..)` method takes [adc_str7::W](adc_str7::W) writer structure"]
impl crate::Writable for ADC_STR7 {}
#[doc = "ADC_STR7"]
pub mod adc_str7;
#[doc = "ADC_STR8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str8](adc_str8) module"]
pub type ADC_STR8 = crate::Reg<u32, _ADC_STR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR8;
#[doc = "`read()` method returns [adc_str8::R](adc_str8::R) reader structure"]
impl crate::Readable for ADC_STR8 {}
#[doc = "`write(|w| ..)` method takes [adc_str8::W](adc_str8::W) writer structure"]
impl crate::Writable for ADC_STR8 {}
#[doc = "ADC_STR8"]
pub mod adc_str8;
#[doc = "ADC_STR9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str9](adc_str9) module"]
pub type ADC_STR9 = crate::Reg<u32, _ADC_STR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR9;
#[doc = "`read()` method returns [adc_str9::R](adc_str9::R) reader structure"]
impl crate::Readable for ADC_STR9 {}
#[doc = "`write(|w| ..)` method takes [adc_str9::W](adc_str9::W) writer structure"]
impl crate::Writable for ADC_STR9 {}
#[doc = "ADC_STR9"]
pub mod adc_str9;
#[doc = "ADC_STR10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str10](adc_str10) module"]
pub type ADC_STR10 = crate::Reg<u32, _ADC_STR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR10;
#[doc = "`read()` method returns [adc_str10::R](adc_str10::R) reader structure"]
impl crate::Readable for ADC_STR10 {}
#[doc = "`write(|w| ..)` method takes [adc_str10::W](adc_str10::W) writer structure"]
impl crate::Writable for ADC_STR10 {}
#[doc = "ADC_STR10"]
pub mod adc_str10;
#[doc = "ADC_STR11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str11](adc_str11) module"]
pub type ADC_STR11 = crate::Reg<u32, _ADC_STR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR11;
#[doc = "`read()` method returns [adc_str11::R](adc_str11::R) reader structure"]
impl crate::Readable for ADC_STR11 {}
#[doc = "`write(|w| ..)` method takes [adc_str11::W](adc_str11::W) writer structure"]
impl crate::Writable for ADC_STR11 {}
#[doc = "ADC_STR11"]
pub mod adc_str11;
#[doc = "ADC_STR12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str12](adc_str12) module"]
pub type ADC_STR12 = crate::Reg<u32, _ADC_STR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR12;
#[doc = "`read()` method returns [adc_str12::R](adc_str12::R) reader structure"]
impl crate::Readable for ADC_STR12 {}
#[doc = "`write(|w| ..)` method takes [adc_str12::W](adc_str12::W) writer structure"]
impl crate::Writable for ADC_STR12 {}
#[doc = "ADC_STR12"]
pub mod adc_str12;
#[doc = "ADC_STR13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str13](adc_str13) module"]
pub type ADC_STR13 = crate::Reg<u32, _ADC_STR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR13;
#[doc = "`read()` method returns [adc_str13::R](adc_str13::R) reader structure"]
impl crate::Readable for ADC_STR13 {}
#[doc = "`write(|w| ..)` method takes [adc_str13::W](adc_str13::W) writer structure"]
impl crate::Writable for ADC_STR13 {}
#[doc = "ADC_STR13"]
pub mod adc_str13;
#[doc = "ADC_STR14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str14](adc_str14) module"]
pub type ADC_STR14 = crate::Reg<u32, _ADC_STR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR14;
#[doc = "`read()` method returns [adc_str14::R](adc_str14::R) reader structure"]
impl crate::Readable for ADC_STR14 {}
#[doc = "`write(|w| ..)` method takes [adc_str14::W](adc_str14::W) writer structure"]
impl crate::Writable for ADC_STR14 {}
#[doc = "ADC_STR14"]
pub mod adc_str14;
#[doc = "ADC_STR15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str15](adc_str15) module"]
pub type ADC_STR15 = crate::Reg<u32, _ADC_STR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR15;
#[doc = "`read()` method returns [adc_str15::R](adc_str15::R) reader structure"]
impl crate::Readable for ADC_STR15 {}
#[doc = "`write(|w| ..)` method takes [adc_str15::W](adc_str15::W) writer structure"]
impl crate::Writable for ADC_STR15 {}
#[doc = "ADC_STR15"]
pub mod adc_str15;
#[doc = "ADC_DR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr0](adc_dr0) module"]
pub type ADC_DR0 = crate::Reg<u32, _ADC_DR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR0;
#[doc = "`read()` method returns [adc_dr0::R](adc_dr0::R) reader structure"]
impl crate::Readable for ADC_DR0 {}
#[doc = "`write(|w| ..)` method takes [adc_dr0::W](adc_dr0::W) writer structure"]
impl crate::Writable for ADC_DR0 {}
#[doc = "ADC_DR0"]
pub mod adc_dr0;
#[doc = "ADC_DR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr1](adc_dr1) module"]
pub type ADC_DR1 = crate::Reg<u32, _ADC_DR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR1;
#[doc = "`read()` method returns [adc_dr1::R](adc_dr1::R) reader structure"]
impl crate::Readable for ADC_DR1 {}
#[doc = "`write(|w| ..)` method takes [adc_dr1::W](adc_dr1::W) writer structure"]
impl crate::Writable for ADC_DR1 {}
#[doc = "ADC_DR1"]
pub mod adc_dr1;
#[doc = "ADC_DR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr2](adc_dr2) module"]
pub type ADC_DR2 = crate::Reg<u32, _ADC_DR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR2;
#[doc = "`read()` method returns [adc_dr2::R](adc_dr2::R) reader structure"]
impl crate::Readable for ADC_DR2 {}
#[doc = "`write(|w| ..)` method takes [adc_dr2::W](adc_dr2::W) writer structure"]
impl crate::Writable for ADC_DR2 {}
#[doc = "ADC_DR2"]
pub mod adc_dr2;
#[doc = "ADC_DR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr3](adc_dr3) module"]
pub type ADC_DR3 = crate::Reg<u32, _ADC_DR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR3;
#[doc = "`read()` method returns [adc_dr3::R](adc_dr3::R) reader structure"]
impl crate::Readable for ADC_DR3 {}
#[doc = "`write(|w| ..)` method takes [adc_dr3::W](adc_dr3::W) writer structure"]
impl crate::Writable for ADC_DR3 {}
#[doc = "ADC_DR3"]
pub mod adc_dr3;
#[doc = "ADC_DR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr4](adc_dr4) module"]
pub type ADC_DR4 = crate::Reg<u32, _ADC_DR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR4;
#[doc = "`read()` method returns [adc_dr4::R](adc_dr4::R) reader structure"]
impl crate::Readable for ADC_DR4 {}
#[doc = "`write(|w| ..)` method takes [adc_dr4::W](adc_dr4::W) writer structure"]
impl crate::Writable for ADC_DR4 {}
#[doc = "ADC_DR4"]
pub mod adc_dr4;
#[doc = "ADC_DR5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr5](adc_dr5) module"]
pub type ADC_DR5 = crate::Reg<u32, _ADC_DR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR5;
#[doc = "`read()` method returns [adc_dr5::R](adc_dr5::R) reader structure"]
impl crate::Readable for ADC_DR5 {}
#[doc = "`write(|w| ..)` method takes [adc_dr5::W](adc_dr5::W) writer structure"]
impl crate::Writable for ADC_DR5 {}
#[doc = "ADC_DR5"]
pub mod adc_dr5;
#[doc = "ADC_DR6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr6](adc_dr6) module"]
pub type ADC_DR6 = crate::Reg<u32, _ADC_DR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR6;
#[doc = "`read()` method returns [adc_dr6::R](adc_dr6::R) reader structure"]
impl crate::Readable for ADC_DR6 {}
#[doc = "`write(|w| ..)` method takes [adc_dr6::W](adc_dr6::W) writer structure"]
impl crate::Writable for ADC_DR6 {}
#[doc = "ADC_DR6"]
pub mod adc_dr6;
#[doc = "ADC_DR7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr7](adc_dr7) module"]
pub type ADC_DR7 = crate::Reg<u32, _ADC_DR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR7;
#[doc = "`read()` method returns [adc_dr7::R](adc_dr7::R) reader structure"]
impl crate::Readable for ADC_DR7 {}
#[doc = "`write(|w| ..)` method takes [adc_dr7::W](adc_dr7::W) writer structure"]
impl crate::Writable for ADC_DR7 {}
#[doc = "ADC_DR7"]
pub mod adc_dr7;
#[doc = "ADC_DR8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr8](adc_dr8) module"]
pub type ADC_DR8 = crate::Reg<u32, _ADC_DR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR8;
#[doc = "`read()` method returns [adc_dr8::R](adc_dr8::R) reader structure"]
impl crate::Readable for ADC_DR8 {}
#[doc = "`write(|w| ..)` method takes [adc_dr8::W](adc_dr8::W) writer structure"]
impl crate::Writable for ADC_DR8 {}
#[doc = "ADC_DR8"]
pub mod adc_dr8;
#[doc = "ADC_DR9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr9](adc_dr9) module"]
pub type ADC_DR9 = crate::Reg<u32, _ADC_DR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR9;
#[doc = "`read()` method returns [adc_dr9::R](adc_dr9::R) reader structure"]
impl crate::Readable for ADC_DR9 {}
#[doc = "`write(|w| ..)` method takes [adc_dr9::W](adc_dr9::W) writer structure"]
impl crate::Writable for ADC_DR9 {}
#[doc = "ADC_DR9"]
pub mod adc_dr9;
#[doc = "ADC_DR10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr10](adc_dr10) module"]
pub type ADC_DR10 = crate::Reg<u32, _ADC_DR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR10;
#[doc = "`read()` method returns [adc_dr10::R](adc_dr10::R) reader structure"]
impl crate::Readable for ADC_DR10 {}
#[doc = "`write(|w| ..)` method takes [adc_dr10::W](adc_dr10::W) writer structure"]
impl crate::Writable for ADC_DR10 {}
#[doc = "ADC_DR10"]
pub mod adc_dr10;
#[doc = "ADC_DR11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr11](adc_dr11) module"]
pub type ADC_DR11 = crate::Reg<u32, _ADC_DR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR11;
#[doc = "`read()` method returns [adc_dr11::R](adc_dr11::R) reader structure"]
impl crate::Readable for ADC_DR11 {}
#[doc = "`write(|w| ..)` method takes [adc_dr11::W](adc_dr11::W) writer structure"]
impl crate::Writable for ADC_DR11 {}
#[doc = "ADC_DR11"]
pub mod adc_dr11;
#[doc = "ADC_DR12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr12](adc_dr12) module"]
pub type ADC_DR12 = crate::Reg<u32, _ADC_DR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR12;
#[doc = "`read()` method returns [adc_dr12::R](adc_dr12::R) reader structure"]
impl crate::Readable for ADC_DR12 {}
#[doc = "`write(|w| ..)` method takes [adc_dr12::W](adc_dr12::W) writer structure"]
impl crate::Writable for ADC_DR12 {}
#[doc = "ADC_DR12"]
pub mod adc_dr12;
#[doc = "ADC_DR13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr13](adc_dr13) module"]
pub type ADC_DR13 = crate::Reg<u32, _ADC_DR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR13;
#[doc = "`read()` method returns [adc_dr13::R](adc_dr13::R) reader structure"]
impl crate::Readable for ADC_DR13 {}
#[doc = "`write(|w| ..)` method takes [adc_dr13::W](adc_dr13::W) writer structure"]
impl crate::Writable for ADC_DR13 {}
#[doc = "ADC_DR13"]
pub mod adc_dr13;
#[doc = "ADC_DR14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr14](adc_dr14) module"]
pub type ADC_DR14 = crate::Reg<u32, _ADC_DR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR14;
#[doc = "`read()` method returns [adc_dr14::R](adc_dr14::R) reader structure"]
impl crate::Readable for ADC_DR14 {}
#[doc = "`write(|w| ..)` method takes [adc_dr14::W](adc_dr14::W) writer structure"]
impl crate::Writable for ADC_DR14 {}
#[doc = "ADC_DR14"]
pub mod adc_dr14;
#[doc = "ADC_DR15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr15](adc_dr15) module"]
pub type ADC_DR15 = crate::Reg<u32, _ADC_DR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR15;
#[doc = "`read()` method returns [adc_dr15::R](adc_dr15::R) reader structure"]
impl crate::Readable for ADC_DR15 {}
#[doc = "`write(|w| ..)` method takes [adc_dr15::W](adc_dr15::W) writer structure"]
impl crate::Writable for ADC_DR15 {}
#[doc = "ADC_DR15"]
pub mod adc_dr15;
#[doc = "ADC_HDR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_hdr0](adc_hdr0) module"]
pub type ADC_HDR0 = crate::Reg<u32, _ADC_HDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_HDR0;
#[doc = "`read()` method returns [adc_hdr0::R](adc_hdr0::R) reader structure"]
impl crate::Readable for ADC_HDR0 {}
#[doc = "`write(|w| ..)` method takes [adc_hdr0::W](adc_hdr0::W) writer structure"]
impl crate::Writable for ADC_HDR0 {}
#[doc = "ADC_HDR0"]
pub mod adc_hdr0;
#[doc = "ADC_HDR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_hdr1](adc_hdr1) module"]
pub type ADC_HDR1 = crate::Reg<u32, _ADC_HDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_HDR1;
#[doc = "`read()` method returns [adc_hdr1::R](adc_hdr1::R) reader structure"]
impl crate::Readable for ADC_HDR1 {}
#[doc = "`write(|w| ..)` method takes [adc_hdr1::W](adc_hdr1::W) writer structure"]
impl crate::Writable for ADC_HDR1 {}
#[doc = "ADC_HDR1"]
pub mod adc_hdr1;
#[doc = "ADC_HDR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_hdr2](adc_hdr2) module"]
pub type ADC_HDR2 = crate::Reg<u32, _ADC_HDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_HDR2;
#[doc = "`read()` method returns [adc_hdr2::R](adc_hdr2::R) reader structure"]
impl crate::Readable for ADC_HDR2 {}
#[doc = "`write(|w| ..)` method takes [adc_hdr2::W](adc_hdr2::W) writer structure"]
impl crate::Writable for ADC_HDR2 {}
#[doc = "ADC_HDR2"]
pub mod adc_hdr2;
#[doc = "ADC_HDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_hdr3](adc_hdr3) module"]
pub type ADC_HDR3 = crate::Reg<u32, _ADC_HDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_HDR3;
#[doc = "`read()` method returns [adc_hdr3::R](adc_hdr3::R) reader structure"]
impl crate::Readable for ADC_HDR3 {}
#[doc = "`write(|w| ..)` method takes [adc_hdr3::W](adc_hdr3::W) writer structure"]
impl crate::Writable for ADC_HDR3 {}
#[doc = "ADC_HDR3"]
pub mod adc_hdr3;
#[doc = "ADC_TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_tcr](adc_tcr) module"]
pub type ADC_TCR = crate::Reg<u32, _ADC_TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_TCR;
#[doc = "`read()` method returns [adc_tcr::R](adc_tcr::R) reader structure"]
impl crate::Readable for ADC_TCR {}
#[doc = "`write(|w| ..)` method takes [adc_tcr::W](adc_tcr::W) writer structure"]
impl crate::Writable for ADC_TCR {}
#[doc = "ADC_TCR"]
pub mod adc_tcr;
#[doc = "ADC_TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_tsr](adc_tsr) module"]
pub type ADC_TSR = crate::Reg<u32, _ADC_TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_TSR;
#[doc = "`read()` method returns [adc_tsr::R](adc_tsr::R) reader structure"]
impl crate::Readable for ADC_TSR {}
#[doc = "`write(|w| ..)` method takes [adc_tsr::W](adc_tsr::W) writer structure"]
impl crate::Writable for ADC_TSR {}
#[doc = "ADC_TSR"]
pub mod adc_tsr;
#[doc = "ADC_HTCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_htcr](adc_htcr) module"]
pub type ADC_HTCR = crate::Reg<u32, _ADC_HTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_HTCR;
#[doc = "`read()` method returns [adc_htcr::R](adc_htcr::R) reader structure"]
impl crate::Readable for ADC_HTCR {}
#[doc = "`write(|w| ..)` method takes [adc_htcr::W](adc_htcr::W) writer structure"]
impl crate::Writable for ADC_HTCR {}
#[doc = "ADC_HTCR"]
pub mod adc_htcr;
#[doc = "ADC_HTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_htsr](adc_htsr) module"]
pub type ADC_HTSR = crate::Reg<u32, _ADC_HTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_HTSR;
#[doc = "`read()` method returns [adc_htsr::R](adc_htsr::R) reader structure"]
impl crate::Readable for ADC_HTSR {}
#[doc = "`write(|w| ..)` method takes [adc_htsr::W](adc_htsr::W) writer structure"]
impl crate::Writable for ADC_HTSR {}
#[doc = "ADC_HTSR"]
pub mod adc_htsr;
#[doc = "ADC_WCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_wcr](adc_wcr) module"]
pub type ADC_WCR = crate::Reg<u32, _ADC_WCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_WCR;
#[doc = "`read()` method returns [adc_wcr::R](adc_wcr::R) reader structure"]
impl crate::Readable for ADC_WCR {}
#[doc = "`write(|w| ..)` method takes [adc_wcr::W](adc_wcr::W) writer structure"]
impl crate::Writable for ADC_WCR {}
#[doc = "ADC_WCR"]
pub mod adc_wcr;
#[doc = "ADC_LTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ltr](adc_ltr) module"]
pub type ADC_LTR = crate::Reg<u32, _ADC_LTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_LTR;
#[doc = "`read()` method returns [adc_ltr::R](adc_ltr::R) reader structure"]
impl crate::Readable for ADC_LTR {}
#[doc = "`write(|w| ..)` method takes [adc_ltr::W](adc_ltr::W) writer structure"]
impl crate::Writable for ADC_LTR {}
#[doc = "ADC_LTR"]
pub mod adc_ltr;
#[doc = "ADC_UTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_utr](adc_utr) module"]
pub type ADC_UTR = crate::Reg<u32, _ADC_UTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_UTR;
#[doc = "`read()` method returns [adc_utr::R](adc_utr::R) reader structure"]
impl crate::Readable for ADC_UTR {}
#[doc = "`write(|w| ..)` method takes [adc_utr::W](adc_utr::W) writer structure"]
impl crate::Writable for ADC_UTR {}
#[doc = "ADC_UTR"]
pub mod adc_utr;
#[doc = "ADC_IMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_imr](adc_imr) module"]
pub type ADC_IMR = crate::Reg<u32, _ADC_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_IMR;
#[doc = "`read()` method returns [adc_imr::R](adc_imr::R) reader structure"]
impl crate::Readable for ADC_IMR {}
#[doc = "`write(|w| ..)` method takes [adc_imr::W](adc_imr::W) writer structure"]
impl crate::Writable for ADC_IMR {}
#[doc = "ADC_IMR"]
pub mod adc_imr;
#[doc = "ADC_IRAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_iraw](adc_iraw) module"]
pub type ADC_IRAW = crate::Reg<u32, _ADC_IRAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_IRAW;
#[doc = "`read()` method returns [adc_iraw::R](adc_iraw::R) reader structure"]
impl crate::Readable for ADC_IRAW {}
#[doc = "`write(|w| ..)` method takes [adc_iraw::W](adc_iraw::W) writer structure"]
impl crate::Writable for ADC_IRAW {}
#[doc = "ADC_IRAW"]
pub mod adc_iraw;
#[doc = "ADC_IMASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_imask](adc_imask) module"]
pub type ADC_IMASK = crate::Reg<u32, _ADC_IMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_IMASK;
#[doc = "`read()` method returns [adc_imask::R](adc_imask::R) reader structure"]
impl crate::Readable for ADC_IMASK {}
#[doc = "`write(|w| ..)` method takes [adc_imask::W](adc_imask::W) writer structure"]
impl crate::Writable for ADC_IMASK {}
#[doc = "ADC_IMASK"]
pub mod adc_imask;
#[doc = "ADC_ICLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_iclr](adc_iclr) module"]
pub type ADC_ICLR = crate::Reg<u32, _ADC_ICLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_ICLR;
#[doc = "`read()` method returns [adc_iclr::R](adc_iclr::R) reader structure"]
impl crate::Readable for ADC_ICLR {}
#[doc = "`write(|w| ..)` method takes [adc_iclr::W](adc_iclr::W) writer structure"]
impl crate::Writable for ADC_ICLR {}
#[doc = "ADC_ICLR"]
pub mod adc_iclr;
#[doc = "ADC_DMAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dmar](adc_dmar) module"]
pub type ADC_DMAR = crate::Reg<u32, _ADC_DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DMAR;
#[doc = "`read()` method returns [adc_dmar::R](adc_dmar::R) reader structure"]
impl crate::Readable for ADC_DMAR {}
#[doc = "`write(|w| ..)` method takes [adc_dmar::W](adc_dmar::W) writer structure"]
impl crate::Writable for ADC_DMAR {}
#[doc = "ADC_DMAR"]
pub mod adc_dmar;
