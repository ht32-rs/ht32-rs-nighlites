#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - URDR"]
    pub urdr: URDR,
    #[doc = "0x04 - URCR"]
    pub urcr: URCR,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - URIER"]
    pub urier: URIER,
    #[doc = "0x10 - URSIFR"]
    pub ursifr: URSIFR,
    _reserved4: [u8; 0x10],
    #[doc = "0x24 - URDLR"]
    pub urdlr: URDLR,
    #[doc = "0x28 - URTSTR"]
    pub urtstr: URTSTR,
}
#[doc = "URDR (rw) register accessor: an alias for `Reg<URDR_SPEC>`"]
pub type URDR = crate::Reg<urdr::URDR_SPEC>;
#[doc = "URDR"]
pub mod urdr;
#[doc = "URCR (rw) register accessor: an alias for `Reg<URCR_SPEC>`"]
pub type URCR = crate::Reg<urcr::URCR_SPEC>;
#[doc = "URCR"]
pub mod urcr;
#[doc = "URIER (rw) register accessor: an alias for `Reg<URIER_SPEC>`"]
pub type URIER = crate::Reg<urier::URIER_SPEC>;
#[doc = "URIER"]
pub mod urier;
#[doc = "URSIFR (rw) register accessor: an alias for `Reg<URSIFR_SPEC>`"]
pub type URSIFR = crate::Reg<ursifr::URSIFR_SPEC>;
#[doc = "URSIFR"]
pub mod ursifr;
#[doc = "URDLR (rw) register accessor: an alias for `Reg<URDLR_SPEC>`"]
pub type URDLR = crate::Reg<urdlr::URDLR_SPEC>;
#[doc = "URDLR"]
pub mod urdlr;
#[doc = "URTSTR (rw) register accessor: an alias for `Reg<URTSTR_SPEC>`"]
pub type URTSTR = crate::Reg<urtstr::URTSTR_SPEC>;
#[doc = "URTSTR"]
pub mod urtstr;
