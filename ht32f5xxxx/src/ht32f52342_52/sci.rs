#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCI_CR"]
    pub sci_cr: SCI_CR,
    #[doc = "0x04 - SCI_SR"]
    pub sci_sr: SCI_SR,
    #[doc = "0x08 - SCI_CCR"]
    pub sci_ccr: SCI_CCR,
    #[doc = "0x0c - SCI_ETUR"]
    pub sci_etur: SCI_ETUR,
    #[doc = "0x10 - SCI_GTR"]
    pub sci_gtr: SCI_GTR,
    #[doc = "0x14 - SCI_WTR"]
    pub sci_wtr: SCI_WTR,
    #[doc = "0x18 - SCI_IER"]
    pub sci_ier: SCI_IER,
    #[doc = "0x1c - SCI_IPR"]
    pub sci_ipr: SCI_IPR,
    #[doc = "0x20 - SCI_TXB"]
    pub sci_txb: SCI_TXB,
    #[doc = "0x24 - SCI_RXB"]
    pub sci_rxb: SCI_RXB,
    #[doc = "0x28 - SCI_PSCR"]
    pub sci_pscr: SCI_PSCR,
}
#[doc = "SCI_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_cr](sci_cr) module"]
pub type SCI_CR = crate::Reg<u32, _SCI_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCI_CR;
#[doc = "`read()` method returns [sci_cr::R](sci_cr::R) reader structure"]
impl crate::Readable for SCI_CR {}
#[doc = "`write(|w| ..)` method takes [sci_cr::W](sci_cr::W) writer structure"]
impl crate::Writable for SCI_CR {}
#[doc = "SCI_CR"]
pub mod sci_cr;
#[doc = "SCI_SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_sr](sci_sr) module"]
pub type SCI_SR = crate::Reg<u32, _SCI_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCI_SR;
#[doc = "`read()` method returns [sci_sr::R](sci_sr::R) reader structure"]
impl crate::Readable for SCI_SR {}
#[doc = "`write(|w| ..)` method takes [sci_sr::W](sci_sr::W) writer structure"]
impl crate::Writable for SCI_SR {}
#[doc = "SCI_SR"]
pub mod sci_sr;
#[doc = "SCI_CCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_ccr](sci_ccr) module"]
pub type SCI_CCR = crate::Reg<u32, _SCI_CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCI_CCR;
#[doc = "`read()` method returns [sci_ccr::R](sci_ccr::R) reader structure"]
impl crate::Readable for SCI_CCR {}
#[doc = "`write(|w| ..)` method takes [sci_ccr::W](sci_ccr::W) writer structure"]
impl crate::Writable for SCI_CCR {}
#[doc = "SCI_CCR"]
pub mod sci_ccr;
#[doc = "SCI_ETUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_etur](sci_etur) module"]
pub type SCI_ETUR = crate::Reg<u32, _SCI_ETUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCI_ETUR;
#[doc = "`read()` method returns [sci_etur::R](sci_etur::R) reader structure"]
impl crate::Readable for SCI_ETUR {}
#[doc = "`write(|w| ..)` method takes [sci_etur::W](sci_etur::W) writer structure"]
impl crate::Writable for SCI_ETUR {}
#[doc = "SCI_ETUR"]
pub mod sci_etur;
#[doc = "SCI_GTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_gtr](sci_gtr) module"]
pub type SCI_GTR = crate::Reg<u32, _SCI_GTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCI_GTR;
#[doc = "`read()` method returns [sci_gtr::R](sci_gtr::R) reader structure"]
impl crate::Readable for SCI_GTR {}
#[doc = "`write(|w| ..)` method takes [sci_gtr::W](sci_gtr::W) writer structure"]
impl crate::Writable for SCI_GTR {}
#[doc = "SCI_GTR"]
pub mod sci_gtr;
#[doc = "SCI_WTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_wtr](sci_wtr) module"]
pub type SCI_WTR = crate::Reg<u32, _SCI_WTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCI_WTR;
#[doc = "`read()` method returns [sci_wtr::R](sci_wtr::R) reader structure"]
impl crate::Readable for SCI_WTR {}
#[doc = "`write(|w| ..)` method takes [sci_wtr::W](sci_wtr::W) writer structure"]
impl crate::Writable for SCI_WTR {}
#[doc = "SCI_WTR"]
pub mod sci_wtr;
#[doc = "SCI_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_ier](sci_ier) module"]
pub type SCI_IER = crate::Reg<u32, _SCI_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCI_IER;
#[doc = "`read()` method returns [sci_ier::R](sci_ier::R) reader structure"]
impl crate::Readable for SCI_IER {}
#[doc = "`write(|w| ..)` method takes [sci_ier::W](sci_ier::W) writer structure"]
impl crate::Writable for SCI_IER {}
#[doc = "SCI_IER"]
pub mod sci_ier;
#[doc = "SCI_IPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_ipr](sci_ipr) module"]
pub type SCI_IPR = crate::Reg<u32, _SCI_IPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCI_IPR;
#[doc = "`read()` method returns [sci_ipr::R](sci_ipr::R) reader structure"]
impl crate::Readable for SCI_IPR {}
#[doc = "`write(|w| ..)` method takes [sci_ipr::W](sci_ipr::W) writer structure"]
impl crate::Writable for SCI_IPR {}
#[doc = "SCI_IPR"]
pub mod sci_ipr;
#[doc = "SCI_TXB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_txb](sci_txb) module"]
pub type SCI_TXB = crate::Reg<u32, _SCI_TXB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCI_TXB;
#[doc = "`read()` method returns [sci_txb::R](sci_txb::R) reader structure"]
impl crate::Readable for SCI_TXB {}
#[doc = "`write(|w| ..)` method takes [sci_txb::W](sci_txb::W) writer structure"]
impl crate::Writable for SCI_TXB {}
#[doc = "SCI_TXB"]
pub mod sci_txb;
#[doc = "SCI_RXB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_rxb](sci_rxb) module"]
pub type SCI_RXB = crate::Reg<u32, _SCI_RXB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCI_RXB;
#[doc = "`read()` method returns [sci_rxb::R](sci_rxb::R) reader structure"]
impl crate::Readable for SCI_RXB {}
#[doc = "`write(|w| ..)` method takes [sci_rxb::W](sci_rxb::W) writer structure"]
impl crate::Writable for SCI_RXB {}
#[doc = "SCI_RXB"]
pub mod sci_rxb;
#[doc = "SCI_PSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_pscr](sci_pscr) module"]
pub type SCI_PSCR = crate::Reg<u32, _SCI_PSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCI_PSCR;
#[doc = "`read()` method returns [sci_pscr::R](sci_pscr::R) reader structure"]
impl crate::Readable for SCI_PSCR {}
#[doc = "`write(|w| ..)` method takes [sci_pscr::W](sci_pscr::W) writer structure"]
impl crate::Writable for SCI_PSCR {}
#[doc = "SCI_PSCR"]
pub mod sci_pscr;
