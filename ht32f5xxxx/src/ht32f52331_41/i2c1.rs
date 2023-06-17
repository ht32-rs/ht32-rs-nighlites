#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2CCR"]
    pub i2ccr: I2CCR,
    #[doc = "0x04 - I2CIER"]
    pub i2cier: I2CIER,
    #[doc = "0x08 - I2CADDR"]
    pub i2caddr: I2CADDR,
    #[doc = "0x0c - I2CSR"]
    pub i2csr: I2CSR,
    #[doc = "0x10 - I2CSHPGR"]
    pub i2cshpgr: I2CSHPGR,
    #[doc = "0x14 - I2CSLPGR"]
    pub i2cslpgr: I2CSLPGR,
    #[doc = "0x18 - I2CDR"]
    pub i2cdr: I2CDR,
    #[doc = "0x1c - I2CTAR"]
    pub i2ctar: I2CTAR,
    #[doc = "0x20 - I2CADDMR"]
    pub i2caddmr: I2CADDMR,
    #[doc = "0x24 - I2CADDSR"]
    pub i2caddsr: I2CADDSR,
    #[doc = "0x28 - I2CTOUT"]
    pub i2ctout: I2CTOUT,
}
#[doc = "I2CCR (rw) register accessor: an alias for `Reg<I2CCR_SPEC>`"]
pub type I2CCR = crate::Reg<i2ccr::I2CCR_SPEC>;
#[doc = "I2CCR"]
pub mod i2ccr;
#[doc = "I2CIER (rw) register accessor: an alias for `Reg<I2CIER_SPEC>`"]
pub type I2CIER = crate::Reg<i2cier::I2CIER_SPEC>;
#[doc = "I2CIER"]
pub mod i2cier;
#[doc = "I2CADDR (rw) register accessor: an alias for `Reg<I2CADDR_SPEC>`"]
pub type I2CADDR = crate::Reg<i2caddr::I2CADDR_SPEC>;
#[doc = "I2CADDR"]
pub mod i2caddr;
#[doc = "I2CSR (rw) register accessor: an alias for `Reg<I2CSR_SPEC>`"]
pub type I2CSR = crate::Reg<i2csr::I2CSR_SPEC>;
#[doc = "I2CSR"]
pub mod i2csr;
#[doc = "I2CSHPGR (rw) register accessor: an alias for `Reg<I2CSHPGR_SPEC>`"]
pub type I2CSHPGR = crate::Reg<i2cshpgr::I2CSHPGR_SPEC>;
#[doc = "I2CSHPGR"]
pub mod i2cshpgr;
#[doc = "I2CSLPGR (rw) register accessor: an alias for `Reg<I2CSLPGR_SPEC>`"]
pub type I2CSLPGR = crate::Reg<i2cslpgr::I2CSLPGR_SPEC>;
#[doc = "I2CSLPGR"]
pub mod i2cslpgr;
#[doc = "I2CDR (rw) register accessor: an alias for `Reg<I2CDR_SPEC>`"]
pub type I2CDR = crate::Reg<i2cdr::I2CDR_SPEC>;
#[doc = "I2CDR"]
pub mod i2cdr;
#[doc = "I2CTAR (rw) register accessor: an alias for `Reg<I2CTAR_SPEC>`"]
pub type I2CTAR = crate::Reg<i2ctar::I2CTAR_SPEC>;
#[doc = "I2CTAR"]
pub mod i2ctar;
#[doc = "I2CADDMR (rw) register accessor: an alias for `Reg<I2CADDMR_SPEC>`"]
pub type I2CADDMR = crate::Reg<i2caddmr::I2CADDMR_SPEC>;
#[doc = "I2CADDMR"]
pub mod i2caddmr;
#[doc = "I2CADDSR (rw) register accessor: an alias for `Reg<I2CADDSR_SPEC>`"]
pub type I2CADDSR = crate::Reg<i2caddsr::I2CADDSR_SPEC>;
#[doc = "I2CADDSR"]
pub mod i2caddsr;
#[doc = "I2CTOUT (rw) register accessor: an alias for `Reg<I2CTOUT_SPEC>`"]
pub type I2CTOUT = crate::Reg<i2ctout::I2CTOUT_SPEC>;
#[doc = "I2CTOUT"]
pub mod i2ctout;
