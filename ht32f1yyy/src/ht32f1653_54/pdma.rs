#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDMA_CH0CR"]
    pub pdma_ch0cr: PDMA_CH0CR,
    #[doc = "0x04 - PDMA_CH0SADR"]
    pub pdma_ch0sadr: PDMA_CH0SADR,
    #[doc = "0x08 - PDMA_CH0DADR"]
    pub pdma_ch0dadr: PDMA_CH0DADR,
    #[doc = "0x0c - PDMA_CH0CADR"]
    pub pdma_ch0cadr: PDMA_CH0CADR,
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
    #[doc = "0x24 - PDMA_CH1CADR"]
    pub pdma_ch1cadr: PDMA_CH1CADR,
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
    #[doc = "0x3c - PDMA_CH2CADR"]
    pub pdma_ch2cadr: PDMA_CH2CADR,
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
    #[doc = "0x54 - PDMA_CH3CADR"]
    pub pdma_ch3cadr: PDMA_CH3CADR,
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
    #[doc = "0x6c - PDMA_CH4CADR"]
    pub pdma_ch4cadr: PDMA_CH4CADR,
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
    #[doc = "0x84 - PDMA_CH5CADR"]
    pub pdma_ch5cadr: PDMA_CH5CADR,
    #[doc = "0x88 - PDMA_CH5TSR"]
    pub pdma_ch5tsr: PDMA_CH5TSR,
    #[doc = "0x8c - PDMA_CH5CTSR"]
    pub pdma_ch5ctsr: PDMA_CH5CTSR,
    #[doc = "0x90 - PDMA_CH6CR"]
    pub pdma_ch6cr: PDMA_CH6CR,
    #[doc = "0x94 - PDMA_CH6SADR"]
    pub pdma_ch6sadr: PDMA_CH6SADR,
    #[doc = "0x98 - PDMA_CH6DADR"]
    pub pdma_ch6dadr: PDMA_CH6DADR,
    #[doc = "0x9c - PDMA_CH6CADR"]
    pub pdma_ch6cadr: PDMA_CH6CADR,
    #[doc = "0xa0 - PDMA_CH6TSR"]
    pub pdma_ch6tsr: PDMA_CH6TSR,
    #[doc = "0xa4 - PDMA_CH6CTSR"]
    pub pdma_ch6ctsr: PDMA_CH6CTSR,
    #[doc = "0xa8 - PDMA_CH7CR"]
    pub pdma_ch7cr: PDMA_CH7CR,
    #[doc = "0xac - PDMA_CH7SADR"]
    pub pdma_ch7sadr: PDMA_CH7SADR,
    #[doc = "0xb0 - PDMA_CH7DADR"]
    pub pdma_ch7dadr: PDMA_CH7DADR,
    #[doc = "0xb4 - PDMA_CH7CADR"]
    pub pdma_ch7cadr: PDMA_CH7CADR,
    #[doc = "0xb8 - PDMA_CH7TSR"]
    pub pdma_ch7tsr: PDMA_CH7TSR,
    #[doc = "0xbc - PDMA_CH7CTSR"]
    pub pdma_ch7ctsr: PDMA_CH7CTSR,
    _reserved48: [u8; 0x60],
    #[doc = "0x120 - PDMA_ISR0"]
    pub pdma_isr0: PDMA_ISR0,
    #[doc = "0x124 - PDMA_ISR1"]
    pub pdma_isr1: PDMA_ISR1,
    #[doc = "0x128 - PDMA_ISCR0"]
    pub pdma_iscr0: PDMA_ISCR0,
    #[doc = "0x12c - PDMA_ISCR1"]
    pub pdma_iscr1: PDMA_ISCR1,
    #[doc = "0x130 - PDMA_IER0"]
    pub pdma_ier0: PDMA_IER0,
    #[doc = "0x134 - PDMA_IER1"]
    pub pdma_ier1: PDMA_IER1,
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
#[doc = "PDMA_CH0CADR (rw) register accessor: an alias for `Reg<PDMA_CH0CADR_SPEC>`"]
pub type PDMA_CH0CADR = crate::Reg<pdma_ch0cadr::PDMA_CH0CADR_SPEC>;
#[doc = "PDMA_CH0CADR"]
pub mod pdma_ch0cadr;
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
#[doc = "PDMA_CH1CADR (rw) register accessor: an alias for `Reg<PDMA_CH1CADR_SPEC>`"]
pub type PDMA_CH1CADR = crate::Reg<pdma_ch1cadr::PDMA_CH1CADR_SPEC>;
#[doc = "PDMA_CH1CADR"]
pub mod pdma_ch1cadr;
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
#[doc = "PDMA_CH2CADR (rw) register accessor: an alias for `Reg<PDMA_CH2CADR_SPEC>`"]
pub type PDMA_CH2CADR = crate::Reg<pdma_ch2cadr::PDMA_CH2CADR_SPEC>;
#[doc = "PDMA_CH2CADR"]
pub mod pdma_ch2cadr;
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
#[doc = "PDMA_CH3CADR (rw) register accessor: an alias for `Reg<PDMA_CH3CADR_SPEC>`"]
pub type PDMA_CH3CADR = crate::Reg<pdma_ch3cadr::PDMA_CH3CADR_SPEC>;
#[doc = "PDMA_CH3CADR"]
pub mod pdma_ch3cadr;
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
#[doc = "PDMA_CH4CADR (rw) register accessor: an alias for `Reg<PDMA_CH4CADR_SPEC>`"]
pub type PDMA_CH4CADR = crate::Reg<pdma_ch4cadr::PDMA_CH4CADR_SPEC>;
#[doc = "PDMA_CH4CADR"]
pub mod pdma_ch4cadr;
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
#[doc = "PDMA_CH5CADR (rw) register accessor: an alias for `Reg<PDMA_CH5CADR_SPEC>`"]
pub type PDMA_CH5CADR = crate::Reg<pdma_ch5cadr::PDMA_CH5CADR_SPEC>;
#[doc = "PDMA_CH5CADR"]
pub mod pdma_ch5cadr;
#[doc = "PDMA_CH5TSR (rw) register accessor: an alias for `Reg<PDMA_CH5TSR_SPEC>`"]
pub type PDMA_CH5TSR = crate::Reg<pdma_ch5tsr::PDMA_CH5TSR_SPEC>;
#[doc = "PDMA_CH5TSR"]
pub mod pdma_ch5tsr;
#[doc = "PDMA_CH5CTSR (rw) register accessor: an alias for `Reg<PDMA_CH5CTSR_SPEC>`"]
pub type PDMA_CH5CTSR = crate::Reg<pdma_ch5ctsr::PDMA_CH5CTSR_SPEC>;
#[doc = "PDMA_CH5CTSR"]
pub mod pdma_ch5ctsr;
#[doc = "PDMA_CH6CR (rw) register accessor: an alias for `Reg<PDMA_CH6CR_SPEC>`"]
pub type PDMA_CH6CR = crate::Reg<pdma_ch6cr::PDMA_CH6CR_SPEC>;
#[doc = "PDMA_CH6CR"]
pub mod pdma_ch6cr;
#[doc = "PDMA_CH6SADR (rw) register accessor: an alias for `Reg<PDMA_CH6SADR_SPEC>`"]
pub type PDMA_CH6SADR = crate::Reg<pdma_ch6sadr::PDMA_CH6SADR_SPEC>;
#[doc = "PDMA_CH6SADR"]
pub mod pdma_ch6sadr;
#[doc = "PDMA_CH6DADR (rw) register accessor: an alias for `Reg<PDMA_CH6DADR_SPEC>`"]
pub type PDMA_CH6DADR = crate::Reg<pdma_ch6dadr::PDMA_CH6DADR_SPEC>;
#[doc = "PDMA_CH6DADR"]
pub mod pdma_ch6dadr;
#[doc = "PDMA_CH6CADR (rw) register accessor: an alias for `Reg<PDMA_CH6CADR_SPEC>`"]
pub type PDMA_CH6CADR = crate::Reg<pdma_ch6cadr::PDMA_CH6CADR_SPEC>;
#[doc = "PDMA_CH6CADR"]
pub mod pdma_ch6cadr;
#[doc = "PDMA_CH6TSR (rw) register accessor: an alias for `Reg<PDMA_CH6TSR_SPEC>`"]
pub type PDMA_CH6TSR = crate::Reg<pdma_ch6tsr::PDMA_CH6TSR_SPEC>;
#[doc = "PDMA_CH6TSR"]
pub mod pdma_ch6tsr;
#[doc = "PDMA_CH6CTSR (rw) register accessor: an alias for `Reg<PDMA_CH6CTSR_SPEC>`"]
pub type PDMA_CH6CTSR = crate::Reg<pdma_ch6ctsr::PDMA_CH6CTSR_SPEC>;
#[doc = "PDMA_CH6CTSR"]
pub mod pdma_ch6ctsr;
#[doc = "PDMA_CH7CR (rw) register accessor: an alias for `Reg<PDMA_CH7CR_SPEC>`"]
pub type PDMA_CH7CR = crate::Reg<pdma_ch7cr::PDMA_CH7CR_SPEC>;
#[doc = "PDMA_CH7CR"]
pub mod pdma_ch7cr;
#[doc = "PDMA_CH7SADR (rw) register accessor: an alias for `Reg<PDMA_CH7SADR_SPEC>`"]
pub type PDMA_CH7SADR = crate::Reg<pdma_ch7sadr::PDMA_CH7SADR_SPEC>;
#[doc = "PDMA_CH7SADR"]
pub mod pdma_ch7sadr;
#[doc = "PDMA_CH7DADR (rw) register accessor: an alias for `Reg<PDMA_CH7DADR_SPEC>`"]
pub type PDMA_CH7DADR = crate::Reg<pdma_ch7dadr::PDMA_CH7DADR_SPEC>;
#[doc = "PDMA_CH7DADR"]
pub mod pdma_ch7dadr;
#[doc = "PDMA_CH7CADR (rw) register accessor: an alias for `Reg<PDMA_CH7CADR_SPEC>`"]
pub type PDMA_CH7CADR = crate::Reg<pdma_ch7cadr::PDMA_CH7CADR_SPEC>;
#[doc = "PDMA_CH7CADR"]
pub mod pdma_ch7cadr;
#[doc = "PDMA_CH7TSR (rw) register accessor: an alias for `Reg<PDMA_CH7TSR_SPEC>`"]
pub type PDMA_CH7TSR = crate::Reg<pdma_ch7tsr::PDMA_CH7TSR_SPEC>;
#[doc = "PDMA_CH7TSR"]
pub mod pdma_ch7tsr;
#[doc = "PDMA_CH7CTSR (rw) register accessor: an alias for `Reg<PDMA_CH7CTSR_SPEC>`"]
pub type PDMA_CH7CTSR = crate::Reg<pdma_ch7ctsr::PDMA_CH7CTSR_SPEC>;
#[doc = "PDMA_CH7CTSR"]
pub mod pdma_ch7ctsr;
#[doc = "PDMA_ISR0 (rw) register accessor: an alias for `Reg<PDMA_ISR0_SPEC>`"]
pub type PDMA_ISR0 = crate::Reg<pdma_isr0::PDMA_ISR0_SPEC>;
#[doc = "PDMA_ISR0"]
pub mod pdma_isr0;
#[doc = "PDMA_ISR1 (rw) register accessor: an alias for `Reg<PDMA_ISR1_SPEC>`"]
pub type PDMA_ISR1 = crate::Reg<pdma_isr1::PDMA_ISR1_SPEC>;
#[doc = "PDMA_ISR1"]
pub mod pdma_isr1;
#[doc = "PDMA_ISCR0 (rw) register accessor: an alias for `Reg<PDMA_ISCR0_SPEC>`"]
pub type PDMA_ISCR0 = crate::Reg<pdma_iscr0::PDMA_ISCR0_SPEC>;
#[doc = "PDMA_ISCR0"]
pub mod pdma_iscr0;
#[doc = "PDMA_ISCR1 (rw) register accessor: an alias for `Reg<PDMA_ISCR1_SPEC>`"]
pub type PDMA_ISCR1 = crate::Reg<pdma_iscr1::PDMA_ISCR1_SPEC>;
#[doc = "PDMA_ISCR1"]
pub mod pdma_iscr1;
#[doc = "PDMA_IER0 (rw) register accessor: an alias for `Reg<PDMA_IER0_SPEC>`"]
pub type PDMA_IER0 = crate::Reg<pdma_ier0::PDMA_IER0_SPEC>;
#[doc = "PDMA_IER0"]
pub mod pdma_ier0;
#[doc = "PDMA_IER1 (rw) register accessor: an alias for `Reg<PDMA_IER1_SPEC>`"]
pub type PDMA_IER1 = crate::Reg<pdma_ier1::PDMA_IER1_SPEC>;
#[doc = "PDMA_IER1"]
pub mod pdma_ier1;
