#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CSIF_ENR"]
    pub csif_enr: CSIF_ENR,
    #[doc = "0x04 - CSIF_CR"]
    pub csif_cr: CSIF_CR,
    #[doc = "0x08 - CSIF_IMGWH"]
    pub csif_imgwh: CSIF_IMGWH,
    #[doc = "0x0c - CSIF_WCR0"]
    pub csif_wcr0: CSIF_WCR0,
    #[doc = "0x10 - CSIF_WCR1"]
    pub csif_wcr1: CSIF_WCR1,
    #[doc = "0x14 - CSIF_SMP"]
    pub csif_smp: CSIF_SMP,
    #[doc = "0x18 - CSIF_SMPCOL"]
    pub csif_smpcol: CSIF_SMPCOL,
    #[doc = "0x1c - CSIF_SMPROW"]
    pub csif_smprow: CSIF_SMPROW,
    #[doc = "0x20 - CSIF_FIFO0"]
    pub csif_fifo0: CSIF_FIFO0,
    #[doc = "0x24 - CSIF_FIFO1"]
    pub csif_fifo1: CSIF_FIFO1,
    #[doc = "0x28 - CSIF_FIFO2"]
    pub csif_fifo2: CSIF_FIFO2,
    #[doc = "0x2c - CSIF_FIFO3"]
    pub csif_fifo3: CSIF_FIFO3,
    #[doc = "0x30 - CSIF_FIFO4"]
    pub csif_fifo4: CSIF_FIFO4,
    #[doc = "0x34 - CSIF_FIFO5"]
    pub csif_fifo5: CSIF_FIFO5,
    #[doc = "0x38 - CSIF_FIFO6"]
    pub csif_fifo6: CSIF_FIFO6,
    #[doc = "0x3c - CSIF_FIFO7"]
    pub csif_fifo7: CSIF_FIFO7,
    #[doc = "0x40 - CSIF_IER"]
    pub csif_ier: CSIF_IER,
    #[doc = "0x44 - CSIF_SR"]
    pub csif_sr: CSIF_SR,
}
#[doc = "CSIF_ENR (rw) register accessor: an alias for `Reg<CSIF_ENR_SPEC>`"]
pub type CSIF_ENR = crate::Reg<csif_enr::CSIF_ENR_SPEC>;
#[doc = "CSIF_ENR"]
pub mod csif_enr;
#[doc = "CSIF_CR (rw) register accessor: an alias for `Reg<CSIF_CR_SPEC>`"]
pub type CSIF_CR = crate::Reg<csif_cr::CSIF_CR_SPEC>;
#[doc = "CSIF_CR"]
pub mod csif_cr;
#[doc = "CSIF_IMGWH (rw) register accessor: an alias for `Reg<CSIF_IMGWH_SPEC>`"]
pub type CSIF_IMGWH = crate::Reg<csif_imgwh::CSIF_IMGWH_SPEC>;
#[doc = "CSIF_IMGWH"]
pub mod csif_imgwh;
#[doc = "CSIF_WCR0 (rw) register accessor: an alias for `Reg<CSIF_WCR0_SPEC>`"]
pub type CSIF_WCR0 = crate::Reg<csif_wcr0::CSIF_WCR0_SPEC>;
#[doc = "CSIF_WCR0"]
pub mod csif_wcr0;
#[doc = "CSIF_WCR1 (rw) register accessor: an alias for `Reg<CSIF_WCR1_SPEC>`"]
pub type CSIF_WCR1 = crate::Reg<csif_wcr1::CSIF_WCR1_SPEC>;
#[doc = "CSIF_WCR1"]
pub mod csif_wcr1;
#[doc = "CSIF_SMP (rw) register accessor: an alias for `Reg<CSIF_SMP_SPEC>`"]
pub type CSIF_SMP = crate::Reg<csif_smp::CSIF_SMP_SPEC>;
#[doc = "CSIF_SMP"]
pub mod csif_smp;
#[doc = "CSIF_SMPCOL (rw) register accessor: an alias for `Reg<CSIF_SMPCOL_SPEC>`"]
pub type CSIF_SMPCOL = crate::Reg<csif_smpcol::CSIF_SMPCOL_SPEC>;
#[doc = "CSIF_SMPCOL"]
pub mod csif_smpcol;
#[doc = "CSIF_SMPROW (rw) register accessor: an alias for `Reg<CSIF_SMPROW_SPEC>`"]
pub type CSIF_SMPROW = crate::Reg<csif_smprow::CSIF_SMPROW_SPEC>;
#[doc = "CSIF_SMPROW"]
pub mod csif_smprow;
#[doc = "CSIF_FIFO0 (rw) register accessor: an alias for `Reg<CSIF_FIFO0_SPEC>`"]
pub type CSIF_FIFO0 = crate::Reg<csif_fifo0::CSIF_FIFO0_SPEC>;
#[doc = "CSIF_FIFO0"]
pub mod csif_fifo0;
#[doc = "CSIF_FIFO1 (rw) register accessor: an alias for `Reg<CSIF_FIFO1_SPEC>`"]
pub type CSIF_FIFO1 = crate::Reg<csif_fifo1::CSIF_FIFO1_SPEC>;
#[doc = "CSIF_FIFO1"]
pub mod csif_fifo1;
#[doc = "CSIF_FIFO2 (rw) register accessor: an alias for `Reg<CSIF_FIFO2_SPEC>`"]
pub type CSIF_FIFO2 = crate::Reg<csif_fifo2::CSIF_FIFO2_SPEC>;
#[doc = "CSIF_FIFO2"]
pub mod csif_fifo2;
#[doc = "CSIF_FIFO3 (rw) register accessor: an alias for `Reg<CSIF_FIFO3_SPEC>`"]
pub type CSIF_FIFO3 = crate::Reg<csif_fifo3::CSIF_FIFO3_SPEC>;
#[doc = "CSIF_FIFO3"]
pub mod csif_fifo3;
#[doc = "CSIF_FIFO4 (rw) register accessor: an alias for `Reg<CSIF_FIFO4_SPEC>`"]
pub type CSIF_FIFO4 = crate::Reg<csif_fifo4::CSIF_FIFO4_SPEC>;
#[doc = "CSIF_FIFO4"]
pub mod csif_fifo4;
#[doc = "CSIF_FIFO5 (rw) register accessor: an alias for `Reg<CSIF_FIFO5_SPEC>`"]
pub type CSIF_FIFO5 = crate::Reg<csif_fifo5::CSIF_FIFO5_SPEC>;
#[doc = "CSIF_FIFO5"]
pub mod csif_fifo5;
#[doc = "CSIF_FIFO6 (rw) register accessor: an alias for `Reg<CSIF_FIFO6_SPEC>`"]
pub type CSIF_FIFO6 = crate::Reg<csif_fifo6::CSIF_FIFO6_SPEC>;
#[doc = "CSIF_FIFO6"]
pub mod csif_fifo6;
#[doc = "CSIF_FIFO7 (rw) register accessor: an alias for `Reg<CSIF_FIFO7_SPEC>`"]
pub type CSIF_FIFO7 = crate::Reg<csif_fifo7::CSIF_FIFO7_SPEC>;
#[doc = "CSIF_FIFO7"]
pub mod csif_fifo7;
#[doc = "CSIF_IER (rw) register accessor: an alias for `Reg<CSIF_IER_SPEC>`"]
pub type CSIF_IER = crate::Reg<csif_ier::CSIF_IER_SPEC>;
#[doc = "CSIF_IER"]
pub mod csif_ier;
#[doc = "CSIF_SR (rw) register accessor: an alias for `Reg<CSIF_SR_SPEC>`"]
pub type CSIF_SR = crate::Reg<csif_sr::CSIF_SR_SPEC>;
#[doc = "CSIF_SR"]
pub mod csif_sr;
