#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES_AESCR"]
    pub aes_aescr: AES_AESCR,
    #[doc = "0x04 - AES_AESSR"]
    pub aes_aessr: AES_AESSR,
    #[doc = "0x08 - AES_AESDMAR"]
    pub aes_aesdmar: AES_AESDMAR,
    #[doc = "0x0c - AES_AESISR"]
    pub aes_aesisr: AES_AESISR,
    #[doc = "0x10 - AES_AESIER"]
    pub aes_aesier: AES_AESIER,
    #[doc = "0x14 - AES_AESDINR"]
    pub aes_aesdinr: AES_AESDINR,
    #[doc = "0x18 - AES_AESDOUTR"]
    pub aes_aesdoutr: AES_AESDOUTR,
    #[doc = "0x1c - AES_AESKEYR0"]
    pub aes_aeskeyr0: AES_AESKEYR0,
    #[doc = "0x20 - AES_AESKEYR1"]
    pub aes_aeskeyr1: AES_AESKEYR1,
    #[doc = "0x24 - AES_AESKEYR2"]
    pub aes_aeskeyr2: AES_AESKEYR2,
    #[doc = "0x28 - AES_AESKEYR3"]
    pub aes_aeskeyr3: AES_AESKEYR3,
    _reserved11: [u8; 0x10],
    #[doc = "0x3c - AES_AESIVR0"]
    pub aes_aesivr0: AES_AESIVR0,
    #[doc = "0x40 - AES_AESIVR1"]
    pub aes_aesivr1: AES_AESIVR1,
    #[doc = "0x44 - AES_AESIVR2"]
    pub aes_aesivr2: AES_AESIVR2,
    #[doc = "0x48 - AES_AESIVR3"]
    pub aes_aesivr3: AES_AESIVR3,
}
#[doc = "AES_AESCR (rw) register accessor: an alias for `Reg<AES_AESCR_SPEC>`"]
pub type AES_AESCR = crate::Reg<aes_aescr::AES_AESCR_SPEC>;
#[doc = "AES_AESCR"]
pub mod aes_aescr;
#[doc = "AES_AESSR (rw) register accessor: an alias for `Reg<AES_AESSR_SPEC>`"]
pub type AES_AESSR = crate::Reg<aes_aessr::AES_AESSR_SPEC>;
#[doc = "AES_AESSR"]
pub mod aes_aessr;
#[doc = "AES_AESDMAR (rw) register accessor: an alias for `Reg<AES_AESDMAR_SPEC>`"]
pub type AES_AESDMAR = crate::Reg<aes_aesdmar::AES_AESDMAR_SPEC>;
#[doc = "AES_AESDMAR"]
pub mod aes_aesdmar;
#[doc = "AES_AESISR (rw) register accessor: an alias for `Reg<AES_AESISR_SPEC>`"]
pub type AES_AESISR = crate::Reg<aes_aesisr::AES_AESISR_SPEC>;
#[doc = "AES_AESISR"]
pub mod aes_aesisr;
#[doc = "AES_AESIER (rw) register accessor: an alias for `Reg<AES_AESIER_SPEC>`"]
pub type AES_AESIER = crate::Reg<aes_aesier::AES_AESIER_SPEC>;
#[doc = "AES_AESIER"]
pub mod aes_aesier;
#[doc = "AES_AESDINR (rw) register accessor: an alias for `Reg<AES_AESDINR_SPEC>`"]
pub type AES_AESDINR = crate::Reg<aes_aesdinr::AES_AESDINR_SPEC>;
#[doc = "AES_AESDINR"]
pub mod aes_aesdinr;
#[doc = "AES_AESDOUTR (rw) register accessor: an alias for `Reg<AES_AESDOUTR_SPEC>`"]
pub type AES_AESDOUTR = crate::Reg<aes_aesdoutr::AES_AESDOUTR_SPEC>;
#[doc = "AES_AESDOUTR"]
pub mod aes_aesdoutr;
#[doc = "AES_AESKEYR0 (rw) register accessor: an alias for `Reg<AES_AESKEYR0_SPEC>`"]
pub type AES_AESKEYR0 = crate::Reg<aes_aeskeyr0::AES_AESKEYR0_SPEC>;
#[doc = "AES_AESKEYR0"]
pub mod aes_aeskeyr0;
#[doc = "AES_AESKEYR1 (rw) register accessor: an alias for `Reg<AES_AESKEYR1_SPEC>`"]
pub type AES_AESKEYR1 = crate::Reg<aes_aeskeyr1::AES_AESKEYR1_SPEC>;
#[doc = "AES_AESKEYR1"]
pub mod aes_aeskeyr1;
#[doc = "AES_AESKEYR2 (rw) register accessor: an alias for `Reg<AES_AESKEYR2_SPEC>`"]
pub type AES_AESKEYR2 = crate::Reg<aes_aeskeyr2::AES_AESKEYR2_SPEC>;
#[doc = "AES_AESKEYR2"]
pub mod aes_aeskeyr2;
#[doc = "AES_AESKEYR3 (rw) register accessor: an alias for `Reg<AES_AESKEYR3_SPEC>`"]
pub type AES_AESKEYR3 = crate::Reg<aes_aeskeyr3::AES_AESKEYR3_SPEC>;
#[doc = "AES_AESKEYR3"]
pub mod aes_aeskeyr3;
#[doc = "AES_AESIVR0 (rw) register accessor: an alias for `Reg<AES_AESIVR0_SPEC>`"]
pub type AES_AESIVR0 = crate::Reg<aes_aesivr0::AES_AESIVR0_SPEC>;
#[doc = "AES_AESIVR0"]
pub mod aes_aesivr0;
#[doc = "AES_AESIVR1 (rw) register accessor: an alias for `Reg<AES_AESIVR1_SPEC>`"]
pub type AES_AESIVR1 = crate::Reg<aes_aesivr1::AES_AESIVR1_SPEC>;
#[doc = "AES_AESIVR1"]
pub mod aes_aesivr1;
#[doc = "AES_AESIVR2 (rw) register accessor: an alias for `Reg<AES_AESIVR2_SPEC>`"]
pub type AES_AESIVR2 = crate::Reg<aes_aesivr2::AES_AESIVR2_SPEC>;
#[doc = "AES_AESIVR2"]
pub mod aes_aesivr2;
#[doc = "AES_AESIVR3 (rw) register accessor: an alias for `Reg<AES_AESIVR3_SPEC>`"]
pub type AES_AESIVR3 = crate::Reg<aes_aesivr3::AES_AESIVR3_SPEC>;
#[doc = "AES_AESIVR3"]
pub mod aes_aesivr3;
