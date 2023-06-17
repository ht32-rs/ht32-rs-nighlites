#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DACCFGR"]
    pub daccfgr: DACCFGR,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - DAC0CR"]
    pub dac0cr: DAC0CR,
    _reserved2: [u8; 0x08],
    #[doc = "0x1c - DAC0DHR"]
    pub dac0dhr: DAC0DHR,
    #[doc = "0x20 - DAC0DOR"]
    pub dac0dor: DAC0DOR,
    _reserved4: [u8; 0x0c],
    #[doc = "0x30 - DAC1CR"]
    pub dac1cr: DAC1CR,
    _reserved5: [u8; 0x08],
    #[doc = "0x3c - DAC1DHR"]
    pub dac1dhr: DAC1DHR,
    #[doc = "0x40 - DAC1DOR"]
    pub dac1dor: DAC1DOR,
}
#[doc = "DACCFGR (rw) register accessor: an alias for `Reg<DACCFGR_SPEC>`"]
pub type DACCFGR = crate::Reg<daccfgr::DACCFGR_SPEC>;
#[doc = "DACCFGR"]
pub mod daccfgr;
#[doc = "DAC0CR (rw) register accessor: an alias for `Reg<DAC0CR_SPEC>`"]
pub type DAC0CR = crate::Reg<dac0cr::DAC0CR_SPEC>;
#[doc = "DAC0CR"]
pub mod dac0cr;
#[doc = "DAC0DHR (rw) register accessor: an alias for `Reg<DAC0DHR_SPEC>`"]
pub type DAC0DHR = crate::Reg<dac0dhr::DAC0DHR_SPEC>;
#[doc = "DAC0DHR"]
pub mod dac0dhr;
#[doc = "DAC0DOR (rw) register accessor: an alias for `Reg<DAC0DOR_SPEC>`"]
pub type DAC0DOR = crate::Reg<dac0dor::DAC0DOR_SPEC>;
#[doc = "DAC0DOR"]
pub mod dac0dor;
#[doc = "DAC1CR (rw) register accessor: an alias for `Reg<DAC1CR_SPEC>`"]
pub type DAC1CR = crate::Reg<dac1cr::DAC1CR_SPEC>;
#[doc = "DAC1CR"]
pub mod dac1cr;
#[doc = "DAC1DHR (rw) register accessor: an alias for `Reg<DAC1DHR_SPEC>`"]
pub type DAC1DHR = crate::Reg<dac1dhr::DAC1DHR_SPEC>;
#[doc = "DAC1DHR"]
pub mod dac1dhr;
#[doc = "DAC1DOR (rw) register accessor: an alias for `Reg<DAC1DOR_SPEC>`"]
pub type DAC1DOR = crate::Reg<dac1dor::DAC1DOR_SPEC>;
#[doc = "DAC1DOR"]
pub mod dac1dor;
