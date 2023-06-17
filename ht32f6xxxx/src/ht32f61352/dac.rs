#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAC_CR"]
    pub dac_cr: DAC_CR,
    #[doc = "0x04 - DAC_RH"]
    pub dac_rh: DAC_RH,
    #[doc = "0x08 - DAC_LH"]
    pub dac_lh: DAC_LH,
    #[doc = "0x0c - DAC_TG"]
    pub dac_tg: DAC_TG,
}
#[doc = "DAC_CR (rw) register accessor: an alias for `Reg<DAC_CR_SPEC>`"]
pub type DAC_CR = crate::Reg<dac_cr::DAC_CR_SPEC>;
#[doc = "DAC_CR"]
pub mod dac_cr;
#[doc = "DAC_RH (rw) register accessor: an alias for `Reg<DAC_RH_SPEC>`"]
pub type DAC_RH = crate::Reg<dac_rh::DAC_RH_SPEC>;
#[doc = "DAC_RH"]
pub mod dac_rh;
#[doc = "DAC_LH (rw) register accessor: an alias for `Reg<DAC_LH_SPEC>`"]
pub type DAC_LH = crate::Reg<dac_lh::DAC_LH_SPEC>;
#[doc = "DAC_LH"]
pub mod dac_lh;
#[doc = "DAC_TG (rw) register accessor: an alias for `Reg<DAC_TG_SPEC>`"]
pub type DAC_TG = crate::Reg<dac_tg::DAC_TG_SPEC>;
#[doc = "DAC_TG"]
pub mod dac_tg;
