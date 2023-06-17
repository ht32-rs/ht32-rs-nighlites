#[doc = "Register `USB_EP1IER` reader"]
pub struct R(crate::R<USB_EP1IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_EP1IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_EP1IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_EP1IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_EP1IER` writer"]
pub struct W(crate::W<USB_EP1IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_EP1IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<USB_EP1IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_EP1IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTRXIE` reader - OTRXIE"]
pub type OTRXIE_R = crate::BitReader;
#[doc = "Field `OTRXIE` writer - OTRXIE"]
pub type OTRXIE_W<'a, const O: u8> = crate::BitWriter<'a, USB_EP1IER_SPEC, O>;
#[doc = "Field `ODRXIE` reader - ODRXIE"]
pub type ODRXIE_R = crate::BitReader;
#[doc = "Field `ODRXIE` writer - ODRXIE"]
pub type ODRXIE_W<'a, const O: u8> = crate::BitWriter<'a, USB_EP1IER_SPEC, O>;
#[doc = "Field `ODOVIE` reader - ODOVIE"]
pub type ODOVIE_R = crate::BitReader;
#[doc = "Field `ODOVIE` writer - ODOVIE"]
pub type ODOVIE_W<'a, const O: u8> = crate::BitWriter<'a, USB_EP1IER_SPEC, O>;
#[doc = "Field `ITRXIE` reader - ITRXIE"]
pub type ITRXIE_R = crate::BitReader;
#[doc = "Field `ITRXIE` writer - ITRXIE"]
pub type ITRXIE_W<'a, const O: u8> = crate::BitWriter<'a, USB_EP1IER_SPEC, O>;
#[doc = "Field `IDTXIE` reader - IDTXIE"]
pub type IDTXIE_R = crate::BitReader;
#[doc = "Field `IDTXIE` writer - IDTXIE"]
pub type IDTXIE_W<'a, const O: u8> = crate::BitWriter<'a, USB_EP1IER_SPEC, O>;
#[doc = "Field `NAKIE` reader - NAKIE"]
pub type NAKIE_R = crate::BitReader;
#[doc = "Field `NAKIE` writer - NAKIE"]
pub type NAKIE_W<'a, const O: u8> = crate::BitWriter<'a, USB_EP1IER_SPEC, O>;
#[doc = "Field `STLIE` reader - STLIE"]
pub type STLIE_R = crate::BitReader;
#[doc = "Field `STLIE` writer - STLIE"]
pub type STLIE_W<'a, const O: u8> = crate::BitWriter<'a, USB_EP1IER_SPEC, O>;
#[doc = "Field `UERIE` reader - UERIE"]
pub type UERIE_R = crate::BitReader;
#[doc = "Field `UERIE` writer - UERIE"]
pub type UERIE_W<'a, const O: u8> = crate::BitWriter<'a, USB_EP1IER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - OTRXIE"]
    #[inline(always)]
    pub fn otrxie(&self) -> OTRXIE_R {
        OTRXIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ODRXIE"]
    #[inline(always)]
    pub fn odrxie(&self) -> ODRXIE_R {
        ODRXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ODOVIE"]
    #[inline(always)]
    pub fn odovie(&self) -> ODOVIE_R {
        ODOVIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ITRXIE"]
    #[inline(always)]
    pub fn itrxie(&self) -> ITRXIE_R {
        ITRXIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDTXIE"]
    #[inline(always)]
    pub fn idtxie(&self) -> IDTXIE_R {
        IDTXIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NAKIE"]
    #[inline(always)]
    pub fn nakie(&self) -> NAKIE_R {
        NAKIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - STLIE"]
    #[inline(always)]
    pub fn stlie(&self) -> STLIE_R {
        STLIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UERIE"]
    #[inline(always)]
    pub fn uerie(&self) -> UERIE_R {
        UERIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OTRXIE"]
    #[inline(always)]
    #[must_use]
    pub fn otrxie(&mut self) -> OTRXIE_W<0> {
        OTRXIE_W::new(self)
    }
    #[doc = "Bit 1 - ODRXIE"]
    #[inline(always)]
    #[must_use]
    pub fn odrxie(&mut self) -> ODRXIE_W<1> {
        ODRXIE_W::new(self)
    }
    #[doc = "Bit 2 - ODOVIE"]
    #[inline(always)]
    #[must_use]
    pub fn odovie(&mut self) -> ODOVIE_W<2> {
        ODOVIE_W::new(self)
    }
    #[doc = "Bit 3 - ITRXIE"]
    #[inline(always)]
    #[must_use]
    pub fn itrxie(&mut self) -> ITRXIE_W<3> {
        ITRXIE_W::new(self)
    }
    #[doc = "Bit 4 - IDTXIE"]
    #[inline(always)]
    #[must_use]
    pub fn idtxie(&mut self) -> IDTXIE_W<4> {
        IDTXIE_W::new(self)
    }
    #[doc = "Bit 5 - NAKIE"]
    #[inline(always)]
    #[must_use]
    pub fn nakie(&mut self) -> NAKIE_W<5> {
        NAKIE_W::new(self)
    }
    #[doc = "Bit 6 - STLIE"]
    #[inline(always)]
    #[must_use]
    pub fn stlie(&mut self) -> STLIE_W<6> {
        STLIE_W::new(self)
    }
    #[doc = "Bit 7 - UERIE"]
    #[inline(always)]
    #[must_use]
    pub fn uerie(&mut self) -> UERIE_W<7> {
        UERIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_EP1IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ep1ier](index.html) module"]
pub struct USB_EP1IER_SPEC;
impl crate::RegisterSpec for USB_EP1IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_ep1ier::R](R) reader structure"]
impl crate::Readable for USB_EP1IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_ep1ier::W](W) writer structure"]
impl crate::Writable for USB_EP1IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_EP1IER to value 0"]
impl crate::Resettable for USB_EP1IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
