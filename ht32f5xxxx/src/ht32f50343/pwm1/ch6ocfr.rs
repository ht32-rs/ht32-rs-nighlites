#[doc = "Register `CH6OCFR` reader"]
pub struct R(crate::R<CH6OCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH6OCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH6OCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH6OCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH6OCFR` writer"]
pub struct W(crate::W<CH6OCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH6OCFR_SPEC>;
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
impl From<crate::W<CH6OCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH6OCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH6OM012` reader - CH6OM012"]
pub type CH6OM012_R = crate::FieldReader;
#[doc = "Field `CH6OM012` writer - CH6OM012"]
pub type CH6OM012_W<'a, const O: u8> = crate::FieldWriter<'a, CH6OCFR_SPEC, 3, O>;
#[doc = "Field `CH6PRE` reader - CH6PRE"]
pub type CH6PRE_R = crate::BitReader;
#[doc = "Field `CH6PRE` writer - CH6PRE"]
pub type CH6PRE_W<'a, const O: u8> = crate::BitWriter<'a, CH6OCFR_SPEC, O>;
#[doc = "Field `CH6IMAE` reader - CH6IMAE"]
pub type CH6IMAE_R = crate::BitReader;
#[doc = "Field `CH6IMAE` writer - CH6IMAE"]
pub type CH6IMAE_W<'a, const O: u8> = crate::BitWriter<'a, CH6OCFR_SPEC, O>;
#[doc = "Field `CH6OM3` reader - CH6OM3"]
pub type CH6OM3_R = crate::BitReader;
#[doc = "Field `CH6OM3` writer - CH6OM3"]
pub type CH6OM3_W<'a, const O: u8> = crate::BitWriter<'a, CH6OCFR_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - CH6OM012"]
    #[inline(always)]
    pub fn ch6om012(&self) -> CH6OM012_R {
        CH6OM012_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - CH6PRE"]
    #[inline(always)]
    pub fn ch6pre(&self) -> CH6PRE_R {
        CH6PRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH6IMAE"]
    #[inline(always)]
    pub fn ch6imae(&self) -> CH6IMAE_R {
        CH6IMAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CH6OM3"]
    #[inline(always)]
    pub fn ch6om3(&self) -> CH6OM3_R {
        CH6OM3_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH6OM012"]
    #[inline(always)]
    #[must_use]
    pub fn ch6om012(&mut self) -> CH6OM012_W<0> {
        CH6OM012_W::new(self)
    }
    #[doc = "Bit 4 - CH6PRE"]
    #[inline(always)]
    #[must_use]
    pub fn ch6pre(&mut self) -> CH6PRE_W<4> {
        CH6PRE_W::new(self)
    }
    #[doc = "Bit 5 - CH6IMAE"]
    #[inline(always)]
    #[must_use]
    pub fn ch6imae(&mut self) -> CH6IMAE_W<5> {
        CH6IMAE_W::new(self)
    }
    #[doc = "Bit 8 - CH6OM3"]
    #[inline(always)]
    #[must_use]
    pub fn ch6om3(&mut self) -> CH6OM3_W<8> {
        CH6OM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH6OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6ocfr](index.html) module"]
pub struct CH6OCFR_SPEC;
impl crate::RegisterSpec for CH6OCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch6ocfr::R](R) reader structure"]
impl crate::Readable for CH6OCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch6ocfr::W](W) writer structure"]
impl crate::Writable for CH6OCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH6OCFR to value 0"]
impl crate::Resettable for CH6OCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
