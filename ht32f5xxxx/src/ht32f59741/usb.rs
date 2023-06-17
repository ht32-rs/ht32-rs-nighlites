#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB_CSR"]
    pub usb_csr: USB_CSR,
    #[doc = "0x04 - USB_IER"]
    pub usb_ier: USB_IER,
    #[doc = "0x08 - USB_ISR"]
    pub usb_isr: USB_ISR,
    #[doc = "0x0c - USB_FCR"]
    pub usb_fcr: USB_FCR,
    #[doc = "0x10 - USB_DEVAR"]
    pub usb_devar: USB_DEVAR,
    #[doc = "0x14 - USB_EP0CSR"]
    pub usb_ep0csr: USB_EP0CSR,
    #[doc = "0x18 - USB_EP0IER"]
    pub usb_ep0ier: USB_EP0IER,
    #[doc = "0x1c - USB_EP0ISR"]
    pub usb_ep0isr: USB_EP0ISR,
    #[doc = "0x20 - USB_EP0TCR"]
    pub usb_ep0tcr: USB_EP0TCR,
    #[doc = "0x24 - USB_EP0CFGR"]
    pub usb_ep0cfgr: USB_EP0CFGR,
    #[doc = "0x28 - USB_EP1CSR"]
    pub usb_ep1csr: USB_EP1CSR,
    #[doc = "0x2c - USB_EP1IER"]
    pub usb_ep1ier: USB_EP1IER,
    #[doc = "0x30 - USB_EP1ISR"]
    pub usb_ep1isr: USB_EP1ISR,
    #[doc = "0x34 - USB_EP1TCR"]
    pub usb_ep1tcr: USB_EP1TCR,
    #[doc = "0x38 - USB_EP1CFGR"]
    pub usb_ep1cfgr: USB_EP1CFGR,
    #[doc = "0x3c - USB_EP2CSR"]
    pub usb_ep2csr: USB_EP2CSR,
    #[doc = "0x40 - USB_EP2IER"]
    pub usb_ep2ier: USB_EP2IER,
    #[doc = "0x44 - USB_EP2ISR"]
    pub usb_ep2isr: USB_EP2ISR,
    #[doc = "0x48 - USB_EP2TCR"]
    pub usb_ep2tcr: USB_EP2TCR,
    #[doc = "0x4c - USB_EP2CFGR"]
    pub usb_ep2cfgr: USB_EP2CFGR,
    #[doc = "0x50 - USB_EP3CSR"]
    pub usb_ep3csr: USB_EP3CSR,
    #[doc = "0x54 - USB_EP3IER"]
    pub usb_ep3ier: USB_EP3IER,
    #[doc = "0x58 - USB_EP3ISR"]
    pub usb_ep3isr: USB_EP3ISR,
    #[doc = "0x5c - USB_EP3TCR"]
    pub usb_ep3tcr: USB_EP3TCR,
    #[doc = "0x60 - USB_EP3CFGR"]
    pub usb_ep3cfgr: USB_EP3CFGR,
    #[doc = "0x64 - USB_EP4CSR"]
    pub usb_ep4csr: USB_EP4CSR,
    #[doc = "0x68 - USB_EP4IER"]
    pub usb_ep4ier: USB_EP4IER,
    #[doc = "0x6c - USB_EP4ISR"]
    pub usb_ep4isr: USB_EP4ISR,
    #[doc = "0x70 - USB_EP4TCR"]
    pub usb_ep4tcr: USB_EP4TCR,
    #[doc = "0x74 - USB_EP4CFGR"]
    pub usb_ep4cfgr: USB_EP4CFGR,
    #[doc = "0x78 - USB_EP5CSR"]
    pub usb_ep5csr: USB_EP5CSR,
    #[doc = "0x7c - USB_EP5IER"]
    pub usb_ep5ier: USB_EP5IER,
    #[doc = "0x80 - USB_EP5ISR"]
    pub usb_ep5isr: USB_EP5ISR,
    #[doc = "0x84 - USB_EP5TCR"]
    pub usb_ep5tcr: USB_EP5TCR,
    #[doc = "0x88 - USB_EP5CFGR"]
    pub usb_ep5cfgr: USB_EP5CFGR,
    #[doc = "0x8c - USB_EP6CSR"]
    pub usb_ep6csr: USB_EP6CSR,
    #[doc = "0x90 - USB_EP6IER"]
    pub usb_ep6ier: USB_EP6IER,
    #[doc = "0x94 - USB_EP6ISR"]
    pub usb_ep6isr: USB_EP6ISR,
    #[doc = "0x98 - USB_EP6TCR"]
    pub usb_ep6tcr: USB_EP6TCR,
    #[doc = "0x9c - USB_EP6CFGR"]
    pub usb_ep6cfgr: USB_EP6CFGR,
    #[doc = "0xa0 - USB_EP7CSR"]
    pub usb_ep7csr: USB_EP7CSR,
    #[doc = "0xa4 - USB_EP7IER"]
    pub usb_ep7ier: USB_EP7IER,
    #[doc = "0xa8 - USB_EP7ISR"]
    pub usb_ep7isr: USB_EP7ISR,
    #[doc = "0xac - USB_EP7TCR"]
    pub usb_ep7tcr: USB_EP7TCR,
    #[doc = "0xb0 - USB_EP7CFGR"]
    pub usb_ep7cfgr: USB_EP7CFGR,
}
#[doc = "USB_CSR (rw) register accessor: an alias for `Reg<USB_CSR_SPEC>`"]
pub type USB_CSR = crate::Reg<usb_csr::USB_CSR_SPEC>;
#[doc = "USB_CSR"]
pub mod usb_csr;
#[doc = "USB_IER (rw) register accessor: an alias for `Reg<USB_IER_SPEC>`"]
pub type USB_IER = crate::Reg<usb_ier::USB_IER_SPEC>;
#[doc = "USB_IER"]
pub mod usb_ier;
#[doc = "USB_ISR (rw) register accessor: an alias for `Reg<USB_ISR_SPEC>`"]
pub type USB_ISR = crate::Reg<usb_isr::USB_ISR_SPEC>;
#[doc = "USB_ISR"]
pub mod usb_isr;
#[doc = "USB_FCR (rw) register accessor: an alias for `Reg<USB_FCR_SPEC>`"]
pub type USB_FCR = crate::Reg<usb_fcr::USB_FCR_SPEC>;
#[doc = "USB_FCR"]
pub mod usb_fcr;
#[doc = "USB_DEVAR (rw) register accessor: an alias for `Reg<USB_DEVAR_SPEC>`"]
pub type USB_DEVAR = crate::Reg<usb_devar::USB_DEVAR_SPEC>;
#[doc = "USB_DEVAR"]
pub mod usb_devar;
#[doc = "USB_EP0CSR (rw) register accessor: an alias for `Reg<USB_EP0CSR_SPEC>`"]
pub type USB_EP0CSR = crate::Reg<usb_ep0csr::USB_EP0CSR_SPEC>;
#[doc = "USB_EP0CSR"]
pub mod usb_ep0csr;
#[doc = "USB_EP0IER (rw) register accessor: an alias for `Reg<USB_EP0IER_SPEC>`"]
pub type USB_EP0IER = crate::Reg<usb_ep0ier::USB_EP0IER_SPEC>;
#[doc = "USB_EP0IER"]
pub mod usb_ep0ier;
#[doc = "USB_EP0ISR (rw) register accessor: an alias for `Reg<USB_EP0ISR_SPEC>`"]
pub type USB_EP0ISR = crate::Reg<usb_ep0isr::USB_EP0ISR_SPEC>;
#[doc = "USB_EP0ISR"]
pub mod usb_ep0isr;
#[doc = "USB_EP0TCR (rw) register accessor: an alias for `Reg<USB_EP0TCR_SPEC>`"]
pub type USB_EP0TCR = crate::Reg<usb_ep0tcr::USB_EP0TCR_SPEC>;
#[doc = "USB_EP0TCR"]
pub mod usb_ep0tcr;
#[doc = "USB_EP0CFGR (rw) register accessor: an alias for `Reg<USB_EP0CFGR_SPEC>`"]
pub type USB_EP0CFGR = crate::Reg<usb_ep0cfgr::USB_EP0CFGR_SPEC>;
#[doc = "USB_EP0CFGR"]
pub mod usb_ep0cfgr;
#[doc = "USB_EP1CSR (rw) register accessor: an alias for `Reg<USB_EP1CSR_SPEC>`"]
pub type USB_EP1CSR = crate::Reg<usb_ep1csr::USB_EP1CSR_SPEC>;
#[doc = "USB_EP1CSR"]
pub mod usb_ep1csr;
#[doc = "USB_EP1IER (rw) register accessor: an alias for `Reg<USB_EP1IER_SPEC>`"]
pub type USB_EP1IER = crate::Reg<usb_ep1ier::USB_EP1IER_SPEC>;
#[doc = "USB_EP1IER"]
pub mod usb_ep1ier;
#[doc = "USB_EP1ISR (rw) register accessor: an alias for `Reg<USB_EP1ISR_SPEC>`"]
pub type USB_EP1ISR = crate::Reg<usb_ep1isr::USB_EP1ISR_SPEC>;
#[doc = "USB_EP1ISR"]
pub mod usb_ep1isr;
#[doc = "USB_EP1TCR (rw) register accessor: an alias for `Reg<USB_EP1TCR_SPEC>`"]
pub type USB_EP1TCR = crate::Reg<usb_ep1tcr::USB_EP1TCR_SPEC>;
#[doc = "USB_EP1TCR"]
pub mod usb_ep1tcr;
#[doc = "USB_EP1CFGR (rw) register accessor: an alias for `Reg<USB_EP1CFGR_SPEC>`"]
pub type USB_EP1CFGR = crate::Reg<usb_ep1cfgr::USB_EP1CFGR_SPEC>;
#[doc = "USB_EP1CFGR"]
pub mod usb_ep1cfgr;
#[doc = "USB_EP2CSR (rw) register accessor: an alias for `Reg<USB_EP2CSR_SPEC>`"]
pub type USB_EP2CSR = crate::Reg<usb_ep2csr::USB_EP2CSR_SPEC>;
#[doc = "USB_EP2CSR"]
pub mod usb_ep2csr;
#[doc = "USB_EP2IER (rw) register accessor: an alias for `Reg<USB_EP2IER_SPEC>`"]
pub type USB_EP2IER = crate::Reg<usb_ep2ier::USB_EP2IER_SPEC>;
#[doc = "USB_EP2IER"]
pub mod usb_ep2ier;
#[doc = "USB_EP2ISR (rw) register accessor: an alias for `Reg<USB_EP2ISR_SPEC>`"]
pub type USB_EP2ISR = crate::Reg<usb_ep2isr::USB_EP2ISR_SPEC>;
#[doc = "USB_EP2ISR"]
pub mod usb_ep2isr;
#[doc = "USB_EP2TCR (rw) register accessor: an alias for `Reg<USB_EP2TCR_SPEC>`"]
pub type USB_EP2TCR = crate::Reg<usb_ep2tcr::USB_EP2TCR_SPEC>;
#[doc = "USB_EP2TCR"]
pub mod usb_ep2tcr;
#[doc = "USB_EP2CFGR (rw) register accessor: an alias for `Reg<USB_EP2CFGR_SPEC>`"]
pub type USB_EP2CFGR = crate::Reg<usb_ep2cfgr::USB_EP2CFGR_SPEC>;
#[doc = "USB_EP2CFGR"]
pub mod usb_ep2cfgr;
#[doc = "USB_EP3CSR (rw) register accessor: an alias for `Reg<USB_EP3CSR_SPEC>`"]
pub type USB_EP3CSR = crate::Reg<usb_ep3csr::USB_EP3CSR_SPEC>;
#[doc = "USB_EP3CSR"]
pub mod usb_ep3csr;
#[doc = "USB_EP3IER (rw) register accessor: an alias for `Reg<USB_EP3IER_SPEC>`"]
pub type USB_EP3IER = crate::Reg<usb_ep3ier::USB_EP3IER_SPEC>;
#[doc = "USB_EP3IER"]
pub mod usb_ep3ier;
#[doc = "USB_EP3ISR (rw) register accessor: an alias for `Reg<USB_EP3ISR_SPEC>`"]
pub type USB_EP3ISR = crate::Reg<usb_ep3isr::USB_EP3ISR_SPEC>;
#[doc = "USB_EP3ISR"]
pub mod usb_ep3isr;
#[doc = "USB_EP3TCR (rw) register accessor: an alias for `Reg<USB_EP3TCR_SPEC>`"]
pub type USB_EP3TCR = crate::Reg<usb_ep3tcr::USB_EP3TCR_SPEC>;
#[doc = "USB_EP3TCR"]
pub mod usb_ep3tcr;
#[doc = "USB_EP3CFGR (rw) register accessor: an alias for `Reg<USB_EP3CFGR_SPEC>`"]
pub type USB_EP3CFGR = crate::Reg<usb_ep3cfgr::USB_EP3CFGR_SPEC>;
#[doc = "USB_EP3CFGR"]
pub mod usb_ep3cfgr;
#[doc = "USB_EP4CSR (rw) register accessor: an alias for `Reg<USB_EP4CSR_SPEC>`"]
pub type USB_EP4CSR = crate::Reg<usb_ep4csr::USB_EP4CSR_SPEC>;
#[doc = "USB_EP4CSR"]
pub mod usb_ep4csr;
#[doc = "USB_EP4IER (rw) register accessor: an alias for `Reg<USB_EP4IER_SPEC>`"]
pub type USB_EP4IER = crate::Reg<usb_ep4ier::USB_EP4IER_SPEC>;
#[doc = "USB_EP4IER"]
pub mod usb_ep4ier;
#[doc = "USB_EP4ISR (rw) register accessor: an alias for `Reg<USB_EP4ISR_SPEC>`"]
pub type USB_EP4ISR = crate::Reg<usb_ep4isr::USB_EP4ISR_SPEC>;
#[doc = "USB_EP4ISR"]
pub mod usb_ep4isr;
#[doc = "USB_EP4TCR (rw) register accessor: an alias for `Reg<USB_EP4TCR_SPEC>`"]
pub type USB_EP4TCR = crate::Reg<usb_ep4tcr::USB_EP4TCR_SPEC>;
#[doc = "USB_EP4TCR"]
pub mod usb_ep4tcr;
#[doc = "USB_EP4CFGR (rw) register accessor: an alias for `Reg<USB_EP4CFGR_SPEC>`"]
pub type USB_EP4CFGR = crate::Reg<usb_ep4cfgr::USB_EP4CFGR_SPEC>;
#[doc = "USB_EP4CFGR"]
pub mod usb_ep4cfgr;
#[doc = "USB_EP5CSR (rw) register accessor: an alias for `Reg<USB_EP5CSR_SPEC>`"]
pub type USB_EP5CSR = crate::Reg<usb_ep5csr::USB_EP5CSR_SPEC>;
#[doc = "USB_EP5CSR"]
pub mod usb_ep5csr;
#[doc = "USB_EP5IER (rw) register accessor: an alias for `Reg<USB_EP5IER_SPEC>`"]
pub type USB_EP5IER = crate::Reg<usb_ep5ier::USB_EP5IER_SPEC>;
#[doc = "USB_EP5IER"]
pub mod usb_ep5ier;
#[doc = "USB_EP5ISR (rw) register accessor: an alias for `Reg<USB_EP5ISR_SPEC>`"]
pub type USB_EP5ISR = crate::Reg<usb_ep5isr::USB_EP5ISR_SPEC>;
#[doc = "USB_EP5ISR"]
pub mod usb_ep5isr;
#[doc = "USB_EP5TCR (rw) register accessor: an alias for `Reg<USB_EP5TCR_SPEC>`"]
pub type USB_EP5TCR = crate::Reg<usb_ep5tcr::USB_EP5TCR_SPEC>;
#[doc = "USB_EP5TCR"]
pub mod usb_ep5tcr;
#[doc = "USB_EP5CFGR (rw) register accessor: an alias for `Reg<USB_EP5CFGR_SPEC>`"]
pub type USB_EP5CFGR = crate::Reg<usb_ep5cfgr::USB_EP5CFGR_SPEC>;
#[doc = "USB_EP5CFGR"]
pub mod usb_ep5cfgr;
#[doc = "USB_EP6CSR (rw) register accessor: an alias for `Reg<USB_EP6CSR_SPEC>`"]
pub type USB_EP6CSR = crate::Reg<usb_ep6csr::USB_EP6CSR_SPEC>;
#[doc = "USB_EP6CSR"]
pub mod usb_ep6csr;
#[doc = "USB_EP6IER (rw) register accessor: an alias for `Reg<USB_EP6IER_SPEC>`"]
pub type USB_EP6IER = crate::Reg<usb_ep6ier::USB_EP6IER_SPEC>;
#[doc = "USB_EP6IER"]
pub mod usb_ep6ier;
#[doc = "USB_EP6ISR (rw) register accessor: an alias for `Reg<USB_EP6ISR_SPEC>`"]
pub type USB_EP6ISR = crate::Reg<usb_ep6isr::USB_EP6ISR_SPEC>;
#[doc = "USB_EP6ISR"]
pub mod usb_ep6isr;
#[doc = "USB_EP6TCR (rw) register accessor: an alias for `Reg<USB_EP6TCR_SPEC>`"]
pub type USB_EP6TCR = crate::Reg<usb_ep6tcr::USB_EP6TCR_SPEC>;
#[doc = "USB_EP6TCR"]
pub mod usb_ep6tcr;
#[doc = "USB_EP6CFGR (rw) register accessor: an alias for `Reg<USB_EP6CFGR_SPEC>`"]
pub type USB_EP6CFGR = crate::Reg<usb_ep6cfgr::USB_EP6CFGR_SPEC>;
#[doc = "USB_EP6CFGR"]
pub mod usb_ep6cfgr;
#[doc = "USB_EP7CSR (rw) register accessor: an alias for `Reg<USB_EP7CSR_SPEC>`"]
pub type USB_EP7CSR = crate::Reg<usb_ep7csr::USB_EP7CSR_SPEC>;
#[doc = "USB_EP7CSR"]
pub mod usb_ep7csr;
#[doc = "USB_EP7IER (rw) register accessor: an alias for `Reg<USB_EP7IER_SPEC>`"]
pub type USB_EP7IER = crate::Reg<usb_ep7ier::USB_EP7IER_SPEC>;
#[doc = "USB_EP7IER"]
pub mod usb_ep7ier;
#[doc = "USB_EP7ISR (rw) register accessor: an alias for `Reg<USB_EP7ISR_SPEC>`"]
pub type USB_EP7ISR = crate::Reg<usb_ep7isr::USB_EP7ISR_SPEC>;
#[doc = "USB_EP7ISR"]
pub mod usb_ep7isr;
#[doc = "USB_EP7TCR (rw) register accessor: an alias for `Reg<USB_EP7TCR_SPEC>`"]
pub type USB_EP7TCR = crate::Reg<usb_ep7tcr::USB_EP7TCR_SPEC>;
#[doc = "USB_EP7TCR"]
pub mod usb_ep7tcr;
#[doc = "USB_EP7CFGR (rw) register accessor: an alias for `Reg<USB_EP7CFGR_SPEC>`"]
pub type USB_EP7CFGR = crate::Reg<usb_ep7cfgr::USB_EP7CFGR_SPEC>;
#[doc = "USB_EP7CFGR"]
pub mod usb_ep7cfgr;
