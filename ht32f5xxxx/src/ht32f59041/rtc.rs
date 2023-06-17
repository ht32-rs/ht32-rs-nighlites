#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTCCNT"]
    pub rtccnt: RTCCNT,
    #[doc = "0x04 - RTCCMP"]
    pub rtccmp: RTCCMP,
    #[doc = "0x08 - RTCCR"]
    pub rtccr: RTCCR,
    #[doc = "0x0c - RTCSR"]
    pub rtcsr: RTCSR,
    #[doc = "0x10 - RTCIWEN"]
    pub rtciwen: RTCIWEN,
}
#[doc = "RTCCNT (rw) register accessor: an alias for `Reg<RTCCNT_SPEC>`"]
pub type RTCCNT = crate::Reg<rtccnt::RTCCNT_SPEC>;
#[doc = "RTCCNT"]
pub mod rtccnt;
#[doc = "RTCCMP (rw) register accessor: an alias for `Reg<RTCCMP_SPEC>`"]
pub type RTCCMP = crate::Reg<rtccmp::RTCCMP_SPEC>;
#[doc = "RTCCMP"]
pub mod rtccmp;
#[doc = "RTCCR (rw) register accessor: an alias for `Reg<RTCCR_SPEC>`"]
pub type RTCCR = crate::Reg<rtccr::RTCCR_SPEC>;
#[doc = "RTCCR"]
pub mod rtccr;
#[doc = "RTCSR (rw) register accessor: an alias for `Reg<RTCSR_SPEC>`"]
pub type RTCSR = crate::Reg<rtcsr::RTCSR_SPEC>;
#[doc = "RTCSR"]
pub mod rtcsr;
#[doc = "RTCIWEN (rw) register accessor: an alias for `Reg<RTCIWEN_SPEC>`"]
pub type RTCIWEN = crate::Reg<rtciwen::RTCIWEN_SPEC>;
#[doc = "RTCIWEN"]
pub mod rtciwen;
