#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC_CR"]
    pub crc_cr: CRC_CR,
    #[doc = "0x04 - CRC_SDR"]
    pub crc_sdr: CRC_SDR,
    #[doc = "0x08 - CRC_CSR"]
    pub crc_csr: CRC_CSR,
    #[doc = "0x0c - CRC_DR"]
    pub crc_dr: CRC_DR,
}
#[doc = "CRC_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_cr](crc_cr) module"]
pub type CRC_CR = crate::Reg<u32, _CRC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_CR;
#[doc = "`read()` method returns [crc_cr::R](crc_cr::R) reader structure"]
impl crate::Readable for CRC_CR {}
#[doc = "`write(|w| ..)` method takes [crc_cr::W](crc_cr::W) writer structure"]
impl crate::Writable for CRC_CR {}
#[doc = "CRC_CR"]
pub mod crc_cr;
#[doc = "CRC_SDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_sdr](crc_sdr) module"]
pub type CRC_SDR = crate::Reg<u32, _CRC_SDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_SDR;
#[doc = "`read()` method returns [crc_sdr::R](crc_sdr::R) reader structure"]
impl crate::Readable for CRC_SDR {}
#[doc = "`write(|w| ..)` method takes [crc_sdr::W](crc_sdr::W) writer structure"]
impl crate::Writable for CRC_SDR {}
#[doc = "CRC_SDR"]
pub mod crc_sdr;
#[doc = "CRC_CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_csr](crc_csr) module"]
pub type CRC_CSR = crate::Reg<u32, _CRC_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_CSR;
#[doc = "`read()` method returns [crc_csr::R](crc_csr::R) reader structure"]
impl crate::Readable for CRC_CSR {}
#[doc = "`write(|w| ..)` method takes [crc_csr::W](crc_csr::W) writer structure"]
impl crate::Writable for CRC_CSR {}
#[doc = "CRC_CSR"]
pub mod crc_csr;
#[doc = "CRC_DR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_dr](crc_dr) module"]
pub type CRC_DR = crate::Reg<u32, _CRC_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_DR;
#[doc = "`read()` method returns [crc_dr::R](crc_dr::R) reader structure"]
impl crate::Readable for CRC_DR {}
#[doc = "`write(|w| ..)` method takes [crc_dr::W](crc_dr::W) writer structure"]
impl crate::Writable for CRC_DR {}
#[doc = "CRC_DR"]
pub mod crc_dr;
