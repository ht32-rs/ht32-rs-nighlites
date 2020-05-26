#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI_CR0"]
    pub spi_cr0: SPI_CR0,
    #[doc = "0x04 - SPI_CR1"]
    pub spi_cr1: SPI_CR1,
    #[doc = "0x08 - SPI_IER"]
    pub spi_ier: SPI_IER,
    #[doc = "0x0c - SPI_CPR"]
    pub spi_cpr: SPI_CPR,
    #[doc = "0x10 - SPI_DR"]
    pub spi_dr: SPI_DR,
    #[doc = "0x14 - SPI_SR"]
    pub spi_sr: SPI_SR,
    #[doc = "0x18 - SPI_FCR"]
    pub spi_fcr: SPI_FCR,
    #[doc = "0x1c - SPI_FSR"]
    pub spi_fsr: SPI_FSR,
    #[doc = "0x20 - SPI_FTOCR"]
    pub spi_ftocr: SPI_FTOCR,
}
#[doc = "SPI_CR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cr0](spi_cr0) module"]
pub type SPI_CR0 = crate::Reg<u32, _SPI_CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CR0;
#[doc = "`read()` method returns [spi_cr0::R](spi_cr0::R) reader structure"]
impl crate::Readable for SPI_CR0 {}
#[doc = "`write(|w| ..)` method takes [spi_cr0::W](spi_cr0::W) writer structure"]
impl crate::Writable for SPI_CR0 {}
#[doc = "SPI_CR0"]
pub mod spi_cr0;
#[doc = "SPI_CR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cr1](spi_cr1) module"]
pub type SPI_CR1 = crate::Reg<u32, _SPI_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CR1;
#[doc = "`read()` method returns [spi_cr1::R](spi_cr1::R) reader structure"]
impl crate::Readable for SPI_CR1 {}
#[doc = "`write(|w| ..)` method takes [spi_cr1::W](spi_cr1::W) writer structure"]
impl crate::Writable for SPI_CR1 {}
#[doc = "SPI_CR1"]
pub mod spi_cr1;
#[doc = "SPI_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ier](spi_ier) module"]
pub type SPI_IER = crate::Reg<u32, _SPI_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_IER;
#[doc = "`read()` method returns [spi_ier::R](spi_ier::R) reader structure"]
impl crate::Readable for SPI_IER {}
#[doc = "`write(|w| ..)` method takes [spi_ier::W](spi_ier::W) writer structure"]
impl crate::Writable for SPI_IER {}
#[doc = "SPI_IER"]
pub mod spi_ier;
#[doc = "SPI_CPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cpr](spi_cpr) module"]
pub type SPI_CPR = crate::Reg<u32, _SPI_CPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CPR;
#[doc = "`read()` method returns [spi_cpr::R](spi_cpr::R) reader structure"]
impl crate::Readable for SPI_CPR {}
#[doc = "`write(|w| ..)` method takes [spi_cpr::W](spi_cpr::W) writer structure"]
impl crate::Writable for SPI_CPR {}
#[doc = "SPI_CPR"]
pub mod spi_cpr;
#[doc = "SPI_DR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_dr](spi_dr) module"]
pub type SPI_DR = crate::Reg<u32, _SPI_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DR;
#[doc = "`read()` method returns [spi_dr::R](spi_dr::R) reader structure"]
impl crate::Readable for SPI_DR {}
#[doc = "`write(|w| ..)` method takes [spi_dr::W](spi_dr::W) writer structure"]
impl crate::Writable for SPI_DR {}
#[doc = "SPI_DR"]
pub mod spi_dr;
#[doc = "SPI_SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_sr](spi_sr) module"]
pub type SPI_SR = crate::Reg<u32, _SPI_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SR;
#[doc = "`read()` method returns [spi_sr::R](spi_sr::R) reader structure"]
impl crate::Readable for SPI_SR {}
#[doc = "`write(|w| ..)` method takes [spi_sr::W](spi_sr::W) writer structure"]
impl crate::Writable for SPI_SR {}
#[doc = "SPI_SR"]
pub mod spi_sr;
#[doc = "SPI_FCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fcr](spi_fcr) module"]
pub type SPI_FCR = crate::Reg<u32, _SPI_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_FCR;
#[doc = "`read()` method returns [spi_fcr::R](spi_fcr::R) reader structure"]
impl crate::Readable for SPI_FCR {}
#[doc = "`write(|w| ..)` method takes [spi_fcr::W](spi_fcr::W) writer structure"]
impl crate::Writable for SPI_FCR {}
#[doc = "SPI_FCR"]
pub mod spi_fcr;
#[doc = "SPI_FSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fsr](spi_fsr) module"]
pub type SPI_FSR = crate::Reg<u32, _SPI_FSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_FSR;
#[doc = "`read()` method returns [spi_fsr::R](spi_fsr::R) reader structure"]
impl crate::Readable for SPI_FSR {}
#[doc = "`write(|w| ..)` method takes [spi_fsr::W](spi_fsr::W) writer structure"]
impl crate::Writable for SPI_FSR {}
#[doc = "SPI_FSR"]
pub mod spi_fsr;
#[doc = "SPI_FTOCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ftocr](spi_ftocr) module"]
pub type SPI_FTOCR = crate::Reg<u32, _SPI_FTOCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_FTOCR;
#[doc = "`read()` method returns [spi_ftocr::R](spi_ftocr::R) reader structure"]
impl crate::Readable for SPI_FTOCR {}
#[doc = "`write(|w| ..)` method takes [spi_ftocr::W](spi_ftocr::W) writer structure"]
impl crate::Writable for SPI_FTOCR {}
#[doc = "SPI_FTOCR"]
pub mod spi_ftocr;
