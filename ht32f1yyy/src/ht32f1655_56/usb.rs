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
#[doc = "USB_CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_csr](usb_csr) module"]
pub type USB_CSR = crate::Reg<u32, _USB_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CSR;
#[doc = "`read()` method returns [usb_csr::R](usb_csr::R) reader structure"]
impl crate::Readable for USB_CSR {}
#[doc = "`write(|w| ..)` method takes [usb_csr::W](usb_csr::W) writer structure"]
impl crate::Writable for USB_CSR {}
#[doc = "USB_CSR"]
pub mod usb_csr;
#[doc = "USB_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ier](usb_ier) module"]
pub type USB_IER = crate::Reg<u32, _USB_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_IER;
#[doc = "`read()` method returns [usb_ier::R](usb_ier::R) reader structure"]
impl crate::Readable for USB_IER {}
#[doc = "`write(|w| ..)` method takes [usb_ier::W](usb_ier::W) writer structure"]
impl crate::Writable for USB_IER {}
#[doc = "USB_IER"]
pub mod usb_ier;
#[doc = "USB_ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_isr](usb_isr) module"]
pub type USB_ISR = crate::Reg<u32, _USB_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_ISR;
#[doc = "`read()` method returns [usb_isr::R](usb_isr::R) reader structure"]
impl crate::Readable for USB_ISR {}
#[doc = "`write(|w| ..)` method takes [usb_isr::W](usb_isr::W) writer structure"]
impl crate::Writable for USB_ISR {}
#[doc = "USB_ISR"]
pub mod usb_isr;
#[doc = "USB_FCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_fcr](usb_fcr) module"]
pub type USB_FCR = crate::Reg<u32, _USB_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_FCR;
#[doc = "`read()` method returns [usb_fcr::R](usb_fcr::R) reader structure"]
impl crate::Readable for USB_FCR {}
#[doc = "`write(|w| ..)` method takes [usb_fcr::W](usb_fcr::W) writer structure"]
impl crate::Writable for USB_FCR {}
#[doc = "USB_FCR"]
pub mod usb_fcr;
#[doc = "USB_DEVAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_devar](usb_devar) module"]
pub type USB_DEVAR = crate::Reg<u32, _USB_DEVAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_DEVAR;
#[doc = "`read()` method returns [usb_devar::R](usb_devar::R) reader structure"]
impl crate::Readable for USB_DEVAR {}
#[doc = "`write(|w| ..)` method takes [usb_devar::W](usb_devar::W) writer structure"]
impl crate::Writable for USB_DEVAR {}
#[doc = "USB_DEVAR"]
pub mod usb_devar;
#[doc = "USB_EP0CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep0csr](usb_ep0csr) module"]
pub type USB_EP0CSR = crate::Reg<u32, _USB_EP0CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP0CSR;
#[doc = "`read()` method returns [usb_ep0csr::R](usb_ep0csr::R) reader structure"]
impl crate::Readable for USB_EP0CSR {}
#[doc = "`write(|w| ..)` method takes [usb_ep0csr::W](usb_ep0csr::W) writer structure"]
impl crate::Writable for USB_EP0CSR {}
#[doc = "USB_EP0CSR"]
pub mod usb_ep0csr;
#[doc = "USB_EP0IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep0ier](usb_ep0ier) module"]
pub type USB_EP0IER = crate::Reg<u32, _USB_EP0IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP0IER;
#[doc = "`read()` method returns [usb_ep0ier::R](usb_ep0ier::R) reader structure"]
impl crate::Readable for USB_EP0IER {}
#[doc = "`write(|w| ..)` method takes [usb_ep0ier::W](usb_ep0ier::W) writer structure"]
impl crate::Writable for USB_EP0IER {}
#[doc = "USB_EP0IER"]
pub mod usb_ep0ier;
#[doc = "USB_EP0ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep0isr](usb_ep0isr) module"]
pub type USB_EP0ISR = crate::Reg<u32, _USB_EP0ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP0ISR;
#[doc = "`read()` method returns [usb_ep0isr::R](usb_ep0isr::R) reader structure"]
impl crate::Readable for USB_EP0ISR {}
#[doc = "`write(|w| ..)` method takes [usb_ep0isr::W](usb_ep0isr::W) writer structure"]
impl crate::Writable for USB_EP0ISR {}
#[doc = "USB_EP0ISR"]
pub mod usb_ep0isr;
#[doc = "USB_EP0TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep0tcr](usb_ep0tcr) module"]
pub type USB_EP0TCR = crate::Reg<u32, _USB_EP0TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP0TCR;
#[doc = "`read()` method returns [usb_ep0tcr::R](usb_ep0tcr::R) reader structure"]
impl crate::Readable for USB_EP0TCR {}
#[doc = "`write(|w| ..)` method takes [usb_ep0tcr::W](usb_ep0tcr::W) writer structure"]
impl crate::Writable for USB_EP0TCR {}
#[doc = "USB_EP0TCR"]
pub mod usb_ep0tcr;
#[doc = "USB_EP0CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep0cfgr](usb_ep0cfgr) module"]
pub type USB_EP0CFGR = crate::Reg<u32, _USB_EP0CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP0CFGR;
#[doc = "`read()` method returns [usb_ep0cfgr::R](usb_ep0cfgr::R) reader structure"]
impl crate::Readable for USB_EP0CFGR {}
#[doc = "`write(|w| ..)` method takes [usb_ep0cfgr::W](usb_ep0cfgr::W) writer structure"]
impl crate::Writable for USB_EP0CFGR {}
#[doc = "USB_EP0CFGR"]
pub mod usb_ep0cfgr;
#[doc = "USB_EP1CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep1csr](usb_ep1csr) module"]
pub type USB_EP1CSR = crate::Reg<u32, _USB_EP1CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP1CSR;
#[doc = "`read()` method returns [usb_ep1csr::R](usb_ep1csr::R) reader structure"]
impl crate::Readable for USB_EP1CSR {}
#[doc = "`write(|w| ..)` method takes [usb_ep1csr::W](usb_ep1csr::W) writer structure"]
impl crate::Writable for USB_EP1CSR {}
#[doc = "USB_EP1CSR"]
pub mod usb_ep1csr;
#[doc = "USB_EP1IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep1ier](usb_ep1ier) module"]
pub type USB_EP1IER = crate::Reg<u32, _USB_EP1IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP1IER;
#[doc = "`read()` method returns [usb_ep1ier::R](usb_ep1ier::R) reader structure"]
impl crate::Readable for USB_EP1IER {}
#[doc = "`write(|w| ..)` method takes [usb_ep1ier::W](usb_ep1ier::W) writer structure"]
impl crate::Writable for USB_EP1IER {}
#[doc = "USB_EP1IER"]
pub mod usb_ep1ier;
#[doc = "USB_EP1ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep1isr](usb_ep1isr) module"]
pub type USB_EP1ISR = crate::Reg<u32, _USB_EP1ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP1ISR;
#[doc = "`read()` method returns [usb_ep1isr::R](usb_ep1isr::R) reader structure"]
impl crate::Readable for USB_EP1ISR {}
#[doc = "`write(|w| ..)` method takes [usb_ep1isr::W](usb_ep1isr::W) writer structure"]
impl crate::Writable for USB_EP1ISR {}
#[doc = "USB_EP1ISR"]
pub mod usb_ep1isr;
#[doc = "USB_EP1TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep1tcr](usb_ep1tcr) module"]
pub type USB_EP1TCR = crate::Reg<u32, _USB_EP1TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP1TCR;
#[doc = "`read()` method returns [usb_ep1tcr::R](usb_ep1tcr::R) reader structure"]
impl crate::Readable for USB_EP1TCR {}
#[doc = "`write(|w| ..)` method takes [usb_ep1tcr::W](usb_ep1tcr::W) writer structure"]
impl crate::Writable for USB_EP1TCR {}
#[doc = "USB_EP1TCR"]
pub mod usb_ep1tcr;
#[doc = "USB_EP1CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep1cfgr](usb_ep1cfgr) module"]
pub type USB_EP1CFGR = crate::Reg<u32, _USB_EP1CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP1CFGR;
#[doc = "`read()` method returns [usb_ep1cfgr::R](usb_ep1cfgr::R) reader structure"]
impl crate::Readable for USB_EP1CFGR {}
#[doc = "`write(|w| ..)` method takes [usb_ep1cfgr::W](usb_ep1cfgr::W) writer structure"]
impl crate::Writable for USB_EP1CFGR {}
#[doc = "USB_EP1CFGR"]
pub mod usb_ep1cfgr;
#[doc = "USB_EP2CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep2csr](usb_ep2csr) module"]
pub type USB_EP2CSR = crate::Reg<u32, _USB_EP2CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP2CSR;
#[doc = "`read()` method returns [usb_ep2csr::R](usb_ep2csr::R) reader structure"]
impl crate::Readable for USB_EP2CSR {}
#[doc = "`write(|w| ..)` method takes [usb_ep2csr::W](usb_ep2csr::W) writer structure"]
impl crate::Writable for USB_EP2CSR {}
#[doc = "USB_EP2CSR"]
pub mod usb_ep2csr;
#[doc = "USB_EP2IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep2ier](usb_ep2ier) module"]
pub type USB_EP2IER = crate::Reg<u32, _USB_EP2IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP2IER;
#[doc = "`read()` method returns [usb_ep2ier::R](usb_ep2ier::R) reader structure"]
impl crate::Readable for USB_EP2IER {}
#[doc = "`write(|w| ..)` method takes [usb_ep2ier::W](usb_ep2ier::W) writer structure"]
impl crate::Writable for USB_EP2IER {}
#[doc = "USB_EP2IER"]
pub mod usb_ep2ier;
#[doc = "USB_EP2ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep2isr](usb_ep2isr) module"]
pub type USB_EP2ISR = crate::Reg<u32, _USB_EP2ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP2ISR;
#[doc = "`read()` method returns [usb_ep2isr::R](usb_ep2isr::R) reader structure"]
impl crate::Readable for USB_EP2ISR {}
#[doc = "`write(|w| ..)` method takes [usb_ep2isr::W](usb_ep2isr::W) writer structure"]
impl crate::Writable for USB_EP2ISR {}
#[doc = "USB_EP2ISR"]
pub mod usb_ep2isr;
#[doc = "USB_EP2TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep2tcr](usb_ep2tcr) module"]
pub type USB_EP2TCR = crate::Reg<u32, _USB_EP2TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP2TCR;
#[doc = "`read()` method returns [usb_ep2tcr::R](usb_ep2tcr::R) reader structure"]
impl crate::Readable for USB_EP2TCR {}
#[doc = "`write(|w| ..)` method takes [usb_ep2tcr::W](usb_ep2tcr::W) writer structure"]
impl crate::Writable for USB_EP2TCR {}
#[doc = "USB_EP2TCR"]
pub mod usb_ep2tcr;
#[doc = "USB_EP2CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep2cfgr](usb_ep2cfgr) module"]
pub type USB_EP2CFGR = crate::Reg<u32, _USB_EP2CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP2CFGR;
#[doc = "`read()` method returns [usb_ep2cfgr::R](usb_ep2cfgr::R) reader structure"]
impl crate::Readable for USB_EP2CFGR {}
#[doc = "`write(|w| ..)` method takes [usb_ep2cfgr::W](usb_ep2cfgr::W) writer structure"]
impl crate::Writable for USB_EP2CFGR {}
#[doc = "USB_EP2CFGR"]
pub mod usb_ep2cfgr;
#[doc = "USB_EP3CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep3csr](usb_ep3csr) module"]
pub type USB_EP3CSR = crate::Reg<u32, _USB_EP3CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP3CSR;
#[doc = "`read()` method returns [usb_ep3csr::R](usb_ep3csr::R) reader structure"]
impl crate::Readable for USB_EP3CSR {}
#[doc = "`write(|w| ..)` method takes [usb_ep3csr::W](usb_ep3csr::W) writer structure"]
impl crate::Writable for USB_EP3CSR {}
#[doc = "USB_EP3CSR"]
pub mod usb_ep3csr;
#[doc = "USB_EP3IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep3ier](usb_ep3ier) module"]
pub type USB_EP3IER = crate::Reg<u32, _USB_EP3IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP3IER;
#[doc = "`read()` method returns [usb_ep3ier::R](usb_ep3ier::R) reader structure"]
impl crate::Readable for USB_EP3IER {}
#[doc = "`write(|w| ..)` method takes [usb_ep3ier::W](usb_ep3ier::W) writer structure"]
impl crate::Writable for USB_EP3IER {}
#[doc = "USB_EP3IER"]
pub mod usb_ep3ier;
#[doc = "USB_EP3ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep3isr](usb_ep3isr) module"]
pub type USB_EP3ISR = crate::Reg<u32, _USB_EP3ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP3ISR;
#[doc = "`read()` method returns [usb_ep3isr::R](usb_ep3isr::R) reader structure"]
impl crate::Readable for USB_EP3ISR {}
#[doc = "`write(|w| ..)` method takes [usb_ep3isr::W](usb_ep3isr::W) writer structure"]
impl crate::Writable for USB_EP3ISR {}
#[doc = "USB_EP3ISR"]
pub mod usb_ep3isr;
#[doc = "USB_EP3TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep3tcr](usb_ep3tcr) module"]
pub type USB_EP3TCR = crate::Reg<u32, _USB_EP3TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP3TCR;
#[doc = "`read()` method returns [usb_ep3tcr::R](usb_ep3tcr::R) reader structure"]
impl crate::Readable for USB_EP3TCR {}
#[doc = "`write(|w| ..)` method takes [usb_ep3tcr::W](usb_ep3tcr::W) writer structure"]
impl crate::Writable for USB_EP3TCR {}
#[doc = "USB_EP3TCR"]
pub mod usb_ep3tcr;
#[doc = "USB_EP3CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep3cfgr](usb_ep3cfgr) module"]
pub type USB_EP3CFGR = crate::Reg<u32, _USB_EP3CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP3CFGR;
#[doc = "`read()` method returns [usb_ep3cfgr::R](usb_ep3cfgr::R) reader structure"]
impl crate::Readable for USB_EP3CFGR {}
#[doc = "`write(|w| ..)` method takes [usb_ep3cfgr::W](usb_ep3cfgr::W) writer structure"]
impl crate::Writable for USB_EP3CFGR {}
#[doc = "USB_EP3CFGR"]
pub mod usb_ep3cfgr;
#[doc = "USB_EP4CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep4csr](usb_ep4csr) module"]
pub type USB_EP4CSR = crate::Reg<u32, _USB_EP4CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP4CSR;
#[doc = "`read()` method returns [usb_ep4csr::R](usb_ep4csr::R) reader structure"]
impl crate::Readable for USB_EP4CSR {}
#[doc = "`write(|w| ..)` method takes [usb_ep4csr::W](usb_ep4csr::W) writer structure"]
impl crate::Writable for USB_EP4CSR {}
#[doc = "USB_EP4CSR"]
pub mod usb_ep4csr;
#[doc = "USB_EP4IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep4ier](usb_ep4ier) module"]
pub type USB_EP4IER = crate::Reg<u32, _USB_EP4IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP4IER;
#[doc = "`read()` method returns [usb_ep4ier::R](usb_ep4ier::R) reader structure"]
impl crate::Readable for USB_EP4IER {}
#[doc = "`write(|w| ..)` method takes [usb_ep4ier::W](usb_ep4ier::W) writer structure"]
impl crate::Writable for USB_EP4IER {}
#[doc = "USB_EP4IER"]
pub mod usb_ep4ier;
#[doc = "USB_EP4ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep4isr](usb_ep4isr) module"]
pub type USB_EP4ISR = crate::Reg<u32, _USB_EP4ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP4ISR;
#[doc = "`read()` method returns [usb_ep4isr::R](usb_ep4isr::R) reader structure"]
impl crate::Readable for USB_EP4ISR {}
#[doc = "`write(|w| ..)` method takes [usb_ep4isr::W](usb_ep4isr::W) writer structure"]
impl crate::Writable for USB_EP4ISR {}
#[doc = "USB_EP4ISR"]
pub mod usb_ep4isr;
#[doc = "USB_EP4TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep4tcr](usb_ep4tcr) module"]
pub type USB_EP4TCR = crate::Reg<u32, _USB_EP4TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP4TCR;
#[doc = "`read()` method returns [usb_ep4tcr::R](usb_ep4tcr::R) reader structure"]
impl crate::Readable for USB_EP4TCR {}
#[doc = "`write(|w| ..)` method takes [usb_ep4tcr::W](usb_ep4tcr::W) writer structure"]
impl crate::Writable for USB_EP4TCR {}
#[doc = "USB_EP4TCR"]
pub mod usb_ep4tcr;
#[doc = "USB_EP4CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep4cfgr](usb_ep4cfgr) module"]
pub type USB_EP4CFGR = crate::Reg<u32, _USB_EP4CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP4CFGR;
#[doc = "`read()` method returns [usb_ep4cfgr::R](usb_ep4cfgr::R) reader structure"]
impl crate::Readable for USB_EP4CFGR {}
#[doc = "`write(|w| ..)` method takes [usb_ep4cfgr::W](usb_ep4cfgr::W) writer structure"]
impl crate::Writable for USB_EP4CFGR {}
#[doc = "USB_EP4CFGR"]
pub mod usb_ep4cfgr;
#[doc = "USB_EP5CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep5csr](usb_ep5csr) module"]
pub type USB_EP5CSR = crate::Reg<u32, _USB_EP5CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP5CSR;
#[doc = "`read()` method returns [usb_ep5csr::R](usb_ep5csr::R) reader structure"]
impl crate::Readable for USB_EP5CSR {}
#[doc = "`write(|w| ..)` method takes [usb_ep5csr::W](usb_ep5csr::W) writer structure"]
impl crate::Writable for USB_EP5CSR {}
#[doc = "USB_EP5CSR"]
pub mod usb_ep5csr;
#[doc = "USB_EP5IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep5ier](usb_ep5ier) module"]
pub type USB_EP5IER = crate::Reg<u32, _USB_EP5IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP5IER;
#[doc = "`read()` method returns [usb_ep5ier::R](usb_ep5ier::R) reader structure"]
impl crate::Readable for USB_EP5IER {}
#[doc = "`write(|w| ..)` method takes [usb_ep5ier::W](usb_ep5ier::W) writer structure"]
impl crate::Writable for USB_EP5IER {}
#[doc = "USB_EP5IER"]
pub mod usb_ep5ier;
#[doc = "USB_EP5ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep5isr](usb_ep5isr) module"]
pub type USB_EP5ISR = crate::Reg<u32, _USB_EP5ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP5ISR;
#[doc = "`read()` method returns [usb_ep5isr::R](usb_ep5isr::R) reader structure"]
impl crate::Readable for USB_EP5ISR {}
#[doc = "`write(|w| ..)` method takes [usb_ep5isr::W](usb_ep5isr::W) writer structure"]
impl crate::Writable for USB_EP5ISR {}
#[doc = "USB_EP5ISR"]
pub mod usb_ep5isr;
#[doc = "USB_EP5TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep5tcr](usb_ep5tcr) module"]
pub type USB_EP5TCR = crate::Reg<u32, _USB_EP5TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP5TCR;
#[doc = "`read()` method returns [usb_ep5tcr::R](usb_ep5tcr::R) reader structure"]
impl crate::Readable for USB_EP5TCR {}
#[doc = "`write(|w| ..)` method takes [usb_ep5tcr::W](usb_ep5tcr::W) writer structure"]
impl crate::Writable for USB_EP5TCR {}
#[doc = "USB_EP5TCR"]
pub mod usb_ep5tcr;
#[doc = "USB_EP5CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep5cfgr](usb_ep5cfgr) module"]
pub type USB_EP5CFGR = crate::Reg<u32, _USB_EP5CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP5CFGR;
#[doc = "`read()` method returns [usb_ep5cfgr::R](usb_ep5cfgr::R) reader structure"]
impl crate::Readable for USB_EP5CFGR {}
#[doc = "`write(|w| ..)` method takes [usb_ep5cfgr::W](usb_ep5cfgr::W) writer structure"]
impl crate::Writable for USB_EP5CFGR {}
#[doc = "USB_EP5CFGR"]
pub mod usb_ep5cfgr;
#[doc = "USB_EP6CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep6csr](usb_ep6csr) module"]
pub type USB_EP6CSR = crate::Reg<u32, _USB_EP6CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP6CSR;
#[doc = "`read()` method returns [usb_ep6csr::R](usb_ep6csr::R) reader structure"]
impl crate::Readable for USB_EP6CSR {}
#[doc = "`write(|w| ..)` method takes [usb_ep6csr::W](usb_ep6csr::W) writer structure"]
impl crate::Writable for USB_EP6CSR {}
#[doc = "USB_EP6CSR"]
pub mod usb_ep6csr;
#[doc = "USB_EP6IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep6ier](usb_ep6ier) module"]
pub type USB_EP6IER = crate::Reg<u32, _USB_EP6IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP6IER;
#[doc = "`read()` method returns [usb_ep6ier::R](usb_ep6ier::R) reader structure"]
impl crate::Readable for USB_EP6IER {}
#[doc = "`write(|w| ..)` method takes [usb_ep6ier::W](usb_ep6ier::W) writer structure"]
impl crate::Writable for USB_EP6IER {}
#[doc = "USB_EP6IER"]
pub mod usb_ep6ier;
#[doc = "USB_EP6ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep6isr](usb_ep6isr) module"]
pub type USB_EP6ISR = crate::Reg<u32, _USB_EP6ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP6ISR;
#[doc = "`read()` method returns [usb_ep6isr::R](usb_ep6isr::R) reader structure"]
impl crate::Readable for USB_EP6ISR {}
#[doc = "`write(|w| ..)` method takes [usb_ep6isr::W](usb_ep6isr::W) writer structure"]
impl crate::Writable for USB_EP6ISR {}
#[doc = "USB_EP6ISR"]
pub mod usb_ep6isr;
#[doc = "USB_EP6TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep6tcr](usb_ep6tcr) module"]
pub type USB_EP6TCR = crate::Reg<u32, _USB_EP6TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP6TCR;
#[doc = "`read()` method returns [usb_ep6tcr::R](usb_ep6tcr::R) reader structure"]
impl crate::Readable for USB_EP6TCR {}
#[doc = "`write(|w| ..)` method takes [usb_ep6tcr::W](usb_ep6tcr::W) writer structure"]
impl crate::Writable for USB_EP6TCR {}
#[doc = "USB_EP6TCR"]
pub mod usb_ep6tcr;
#[doc = "USB_EP6CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep6cfgr](usb_ep6cfgr) module"]
pub type USB_EP6CFGR = crate::Reg<u32, _USB_EP6CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP6CFGR;
#[doc = "`read()` method returns [usb_ep6cfgr::R](usb_ep6cfgr::R) reader structure"]
impl crate::Readable for USB_EP6CFGR {}
#[doc = "`write(|w| ..)` method takes [usb_ep6cfgr::W](usb_ep6cfgr::W) writer structure"]
impl crate::Writable for USB_EP6CFGR {}
#[doc = "USB_EP6CFGR"]
pub mod usb_ep6cfgr;
#[doc = "USB_EP7CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep7csr](usb_ep7csr) module"]
pub type USB_EP7CSR = crate::Reg<u32, _USB_EP7CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP7CSR;
#[doc = "`read()` method returns [usb_ep7csr::R](usb_ep7csr::R) reader structure"]
impl crate::Readable for USB_EP7CSR {}
#[doc = "`write(|w| ..)` method takes [usb_ep7csr::W](usb_ep7csr::W) writer structure"]
impl crate::Writable for USB_EP7CSR {}
#[doc = "USB_EP7CSR"]
pub mod usb_ep7csr;
#[doc = "USB_EP7IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep7ier](usb_ep7ier) module"]
pub type USB_EP7IER = crate::Reg<u32, _USB_EP7IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP7IER;
#[doc = "`read()` method returns [usb_ep7ier::R](usb_ep7ier::R) reader structure"]
impl crate::Readable for USB_EP7IER {}
#[doc = "`write(|w| ..)` method takes [usb_ep7ier::W](usb_ep7ier::W) writer structure"]
impl crate::Writable for USB_EP7IER {}
#[doc = "USB_EP7IER"]
pub mod usb_ep7ier;
#[doc = "USB_EP7ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep7isr](usb_ep7isr) module"]
pub type USB_EP7ISR = crate::Reg<u32, _USB_EP7ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP7ISR;
#[doc = "`read()` method returns [usb_ep7isr::R](usb_ep7isr::R) reader structure"]
impl crate::Readable for USB_EP7ISR {}
#[doc = "`write(|w| ..)` method takes [usb_ep7isr::W](usb_ep7isr::W) writer structure"]
impl crate::Writable for USB_EP7ISR {}
#[doc = "USB_EP7ISR"]
pub mod usb_ep7isr;
#[doc = "USB_EP7TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep7tcr](usb_ep7tcr) module"]
pub type USB_EP7TCR = crate::Reg<u32, _USB_EP7TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP7TCR;
#[doc = "`read()` method returns [usb_ep7tcr::R](usb_ep7tcr::R) reader structure"]
impl crate::Readable for USB_EP7TCR {}
#[doc = "`write(|w| ..)` method takes [usb_ep7tcr::W](usb_ep7tcr::W) writer structure"]
impl crate::Writable for USB_EP7TCR {}
#[doc = "USB_EP7TCR"]
pub mod usb_ep7tcr;
#[doc = "USB_EP7CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep7cfgr](usb_ep7cfgr) module"]
pub type USB_EP7CFGR = crate::Reg<u32, _USB_EP7CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP7CFGR;
#[doc = "`read()` method returns [usb_ep7cfgr::R](usb_ep7cfgr::R) reader structure"]
impl crate::Readable for USB_EP7CFGR {}
#[doc = "`write(|w| ..)` method takes [usb_ep7cfgr::W](usb_ep7cfgr::W) writer structure"]
impl crate::Writable for USB_EP7CFGR {}
#[doc = "USB_EP7CFGR"]
pub mod usb_ep7cfgr;
