#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2SCR"]
    pub i2scr: I2SCR,
    #[doc = "0x04 - I2SIER"]
    pub i2sier: I2SIER,
    #[doc = "0x08 - I2SCDR"]
    pub i2scdr: I2SCDR,
    #[doc = "0x0c - I2STXDR"]
    pub i2stxdr: I2STXDR,
    #[doc = "0x10 - I2SRXDR"]
    pub i2srxdr: I2SRXDR,
    #[doc = "0x14 - I2SFCR"]
    pub i2sfcr: I2SFCR,
    #[doc = "0x18 - I2SSR"]
    pub i2ssr: I2SSR,
    #[doc = "0x1c - I2SRCNTR"]
    pub i2srcntr: I2SRCNTR,
}
#[doc = "I2SCR (rw) register accessor: an alias for `Reg<I2SCR_SPEC>`"]
pub type I2SCR = crate::Reg<i2scr::I2SCR_SPEC>;
#[doc = "I2SCR"]
pub mod i2scr;
#[doc = "I2SIER (rw) register accessor: an alias for `Reg<I2SIER_SPEC>`"]
pub type I2SIER = crate::Reg<i2sier::I2SIER_SPEC>;
#[doc = "I2SIER"]
pub mod i2sier;
#[doc = "I2SCDR (rw) register accessor: an alias for `Reg<I2SCDR_SPEC>`"]
pub type I2SCDR = crate::Reg<i2scdr::I2SCDR_SPEC>;
#[doc = "I2SCDR"]
pub mod i2scdr;
#[doc = "I2STXDR (rw) register accessor: an alias for `Reg<I2STXDR_SPEC>`"]
pub type I2STXDR = crate::Reg<i2stxdr::I2STXDR_SPEC>;
#[doc = "I2STXDR"]
pub mod i2stxdr;
#[doc = "I2SRXDR (rw) register accessor: an alias for `Reg<I2SRXDR_SPEC>`"]
pub type I2SRXDR = crate::Reg<i2srxdr::I2SRXDR_SPEC>;
#[doc = "I2SRXDR"]
pub mod i2srxdr;
#[doc = "I2SFCR (rw) register accessor: an alias for `Reg<I2SFCR_SPEC>`"]
pub type I2SFCR = crate::Reg<i2sfcr::I2SFCR_SPEC>;
#[doc = "I2SFCR"]
pub mod i2sfcr;
#[doc = "I2SSR (rw) register accessor: an alias for `Reg<I2SSR_SPEC>`"]
pub type I2SSR = crate::Reg<i2ssr::I2SSR_SPEC>;
#[doc = "I2SSR"]
pub mod i2ssr;
#[doc = "I2SRCNTR (rw) register accessor: an alias for `Reg<I2SRCNTR_SPEC>`"]
pub type I2SRCNTR = crate::Reg<i2srcntr::I2SRCNTR_SPEC>;
#[doc = "I2SRCNTR"]
pub mod i2srcntr;
