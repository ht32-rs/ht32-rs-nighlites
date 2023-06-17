#[doc = "Register `CH4OCFR` reader"]
pub struct R(crate::R<CH4OCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH4OCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH4OCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH4OCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH4OCFR` writer"]
pub struct W(crate::W<CH4OCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH4OCFR_SPEC>;
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
impl From<crate::W<CH4OCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH4OCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH4OM012` reader - CH4OM012"]
pub type CH4OM012_R = crate::FieldReader;
#[doc = "Field `CH4OM012` writer - CH4OM012"]
pub type CH4OM012_W<'a, const O: u8> = crate::FieldWriter<'a, CH4OCFR_SPEC, 3, O>;
#[doc = "Field `CH4PRE` reader - CH4PRE"]
pub type CH4PRE_R = crate::BitReader;
#[doc = "Field `CH4PRE` writer - CH4PRE"]
pub type CH4PRE_W<'a, const O: u8> = crate::BitWriter<'a, CH4OCFR_SPEC, O>;
#[doc = "Field `CH4IMAE` reader - CH4IMAE"]
pub type CH4IMAE_R = crate::BitReader;
#[doc = "Field `CH4IMAE` writer - CH4IMAE"]
pub type CH4IMAE_W<'a, const O: u8> = crate::BitWriter<'a, CH4OCFR_SPEC, O>;
#[doc = "Field `CH4OM3` reader - CH4OM3"]
pub type CH4OM3_R = crate::BitReader;
#[doc = "Field `CH4OM3` writer - CH4OM3"]
pub type CH4OM3_W<'a, const O: u8> = crate::BitWriter<'a, CH4OCFR_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - CH4OM012"]
    #[inline(always)]
    pub fn ch4om012(&self) -> CH4OM012_R {
        CH4OM012_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - CH4PRE"]
    #[inline(always)]
    pub fn ch4pre(&self) -> CH4PRE_R {
        CH4PRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH4IMAE"]
    #[inline(always)]
    pub fn ch4imae(&self) -> CH4IMAE_R {
        CH4IMAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CH4OM3"]
    #[inline(always)]
    pub fn ch4om3(&self) -> CH4OM3_R {
        CH4OM3_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH4OM012"]
    #[inline(always)]
    #[must_use]
    pub fn ch4om012(&mut self) -> CH4OM012_W<0> {
        CH4OM012_W::new(self)
    }
    #[doc = "Bit 4 - CH4PRE"]
    #[inline(always)]
    #[must_use]
    pub fn ch4pre(&mut self) -> CH4PRE_W<4> {
        CH4PRE_W::new(self)
    }
    #[doc = "Bit 5 - CH4IMAE"]
    #[inline(always)]
    #[must_use]
    pub fn ch4imae(&mut self) -> CH4IMAE_W<5> {
        CH4IMAE_W::new(self)
    }
    #[doc = "Bit 8 - CH4OM3"]
    #[inline(always)]
    #[must_use]
    pub fn ch4om3(&mut self) -> CH4OM3_W<8> {
        CH4OM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH4OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4ocfr](index.html) module"]
pub struct CH4OCFR_SPEC;
impl crate::RegisterSpec for CH4OCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch4ocfr::R](R) reader structure"]
impl crate::Readable for CH4OCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch4ocfr::W](W) writer structure"]
impl crate::Writable for CH4OCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH4OCFR to value 0"]
impl crate::Resettable for CH4OCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
