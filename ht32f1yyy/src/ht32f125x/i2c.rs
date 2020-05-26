#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C_CR"]
    pub i2c_cr: I2C_CR,
    #[doc = "0x04 - I2C_IER"]
    pub i2c_ier: I2C_IER,
    #[doc = "0x08 - I2C_ADDR"]
    pub i2c_addr: I2C_ADDR,
    #[doc = "0x0c - I2C_SR"]
    pub i2c_sr: I2C_SR,
    #[doc = "0x10 - I2C_SHPGR"]
    pub i2c_shpgr: I2C_SHPGR,
    #[doc = "0x14 - I2C_SLPGR"]
    pub i2c_slpgr: I2C_SLPGR,
    #[doc = "0x18 - I2C_DR"]
    pub i2c_dr: I2C_DR,
    #[doc = "0x1c - I2C_TAR"]
    pub i2c_tar: I2C_TAR,
}
#[doc = "I2C_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_cr](i2c_cr) module"]
pub type I2C_CR = crate::Reg<u32, _I2C_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_CR;
#[doc = "`read()` method returns [i2c_cr::R](i2c_cr::R) reader structure"]
impl crate::Readable for I2C_CR {}
#[doc = "`write(|w| ..)` method takes [i2c_cr::W](i2c_cr::W) writer structure"]
impl crate::Writable for I2C_CR {}
#[doc = "I2C_CR"]
pub mod i2c_cr;
#[doc = "I2C_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ier](i2c_ier) module"]
pub type I2C_IER = crate::Reg<u32, _I2C_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_IER;
#[doc = "`read()` method returns [i2c_ier::R](i2c_ier::R) reader structure"]
impl crate::Readable for I2C_IER {}
#[doc = "`write(|w| ..)` method takes [i2c_ier::W](i2c_ier::W) writer structure"]
impl crate::Writable for I2C_IER {}
#[doc = "I2C_IER"]
pub mod i2c_ier;
#[doc = "I2C_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_addr](i2c_addr) module"]
pub type I2C_ADDR = crate::Reg<u32, _I2C_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_ADDR;
#[doc = "`read()` method returns [i2c_addr::R](i2c_addr::R) reader structure"]
impl crate::Readable for I2C_ADDR {}
#[doc = "`write(|w| ..)` method takes [i2c_addr::W](i2c_addr::W) writer structure"]
impl crate::Writable for I2C_ADDR {}
#[doc = "I2C_ADDR"]
pub mod i2c_addr;
#[doc = "I2C_SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_sr](i2c_sr) module"]
pub type I2C_SR = crate::Reg<u32, _I2C_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SR;
#[doc = "`read()` method returns [i2c_sr::R](i2c_sr::R) reader structure"]
impl crate::Readable for I2C_SR {}
#[doc = "`write(|w| ..)` method takes [i2c_sr::W](i2c_sr::W) writer structure"]
impl crate::Writable for I2C_SR {}
#[doc = "I2C_SR"]
pub mod i2c_sr;
#[doc = "I2C_SHPGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_shpgr](i2c_shpgr) module"]
pub type I2C_SHPGR = crate::Reg<u32, _I2C_SHPGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SHPGR;
#[doc = "`read()` method returns [i2c_shpgr::R](i2c_shpgr::R) reader structure"]
impl crate::Readable for I2C_SHPGR {}
#[doc = "`write(|w| ..)` method takes [i2c_shpgr::W](i2c_shpgr::W) writer structure"]
impl crate::Writable for I2C_SHPGR {}
#[doc = "I2C_SHPGR"]
pub mod i2c_shpgr;
#[doc = "I2C_SLPGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_slpgr](i2c_slpgr) module"]
pub type I2C_SLPGR = crate::Reg<u32, _I2C_SLPGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SLPGR;
#[doc = "`read()` method returns [i2c_slpgr::R](i2c_slpgr::R) reader structure"]
impl crate::Readable for I2C_SLPGR {}
#[doc = "`write(|w| ..)` method takes [i2c_slpgr::W](i2c_slpgr::W) writer structure"]
impl crate::Writable for I2C_SLPGR {}
#[doc = "I2C_SLPGR"]
pub mod i2c_slpgr;
#[doc = "I2C_DR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_dr](i2c_dr) module"]
pub type I2C_DR = crate::Reg<u32, _I2C_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_DR;
#[doc = "`read()` method returns [i2c_dr::R](i2c_dr::R) reader structure"]
impl crate::Readable for I2C_DR {}
#[doc = "`write(|w| ..)` method takes [i2c_dr::W](i2c_dr::W) writer structure"]
impl crate::Writable for I2C_DR {}
#[doc = "I2C_DR"]
pub mod i2c_dr;
#[doc = "I2C_TAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_tar](i2c_tar) module"]
pub type I2C_TAR = crate::Reg<u32, _I2C_TAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_TAR;
#[doc = "`read()` method returns [i2c_tar::R](i2c_tar::R) reader structure"]
impl crate::Readable for I2C_TAR {}
#[doc = "`write(|w| ..)` method takes [i2c_tar::W](i2c_tar::W) writer structure"]
impl crate::Writable for I2C_TAR {}
#[doc = "I2C_TAR"]
pub mod i2c_tar;
