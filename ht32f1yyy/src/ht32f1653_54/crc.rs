#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC_CR"]
    pub crc_cr: CRC_CR,
    #[doc = "0x04 - CRC_SDR"]
    pub crc_sdr: CRC_SDR,
    #[doc = "0x08 - CRC_CSR"]
    pub crc_csr: CRC_CSR,
    #[doc = "0x0c - CRC_DR"]
    pub crc_dr: CRC_DR,
}
#[doc = "CRC_CR (rw) register accessor: an alias for `Reg<CRC_CR_SPEC>`"]
pub type CRC_CR = crate::Reg<crc_cr::CRC_CR_SPEC>;
#[doc = "CRC_CR"]
pub mod crc_cr;
#[doc = "CRC_SDR (rw) register accessor: an alias for `Reg<CRC_SDR_SPEC>`"]
pub type CRC_SDR = crate::Reg<crc_sdr::CRC_SDR_SPEC>;
#[doc = "CRC_SDR"]
pub mod crc_sdr;
#[doc = "CRC_CSR (rw) register accessor: an alias for `Reg<CRC_CSR_SPEC>`"]
pub type CRC_CSR = crate::Reg<crc_csr::CRC_CSR_SPEC>;
#[doc = "CRC_CSR"]
pub mod crc_csr;
#[doc = "CRC_DR (rw) register accessor: an alias for `Reg<CRC_DR_SPEC>`"]
pub type CRC_DR = crate::Reg<crc_dr::CRC_DR_SPEC>;
#[doc = "CRC_DR"]
pub mod crc_dr;
