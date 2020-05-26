#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FMC_TADR"]
    pub fmc_tadr: FMC_TADR,
    #[doc = "0x04 - FMC_WRDR"]
    pub fmc_wrdr: FMC_WRDR,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - FMC_OCMR"]
    pub fmc_ocmr: FMC_OCMR,
    #[doc = "0x10 - FMC_OPCR"]
    pub fmc_opcr: FMC_OPCR,
    #[doc = "0x14 - FMC_OIER"]
    pub fmc_oier: FMC_OIER,
    #[doc = "0x18 - FMC_OISR"]
    pub fmc_oisr: FMC_OISR,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - FMC_PPSR0"]
    pub fmc_ppsr0: FMC_PPSR0,
    #[doc = "0x24 - FMC_PPSR1"]
    pub fmc_ppsr1: FMC_PPSR1,
    #[doc = "0x28 - FMC_PPSR2"]
    pub fmc_ppsr2: FMC_PPSR2,
    #[doc = "0x2c - FMC_PPSR3"]
    pub fmc_ppsr3: FMC_PPSR3,
    #[doc = "0x30 - FMC_CPSR"]
    pub fmc_cpsr: FMC_CPSR,
    _reserved11: [u8; 204usize],
    #[doc = "0x100 - FMC_VMCR"]
    pub fmc_vmcr: FMC_VMCR,
    _reserved12: [u8; 252usize],
    #[doc = "0x200 - FMC_CFCR"]
    pub fmc_cfcr: FMC_CFCR,
    _reserved13: [u8; 252usize],
    #[doc = "0x300 - FMC_SBVT0"]
    pub fmc_sbvt0: FMC_SBVT0,
    #[doc = "0x304 - FMC_SBVT1"]
    pub fmc_sbvt1: FMC_SBVT1,
    #[doc = "0x308 - FMC_SBVT2"]
    pub fmc_sbvt2: FMC_SBVT2,
    #[doc = "0x30c - FMC_SBVT3"]
    pub fmc_sbvt3: FMC_SBVT3,
}
#[doc = "FMC_TADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_tadr](fmc_tadr) module"]
pub type FMC_TADR = crate::Reg<u32, _FMC_TADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_TADR;
#[doc = "`read()` method returns [fmc_tadr::R](fmc_tadr::R) reader structure"]
impl crate::Readable for FMC_TADR {}
#[doc = "`write(|w| ..)` method takes [fmc_tadr::W](fmc_tadr::W) writer structure"]
impl crate::Writable for FMC_TADR {}
#[doc = "FMC_TADR"]
pub mod fmc_tadr;
#[doc = "FMC_WRDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_wrdr](fmc_wrdr) module"]
pub type FMC_WRDR = crate::Reg<u32, _FMC_WRDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_WRDR;
#[doc = "`read()` method returns [fmc_wrdr::R](fmc_wrdr::R) reader structure"]
impl crate::Readable for FMC_WRDR {}
#[doc = "`write(|w| ..)` method takes [fmc_wrdr::W](fmc_wrdr::W) writer structure"]
impl crate::Writable for FMC_WRDR {}
#[doc = "FMC_WRDR"]
pub mod fmc_wrdr;
#[doc = "FMC_OCMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_ocmr](fmc_ocmr) module"]
pub type FMC_OCMR = crate::Reg<u32, _FMC_OCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_OCMR;
#[doc = "`read()` method returns [fmc_ocmr::R](fmc_ocmr::R) reader structure"]
impl crate::Readable for FMC_OCMR {}
#[doc = "`write(|w| ..)` method takes [fmc_ocmr::W](fmc_ocmr::W) writer structure"]
impl crate::Writable for FMC_OCMR {}
#[doc = "FMC_OCMR"]
pub mod fmc_ocmr;
#[doc = "FMC_OPCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_opcr](fmc_opcr) module"]
pub type FMC_OPCR = crate::Reg<u32, _FMC_OPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_OPCR;
#[doc = "`read()` method returns [fmc_opcr::R](fmc_opcr::R) reader structure"]
impl crate::Readable for FMC_OPCR {}
#[doc = "`write(|w| ..)` method takes [fmc_opcr::W](fmc_opcr::W) writer structure"]
impl crate::Writable for FMC_OPCR {}
#[doc = "FMC_OPCR"]
pub mod fmc_opcr;
#[doc = "FMC_OIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_oier](fmc_oier) module"]
pub type FMC_OIER = crate::Reg<u32, _FMC_OIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_OIER;
#[doc = "`read()` method returns [fmc_oier::R](fmc_oier::R) reader structure"]
impl crate::Readable for FMC_OIER {}
#[doc = "`write(|w| ..)` method takes [fmc_oier::W](fmc_oier::W) writer structure"]
impl crate::Writable for FMC_OIER {}
#[doc = "FMC_OIER"]
pub mod fmc_oier;
#[doc = "FMC_OISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_oisr](fmc_oisr) module"]
pub type FMC_OISR = crate::Reg<u32, _FMC_OISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_OISR;
#[doc = "`read()` method returns [fmc_oisr::R](fmc_oisr::R) reader structure"]
impl crate::Readable for FMC_OISR {}
#[doc = "`write(|w| ..)` method takes [fmc_oisr::W](fmc_oisr::W) writer structure"]
impl crate::Writable for FMC_OISR {}
#[doc = "FMC_OISR"]
pub mod fmc_oisr;
#[doc = "FMC_PPSR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_ppsr0](fmc_ppsr0) module"]
pub type FMC_PPSR0 = crate::Reg<u32, _FMC_PPSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_PPSR0;
#[doc = "`read()` method returns [fmc_ppsr0::R](fmc_ppsr0::R) reader structure"]
impl crate::Readable for FMC_PPSR0 {}
#[doc = "`write(|w| ..)` method takes [fmc_ppsr0::W](fmc_ppsr0::W) writer structure"]
impl crate::Writable for FMC_PPSR0 {}
#[doc = "FMC_PPSR0"]
pub mod fmc_ppsr0;
#[doc = "FMC_PPSR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_ppsr1](fmc_ppsr1) module"]
pub type FMC_PPSR1 = crate::Reg<u32, _FMC_PPSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_PPSR1;
#[doc = "`read()` method returns [fmc_ppsr1::R](fmc_ppsr1::R) reader structure"]
impl crate::Readable for FMC_PPSR1 {}
#[doc = "`write(|w| ..)` method takes [fmc_ppsr1::W](fmc_ppsr1::W) writer structure"]
impl crate::Writable for FMC_PPSR1 {}
#[doc = "FMC_PPSR1"]
pub mod fmc_ppsr1;
#[doc = "FMC_PPSR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_ppsr2](fmc_ppsr2) module"]
pub type FMC_PPSR2 = crate::Reg<u32, _FMC_PPSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_PPSR2;
#[doc = "`read()` method returns [fmc_ppsr2::R](fmc_ppsr2::R) reader structure"]
impl crate::Readable for FMC_PPSR2 {}
#[doc = "`write(|w| ..)` method takes [fmc_ppsr2::W](fmc_ppsr2::W) writer structure"]
impl crate::Writable for FMC_PPSR2 {}
#[doc = "FMC_PPSR2"]
pub mod fmc_ppsr2;
#[doc = "FMC_PPSR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_ppsr3](fmc_ppsr3) module"]
pub type FMC_PPSR3 = crate::Reg<u32, _FMC_PPSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_PPSR3;
#[doc = "`read()` method returns [fmc_ppsr3::R](fmc_ppsr3::R) reader structure"]
impl crate::Readable for FMC_PPSR3 {}
#[doc = "`write(|w| ..)` method takes [fmc_ppsr3::W](fmc_ppsr3::W) writer structure"]
impl crate::Writable for FMC_PPSR3 {}
#[doc = "FMC_PPSR3"]
pub mod fmc_ppsr3;
#[doc = "FMC_CPSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_cpsr](fmc_cpsr) module"]
pub type FMC_CPSR = crate::Reg<u32, _FMC_CPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_CPSR;
#[doc = "`read()` method returns [fmc_cpsr::R](fmc_cpsr::R) reader structure"]
impl crate::Readable for FMC_CPSR {}
#[doc = "`write(|w| ..)` method takes [fmc_cpsr::W](fmc_cpsr::W) writer structure"]
impl crate::Writable for FMC_CPSR {}
#[doc = "FMC_CPSR"]
pub mod fmc_cpsr;
#[doc = "FMC_VMCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_vmcr](fmc_vmcr) module"]
pub type FMC_VMCR = crate::Reg<u32, _FMC_VMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_VMCR;
#[doc = "`read()` method returns [fmc_vmcr::R](fmc_vmcr::R) reader structure"]
impl crate::Readable for FMC_VMCR {}
#[doc = "`write(|w| ..)` method takes [fmc_vmcr::W](fmc_vmcr::W) writer structure"]
impl crate::Writable for FMC_VMCR {}
#[doc = "FMC_VMCR"]
pub mod fmc_vmcr;
#[doc = "FMC_CFCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_cfcr](fmc_cfcr) module"]
pub type FMC_CFCR = crate::Reg<u32, _FMC_CFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_CFCR;
#[doc = "`read()` method returns [fmc_cfcr::R](fmc_cfcr::R) reader structure"]
impl crate::Readable for FMC_CFCR {}
#[doc = "`write(|w| ..)` method takes [fmc_cfcr::W](fmc_cfcr::W) writer structure"]
impl crate::Writable for FMC_CFCR {}
#[doc = "FMC_CFCR"]
pub mod fmc_cfcr;
#[doc = "FMC_SBVT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_sbvt0](fmc_sbvt0) module"]
pub type FMC_SBVT0 = crate::Reg<u32, _FMC_SBVT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_SBVT0;
#[doc = "`read()` method returns [fmc_sbvt0::R](fmc_sbvt0::R) reader structure"]
impl crate::Readable for FMC_SBVT0 {}
#[doc = "`write(|w| ..)` method takes [fmc_sbvt0::W](fmc_sbvt0::W) writer structure"]
impl crate::Writable for FMC_SBVT0 {}
#[doc = "FMC_SBVT0"]
pub mod fmc_sbvt0;
#[doc = "FMC_SBVT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_sbvt1](fmc_sbvt1) module"]
pub type FMC_SBVT1 = crate::Reg<u32, _FMC_SBVT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_SBVT1;
#[doc = "`read()` method returns [fmc_sbvt1::R](fmc_sbvt1::R) reader structure"]
impl crate::Readable for FMC_SBVT1 {}
#[doc = "`write(|w| ..)` method takes [fmc_sbvt1::W](fmc_sbvt1::W) writer structure"]
impl crate::Writable for FMC_SBVT1 {}
#[doc = "FMC_SBVT1"]
pub mod fmc_sbvt1;
#[doc = "FMC_SBVT2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_sbvt2](fmc_sbvt2) module"]
pub type FMC_SBVT2 = crate::Reg<u32, _FMC_SBVT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_SBVT2;
#[doc = "`read()` method returns [fmc_sbvt2::R](fmc_sbvt2::R) reader structure"]
impl crate::Readable for FMC_SBVT2 {}
#[doc = "`write(|w| ..)` method takes [fmc_sbvt2::W](fmc_sbvt2::W) writer structure"]
impl crate::Writable for FMC_SBVT2 {}
#[doc = "FMC_SBVT2"]
pub mod fmc_sbvt2;
#[doc = "FMC_SBVT3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_sbvt3](fmc_sbvt3) module"]
pub type FMC_SBVT3 = crate::Reg<u32, _FMC_SBVT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_SBVT3;
#[doc = "`read()` method returns [fmc_sbvt3::R](fmc_sbvt3::R) reader structure"]
impl crate::Readable for FMC_SBVT3 {}
#[doc = "`write(|w| ..)` method takes [fmc_sbvt3::W](fmc_sbvt3::W) writer structure"]
impl crate::Writable for FMC_SBVT3 {}
#[doc = "FMC_SBVT3"]
pub mod fmc_sbvt3;
