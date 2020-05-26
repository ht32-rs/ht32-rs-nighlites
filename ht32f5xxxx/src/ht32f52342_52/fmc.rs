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
    _reserved12: [u8; 124usize],
    #[doc = "0x180 - FMC_MDID"]
    pub fmc_mdid: FMC_MDID,
    #[doc = "0x184 - FMC_PNSR"]
    pub fmc_pnsr: FMC_PNSR,
    #[doc = "0x188 - FMC_PSSR"]
    pub fmc_pssr: FMC_PSSR,
    _reserved15: [u8; 116usize],
    #[doc = "0x200 - FMC_CFCR"]
    pub fmc_cfcr: FMC_CFCR,
    _reserved16: [u8; 268usize],
    #[doc = "0x310 - FMC_CID0"]
    pub fmc_cid0: FMC_CID0,
    #[doc = "0x314 - FMC_CID1"]
    pub fmc_cid1: FMC_CID1,
    #[doc = "0x318 - FMC_CID2"]
    pub fmc_cid2: FMC_CID2,
    #[doc = "0x31c - FMC_CID3"]
    pub fmc_cid3: FMC_CID3,
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
#[doc = "FMC_MDID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_mdid](fmc_mdid) module"]
pub type FMC_MDID = crate::Reg<u32, _FMC_MDID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_MDID;
#[doc = "`read()` method returns [fmc_mdid::R](fmc_mdid::R) reader structure"]
impl crate::Readable for FMC_MDID {}
#[doc = "`write(|w| ..)` method takes [fmc_mdid::W](fmc_mdid::W) writer structure"]
impl crate::Writable for FMC_MDID {}
#[doc = "FMC_MDID"]
pub mod fmc_mdid;
#[doc = "FMC_PNSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_pnsr](fmc_pnsr) module"]
pub type FMC_PNSR = crate::Reg<u32, _FMC_PNSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_PNSR;
#[doc = "`read()` method returns [fmc_pnsr::R](fmc_pnsr::R) reader structure"]
impl crate::Readable for FMC_PNSR {}
#[doc = "`write(|w| ..)` method takes [fmc_pnsr::W](fmc_pnsr::W) writer structure"]
impl crate::Writable for FMC_PNSR {}
#[doc = "FMC_PNSR"]
pub mod fmc_pnsr;
#[doc = "FMC_PSSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_pssr](fmc_pssr) module"]
pub type FMC_PSSR = crate::Reg<u32, _FMC_PSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_PSSR;
#[doc = "`read()` method returns [fmc_pssr::R](fmc_pssr::R) reader structure"]
impl crate::Readable for FMC_PSSR {}
#[doc = "`write(|w| ..)` method takes [fmc_pssr::W](fmc_pssr::W) writer structure"]
impl crate::Writable for FMC_PSSR {}
#[doc = "FMC_PSSR"]
pub mod fmc_pssr;
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
#[doc = "FMC_CID0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_cid0](fmc_cid0) module"]
pub type FMC_CID0 = crate::Reg<u32, _FMC_CID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_CID0;
#[doc = "`read()` method returns [fmc_cid0::R](fmc_cid0::R) reader structure"]
impl crate::Readable for FMC_CID0 {}
#[doc = "`write(|w| ..)` method takes [fmc_cid0::W](fmc_cid0::W) writer structure"]
impl crate::Writable for FMC_CID0 {}
#[doc = "FMC_CID0"]
pub mod fmc_cid0;
#[doc = "FMC_CID1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_cid1](fmc_cid1) module"]
pub type FMC_CID1 = crate::Reg<u32, _FMC_CID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_CID1;
#[doc = "`read()` method returns [fmc_cid1::R](fmc_cid1::R) reader structure"]
impl crate::Readable for FMC_CID1 {}
#[doc = "`write(|w| ..)` method takes [fmc_cid1::W](fmc_cid1::W) writer structure"]
impl crate::Writable for FMC_CID1 {}
#[doc = "FMC_CID1"]
pub mod fmc_cid1;
#[doc = "FMC_CID2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_cid2](fmc_cid2) module"]
pub type FMC_CID2 = crate::Reg<u32, _FMC_CID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_CID2;
#[doc = "`read()` method returns [fmc_cid2::R](fmc_cid2::R) reader structure"]
impl crate::Readable for FMC_CID2 {}
#[doc = "`write(|w| ..)` method takes [fmc_cid2::W](fmc_cid2::W) writer structure"]
impl crate::Writable for FMC_CID2 {}
#[doc = "FMC_CID2"]
pub mod fmc_cid2;
#[doc = "FMC_CID3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_cid3](fmc_cid3) module"]
pub type FMC_CID3 = crate::Reg<u32, _FMC_CID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_CID3;
#[doc = "`read()` method returns [fmc_cid3::R](fmc_cid3::R) reader structure"]
impl crate::Readable for FMC_CID3 {}
#[doc = "`write(|w| ..)` method takes [fmc_cid3::W](fmc_cid3::W) writer structure"]
impl crate::Writable for FMC_CID3 {}
#[doc = "FMC_CID3"]
pub mod fmc_cid3;
