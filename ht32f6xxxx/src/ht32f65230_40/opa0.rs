#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OPACR0"]
    pub opacr0: OPACR0,
    _reserved1: [u8; 0xfc],
    #[doc = "0x100 - OPACR1"]
    pub opacr1: OPACR1,
}
#[doc = "OPACR0 (rw) register accessor: an alias for `Reg<OPACR0_SPEC>`"]
pub type OPACR0 = crate::Reg<opacr0::OPACR0_SPEC>;
#[doc = "OPACR0"]
pub mod opacr0;
#[doc = "OPACR1 (rw) register accessor: an alias for `Reg<OPACR1_SPEC>`"]
pub type OPACR1 = crate::Reg<opacr1::OPACR1_SPEC>;
#[doc = "OPACR1"]
pub mod opacr1;
