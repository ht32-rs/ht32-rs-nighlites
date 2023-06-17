#[doc = "Register `USBEP0IER` reader"]
pub struct R(crate::R<USBEP0IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBEP0IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBEP0IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBEP0IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBEP0IER` writer"]
pub struct W(crate::W<USBEP0IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBEP0IER_SPEC>;
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
impl From<crate::W<USBEP0IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBEP0IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTRXIE` reader - OTRXIE"]
pub type OTRXIE_R = crate::BitReader;
#[doc = "Field `OTRXIE` writer - OTRXIE"]
pub type OTRXIE_W<'a, const O: u8> = crate::BitWriter<'a, USBEP0IER_SPEC, O>;
#[doc = "Field `ODRXIE` reader - ODRXIE"]
pub type ODRXIE_R = crate::BitReader;
#[doc = "Field `ODRXIE` writer - ODRXIE"]
pub type ODRXIE_W<'a, const O: u8> = crate::BitWriter<'a, USBEP0IER_SPEC, O>;
#[doc = "Field `ODOVIE` reader - ODOVIE"]
pub type ODOVIE_R = crate::BitReader;
#[doc = "Field `ODOVIE` writer - ODOVIE"]
pub type ODOVIE_W<'a, const O: u8> = crate::BitWriter<'a, USBEP0IER_SPEC, O>;
#[doc = "Field `ITRXIE` reader - ITRXIE"]
pub type ITRXIE_R = crate::BitReader;
#[doc = "Field `ITRXIE` writer - ITRXIE"]
pub type ITRXIE_W<'a, const O: u8> = crate::BitWriter<'a, USBEP0IER_SPEC, O>;
#[doc = "Field `IDTXIE` reader - IDTXIE"]
pub type IDTXIE_R = crate::BitReader;
#[doc = "Field `IDTXIE` writer - IDTXIE"]
pub type IDTXIE_W<'a, const O: u8> = crate::BitWriter<'a, USBEP0IER_SPEC, O>;
#[doc = "Field `NAKIE` reader - NAKIE"]
pub type NAKIE_R = crate::BitReader;
#[doc = "Field `NAKIE` writer - NAKIE"]
pub type NAKIE_W<'a, const O: u8> = crate::BitWriter<'a, USBEP0IER_SPEC, O>;
#[doc = "Field `STLIE` reader - STLIE"]
pub type STLIE_R = crate::BitReader;
#[doc = "Field `STLIE` writer - STLIE"]
pub type STLIE_W<'a, const O: u8> = crate::BitWriter<'a, USBEP0IER_SPEC, O>;
#[doc = "Field `UERIE` reader - UERIE"]
pub type UERIE_R = crate::BitReader;
#[doc = "Field `UERIE` writer - UERIE"]
pub type UERIE_W<'a, const O: u8> = crate::BitWriter<'a, USBEP0IER_SPEC, O>;
#[doc = "Field `STRXIE` reader - STRXIE"]
pub type STRXIE_R = crate::BitReader;
#[doc = "Field `STRXIE` writer - STRXIE"]
pub type STRXIE_W<'a, const O: u8> = crate::BitWriter<'a, USBEP0IER_SPEC, O>;
#[doc = "Field `SDRXIE` reader - SDRXIE"]
pub type SDRXIE_R = crate::BitReader;
#[doc = "Field `SDRXIE` writer - SDRXIE"]
pub type SDRXIE_W<'a, const O: u8> = crate::BitWriter<'a, USBEP0IER_SPEC, O>;
#[doc = "Field `SDERIE` reader - SDERIE"]
pub type SDERIE_R = crate::BitReader;
#[doc = "Field `SDERIE` writer - SDERIE"]
pub type SDERIE_W<'a, const O: u8> = crate::BitWriter<'a, USBEP0IER_SPEC, O>;
#[doc = "Field `ZLRXIE` reader - ZLRXIE"]
pub type ZLRXIE_R = crate::BitReader;
#[doc = "Field `ZLRXIE` writer - ZLRXIE"]
pub type ZLRXIE_W<'a, const O: u8> = crate::BitWriter<'a, USBEP0IER_SPEC, O>;
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
    #[doc = "Bit 8 - STRXIE"]
    #[inline(always)]
    pub fn strxie(&self) -> STRXIE_R {
        STRXIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SDRXIE"]
    #[inline(always)]
    pub fn sdrxie(&self) -> SDRXIE_R {
        SDRXIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SDERIE"]
    #[inline(always)]
    pub fn sderie(&self) -> SDERIE_R {
        SDERIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ZLRXIE"]
    #[inline(always)]
    pub fn zlrxie(&self) -> ZLRXIE_R {
        ZLRXIE_R::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 8 - STRXIE"]
    #[inline(always)]
    #[must_use]
    pub fn strxie(&mut self) -> STRXIE_W<8> {
        STRXIE_W::new(self)
    }
    #[doc = "Bit 9 - SDRXIE"]
    #[inline(always)]
    #[must_use]
    pub fn sdrxie(&mut self) -> SDRXIE_W<9> {
        SDRXIE_W::new(self)
    }
    #[doc = "Bit 10 - SDERIE"]
    #[inline(always)]
    #[must_use]
    pub fn sderie(&mut self) -> SDERIE_W<10> {
        SDERIE_W::new(self)
    }
    #[doc = "Bit 11 - ZLRXIE"]
    #[inline(always)]
    #[must_use]
    pub fn zlrxie(&mut self) -> ZLRXIE_W<11> {
        ZLRXIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USBEP0IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbep0ier](index.html) module"]
pub struct USBEP0IER_SPEC;
impl crate::RegisterSpec for USBEP0IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbep0ier::R](R) reader structure"]
impl crate::Readable for USBEP0IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbep0ier::W](W) writer structure"]
impl crate::Writable for USBEP0IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBEP0IER to value 0"]
impl crate::Resettable for USBEP0IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
