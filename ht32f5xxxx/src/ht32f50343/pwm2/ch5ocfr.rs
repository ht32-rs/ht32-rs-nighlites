#[doc = "Register `CH5OCFR` reader"]
pub struct R(crate::R<CH5OCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH5OCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH5OCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH5OCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH5OCFR` writer"]
pub struct W(crate::W<CH5OCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH5OCFR_SPEC>;
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
impl From<crate::W<CH5OCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH5OCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH5OM012` reader - CH5OM012"]
pub type CH5OM012_R = crate::FieldReader;
#[doc = "Field `CH5OM012` writer - CH5OM012"]
pub type CH5OM012_W<'a, const O: u8> = crate::FieldWriter<'a, CH5OCFR_SPEC, 3, O>;
#[doc = "Field `CH5PRE` reader - CH5PRE"]
pub type CH5PRE_R = crate::BitReader;
#[doc = "Field `CH5PRE` writer - CH5PRE"]
pub type CH5PRE_W<'a, const O: u8> = crate::BitWriter<'a, CH5OCFR_SPEC, O>;
#[doc = "Field `CH5IMAE` reader - CH5IMAE"]
pub type CH5IMAE_R = crate::BitReader;
#[doc = "Field `CH5IMAE` writer - CH5IMAE"]
pub type CH5IMAE_W<'a, const O: u8> = crate::BitWriter<'a, CH5OCFR_SPEC, O>;
#[doc = "Field `CH5OM3` reader - CH5OM3"]
pub type CH5OM3_R = crate::BitReader;
#[doc = "Field `CH5OM3` writer - CH5OM3"]
pub type CH5OM3_W<'a, const O: u8> = crate::BitWriter<'a, CH5OCFR_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - CH5OM012"]
    #[inline(always)]
    pub fn ch5om012(&self) -> CH5OM012_R {
        CH5OM012_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - CH5PRE"]
    #[inline(always)]
    pub fn ch5pre(&self) -> CH5PRE_R {
        CH5PRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH5IMAE"]
    #[inline(always)]
    pub fn ch5imae(&self) -> CH5IMAE_R {
        CH5IMAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CH5OM3"]
    #[inline(always)]
    pub fn ch5om3(&self) -> CH5OM3_R {
        CH5OM3_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH5OM012"]
    #[inline(always)]
    #[must_use]
    pub fn ch5om012(&mut self) -> CH5OM012_W<0> {
        CH5OM012_W::new(self)
    }
    #[doc = "Bit 4 - CH5PRE"]
    #[inline(always)]
    #[must_use]
    pub fn ch5pre(&mut self) -> CH5PRE_W<4> {
        CH5PRE_W::new(self)
    }
    #[doc = "Bit 5 - CH5IMAE"]
    #[inline(always)]
    #[must_use]
    pub fn ch5imae(&mut self) -> CH5IMAE_W<5> {
        CH5IMAE_W::new(self)
    }
    #[doc = "Bit 8 - CH5OM3"]
    #[inline(always)]
    #[must_use]
    pub fn ch5om3(&mut self) -> CH5OM3_W<8> {
        CH5OM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH5OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5ocfr](index.html) module"]
pub struct CH5OCFR_SPEC;
impl crate::RegisterSpec for CH5OCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch5ocfr::R](R) reader structure"]
impl crate::Readable for CH5OCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch5ocfr::W](W) writer structure"]
impl crate::Writable for CH5OCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH5OCFR to value 0"]
impl crate::Resettable for CH5OCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
