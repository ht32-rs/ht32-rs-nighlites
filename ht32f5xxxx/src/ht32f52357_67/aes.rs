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
    #[doc = "0x1c - AESKEY0"]
    pub aeskey0: AESKEY0,
    #[doc = "0x20 - AESKEY1"]
    pub aeskey1: AESKEY1,
    #[doc = "0x24 - AESKEY2"]
    pub aeskey2: AESKEY2,
    #[doc = "0x28 - AESKEY3"]
    pub aeskey3: AESKEY3,
    #[doc = "0x2c - AESKEY4"]
    pub aeskey4: AESKEY4,
    #[doc = "0x30 - AESKEY5"]
    pub aeskey5: AESKEY5,
    #[doc = "0x34 - AESKEY6"]
    pub aeskey6: AESKEY6,
    #[doc = "0x38 - AESKEY7"]
    pub aeskey7: AESKEY7,
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
#[doc = "AESKEY0 (rw) register accessor: an alias for `Reg<AESKEY0_SPEC>`"]
pub type AESKEY0 = crate::Reg<aeskey0::AESKEY0_SPEC>;
#[doc = "AESKEY0"]
pub mod aeskey0;
#[doc = "AESKEY1 (rw) register accessor: an alias for `Reg<AESKEY1_SPEC>`"]
pub type AESKEY1 = crate::Reg<aeskey1::AESKEY1_SPEC>;
#[doc = "AESKEY1"]
pub mod aeskey1;
#[doc = "AESKEY2 (rw) register accessor: an alias for `Reg<AESKEY2_SPEC>`"]
pub type AESKEY2 = crate::Reg<aeskey2::AESKEY2_SPEC>;
#[doc = "AESKEY2"]
pub mod aeskey2;
#[doc = "AESKEY3 (rw) register accessor: an alias for `Reg<AESKEY3_SPEC>`"]
pub type AESKEY3 = crate::Reg<aeskey3::AESKEY3_SPEC>;
#[doc = "AESKEY3"]
pub mod aeskey3;
#[doc = "AESKEY4 (rw) register accessor: an alias for `Reg<AESKEY4_SPEC>`"]
pub type AESKEY4 = crate::Reg<aeskey4::AESKEY4_SPEC>;
#[doc = "AESKEY4"]
pub mod aeskey4;
#[doc = "AESKEY5 (rw) register accessor: an alias for `Reg<AESKEY5_SPEC>`"]
pub type AESKEY5 = crate::Reg<aeskey5::AESKEY5_SPEC>;
#[doc = "AESKEY5"]
pub mod aeskey5;
#[doc = "AESKEY6 (rw) register accessor: an alias for `Reg<AESKEY6_SPEC>`"]
pub type AESKEY6 = crate::Reg<aeskey6::AESKEY6_SPEC>;
#[doc = "AESKEY6"]
pub mod aeskey6;
#[doc = "AESKEY7 (rw) register accessor: an alias for `Reg<AESKEY7_SPEC>`"]
pub type AESKEY7 = crate::Reg<aeskey7::AESKEY7_SPEC>;
#[doc = "AESKEY7"]
pub mod aeskey7;
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
