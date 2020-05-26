#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCI_CR"]
    pub sci_cr: SCI_CR,
    #[doc = "0x04 - SCI_SR"]
    pub sci_sr: SCI_SR,
    #[doc = "0x08 - SCI_CCR"]
    pub sci_ccr: SCI_CCR,
    #[doc = "0x0c - SCI_ETU"]
    pub sci_etu: SCI_ETU,
    #[doc = "0x10 - SCI_GT"]
    pub sci_gt: SCI_GT,
    #[doc = "0x14 - SCI_WT"]
    pub sci_wt: SCI_WT,
    #[doc = "0x18 - SCI_IER"]
    pub sci_ier: SCI_IER,
    #[doc = "0x1c - SCI_IPR"]
    pub sci_ipr: SCI_IPR,
    #[doc = "0x20 - SCI_TXB"]
    pub sci_txb: SCI_TXB,
    #[doc = "0x24 - SCI_RXB"]
    pub sci_rxb: SCI_RXB,
    #[doc = "0x28 - SCI_PSC"]
    pub sci_psc: SCI_PSC,
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
#[doc = "SCI_ETU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_etu](sci_etu) module"]
pub type SCI_ETU = crate::Reg<u32, _SCI_ETU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCI_ETU;
#[doc = "`read()` method returns [sci_etu::R](sci_etu::R) reader structure"]
impl crate::Readable for SCI_ETU {}
#[doc = "`write(|w| ..)` method takes [sci_etu::W](sci_etu::W) writer structure"]
impl crate::Writable for SCI_ETU {}
#[doc = "SCI_ETU"]
pub mod sci_etu;
#[doc = "SCI_GT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_gt](sci_gt) module"]
pub type SCI_GT = crate::Reg<u32, _SCI_GT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCI_GT;
#[doc = "`read()` method returns [sci_gt::R](sci_gt::R) reader structure"]
impl crate::Readable for SCI_GT {}
#[doc = "`write(|w| ..)` method takes [sci_gt::W](sci_gt::W) writer structure"]
impl crate::Writable for SCI_GT {}
#[doc = "SCI_GT"]
pub mod sci_gt;
#[doc = "SCI_WT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_wt](sci_wt) module"]
pub type SCI_WT = crate::Reg<u32, _SCI_WT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCI_WT;
#[doc = "`read()` method returns [sci_wt::R](sci_wt::R) reader structure"]
impl crate::Readable for SCI_WT {}
#[doc = "`write(|w| ..)` method takes [sci_wt::W](sci_wt::W) writer structure"]
impl crate::Writable for SCI_WT {}
#[doc = "SCI_WT"]
pub mod sci_wt;
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
#[doc = "SCI_PSC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_psc](sci_psc) module"]
pub type SCI_PSC = crate::Reg<u32, _SCI_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCI_PSC;
#[doc = "`read()` method returns [sci_psc::R](sci_psc::R) reader structure"]
impl crate::Readable for SCI_PSC {}
#[doc = "`write(|w| ..)` method takes [sci_psc::W](sci_psc::W) writer structure"]
impl crate::Writable for SCI_PSC {}
#[doc = "SCI_PSC"]
pub mod sci_psc;
