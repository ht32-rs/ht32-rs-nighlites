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
    #[doc = "0x10 - CMPCI0"]
    pub cmpci0: CMPCI0,
    _reserved5: [u8; 0xec],
    #[doc = "0x100 - CMPCR1"]
    pub cmpcr1: CMPCR1,
    #[doc = "0x104 - CVRVALR1"]
    pub cvrvalr1: CVRVALR1,
    #[doc = "0x108 - CMPIER1"]
    pub cmpier1: CMPIER1,
    #[doc = "0x10c - CMPTFR1"]
    pub cmptfr1: CMPTFR1,
    #[doc = "0x110 - CMPCI1"]
    pub cmpci1: CMPCI1,
    _reserved10: [u8; 0xec],
    #[doc = "0x200 - CMPCR2"]
    pub cmpcr2: CMPCR2,
    #[doc = "0x204 - CVRVALR2"]
    pub cvrvalr2: CVRVALR2,
    #[doc = "0x208 - CMPIER2"]
    pub cmpier2: CMPIER2,
    #[doc = "0x20c - CMPTFR2"]
    pub cmptfr2: CMPTFR2,
    #[doc = "0x210 - CMPCI2"]
    pub cmpci2: CMPCI2,
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
#[doc = "CMPCI0 (rw) register accessor: an alias for `Reg<CMPCI0_SPEC>`"]
pub type CMPCI0 = crate::Reg<cmpci0::CMPCI0_SPEC>;
#[doc = "CMPCI0"]
pub mod cmpci0;
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
#[doc = "CMPCI1 (rw) register accessor: an alias for `Reg<CMPCI1_SPEC>`"]
pub type CMPCI1 = crate::Reg<cmpci1::CMPCI1_SPEC>;
#[doc = "CMPCI1"]
pub mod cmpci1;
#[doc = "CMPCR2 (rw) register accessor: an alias for `Reg<CMPCR2_SPEC>`"]
pub type CMPCR2 = crate::Reg<cmpcr2::CMPCR2_SPEC>;
#[doc = "CMPCR2"]
pub mod cmpcr2;
#[doc = "CVRVALR2 (rw) register accessor: an alias for `Reg<CVRVALR2_SPEC>`"]
pub type CVRVALR2 = crate::Reg<cvrvalr2::CVRVALR2_SPEC>;
#[doc = "CVRVALR2"]
pub mod cvrvalr2;
#[doc = "CMPIER2 (rw) register accessor: an alias for `Reg<CMPIER2_SPEC>`"]
pub type CMPIER2 = crate::Reg<cmpier2::CMPIER2_SPEC>;
#[doc = "CMPIER2"]
pub mod cmpier2;
#[doc = "CMPTFR2 (rw) register accessor: an alias for `Reg<CMPTFR2_SPEC>`"]
pub type CMPTFR2 = crate::Reg<cmptfr2::CMPTFR2_SPEC>;
#[doc = "CMPTFR2"]
pub mod cmptfr2;
#[doc = "CMPCI2 (rw) register accessor: an alias for `Reg<CMPCI2_SPEC>`"]
pub type CMPCI2 = crate::Reg<cmpci2::CMPCI2_SPEC>;
#[doc = "CMPCI2"]
pub mod cmpci2;
