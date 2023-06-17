#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - QSPI_CR0"]
    pub qspi_cr0: QSPI_CR0,
    #[doc = "0x04 - QSPI_CR1"]
    pub qspi_cr1: QSPI_CR1,
    #[doc = "0x08 - QSPI_IER"]
    pub qspi_ier: QSPI_IER,
    #[doc = "0x0c - QSPI_CPR"]
    pub qspi_cpr: QSPI_CPR,
    #[doc = "0x10 - QSPI_DR"]
    pub qspi_dr: QSPI_DR,
    #[doc = "0x14 - QSPI_SR"]
    pub qspi_sr: QSPI_SR,
    #[doc = "0x18 - QSPI_FCR"]
    pub qspi_fcr: QSPI_FCR,
    #[doc = "0x1c - QSPI_FSR"]
    pub qspi_fsr: QSPI_FSR,
    #[doc = "0x20 - QSPI_FTOCR"]
    pub qspi_ftocr: QSPI_FTOCR,
    _reserved9: [u8; 0x1c],
    #[doc = "0x40 - QSPI_MIDICR0"]
    pub qspi_midicr0: QSPI_MIDICR0,
    #[doc = "0x44 - QSPI_MIDICR1"]
    pub qspi_midicr1: QSPI_MIDICR1,
}
#[doc = "QSPI_CR0 (rw) register accessor: an alias for `Reg<QSPI_CR0_SPEC>`"]
pub type QSPI_CR0 = crate::Reg<qspi_cr0::QSPI_CR0_SPEC>;
#[doc = "QSPI_CR0"]
pub mod qspi_cr0;
#[doc = "QSPI_CR1 (rw) register accessor: an alias for `Reg<QSPI_CR1_SPEC>`"]
pub type QSPI_CR1 = crate::Reg<qspi_cr1::QSPI_CR1_SPEC>;
#[doc = "QSPI_CR1"]
pub mod qspi_cr1;
#[doc = "QSPI_IER (rw) register accessor: an alias for `Reg<QSPI_IER_SPEC>`"]
pub type QSPI_IER = crate::Reg<qspi_ier::QSPI_IER_SPEC>;
#[doc = "QSPI_IER"]
pub mod qspi_ier;
#[doc = "QSPI_CPR (rw) register accessor: an alias for `Reg<QSPI_CPR_SPEC>`"]
pub type QSPI_CPR = crate::Reg<qspi_cpr::QSPI_CPR_SPEC>;
#[doc = "QSPI_CPR"]
pub mod qspi_cpr;
#[doc = "QSPI_DR (rw) register accessor: an alias for `Reg<QSPI_DR_SPEC>`"]
pub type QSPI_DR = crate::Reg<qspi_dr::QSPI_DR_SPEC>;
#[doc = "QSPI_DR"]
pub mod qspi_dr;
#[doc = "QSPI_SR (rw) register accessor: an alias for `Reg<QSPI_SR_SPEC>`"]
pub type QSPI_SR = crate::Reg<qspi_sr::QSPI_SR_SPEC>;
#[doc = "QSPI_SR"]
pub mod qspi_sr;
#[doc = "QSPI_FCR (rw) register accessor: an alias for `Reg<QSPI_FCR_SPEC>`"]
pub type QSPI_FCR = crate::Reg<qspi_fcr::QSPI_FCR_SPEC>;
#[doc = "QSPI_FCR"]
pub mod qspi_fcr;
#[doc = "QSPI_FSR (rw) register accessor: an alias for `Reg<QSPI_FSR_SPEC>`"]
pub type QSPI_FSR = crate::Reg<qspi_fsr::QSPI_FSR_SPEC>;
#[doc = "QSPI_FSR"]
pub mod qspi_fsr;
#[doc = "QSPI_FTOCR (rw) register accessor: an alias for `Reg<QSPI_FTOCR_SPEC>`"]
pub type QSPI_FTOCR = crate::Reg<qspi_ftocr::QSPI_FTOCR_SPEC>;
#[doc = "QSPI_FTOCR"]
pub mod qspi_ftocr;
#[doc = "QSPI_MIDICR0 (rw) register accessor: an alias for `Reg<QSPI_MIDICR0_SPEC>`"]
pub type QSPI_MIDICR0 = crate::Reg<qspi_midicr0::QSPI_MIDICR0_SPEC>;
#[doc = "QSPI_MIDICR0"]
pub mod qspi_midicr0;
#[doc = "QSPI_MIDICR1 (rw) register accessor: an alias for `Reg<QSPI_MIDICR1_SPEC>`"]
pub type QSPI_MIDICR1 = crate::Reg<qspi_midicr1::QSPI_MIDICR1_SPEC>;
#[doc = "QSPI_MIDICR1"]
pub mod qspi_midicr1;
