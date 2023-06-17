#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAC_DACCFGR"]
    pub dac_daccfgr: DAC_DACCFGR,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - DAC_DAC0CR"]
    pub dac_dac0cr: DAC_DAC0CR,
    _reserved2: [u8; 0x08],
    #[doc = "0x1c - DAC_DAC0DHR"]
    pub dac_dac0dhr: DAC_DAC0DHR,
    #[doc = "0x20 - DAC_DAC0DOR"]
    pub dac_dac0dor: DAC_DAC0DOR,
    _reserved4: [u8; 0x0c],
    #[doc = "0x30 - DAC_DAC1CR"]
    pub dac_dac1cr: DAC_DAC1CR,
    _reserved5: [u8; 0x08],
    #[doc = "0x3c - DAC_DAC1DHR"]
    pub dac_dac1dhr: DAC_DAC1DHR,
    #[doc = "0x40 - DAC_DAC1DOR"]
    pub dac_dac1dor: DAC_DAC1DOR,
}
#[doc = "DAC_DACCFGR (rw) register accessor: an alias for `Reg<DAC_DACCFGR_SPEC>`"]
pub type DAC_DACCFGR = crate::Reg<dac_daccfgr::DAC_DACCFGR_SPEC>;
#[doc = "DAC_DACCFGR"]
pub mod dac_daccfgr;
#[doc = "DAC_DAC0CR (rw) register accessor: an alias for `Reg<DAC_DAC0CR_SPEC>`"]
pub type DAC_DAC0CR = crate::Reg<dac_dac0cr::DAC_DAC0CR_SPEC>;
#[doc = "DAC_DAC0CR"]
pub mod dac_dac0cr;
#[doc = "DAC_DAC0DHR (rw) register accessor: an alias for `Reg<DAC_DAC0DHR_SPEC>`"]
pub type DAC_DAC0DHR = crate::Reg<dac_dac0dhr::DAC_DAC0DHR_SPEC>;
#[doc = "DAC_DAC0DHR"]
pub mod dac_dac0dhr;
#[doc = "DAC_DAC0DOR (rw) register accessor: an alias for `Reg<DAC_DAC0DOR_SPEC>`"]
pub type DAC_DAC0DOR = crate::Reg<dac_dac0dor::DAC_DAC0DOR_SPEC>;
#[doc = "DAC_DAC0DOR"]
pub mod dac_dac0dor;
#[doc = "DAC_DAC1CR (rw) register accessor: an alias for `Reg<DAC_DAC1CR_SPEC>`"]
pub type DAC_DAC1CR = crate::Reg<dac_dac1cr::DAC_DAC1CR_SPEC>;
#[doc = "DAC_DAC1CR"]
pub mod dac_dac1cr;
#[doc = "DAC_DAC1DHR (rw) register accessor: an alias for `Reg<DAC_DAC1DHR_SPEC>`"]
pub type DAC_DAC1DHR = crate::Reg<dac_dac1dhr::DAC_DAC1DHR_SPEC>;
#[doc = "DAC_DAC1DHR"]
pub mod dac_dac1dhr;
#[doc = "DAC_DAC1DOR (rw) register accessor: an alias for `Reg<DAC_DAC1DOR_SPEC>`"]
pub type DAC_DAC1DOR = crate::Reg<dac_dac1dor::DAC_DAC1DOR_SPEC>;
#[doc = "DAC_DAC1DOR"]
pub mod dac_dac1dor;
