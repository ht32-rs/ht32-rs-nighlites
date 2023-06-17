#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB control bits and USB data line status"]
    pub usb_csr: USB_CSR,
    #[doc = "0x04 - USB interrupt enable control"]
    pub usb_ier: USB_IER,
    #[doc = "0x08 - USB interrupt status"]
    pub usb_isr: USB_ISR,
    #[doc = "0x0c - Lost Start-of-Frame number and the USB frame count"]
    pub usb_fcr: USB_FCR,
    #[doc = "0x10 - USB device address"]
    pub usb_devar: USB_DEVAR,
    #[doc = "0x14..0x28 - USB control endpoint"]
    pub ep0: EP0,
    #[doc = "0x28..0x64 - USB single-buffered endpoints"]
    pub eps: [EPS; 3],
    #[doc = "0x64..0xb4 - USB double-buffered endpoints"]
    pub epd: [EPD; 4],
}
impl RegisterBlock {
    #[doc = "0x28..0x3c - USB single-buffered endpoints"]
    #[inline(always)]
    pub fn ep1s(&self) -> &EPS {
        &self.eps[0]
    }
    #[doc = "0x3c..0x50 - USB single-buffered endpoints"]
    #[inline(always)]
    pub fn ep2s(&self) -> &EPS {
        &self.eps[1]
    }
    #[doc = "0x50..0x64 - USB single-buffered endpoints"]
    #[inline(always)]
    pub fn ep3s(&self) -> &EPS {
        &self.eps[2]
    }
    #[doc = "0x64..0x78 - USB double-buffered endpoints"]
    #[inline(always)]
    pub fn ep4d(&self) -> &EPD {
        &self.epd[0]
    }
    #[doc = "0x78..0x8c - USB double-buffered endpoints"]
    #[inline(always)]
    pub fn ep5d(&self) -> &EPD {
        &self.epd[1]
    }
    #[doc = "0x8c..0xa0 - USB double-buffered endpoints"]
    #[inline(always)]
    pub fn ep6d(&self) -> &EPD {
        &self.epd[2]
    }
    #[doc = "0xa0..0xb4 - USB double-buffered endpoints"]
    #[inline(always)]
    pub fn ep7d(&self) -> &EPD {
        &self.epd[3]
    }
}
#[doc = "USB_CSR (rw) register accessor: an alias for `Reg<USB_CSR_SPEC>`"]
pub type USB_CSR = crate::Reg<usb_csr::USB_CSR_SPEC>;
#[doc = "USB control bits and USB data line status"]
pub mod usb_csr;
#[doc = "USB_IER (rw) register accessor: an alias for `Reg<USB_IER_SPEC>`"]
pub type USB_IER = crate::Reg<usb_ier::USB_IER_SPEC>;
#[doc = "USB interrupt enable control"]
pub mod usb_ier;
#[doc = "USB_ISR (rw) register accessor: an alias for `Reg<USB_ISR_SPEC>`"]
pub type USB_ISR = crate::Reg<usb_isr::USB_ISR_SPEC>;
#[doc = "USB interrupt status"]
pub mod usb_isr;
#[doc = "USB_FCR (rw) register accessor: an alias for `Reg<USB_FCR_SPEC>`"]
pub type USB_FCR = crate::Reg<usb_fcr::USB_FCR_SPEC>;
#[doc = "Lost Start-of-Frame number and the USB frame count"]
pub mod usb_fcr;
#[doc = "USB_DEVAR (rw) register accessor: an alias for `Reg<USB_DEVAR_SPEC>`"]
pub type USB_DEVAR = crate::Reg<usb_devar::USB_DEVAR_SPEC>;
#[doc = "USB device address"]
pub mod usb_devar;
#[doc = "USB control endpoint"]
pub use self::ep0::EP0;
#[doc = r"Cluster"]
#[doc = "USB control endpoint"]
pub mod ep0;
#[doc = "USB single-buffered endpoints"]
pub use self::eps::EPS;
#[doc = r"Cluster"]
#[doc = "USB single-buffered endpoints"]
pub mod eps;
#[doc = "USB double-buffered endpoints"]
pub use self::epd::EPD;
#[doc = r"Cluster"]
#[doc = "USB double-buffered endpoints"]
pub mod epd;
