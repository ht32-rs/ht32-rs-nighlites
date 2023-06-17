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
#[doc = "I2C_CR (rw) register accessor: an alias for `Reg<I2C_CR_SPEC>`"]
pub type I2C_CR = crate::Reg<i2c_cr::I2C_CR_SPEC>;
#[doc = "I2C_CR"]
pub mod i2c_cr;
#[doc = "I2C_IER (rw) register accessor: an alias for `Reg<I2C_IER_SPEC>`"]
pub type I2C_IER = crate::Reg<i2c_ier::I2C_IER_SPEC>;
#[doc = "I2C_IER"]
pub mod i2c_ier;
#[doc = "I2C_ADDR (rw) register accessor: an alias for `Reg<I2C_ADDR_SPEC>`"]
pub type I2C_ADDR = crate::Reg<i2c_addr::I2C_ADDR_SPEC>;
#[doc = "I2C_ADDR"]
pub mod i2c_addr;
#[doc = "I2C_SR (rw) register accessor: an alias for `Reg<I2C_SR_SPEC>`"]
pub type I2C_SR = crate::Reg<i2c_sr::I2C_SR_SPEC>;
#[doc = "I2C_SR"]
pub mod i2c_sr;
#[doc = "I2C_SHPGR (rw) register accessor: an alias for `Reg<I2C_SHPGR_SPEC>`"]
pub type I2C_SHPGR = crate::Reg<i2c_shpgr::I2C_SHPGR_SPEC>;
#[doc = "I2C_SHPGR"]
pub mod i2c_shpgr;
#[doc = "I2C_SLPGR (rw) register accessor: an alias for `Reg<I2C_SLPGR_SPEC>`"]
pub type I2C_SLPGR = crate::Reg<i2c_slpgr::I2C_SLPGR_SPEC>;
#[doc = "I2C_SLPGR"]
pub mod i2c_slpgr;
#[doc = "I2C_DR (rw) register accessor: an alias for `Reg<I2C_DR_SPEC>`"]
pub type I2C_DR = crate::Reg<i2c_dr::I2C_DR_SPEC>;
#[doc = "I2C_DR"]
pub mod i2c_dr;
#[doc = "I2C_TAR (rw) register accessor: an alias for `Reg<I2C_TAR_SPEC>`"]
pub type I2C_TAR = crate::Reg<i2c_tar::I2C_TAR_SPEC>;
#[doc = "I2C_TAR"]
pub mod i2c_tar;
