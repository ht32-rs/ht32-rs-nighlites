#[doc = "Register `USBIER` reader"]
pub struct R(crate::R<USBIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBIER` writer"]
pub struct W(crate::W<USBIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBIER_SPEC>;
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
impl From<crate::W<USBIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UGIE` reader - UGIE"]
pub type UGIE_R = crate::BitReader;
#[doc = "Field `UGIE` writer - UGIE"]
pub type UGIE_W<'a, const O: u8> = crate::BitWriter<'a, USBIER_SPEC, O>;
#[doc = "Field `SOFIE` reader - SOFIE"]
pub type SOFIE_R = crate::BitReader;
#[doc = "Field `SOFIE` writer - SOFIE"]
pub type SOFIE_W<'a, const O: u8> = crate::BitWriter<'a, USBIER_SPEC, O>;
#[doc = "Field `URSTIE` reader - URSTIE"]
pub type URSTIE_R = crate::BitReader;
#[doc = "Field `URSTIE` writer - URSTIE"]
pub type URSTIE_W<'a, const O: u8> = crate::BitWriter<'a, USBIER_SPEC, O>;
#[doc = "Field `RSMIE` reader - RSMIE"]
pub type RSMIE_R = crate::BitReader;
#[doc = "Field `RSMIE` writer - RSMIE"]
pub type RSMIE_W<'a, const O: u8> = crate::BitWriter<'a, USBIER_SPEC, O>;
#[doc = "Field `SUSPIE` reader - SUSPIE"]
pub type SUSPIE_R = crate::BitReader;
#[doc = "Field `SUSPIE` writer - SUSPIE"]
pub type SUSPIE_W<'a, const O: u8> = crate::BitWriter<'a, USBIER_SPEC, O>;
#[doc = "Field `ESOFIE` reader - ESOFIE"]
pub type ESOFIE_R = crate::BitReader;
#[doc = "Field `ESOFIE` writer - ESOFIE"]
pub type ESOFIE_W<'a, const O: u8> = crate::BitWriter<'a, USBIER_SPEC, O>;
#[doc = "Field `EPnIE` reader - EPnIE"]
pub type EPN_IE_R = crate::FieldReader;
#[doc = "Field `EPnIE` writer - EPnIE"]
pub type EPN_IE_W<'a, const O: u8> = crate::FieldWriter<'a, USBIER_SPEC, 8, O>;
impl R {
    #[doc = "Bit 0 - UGIE"]
    #[inline(always)]
    pub fn ugie(&self) -> UGIE_R {
        UGIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SOFIE"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - URSTIE"]
    #[inline(always)]
    pub fn urstie(&self) -> URSTIE_R {
        URSTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSMIE"]
    #[inline(always)]
    pub fn rsmie(&self) -> RSMIE_R {
        RSMIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SUSPIE"]
    #[inline(always)]
    pub fn suspie(&self) -> SUSPIE_R {
        SUSPIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ESOFIE"]
    #[inline(always)]
    pub fn esofie(&self) -> ESOFIE_R {
        ESOFIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - EPnIE"]
    #[inline(always)]
    pub fn epn_ie(&self) -> EPN_IE_R {
        EPN_IE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - UGIE"]
    #[inline(always)]
    #[must_use]
    pub fn ugie(&mut self) -> UGIE_W<0> {
        UGIE_W::new(self)
    }
    #[doc = "Bit 1 - SOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn sofie(&mut self) -> SOFIE_W<1> {
        SOFIE_W::new(self)
    }
    #[doc = "Bit 2 - URSTIE"]
    #[inline(always)]
    #[must_use]
    pub fn urstie(&mut self) -> URSTIE_W<2> {
        URSTIE_W::new(self)
    }
    #[doc = "Bit 3 - RSMIE"]
    #[inline(always)]
    #[must_use]
    pub fn rsmie(&mut self) -> RSMIE_W<3> {
        RSMIE_W::new(self)
    }
    #[doc = "Bit 4 - SUSPIE"]
    #[inline(always)]
    #[must_use]
    pub fn suspie(&mut self) -> SUSPIE_W<4> {
        SUSPIE_W::new(self)
    }
    #[doc = "Bit 5 - ESOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn esofie(&mut self) -> ESOFIE_W<5> {
        ESOFIE_W::new(self)
    }
    #[doc = "Bits 8:15 - EPnIE"]
    #[inline(always)]
    #[must_use]
    pub fn epn_ie(&mut self) -> EPN_IE_W<8> {
        EPN_IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USBIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbier](index.html) module"]
pub struct USBIER_SPEC;
impl crate::RegisterSpec for USBIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbier::R](R) reader structure"]
impl crate::Readable for USBIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbier::W](W) writer structure"]
impl crate::Writable for USBIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBIER to value 0"]
impl crate::Resettable for USBIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
