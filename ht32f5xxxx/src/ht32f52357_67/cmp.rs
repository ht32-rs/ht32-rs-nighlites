#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMPCR0"]
    pub cmpcr0: CMPCR0,
    #[doc = "0x04 - CVRVALR0"]
    pub cvrvalr0: CVRVALR0,
    #[doc = "0x08 - CMPIER0"]
    pub cmpier0: CMPIER0,
    #[doc = "0x0c - CMPTFR0"]
    pub cmptfr0: CMPTFR0,
    _reserved4: [u8; 0xf0],
    #[doc = "0x100 - CMPCR1"]
    pub cmpcr1: CMPCR1,
    #[doc = "0x104 - CVRVALR1"]
    pub cvrvalr1: CVRVALR1,
    #[doc = "0x108 - CMPIER1"]
    pub cmpier1: CMPIER1,
    #[doc = "0x10c - CMPTFR1"]
    pub cmptfr1: CMPTFR1,
}
#[doc = "CMPCR0 (rw) register accessor: an alias for `Reg<CMPCR0_SPEC>`"]
pub type CMPCR0 = crate::Reg<cmpcr0::CMPCR0_SPEC>;
#[doc = "CMPCR0"]
pub mod cmpcr0;
#[doc = "CVRVALR0 (rw) register accessor: an alias for `Reg<CVRVALR0_SPEC>`"]
pub type CVRVALR0 = crate::Reg<cvrvalr0::CVRVALR0_SPEC>;
#[doc = "CVRVALR0"]
pub mod cvrvalr0;
#[doc = "CMPIER0 (rw) register accessor: an alias for `Reg<CMPIER0_SPEC>`"]
pub type CMPIER0 = crate::Reg<cmpier0::CMPIER0_SPEC>;
#[doc = "CMPIER0"]
pub mod cmpier0;
#[doc = "CMPTFR0 (rw) register accessor: an alias for `Reg<CMPTFR0_SPEC>`"]
pub type CMPTFR0 = crate::Reg<cmptfr0::CMPTFR0_SPEC>;
#[doc = "CMPTFR0"]
pub mod cmptfr0;
#[doc = "CMPCR1 (rw) register accessor: an alias for `Reg<CMPCR1_SPEC>`"]
pub type CMPCR1 = crate::Reg<cmpcr1::CMPCR1_SPEC>;
#[doc = "CMPCR1"]
pub mod cmpcr1;
#[doc = "CVRVALR1 (rw) register accessor: an alias for `Reg<CVRVALR1_SPEC>`"]
pub type CVRVALR1 = crate::Reg<cvrvalr1::CVRVALR1_SPEC>;
#[doc = "CVRVALR1"]
pub mod cvrvalr1;
#[doc = "CMPIER1 (rw) register accessor: an alias for `Reg<CMPIER1_SPEC>`"]
pub type CMPIER1 = crate::Reg<cmpier1::CMPIER1_SPEC>;
#[doc = "CMPIER1"]
pub mod cmpier1;
#[doc = "CMPTFR1 (rw) register accessor: an alias for `Reg<CMPTFR1_SPEC>`"]
pub type CMPTFR1 = crate::Reg<cmptfr1::CMPTFR1_SPEC>;
#[doc = "CMPTFR1"]
pub mod cmptfr1;
