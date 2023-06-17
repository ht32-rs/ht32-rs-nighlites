#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AFIO_ESSR0"]
    pub afio_essr0: AFIO_ESSR0,
    #[doc = "0x04 - AFIO_ESSR1"]
    pub afio_essr1: AFIO_ESSR1,
    #[doc = "0x08 - AFIO_GPACFGR"]
    pub afio_gpacfgr: AFIO_GPACFGR,
    #[doc = "0x0c - AFIO_GPBCFGR"]
    pub afio_gpbcfgr: AFIO_GPBCFGR,
    #[doc = "0x10 - AFIO_GPCCFGR"]
    pub afio_gpccfgr: AFIO_GPCCFGR,
    #[doc = "0x14 - AFIO_GPDCFGR"]
    pub afio_gpdcfgr: AFIO_GPDCFGR,
    #[doc = "0x18 - AFIO_GPECFGR"]
    pub afio_gpecfgr: AFIO_GPECFGR,
}
#[doc = "AFIO_ESSR0 (rw) register accessor: an alias for `Reg<AFIO_ESSR0_SPEC>`"]
pub type AFIO_ESSR0 = crate::Reg<afio_essr0::AFIO_ESSR0_SPEC>;
#[doc = "AFIO_ESSR0"]
pub mod afio_essr0;
#[doc = "AFIO_ESSR1 (rw) register accessor: an alias for `Reg<AFIO_ESSR1_SPEC>`"]
pub type AFIO_ESSR1 = crate::Reg<afio_essr1::AFIO_ESSR1_SPEC>;
#[doc = "AFIO_ESSR1"]
pub mod afio_essr1;
#[doc = "AFIO_GPACFGR (rw) register accessor: an alias for `Reg<AFIO_GPACFGR_SPEC>`"]
pub type AFIO_GPACFGR = crate::Reg<afio_gpacfgr::AFIO_GPACFGR_SPEC>;
#[doc = "AFIO_GPACFGR"]
pub mod afio_gpacfgr;
#[doc = "AFIO_GPBCFGR (rw) register accessor: an alias for `Reg<AFIO_GPBCFGR_SPEC>`"]
pub type AFIO_GPBCFGR = crate::Reg<afio_gpbcfgr::AFIO_GPBCFGR_SPEC>;
#[doc = "AFIO_GPBCFGR"]
pub mod afio_gpbcfgr;
#[doc = "AFIO_GPCCFGR (rw) register accessor: an alias for `Reg<AFIO_GPCCFGR_SPEC>`"]
pub type AFIO_GPCCFGR = crate::Reg<afio_gpccfgr::AFIO_GPCCFGR_SPEC>;
#[doc = "AFIO_GPCCFGR"]
pub mod afio_gpccfgr;
#[doc = "AFIO_GPDCFGR (rw) register accessor: an alias for `Reg<AFIO_GPDCFGR_SPEC>`"]
pub type AFIO_GPDCFGR = crate::Reg<afio_gpdcfgr::AFIO_GPDCFGR_SPEC>;
#[doc = "AFIO_GPDCFGR"]
pub mod afio_gpdcfgr;
#[doc = "AFIO_GPECFGR (rw) register accessor: an alias for `Reg<AFIO_GPECFGR_SPEC>`"]
pub type AFIO_GPECFGR = crate::Reg<afio_gpecfgr::AFIO_GPECFGR_SPEC>;
#[doc = "AFIO_GPECFGR"]
pub mod afio_gpecfgr;
