#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2S_CR"]
    pub i2s_cr: I2S_CR,
    #[doc = "0x04 - I2S_IER"]
    pub i2s_ier: I2S_IER,
    #[doc = "0x08 - I2S_CDR"]
    pub i2s_cdr: I2S_CDR,
    #[doc = "0x0c - I2S_TXDR"]
    pub i2s_txdr: I2S_TXDR,
    #[doc = "0x10 - I2S_RXDR"]
    pub i2s_rxdr: I2S_RXDR,
    #[doc = "0x14 - I2S_FCR"]
    pub i2s_fcr: I2S_FCR,
    #[doc = "0x18 - I2S_SR"]
    pub i2s_sr: I2S_SR,
    #[doc = "0x1c - I2S_RCNTR"]
    pub i2s_rcntr: I2S_RCNTR,
}
#[doc = "I2S_CR (rw) register accessor: an alias for `Reg<I2S_CR_SPEC>`"]
pub type I2S_CR = crate::Reg<i2s_cr::I2S_CR_SPEC>;
#[doc = "I2S_CR"]
pub mod i2s_cr;
#[doc = "I2S_IER (rw) register accessor: an alias for `Reg<I2S_IER_SPEC>`"]
pub type I2S_IER = crate::Reg<i2s_ier::I2S_IER_SPEC>;
#[doc = "I2S_IER"]
pub mod i2s_ier;
#[doc = "I2S_CDR (rw) register accessor: an alias for `Reg<I2S_CDR_SPEC>`"]
pub type I2S_CDR = crate::Reg<i2s_cdr::I2S_CDR_SPEC>;
#[doc = "I2S_CDR"]
pub mod i2s_cdr;
#[doc = "I2S_TXDR (rw) register accessor: an alias for `Reg<I2S_TXDR_SPEC>`"]
pub type I2S_TXDR = crate::Reg<i2s_txdr::I2S_TXDR_SPEC>;
#[doc = "I2S_TXDR"]
pub mod i2s_txdr;
#[doc = "I2S_RXDR (rw) register accessor: an alias for `Reg<I2S_RXDR_SPEC>`"]
pub type I2S_RXDR = crate::Reg<i2s_rxdr::I2S_RXDR_SPEC>;
#[doc = "I2S_RXDR"]
pub mod i2s_rxdr;
#[doc = "I2S_FCR (rw) register accessor: an alias for `Reg<I2S_FCR_SPEC>`"]
pub type I2S_FCR = crate::Reg<i2s_fcr::I2S_FCR_SPEC>;
#[doc = "I2S_FCR"]
pub mod i2s_fcr;
#[doc = "I2S_SR (rw) register accessor: an alias for `Reg<I2S_SR_SPEC>`"]
pub type I2S_SR = crate::Reg<i2s_sr::I2S_SR_SPEC>;
#[doc = "I2S_SR"]
pub mod i2s_sr;
#[doc = "I2S_RCNTR (rw) register accessor: an alias for `Reg<I2S_RCNTR_SPEC>`"]
pub type I2S_RCNTR = crate::Reg<i2s_rcntr::I2S_RCNTR_SPEC>;
#[doc = "I2S_RCNTR"]
pub mod i2s_rcntr;
