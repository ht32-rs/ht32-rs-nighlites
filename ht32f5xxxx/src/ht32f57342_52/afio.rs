#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AFIO_ESSR0"]
    pub afio_essr0: AFIO_ESSR0,
    #[doc = "0x04 - AFIO_ESSR1"]
    pub afio_essr1: AFIO_ESSR1,
    _reserved2: [u8; 0x18],
    #[doc = "0x20 - AFIO_GPACFGLR"]
    pub afio_gpacfglr: AFIO_GPACFGLR,
    #[doc = "0x24 - AFIO_GPACFGHR"]
    pub afio_gpacfghr: AFIO_GPACFGHR,
    #[doc = "0x28 - AFIO_GPBCFGLR"]
    pub afio_gpbcfglr: AFIO_GPBCFGLR,
    #[doc = "0x2c - AFIO_GPBCFGHR"]
    pub afio_gpbcfghr: AFIO_GPBCFGHR,
    #[doc = "0x30 - AFIO_GPCCFGLR"]
    pub afio_gpccfglr: AFIO_GPCCFGLR,
    #[doc = "0x34 - AFIO_GPCCFGHR"]
    pub afio_gpccfghr: AFIO_GPCCFGHR,
    #[doc = "0x38 - AFIO_GPDCFGLR"]
    pub afio_gpdcfglr: AFIO_GPDCFGLR,
    #[doc = "0x3c - AFIO_GPDCFGHR"]
    pub afio_gpdcfghr: AFIO_GPDCFGHR,
    #[doc = "0x40 - AFIO_GPECFGLR"]
    pub afio_gpecfglr: AFIO_GPECFGLR,
    #[doc = "0x44 - AFIO_GPECFGHR"]
    pub afio_gpecfghr: AFIO_GPECFGHR,
}
#[doc = "AFIO_ESSR0 (rw) register accessor: an alias for `Reg<AFIO_ESSR0_SPEC>`"]
pub type AFIO_ESSR0 = crate::Reg<afio_essr0::AFIO_ESSR0_SPEC>;
#[doc = "AFIO_ESSR0"]
pub mod afio_essr0;
#[doc = "AFIO_ESSR1 (rw) register accessor: an alias for `Reg<AFIO_ESSR1_SPEC>`"]
pub type AFIO_ESSR1 = crate::Reg<afio_essr1::AFIO_ESSR1_SPEC>;
#[doc = "AFIO_ESSR1"]
pub mod afio_essr1;
#[doc = "AFIO_GPACFGLR (rw) register accessor: an alias for `Reg<AFIO_GPACFGLR_SPEC>`"]
pub type AFIO_GPACFGLR = crate::Reg<afio_gpacfglr::AFIO_GPACFGLR_SPEC>;
#[doc = "AFIO_GPACFGLR"]
pub mod afio_gpacfglr;
#[doc = "AFIO_GPACFGHR (rw) register accessor: an alias for `Reg<AFIO_GPACFGHR_SPEC>`"]
pub type AFIO_GPACFGHR = crate::Reg<afio_gpacfghr::AFIO_GPACFGHR_SPEC>;
#[doc = "AFIO_GPACFGHR"]
pub mod afio_gpacfghr;
#[doc = "AFIO_GPBCFGLR (rw) register accessor: an alias for `Reg<AFIO_GPBCFGLR_SPEC>`"]
pub type AFIO_GPBCFGLR = crate::Reg<afio_gpbcfglr::AFIO_GPBCFGLR_SPEC>;
#[doc = "AFIO_GPBCFGLR"]
pub mod afio_gpbcfglr;
#[doc = "AFIO_GPBCFGHR (rw) register accessor: an alias for `Reg<AFIO_GPBCFGHR_SPEC>`"]
pub type AFIO_GPBCFGHR = crate::Reg<afio_gpbcfghr::AFIO_GPBCFGHR_SPEC>;
#[doc = "AFIO_GPBCFGHR"]
pub mod afio_gpbcfghr;
#[doc = "AFIO_GPCCFGLR (rw) register accessor: an alias for `Reg<AFIO_GPCCFGLR_SPEC>`"]
pub type AFIO_GPCCFGLR = crate::Reg<afio_gpccfglr::AFIO_GPCCFGLR_SPEC>;
#[doc = "AFIO_GPCCFGLR"]
pub mod afio_gpccfglr;
#[doc = "AFIO_GPCCFGHR (rw) register accessor: an alias for `Reg<AFIO_GPCCFGHR_SPEC>`"]
pub type AFIO_GPCCFGHR = crate::Reg<afio_gpccfghr::AFIO_GPCCFGHR_SPEC>;
#[doc = "AFIO_GPCCFGHR"]
pub mod afio_gpccfghr;
#[doc = "AFIO_GPDCFGLR (rw) register accessor: an alias for `Reg<AFIO_GPDCFGLR_SPEC>`"]
pub type AFIO_GPDCFGLR = crate::Reg<afio_gpdcfglr::AFIO_GPDCFGLR_SPEC>;
#[doc = "AFIO_GPDCFGLR"]
pub mod afio_gpdcfglr;
#[doc = "AFIO_GPDCFGHR (rw) register accessor: an alias for `Reg<AFIO_GPDCFGHR_SPEC>`"]
pub type AFIO_GPDCFGHR = crate::Reg<afio_gpdcfghr::AFIO_GPDCFGHR_SPEC>;
#[doc = "AFIO_GPDCFGHR"]
pub mod afio_gpdcfghr;
#[doc = "AFIO_GPECFGLR (rw) register accessor: an alias for `Reg<AFIO_GPECFGLR_SPEC>`"]
pub type AFIO_GPECFGLR = crate::Reg<afio_gpecfglr::AFIO_GPECFGLR_SPEC>;
#[doc = "AFIO_GPECFGLR"]
pub mod afio_gpecfglr;
#[doc = "AFIO_GPECFGHR (rw) register accessor: an alias for `Reg<AFIO_GPECFGHR_SPEC>`"]
pub type AFIO_GPECFGHR = crate::Reg<afio_gpecfghr::AFIO_GPECFGHR_SPEC>;
#[doc = "AFIO_GPECFGHR"]
pub mod afio_gpecfghr;
