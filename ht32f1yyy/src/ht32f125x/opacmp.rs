#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OPACR0"]
    pub opacr0: OPACR0,
    #[doc = "0x04 - OFVCR0"]
    pub ofvcr0: OFVCR0,
    #[doc = "0x08 - CMPIER0"]
    pub cmpier0: CMPIER0,
    #[doc = "0x0c - CMPRSR0"]
    pub cmprsr0: CMPRSR0,
    #[doc = "0x10 - CMPISR0"]
    pub cmpisr0: CMPISR0,
    #[doc = "0x14 - CMPICLR0"]
    pub cmpiclr0: CMPICLR0,
    _reserved6: [u8; 0xe8],
    #[doc = "0x100 - OPACR1"]
    pub opacr1: OPACR1,
    #[doc = "0x104 - OFVCR1"]
    pub ofvcr1: OFVCR1,
    #[doc = "0x108 - CMPIER1"]
    pub cmpier1: CMPIER1,
    #[doc = "0x10c - CMPRSR1"]
    pub cmprsr1: CMPRSR1,
    #[doc = "0x110 - CMPISR1"]
    pub cmpisr1: CMPISR1,
    #[doc = "0x114 - CMPICLR1"]
    pub cmpiclr1: CMPICLR1,
}
#[doc = "OPACR0 (rw) register accessor: an alias for `Reg<OPACR0_SPEC>`"]
pub type OPACR0 = crate::Reg<opacr0::OPACR0_SPEC>;
#[doc = "OPACR0"]
pub mod opacr0;
#[doc = "OFVCR0 (rw) register accessor: an alias for `Reg<OFVCR0_SPEC>`"]
pub type OFVCR0 = crate::Reg<ofvcr0::OFVCR0_SPEC>;
#[doc = "OFVCR0"]
pub mod ofvcr0;
#[doc = "CMPIER0 (rw) register accessor: an alias for `Reg<CMPIER0_SPEC>`"]
pub type CMPIER0 = crate::Reg<cmpier0::CMPIER0_SPEC>;
#[doc = "CMPIER0"]
pub mod cmpier0;
#[doc = "CMPRSR0 (rw) register accessor: an alias for `Reg<CMPRSR0_SPEC>`"]
pub type CMPRSR0 = crate::Reg<cmprsr0::CMPRSR0_SPEC>;
#[doc = "CMPRSR0"]
pub mod cmprsr0;
#[doc = "CMPISR0 (rw) register accessor: an alias for `Reg<CMPISR0_SPEC>`"]
pub type CMPISR0 = crate::Reg<cmpisr0::CMPISR0_SPEC>;
#[doc = "CMPISR0"]
pub mod cmpisr0;
#[doc = "CMPICLR0 (rw) register accessor: an alias for `Reg<CMPICLR0_SPEC>`"]
pub type CMPICLR0 = crate::Reg<cmpiclr0::CMPICLR0_SPEC>;
#[doc = "CMPICLR0"]
pub mod cmpiclr0;
#[doc = "OPACR1 (rw) register accessor: an alias for `Reg<OPACR1_SPEC>`"]
pub type OPACR1 = crate::Reg<opacr1::OPACR1_SPEC>;
#[doc = "OPACR1"]
pub mod opacr1;
#[doc = "OFVCR1 (rw) register accessor: an alias for `Reg<OFVCR1_SPEC>`"]
pub type OFVCR1 = crate::Reg<ofvcr1::OFVCR1_SPEC>;
#[doc = "OFVCR1"]
pub mod ofvcr1;
#[doc = "CMPIER1 (rw) register accessor: an alias for `Reg<CMPIER1_SPEC>`"]
pub type CMPIER1 = crate::Reg<cmpier1::CMPIER1_SPEC>;
#[doc = "CMPIER1"]
pub mod cmpier1;
#[doc = "CMPRSR1 (rw) register accessor: an alias for `Reg<CMPRSR1_SPEC>`"]
pub type CMPRSR1 = crate::Reg<cmprsr1::CMPRSR1_SPEC>;
#[doc = "CMPRSR1"]
pub mod cmprsr1;
#[doc = "CMPISR1 (rw) register accessor: an alias for `Reg<CMPISR1_SPEC>`"]
pub type CMPISR1 = crate::Reg<cmpisr1::CMPISR1_SPEC>;
#[doc = "CMPISR1"]
pub mod cmpisr1;
#[doc = "CMPICLR1 (rw) register accessor: an alias for `Reg<CMPICLR1_SPEC>`"]
pub type CMPICLR1 = crate::Reg<cmpiclr1::CMPICLR1_SPEC>;
#[doc = "CMPICLR1"]
pub mod cmpiclr1;
