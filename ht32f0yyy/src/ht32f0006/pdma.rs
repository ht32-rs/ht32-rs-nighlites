#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDMA_CH0CR"]
    pub pdma_ch0cr: PDMA_CH0CR,
    #[doc = "0x04 - PDMA_CH0SADR"]
    pub pdma_ch0sadr: PDMA_CH0SADR,
    #[doc = "0x08 - PDMA_CH0DADR"]
    pub pdma_ch0dadr: PDMA_CH0DADR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - PDMA_CH0TSR"]
    pub pdma_ch0tsr: PDMA_CH0TSR,
    #[doc = "0x14 - PDMA_CH0CTSR"]
    pub pdma_ch0ctsr: PDMA_CH0CTSR,
    #[doc = "0x18 - PDMA_CH1CR"]
    pub pdma_ch1cr: PDMA_CH1CR,
    #[doc = "0x1c - PDMA_CH1SADR"]
    pub pdma_ch1sadr: PDMA_CH1SADR,
    #[doc = "0x20 - PDMA_CH1DADR"]
    pub pdma_ch1dadr: PDMA_CH1DADR,
    _reserved8: [u8; 0x04],
    #[doc = "0x28 - PDMA_CH1TSR"]
    pub pdma_ch1tsr: PDMA_CH1TSR,
    #[doc = "0x2c - PDMA_CH1CTSR"]
    pub pdma_ch1ctsr: PDMA_CH1CTSR,
    #[doc = "0x30 - PDMA_CH2CR"]
    pub pdma_ch2cr: PDMA_CH2CR,
    #[doc = "0x34 - PDMA_CH2SADR"]
    pub pdma_ch2sadr: PDMA_CH2SADR,
    #[doc = "0x38 - PDMA_CH2DADR"]
    pub pdma_ch2dadr: PDMA_CH2DADR,
    _reserved13: [u8; 0x04],
    #[doc = "0x40 - PDMA_CH2TSR"]
    pub pdma_ch2tsr: PDMA_CH2TSR,
    #[doc = "0x44 - PDMA_CH2CTSR"]
    pub pdma_ch2ctsr: PDMA_CH2CTSR,
    #[doc = "0x48 - PDMA_CH3CR"]
    pub pdma_ch3cr: PDMA_CH3CR,
    #[doc = "0x4c - PDMA_CH3SADR"]
    pub pdma_ch3sadr: PDMA_CH3SADR,
    #[doc = "0x50 - PDMA_CH3DADR"]
    pub pdma_ch3dadr: PDMA_CH3DADR,
    _reserved18: [u8; 0x04],
    #[doc = "0x58 - PDMA_CH3TSR"]
    pub pdma_ch3tsr: PDMA_CH3TSR,
    #[doc = "0x5c - PDMA_CH3CTSR"]
    pub pdma_ch3ctsr: PDMA_CH3CTSR,
    #[doc = "0x60 - PDMA_CH4CR"]
    pub pdma_ch4cr: PDMA_CH4CR,
    #[doc = "0x64 - PDMA_CH4SADR"]
    pub pdma_ch4sadr: PDMA_CH4SADR,
    #[doc = "0x68 - PDMA_CH4DADR"]
    pub pdma_ch4dadr: PDMA_CH4DADR,
    _reserved23: [u8; 0x04],
    #[doc = "0x70 - PDMA_CH4TSR"]
    pub pdma_ch4tsr: PDMA_CH4TSR,
    #[doc = "0x74 - PDMA_CH4CTSR"]
    pub pdma_ch4ctsr: PDMA_CH4CTSR,
    #[doc = "0x78 - PDMA_CH5CR"]
    pub pdma_ch5cr: PDMA_CH5CR,
    #[doc = "0x7c - PDMA_CH5SADR"]
    pub pdma_ch5sadr: PDMA_CH5SADR,
    #[doc = "0x80 - PDMA_CH5DADR"]
    pub pdma_ch5dadr: PDMA_CH5DADR,
    _reserved28: [u8; 0x04],
    #[doc = "0x88 - PDMA_CH5TSR"]
    pub pdma_ch5tsr: PDMA_CH5TSR,
    #[doc = "0x8c - PDMA_CH5CTSR"]
    pub pdma_ch5ctsr: PDMA_CH5CTSR,
    _reserved30: [u8; 0x90],
    #[doc = "0x120 - PDMA_ISR"]
    pub pdma_isr: PDMA_ISR,
    _reserved31: [u8; 0x04],
    #[doc = "0x128 - PDMA_ISCR"]
    pub pdma_iscr: PDMA_ISCR,
    _reserved32: [u8; 0x04],
    #[doc = "0x130 - PDMA_IER"]
    pub pdma_ier: PDMA_IER,
}
#[doc = "PDMA_CH0CR (rw) register accessor: an alias for `Reg<PDMA_CH0CR_SPEC>`"]
pub type PDMA_CH0CR = crate::Reg<pdma_ch0cr::PDMA_CH0CR_SPEC>;
#[doc = "PDMA_CH0CR"]
pub mod pdma_ch0cr;
#[doc = "PDMA_CH0SADR (rw) register accessor: an alias for `Reg<PDMA_CH0SADR_SPEC>`"]
pub type PDMA_CH0SADR = crate::Reg<pdma_ch0sadr::PDMA_CH0SADR_SPEC>;
#[doc = "PDMA_CH0SADR"]
pub mod pdma_ch0sadr;
#[doc = "PDMA_CH0DADR (rw) register accessor: an alias for `Reg<PDMA_CH0DADR_SPEC>`"]
pub type PDMA_CH0DADR = crate::Reg<pdma_ch0dadr::PDMA_CH0DADR_SPEC>;
#[doc = "PDMA_CH0DADR"]
pub mod pdma_ch0dadr;
#[doc = "PDMA_CH0TSR (rw) register accessor: an alias for `Reg<PDMA_CH0TSR_SPEC>`"]
pub type PDMA_CH0TSR = crate::Reg<pdma_ch0tsr::PDMA_CH0TSR_SPEC>;
#[doc = "PDMA_CH0TSR"]
pub mod pdma_ch0tsr;
#[doc = "PDMA_CH0CTSR (rw) register accessor: an alias for `Reg<PDMA_CH0CTSR_SPEC>`"]
pub type PDMA_CH0CTSR = crate::Reg<pdma_ch0ctsr::PDMA_CH0CTSR_SPEC>;
#[doc = "PDMA_CH0CTSR"]
pub mod pdma_ch0ctsr;
#[doc = "PDMA_CH1CR (rw) register accessor: an alias for `Reg<PDMA_CH1CR_SPEC>`"]
pub type PDMA_CH1CR = crate::Reg<pdma_ch1cr::PDMA_CH1CR_SPEC>;
#[doc = "PDMA_CH1CR"]
pub mod pdma_ch1cr;
#[doc = "PDMA_CH1SADR (rw) register accessor: an alias for `Reg<PDMA_CH1SADR_SPEC>`"]
pub type PDMA_CH1SADR = crate::Reg<pdma_ch1sadr::PDMA_CH1SADR_SPEC>;
#[doc = "PDMA_CH1SADR"]
pub mod pdma_ch1sadr;
#[doc = "PDMA_CH1DADR (rw) register accessor: an alias for `Reg<PDMA_CH1DADR_SPEC>`"]
pub type PDMA_CH1DADR = crate::Reg<pdma_ch1dadr::PDMA_CH1DADR_SPEC>;
#[doc = "PDMA_CH1DADR"]
pub mod pdma_ch1dadr;
#[doc = "PDMA_CH1TSR (rw) register accessor: an alias for `Reg<PDMA_CH1TSR_SPEC>`"]
pub type PDMA_CH1TSR = crate::Reg<pdma_ch1tsr::PDMA_CH1TSR_SPEC>;
#[doc = "PDMA_CH1TSR"]
pub mod pdma_ch1tsr;
#[doc = "PDMA_CH1CTSR (rw) register accessor: an alias for `Reg<PDMA_CH1CTSR_SPEC>`"]
pub type PDMA_CH1CTSR = crate::Reg<pdma_ch1ctsr::PDMA_CH1CTSR_SPEC>;
#[doc = "PDMA_CH1CTSR"]
pub mod pdma_ch1ctsr;
#[doc = "PDMA_CH2CR (rw) register accessor: an alias for `Reg<PDMA_CH2CR_SPEC>`"]
pub type PDMA_CH2CR = crate::Reg<pdma_ch2cr::PDMA_CH2CR_SPEC>;
#[doc = "PDMA_CH2CR"]
pub mod pdma_ch2cr;
#[doc = "PDMA_CH2SADR (rw) register accessor: an alias for `Reg<PDMA_CH2SADR_SPEC>`"]
pub type PDMA_CH2SADR = crate::Reg<pdma_ch2sadr::PDMA_CH2SADR_SPEC>;
#[doc = "PDMA_CH2SADR"]
pub mod pdma_ch2sadr;
#[doc = "PDMA_CH2DADR (rw) register accessor: an alias for `Reg<PDMA_CH2DADR_SPEC>`"]
pub type PDMA_CH2DADR = crate::Reg<pdma_ch2dadr::PDMA_CH2DADR_SPEC>;
#[doc = "PDMA_CH2DADR"]
pub mod pdma_ch2dadr;
#[doc = "PDMA_CH2TSR (rw) register accessor: an alias for `Reg<PDMA_CH2TSR_SPEC>`"]
pub type PDMA_CH2TSR = crate::Reg<pdma_ch2tsr::PDMA_CH2TSR_SPEC>;
#[doc = "PDMA_CH2TSR"]
pub mod pdma_ch2tsr;
#[doc = "PDMA_CH2CTSR (rw) register accessor: an alias for `Reg<PDMA_CH2CTSR_SPEC>`"]
pub type PDMA_CH2CTSR = crate::Reg<pdma_ch2ctsr::PDMA_CH2CTSR_SPEC>;
#[doc = "PDMA_CH2CTSR"]
pub mod pdma_ch2ctsr;
#[doc = "PDMA_CH3CR (rw) register accessor: an alias for `Reg<PDMA_CH3CR_SPEC>`"]
pub type PDMA_CH3CR = crate::Reg<pdma_ch3cr::PDMA_CH3CR_SPEC>;
#[doc = "PDMA_CH3CR"]
pub mod pdma_ch3cr;
#[doc = "PDMA_CH3SADR (rw) register accessor: an alias for `Reg<PDMA_CH3SADR_SPEC>`"]
pub type PDMA_CH3SADR = crate::Reg<pdma_ch3sadr::PDMA_CH3SADR_SPEC>;
#[doc = "PDMA_CH3SADR"]
pub mod pdma_ch3sadr;
#[doc = "PDMA_CH3DADR (rw) register accessor: an alias for `Reg<PDMA_CH3DADR_SPEC>`"]
pub type PDMA_CH3DADR = crate::Reg<pdma_ch3dadr::PDMA_CH3DADR_SPEC>;
#[doc = "PDMA_CH3DADR"]
pub mod pdma_ch3dadr;
#[doc = "PDMA_CH3TSR (rw) register accessor: an alias for `Reg<PDMA_CH3TSR_SPEC>`"]
pub type PDMA_CH3TSR = crate::Reg<pdma_ch3tsr::PDMA_CH3TSR_SPEC>;
#[doc = "PDMA_CH3TSR"]
pub mod pdma_ch3tsr;
#[doc = "PDMA_CH3CTSR (rw) register accessor: an alias for `Reg<PDMA_CH3CTSR_SPEC>`"]
pub type PDMA_CH3CTSR = crate::Reg<pdma_ch3ctsr::PDMA_CH3CTSR_SPEC>;
#[doc = "PDMA_CH3CTSR"]
pub mod pdma_ch3ctsr;
#[doc = "PDMA_CH4CR (rw) register accessor: an alias for `Reg<PDMA_CH4CR_SPEC>`"]
pub type PDMA_CH4CR = crate::Reg<pdma_ch4cr::PDMA_CH4CR_SPEC>;
#[doc = "PDMA_CH4CR"]
pub mod pdma_ch4cr;
#[doc = "PDMA_CH4SADR (rw) register accessor: an alias for `Reg<PDMA_CH4SADR_SPEC>`"]
pub type PDMA_CH4SADR = crate::Reg<pdma_ch4sadr::PDMA_CH4SADR_SPEC>;
#[doc = "PDMA_CH4SADR"]
pub mod pdma_ch4sadr;
#[doc = "PDMA_CH4DADR (rw) register accessor: an alias for `Reg<PDMA_CH4DADR_SPEC>`"]
pub type PDMA_CH4DADR = crate::Reg<pdma_ch4dadr::PDMA_CH4DADR_SPEC>;
#[doc = "PDMA_CH4DADR"]
pub mod pdma_ch4dadr;
#[doc = "PDMA_CH4TSR (rw) register accessor: an alias for `Reg<PDMA_CH4TSR_SPEC>`"]
pub type PDMA_CH4TSR = crate::Reg<pdma_ch4tsr::PDMA_CH4TSR_SPEC>;
#[doc = "PDMA_CH4TSR"]
pub mod pdma_ch4tsr;
#[doc = "PDMA_CH4CTSR (rw) register accessor: an alias for `Reg<PDMA_CH4CTSR_SPEC>`"]
pub type PDMA_CH4CTSR = crate::Reg<pdma_ch4ctsr::PDMA_CH4CTSR_SPEC>;
#[doc = "PDMA_CH4CTSR"]
pub mod pdma_ch4ctsr;
#[doc = "PDMA_CH5CR (rw) register accessor: an alias for `Reg<PDMA_CH5CR_SPEC>`"]
pub type PDMA_CH5CR = crate::Reg<pdma_ch5cr::PDMA_CH5CR_SPEC>;
#[doc = "PDMA_CH5CR"]
pub mod pdma_ch5cr;
#[doc = "PDMA_CH5SADR (rw) register accessor: an alias for `Reg<PDMA_CH5SADR_SPEC>`"]
pub type PDMA_CH5SADR = crate::Reg<pdma_ch5sadr::PDMA_CH5SADR_SPEC>;
#[doc = "PDMA_CH5SADR"]
pub mod pdma_ch5sadr;
#[doc = "PDMA_CH5DADR (rw) register accessor: an alias for `Reg<PDMA_CH5DADR_SPEC>`"]
pub type PDMA_CH5DADR = crate::Reg<pdma_ch5dadr::PDMA_CH5DADR_SPEC>;
#[doc = "PDMA_CH5DADR"]
pub mod pdma_ch5dadr;
#[doc = "PDMA_CH5TSR (rw) register accessor: an alias for `Reg<PDMA_CH5TSR_SPEC>`"]
pub type PDMA_CH5TSR = crate::Reg<pdma_ch5tsr::PDMA_CH5TSR_SPEC>;
#[doc = "PDMA_CH5TSR"]
pub mod pdma_ch5tsr;
#[doc = "PDMA_CH5CTSR (rw) register accessor: an alias for `Reg<PDMA_CH5CTSR_SPEC>`"]
pub type PDMA_CH5CTSR = crate::Reg<pdma_ch5ctsr::PDMA_CH5CTSR_SPEC>;
#[doc = "PDMA_CH5CTSR"]
pub mod pdma_ch5ctsr;
#[doc = "PDMA_ISR (rw) register accessor: an alias for `Reg<PDMA_ISR_SPEC>`"]
pub type PDMA_ISR = crate::Reg<pdma_isr::PDMA_ISR_SPEC>;
#[doc = "PDMA_ISR"]
pub mod pdma_isr;
#[doc = "PDMA_ISCR (rw) register accessor: an alias for `Reg<PDMA_ISCR_SPEC>`"]
pub type PDMA_ISCR = crate::Reg<pdma_iscr::PDMA_ISCR_SPEC>;
#[doc = "PDMA_ISCR"]
pub mod pdma_iscr;
#[doc = "PDMA_IER (rw) register accessor: an alias for `Reg<PDMA_IER_SPEC>`"]
pub type PDMA_IER = crate::Reg<pdma_ier::PDMA_IER_SPEC>;
#[doc = "PDMA_IER"]
pub mod pdma_ier;
