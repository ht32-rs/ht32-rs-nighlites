#[doc = r"Register block"]
#[repr(C)]
pub struct EP0 {
    #[doc = "0x00 - USB_EP0CSR"]
    pub csr: CSR,
    #[doc = "0x04 - USB_EP0IER"]
    pub ier: IER,
    #[doc = "0x08 - USB_EP0ISR"]
    pub isr: ISR,
    #[doc = "0x0c - USB_EP0TCR"]
    pub tcr: TCR,
    #[doc = "0x10 - USB_EP0CFGR"]
    pub cfgr: CFGR,
}
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "USB_EP0CSR"]
pub mod csr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "USB_EP0IER"]
pub mod ier;
#[doc = "ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "USB_EP0ISR"]
pub mod isr;
#[doc = "TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "USB_EP0TCR"]
pub mod tcr;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "USB_EP0CFGR"]
pub mod cfgr;
