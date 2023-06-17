#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USBCSR"]
    pub usbcsr: USBCSR,
    #[doc = "0x04 - USBIER"]
    pub usbier: USBIER,
    #[doc = "0x08 - USBISR"]
    pub usbisr: USBISR,
    #[doc = "0x0c - USBFCR"]
    pub usbfcr: USBFCR,
    #[doc = "0x10 - USBDEVAR"]
    pub usbdevar: USBDEVAR,
    #[doc = "0x14 - USBEP0CSR"]
    pub usbep0csr: USBEP0CSR,
    #[doc = "0x18 - USBEP0IER"]
    pub usbep0ier: USBEP0IER,
    #[doc = "0x1c - USBEP0ISR"]
    pub usbep0isr: USBEP0ISR,
    #[doc = "0x20 - USBEP0TCR"]
    pub usbep0tcr: USBEP0TCR,
    #[doc = "0x24 - USBEP0CFGR"]
    pub usbep0cfgr: USBEP0CFGR,
    #[doc = "0x28 - USBEP1CSR"]
    pub usbep1csr: USBEP1CSR,
    #[doc = "0x2c - USBEP1IER"]
    pub usbep1ier: USBEP1IER,
    #[doc = "0x30 - USBEP1ISR"]
    pub usbep1isr: USBEP1ISR,
    #[doc = "0x34 - USBEP1TCR"]
    pub usbep1tcr: USBEP1TCR,
    #[doc = "0x38 - USBEP1CFGR"]
    pub usbep1cfgr: USBEP1CFGR,
    #[doc = "0x3c - USBEP2CSR"]
    pub usbep2csr: USBEP2CSR,
    #[doc = "0x40 - USBEP2IER"]
    pub usbep2ier: USBEP2IER,
    #[doc = "0x44 - USBEP2ISR"]
    pub usbep2isr: USBEP2ISR,
    #[doc = "0x48 - USBEP2TCR"]
    pub usbep2tcr: USBEP2TCR,
    #[doc = "0x4c - USBEP2CFGR"]
    pub usbep2cfgr: USBEP2CFGR,
    #[doc = "0x50 - USBEP3CSR"]
    pub usbep3csr: USBEP3CSR,
    #[doc = "0x54 - USBEP3IER"]
    pub usbep3ier: USBEP3IER,
    #[doc = "0x58 - USBEP3ISR"]
    pub usbep3isr: USBEP3ISR,
    #[doc = "0x5c - USBEP3TCR"]
    pub usbep3tcr: USBEP3TCR,
    #[doc = "0x60 - USBEP3CFGR"]
    pub usbep3cfgr: USBEP3CFGR,
    #[doc = "0x64 - USBEP4CSR"]
    pub usbep4csr: USBEP4CSR,
    #[doc = "0x68 - USBEP4IER"]
    pub usbep4ier: USBEP4IER,
    #[doc = "0x6c - USBEP4ISR"]
    pub usbep4isr: USBEP4ISR,
    #[doc = "0x70 - USBEP4TCR"]
    pub usbep4tcr: USBEP4TCR,
    #[doc = "0x74 - USBEP4CFGR"]
    pub usbep4cfgr: USBEP4CFGR,
    #[doc = "0x78 - USBEP5CSR"]
    pub usbep5csr: USBEP5CSR,
    #[doc = "0x7c - USBEP5IER"]
    pub usbep5ier: USBEP5IER,
    #[doc = "0x80 - USBEP5ISR"]
    pub usbep5isr: USBEP5ISR,
    #[doc = "0x84 - USBEP5TCR"]
    pub usbep5tcr: USBEP5TCR,
    #[doc = "0x88 - USBEP5CFGR"]
    pub usbep5cfgr: USBEP5CFGR,
    #[doc = "0x8c - USBEP6CSR"]
    pub usbep6csr: USBEP6CSR,
    #[doc = "0x90 - USBEP6IER"]
    pub usbep6ier: USBEP6IER,
    #[doc = "0x94 - USBEP6ISR"]
    pub usbep6isr: USBEP6ISR,
    #[doc = "0x98 - USBEP6TCR"]
    pub usbep6tcr: USBEP6TCR,
    #[doc = "0x9c - USBEP6CFGR"]
    pub usbep6cfgr: USBEP6CFGR,
    #[doc = "0xa0 - USBEP7CSR"]
    pub usbep7csr: USBEP7CSR,
    #[doc = "0xa4 - USBEP7IER"]
    pub usbep7ier: USBEP7IER,
    #[doc = "0xa8 - USBEP7ISR"]
    pub usbep7isr: USBEP7ISR,
    #[doc = "0xac - USBEP7TCR"]
    pub usbep7tcr: USBEP7TCR,
    #[doc = "0xb0 - USBEP7CFGR"]
    pub usbep7cfgr: USBEP7CFGR,
}
#[doc = "USBCSR (rw) register accessor: an alias for `Reg<USBCSR_SPEC>`"]
pub type USBCSR = crate::Reg<usbcsr::USBCSR_SPEC>;
#[doc = "USBCSR"]
pub mod usbcsr;
#[doc = "USBIER (rw) register accessor: an alias for `Reg<USBIER_SPEC>`"]
pub type USBIER = crate::Reg<usbier::USBIER_SPEC>;
#[doc = "USBIER"]
pub mod usbier;
#[doc = "USBISR (rw) register accessor: an alias for `Reg<USBISR_SPEC>`"]
pub type USBISR = crate::Reg<usbisr::USBISR_SPEC>;
#[doc = "USBISR"]
pub mod usbisr;
#[doc = "USBFCR (rw) register accessor: an alias for `Reg<USBFCR_SPEC>`"]
pub type USBFCR = crate::Reg<usbfcr::USBFCR_SPEC>;
#[doc = "USBFCR"]
pub mod usbfcr;
#[doc = "USBDEVAR (rw) register accessor: an alias for `Reg<USBDEVAR_SPEC>`"]
pub type USBDEVAR = crate::Reg<usbdevar::USBDEVAR_SPEC>;
#[doc = "USBDEVAR"]
pub mod usbdevar;
#[doc = "USBEP0CSR (rw) register accessor: an alias for `Reg<USBEP0CSR_SPEC>`"]
pub type USBEP0CSR = crate::Reg<usbep0csr::USBEP0CSR_SPEC>;
#[doc = "USBEP0CSR"]
pub mod usbep0csr;
#[doc = "USBEP0IER (rw) register accessor: an alias for `Reg<USBEP0IER_SPEC>`"]
pub type USBEP0IER = crate::Reg<usbep0ier::USBEP0IER_SPEC>;
#[doc = "USBEP0IER"]
pub mod usbep0ier;
#[doc = "USBEP0ISR (rw) register accessor: an alias for `Reg<USBEP0ISR_SPEC>`"]
pub type USBEP0ISR = crate::Reg<usbep0isr::USBEP0ISR_SPEC>;
#[doc = "USBEP0ISR"]
pub mod usbep0isr;
#[doc = "USBEP0TCR (rw) register accessor: an alias for `Reg<USBEP0TCR_SPEC>`"]
pub type USBEP0TCR = crate::Reg<usbep0tcr::USBEP0TCR_SPEC>;
#[doc = "USBEP0TCR"]
pub mod usbep0tcr;
#[doc = "USBEP0CFGR (rw) register accessor: an alias for `Reg<USBEP0CFGR_SPEC>`"]
pub type USBEP0CFGR = crate::Reg<usbep0cfgr::USBEP0CFGR_SPEC>;
#[doc = "USBEP0CFGR"]
pub mod usbep0cfgr;
#[doc = "USBEP1CSR (rw) register accessor: an alias for `Reg<USBEP1CSR_SPEC>`"]
pub type USBEP1CSR = crate::Reg<usbep1csr::USBEP1CSR_SPEC>;
#[doc = "USBEP1CSR"]
pub mod usbep1csr;
#[doc = "USBEP1IER (rw) register accessor: an alias for `Reg<USBEP1IER_SPEC>`"]
pub type USBEP1IER = crate::Reg<usbep1ier::USBEP1IER_SPEC>;
#[doc = "USBEP1IER"]
pub mod usbep1ier;
#[doc = "USBEP1ISR (rw) register accessor: an alias for `Reg<USBEP1ISR_SPEC>`"]
pub type USBEP1ISR = crate::Reg<usbep1isr::USBEP1ISR_SPEC>;
#[doc = "USBEP1ISR"]
pub mod usbep1isr;
#[doc = "USBEP1TCR (rw) register accessor: an alias for `Reg<USBEP1TCR_SPEC>`"]
pub type USBEP1TCR = crate::Reg<usbep1tcr::USBEP1TCR_SPEC>;
#[doc = "USBEP1TCR"]
pub mod usbep1tcr;
#[doc = "USBEP1CFGR (rw) register accessor: an alias for `Reg<USBEP1CFGR_SPEC>`"]
pub type USBEP1CFGR = crate::Reg<usbep1cfgr::USBEP1CFGR_SPEC>;
#[doc = "USBEP1CFGR"]
pub mod usbep1cfgr;
#[doc = "USBEP2CSR (rw) register accessor: an alias for `Reg<USBEP2CSR_SPEC>`"]
pub type USBEP2CSR = crate::Reg<usbep2csr::USBEP2CSR_SPEC>;
#[doc = "USBEP2CSR"]
pub mod usbep2csr;
#[doc = "USBEP2IER (rw) register accessor: an alias for `Reg<USBEP2IER_SPEC>`"]
pub type USBEP2IER = crate::Reg<usbep2ier::USBEP2IER_SPEC>;
#[doc = "USBEP2IER"]
pub mod usbep2ier;
#[doc = "USBEP2ISR (rw) register accessor: an alias for `Reg<USBEP2ISR_SPEC>`"]
pub type USBEP2ISR = crate::Reg<usbep2isr::USBEP2ISR_SPEC>;
#[doc = "USBEP2ISR"]
pub mod usbep2isr;
#[doc = "USBEP2TCR (rw) register accessor: an alias for `Reg<USBEP2TCR_SPEC>`"]
pub type USBEP2TCR = crate::Reg<usbep2tcr::USBEP2TCR_SPEC>;
#[doc = "USBEP2TCR"]
pub mod usbep2tcr;
#[doc = "USBEP2CFGR (rw) register accessor: an alias for `Reg<USBEP2CFGR_SPEC>`"]
pub type USBEP2CFGR = crate::Reg<usbep2cfgr::USBEP2CFGR_SPEC>;
#[doc = "USBEP2CFGR"]
pub mod usbep2cfgr;
#[doc = "USBEP3CSR (rw) register accessor: an alias for `Reg<USBEP3CSR_SPEC>`"]
pub type USBEP3CSR = crate::Reg<usbep3csr::USBEP3CSR_SPEC>;
#[doc = "USBEP3CSR"]
pub mod usbep3csr;
#[doc = "USBEP3IER (rw) register accessor: an alias for `Reg<USBEP3IER_SPEC>`"]
pub type USBEP3IER = crate::Reg<usbep3ier::USBEP3IER_SPEC>;
#[doc = "USBEP3IER"]
pub mod usbep3ier;
#[doc = "USBEP3ISR (rw) register accessor: an alias for `Reg<USBEP3ISR_SPEC>`"]
pub type USBEP3ISR = crate::Reg<usbep3isr::USBEP3ISR_SPEC>;
#[doc = "USBEP3ISR"]
pub mod usbep3isr;
#[doc = "USBEP3TCR (rw) register accessor: an alias for `Reg<USBEP3TCR_SPEC>`"]
pub type USBEP3TCR = crate::Reg<usbep3tcr::USBEP3TCR_SPEC>;
#[doc = "USBEP3TCR"]
pub mod usbep3tcr;
#[doc = "USBEP3CFGR (rw) register accessor: an alias for `Reg<USBEP3CFGR_SPEC>`"]
pub type USBEP3CFGR = crate::Reg<usbep3cfgr::USBEP3CFGR_SPEC>;
#[doc = "USBEP3CFGR"]
pub mod usbep3cfgr;
#[doc = "USBEP4CSR (rw) register accessor: an alias for `Reg<USBEP4CSR_SPEC>`"]
pub type USBEP4CSR = crate::Reg<usbep4csr::USBEP4CSR_SPEC>;
#[doc = "USBEP4CSR"]
pub mod usbep4csr;
#[doc = "USBEP4IER (rw) register accessor: an alias for `Reg<USBEP4IER_SPEC>`"]
pub type USBEP4IER = crate::Reg<usbep4ier::USBEP4IER_SPEC>;
#[doc = "USBEP4IER"]
pub mod usbep4ier;
#[doc = "USBEP4ISR (rw) register accessor: an alias for `Reg<USBEP4ISR_SPEC>`"]
pub type USBEP4ISR = crate::Reg<usbep4isr::USBEP4ISR_SPEC>;
#[doc = "USBEP4ISR"]
pub mod usbep4isr;
#[doc = "USBEP4TCR (rw) register accessor: an alias for `Reg<USBEP4TCR_SPEC>`"]
pub type USBEP4TCR = crate::Reg<usbep4tcr::USBEP4TCR_SPEC>;
#[doc = "USBEP4TCR"]
pub mod usbep4tcr;
#[doc = "USBEP4CFGR (rw) register accessor: an alias for `Reg<USBEP4CFGR_SPEC>`"]
pub type USBEP4CFGR = crate::Reg<usbep4cfgr::USBEP4CFGR_SPEC>;
#[doc = "USBEP4CFGR"]
pub mod usbep4cfgr;
#[doc = "USBEP5CSR (rw) register accessor: an alias for `Reg<USBEP5CSR_SPEC>`"]
pub type USBEP5CSR = crate::Reg<usbep5csr::USBEP5CSR_SPEC>;
#[doc = "USBEP5CSR"]
pub mod usbep5csr;
#[doc = "USBEP5IER (rw) register accessor: an alias for `Reg<USBEP5IER_SPEC>`"]
pub type USBEP5IER = crate::Reg<usbep5ier::USBEP5IER_SPEC>;
#[doc = "USBEP5IER"]
pub mod usbep5ier;
#[doc = "USBEP5ISR (rw) register accessor: an alias for `Reg<USBEP5ISR_SPEC>`"]
pub type USBEP5ISR = crate::Reg<usbep5isr::USBEP5ISR_SPEC>;
#[doc = "USBEP5ISR"]
pub mod usbep5isr;
#[doc = "USBEP5TCR (rw) register accessor: an alias for `Reg<USBEP5TCR_SPEC>`"]
pub type USBEP5TCR = crate::Reg<usbep5tcr::USBEP5TCR_SPEC>;
#[doc = "USBEP5TCR"]
pub mod usbep5tcr;
#[doc = "USBEP5CFGR (rw) register accessor: an alias for `Reg<USBEP5CFGR_SPEC>`"]
pub type USBEP5CFGR = crate::Reg<usbep5cfgr::USBEP5CFGR_SPEC>;
#[doc = "USBEP5CFGR"]
pub mod usbep5cfgr;
#[doc = "USBEP6CSR (rw) register accessor: an alias for `Reg<USBEP6CSR_SPEC>`"]
pub type USBEP6CSR = crate::Reg<usbep6csr::USBEP6CSR_SPEC>;
#[doc = "USBEP6CSR"]
pub mod usbep6csr;
#[doc = "USBEP6IER (rw) register accessor: an alias for `Reg<USBEP6IER_SPEC>`"]
pub type USBEP6IER = crate::Reg<usbep6ier::USBEP6IER_SPEC>;
#[doc = "USBEP6IER"]
pub mod usbep6ier;
#[doc = "USBEP6ISR (rw) register accessor: an alias for `Reg<USBEP6ISR_SPEC>`"]
pub type USBEP6ISR = crate::Reg<usbep6isr::USBEP6ISR_SPEC>;
#[doc = "USBEP6ISR"]
pub mod usbep6isr;
#[doc = "USBEP6TCR (rw) register accessor: an alias for `Reg<USBEP6TCR_SPEC>`"]
pub type USBEP6TCR = crate::Reg<usbep6tcr::USBEP6TCR_SPEC>;
#[doc = "USBEP6TCR"]
pub mod usbep6tcr;
#[doc = "USBEP6CFGR (rw) register accessor: an alias for `Reg<USBEP6CFGR_SPEC>`"]
pub type USBEP6CFGR = crate::Reg<usbep6cfgr::USBEP6CFGR_SPEC>;
#[doc = "USBEP6CFGR"]
pub mod usbep6cfgr;
#[doc = "USBEP7CSR (rw) register accessor: an alias for `Reg<USBEP7CSR_SPEC>`"]
pub type USBEP7CSR = crate::Reg<usbep7csr::USBEP7CSR_SPEC>;
#[doc = "USBEP7CSR"]
pub mod usbep7csr;
#[doc = "USBEP7IER (rw) register accessor: an alias for `Reg<USBEP7IER_SPEC>`"]
pub type USBEP7IER = crate::Reg<usbep7ier::USBEP7IER_SPEC>;
#[doc = "USBEP7IER"]
pub mod usbep7ier;
#[doc = "USBEP7ISR (rw) register accessor: an alias for `Reg<USBEP7ISR_SPEC>`"]
pub type USBEP7ISR = crate::Reg<usbep7isr::USBEP7ISR_SPEC>;
#[doc = "USBEP7ISR"]
pub mod usbep7isr;
#[doc = "USBEP7TCR (rw) register accessor: an alias for `Reg<USBEP7TCR_SPEC>`"]
pub type USBEP7TCR = crate::Reg<usbep7tcr::USBEP7TCR_SPEC>;
#[doc = "USBEP7TCR"]
pub mod usbep7tcr;
#[doc = "USBEP7CFGR (rw) register accessor: an alias for `Reg<USBEP7CFGR_SPEC>`"]
pub type USBEP7CFGR = crate::Reg<usbep7cfgr::USBEP7CFGR_SPEC>;
#[doc = "USBEP7CFGR"]
pub mod usbep7cfgr;
