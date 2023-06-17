#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI_CR0"]
    pub spi_cr0: SPI_CR0,
    #[doc = "0x04 - SPI_CR1"]
    pub spi_cr1: SPI_CR1,
    #[doc = "0x08 - SPI_IER"]
    pub spi_ier: SPI_IER,
    #[doc = "0x0c - SPI_CPR"]
    pub spi_cpr: SPI_CPR,
    #[doc = "0x10 - SPI_DR"]
    pub spi_dr: SPI_DR,
    #[doc = "0x14 - SPI_SR"]
    pub spi_sr: SPI_SR,
    #[doc = "0x18 - SPI_FCR"]
    pub spi_fcr: SPI_FCR,
    #[doc = "0x1c - SPI_FSR"]
    pub spi_fsr: SPI_FSR,
    #[doc = "0x20 - SPI_FTOCR"]
    pub spi_ftocr: SPI_FTOCR,
}
#[doc = "SPI_CR0 (rw) register accessor: an alias for `Reg<SPI_CR0_SPEC>`"]
pub type SPI_CR0 = crate::Reg<spi_cr0::SPI_CR0_SPEC>;
#[doc = "SPI_CR0"]
pub mod spi_cr0;
#[doc = "SPI_CR1 (rw) register accessor: an alias for `Reg<SPI_CR1_SPEC>`"]
pub type SPI_CR1 = crate::Reg<spi_cr1::SPI_CR1_SPEC>;
#[doc = "SPI_CR1"]
pub mod spi_cr1;
#[doc = "SPI_IER (rw) register accessor: an alias for `Reg<SPI_IER_SPEC>`"]
pub type SPI_IER = crate::Reg<spi_ier::SPI_IER_SPEC>;
#[doc = "SPI_IER"]
pub mod spi_ier;
#[doc = "SPI_CPR (rw) register accessor: an alias for `Reg<SPI_CPR_SPEC>`"]
pub type SPI_CPR = crate::Reg<spi_cpr::SPI_CPR_SPEC>;
#[doc = "SPI_CPR"]
pub mod spi_cpr;
#[doc = "SPI_DR (rw) register accessor: an alias for `Reg<SPI_DR_SPEC>`"]
pub type SPI_DR = crate::Reg<spi_dr::SPI_DR_SPEC>;
#[doc = "SPI_DR"]
pub mod spi_dr;
#[doc = "SPI_SR (rw) register accessor: an alias for `Reg<SPI_SR_SPEC>`"]
pub type SPI_SR = crate::Reg<spi_sr::SPI_SR_SPEC>;
#[doc = "SPI_SR"]
pub mod spi_sr;
#[doc = "SPI_FCR (rw) register accessor: an alias for `Reg<SPI_FCR_SPEC>`"]
pub type SPI_FCR = crate::Reg<spi_fcr::SPI_FCR_SPEC>;
#[doc = "SPI_FCR"]
pub mod spi_fcr;
#[doc = "SPI_FSR (rw) register accessor: an alias for `Reg<SPI_FSR_SPEC>`"]
pub type SPI_FSR = crate::Reg<spi_fsr::SPI_FSR_SPEC>;
#[doc = "SPI_FSR"]
pub mod spi_fsr;
#[doc = "SPI_FTOCR (rw) register accessor: an alias for `Reg<SPI_FTOCR_SPEC>`"]
pub type SPI_FTOCR = crate::Reg<spi_ftocr::SPI_FTOCR_SPEC>;
#[doc = "SPI_FTOCR"]
pub mod spi_ftocr;
