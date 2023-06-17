#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AESCR"]
    pub aescr: AESCR,
    #[doc = "0x04 - AESSR"]
    pub aessr: AESSR,
    #[doc = "0x08 - AESDMAR"]
    pub aesdmar: AESDMAR,
    #[doc = "0x0c - AESISR"]
    pub aesisr: AESISR,
    #[doc = "0x10 - AESIER"]
    pub aesier: AESIER,
    #[doc = "0x14 - AESDINR"]
    pub aesdinr: AESDINR,
    #[doc = "0x18 - AESDOUTR"]
    pub aesdoutr: AESDOUTR,
    #[doc = "0x1c - AESKEYR0"]
    pub aeskeyr0: AESKEYR0,
    #[doc = "0x20 - AESKEYR1"]
    pub aeskeyr1: AESKEYR1,
    #[doc = "0x24 - AESKEYR2"]
    pub aeskeyr2: AESKEYR2,
    #[doc = "0x28 - AESKEYR3"]
    pub aeskeyr3: AESKEYR3,
    _reserved11: [u8; 0x10],
    #[doc = "0x3c - AESIVR0"]
    pub aesivr0: AESIVR0,
    #[doc = "0x40 - AESIVR1"]
    pub aesivr1: AESIVR1,
    #[doc = "0x44 - AESIVR2"]
    pub aesivr2: AESIVR2,
    #[doc = "0x48 - AESIVR3"]
    pub aesivr3: AESIVR3,
}
#[doc = "AESCR (rw) register accessor: an alias for `Reg<AESCR_SPEC>`"]
pub type AESCR = crate::Reg<aescr::AESCR_SPEC>;
#[doc = "AESCR"]
pub mod aescr;
#[doc = "AESSR (rw) register accessor: an alias for `Reg<AESSR_SPEC>`"]
pub type AESSR = crate::Reg<aessr::AESSR_SPEC>;
#[doc = "AESSR"]
pub mod aessr;
#[doc = "AESDMAR (rw) register accessor: an alias for `Reg<AESDMAR_SPEC>`"]
pub type AESDMAR = crate::Reg<aesdmar::AESDMAR_SPEC>;
#[doc = "AESDMAR"]
pub mod aesdmar;
#[doc = "AESISR (rw) register accessor: an alias for `Reg<AESISR_SPEC>`"]
pub type AESISR = crate::Reg<aesisr::AESISR_SPEC>;
#[doc = "AESISR"]
pub mod aesisr;
#[doc = "AESIER (rw) register accessor: an alias for `Reg<AESIER_SPEC>`"]
pub type AESIER = crate::Reg<aesier::AESIER_SPEC>;
#[doc = "AESIER"]
pub mod aesier;
#[doc = "AESDINR (rw) register accessor: an alias for `Reg<AESDINR_SPEC>`"]
pub type AESDINR = crate::Reg<aesdinr::AESDINR_SPEC>;
#[doc = "AESDINR"]
pub mod aesdinr;
#[doc = "AESDOUTR (rw) register accessor: an alias for `Reg<AESDOUTR_SPEC>`"]
pub type AESDOUTR = crate::Reg<aesdoutr::AESDOUTR_SPEC>;
#[doc = "AESDOUTR"]
pub mod aesdoutr;
#[doc = "AESKEYR0 (rw) register accessor: an alias for `Reg<AESKEYR0_SPEC>`"]
pub type AESKEYR0 = crate::Reg<aeskeyr0::AESKEYR0_SPEC>;
#[doc = "AESKEYR0"]
pub mod aeskeyr0;
#[doc = "AESKEYR1 (rw) register accessor: an alias for `Reg<AESKEYR1_SPEC>`"]
pub type AESKEYR1 = crate::Reg<aeskeyr1::AESKEYR1_SPEC>;
#[doc = "AESKEYR1"]
pub mod aeskeyr1;
#[doc = "AESKEYR2 (rw) register accessor: an alias for `Reg<AESKEYR2_SPEC>`"]
pub type AESKEYR2 = crate::Reg<aeskeyr2::AESKEYR2_SPEC>;
#[doc = "AESKEYR2"]
pub mod aeskeyr2;
#[doc = "AESKEYR3 (rw) register accessor: an alias for `Reg<AESKEYR3_SPEC>`"]
pub type AESKEYR3 = crate::Reg<aeskeyr3::AESKEYR3_SPEC>;
#[doc = "AESKEYR3"]
pub mod aeskeyr3;
#[doc = "AESIVR0 (rw) register accessor: an alias for `Reg<AESIVR0_SPEC>`"]
pub type AESIVR0 = crate::Reg<aesivr0::AESIVR0_SPEC>;
#[doc = "AESIVR0"]
pub mod aesivr0;
#[doc = "AESIVR1 (rw) register accessor: an alias for `Reg<AESIVR1_SPEC>`"]
pub type AESIVR1 = crate::Reg<aesivr1::AESIVR1_SPEC>;
#[doc = "AESIVR1"]
pub mod aesivr1;
#[doc = "AESIVR2 (rw) register accessor: an alias for `Reg<AESIVR2_SPEC>`"]
pub type AESIVR2 = crate::Reg<aesivr2::AESIVR2_SPEC>;
#[doc = "AESIVR2"]
pub mod aesivr2;
#[doc = "AESIVR3 (rw) register accessor: an alias for `Reg<AESIVR3_SPEC>`"]
pub type AESIVR3 = crate::Reg<aesivr3::AESIVR3_SPEC>;
#[doc = "AESIVR3"]
pub mod aesivr3;
