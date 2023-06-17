#[doc = "Register `CH7OCFR` reader"]
pub struct R(crate::R<CH7OCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH7OCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH7OCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH7OCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH7OCFR` writer"]
pub struct W(crate::W<CH7OCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH7OCFR_SPEC>;
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
impl From<crate::W<CH7OCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH7OCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH7OM012` reader - CH7OM012"]
pub type CH7OM012_R = crate::FieldReader;
#[doc = "Field `CH7OM012` writer - CH7OM012"]
pub type CH7OM012_W<'a, const O: u8> = crate::FieldWriter<'a, CH7OCFR_SPEC, 3, O>;
#[doc = "Field `CH7PRE` reader - CH7PRE"]
pub type CH7PRE_R = crate::BitReader;
#[doc = "Field `CH7PRE` writer - CH7PRE"]
pub type CH7PRE_W<'a, const O: u8> = crate::BitWriter<'a, CH7OCFR_SPEC, O>;
#[doc = "Field `CH7IMAE` reader - CH7IMAE"]
pub type CH7IMAE_R = crate::BitReader;
#[doc = "Field `CH7IMAE` writer - CH7IMAE"]
pub type CH7IMAE_W<'a, const O: u8> = crate::BitWriter<'a, CH7OCFR_SPEC, O>;
#[doc = "Field `CH7OM3` reader - CH7OM3"]
pub type CH7OM3_R = crate::BitReader;
#[doc = "Field `CH7OM3` writer - CH7OM3"]
pub type CH7OM3_W<'a, const O: u8> = crate::BitWriter<'a, CH7OCFR_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - CH7OM012"]
    #[inline(always)]
    pub fn ch7om012(&self) -> CH7OM012_R {
        CH7OM012_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - CH7PRE"]
    #[inline(always)]
    pub fn ch7pre(&self) -> CH7PRE_R {
        CH7PRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH7IMAE"]
    #[inline(always)]
    pub fn ch7imae(&self) -> CH7IMAE_R {
        CH7IMAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CH7OM3"]
    #[inline(always)]
    pub fn ch7om3(&self) -> CH7OM3_R {
        CH7OM3_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH7OM012"]
    #[inline(always)]
    #[must_use]
    pub fn ch7om012(&mut self) -> CH7OM012_W<0> {
        CH7OM012_W::new(self)
    }
    #[doc = "Bit 4 - CH7PRE"]
    #[inline(always)]
    #[must_use]
    pub fn ch7pre(&mut self) -> CH7PRE_W<4> {
        CH7PRE_W::new(self)
    }
    #[doc = "Bit 5 - CH7IMAE"]
    #[inline(always)]
    #[must_use]
    pub fn ch7imae(&mut self) -> CH7IMAE_W<5> {
        CH7IMAE_W::new(self)
    }
    #[doc = "Bit 8 - CH7OM3"]
    #[inline(always)]
    #[must_use]
    pub fn ch7om3(&mut self) -> CH7OM3_W<8> {
        CH7OM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH7OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7ocfr](index.html) module"]
pub struct CH7OCFR_SPEC;
impl crate::RegisterSpec for CH7OCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch7ocfr::R](R) reader structure"]
impl crate::Readable for CH7OCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch7ocfr::W](W) writer structure"]
impl crate::Writable for CH7OCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH7OCFR to value 0"]
impl crate::Resettable for CH7OCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
