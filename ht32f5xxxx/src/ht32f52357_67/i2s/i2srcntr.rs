#[doc = "Register `I2SRCNTR` reader"]
pub struct R(crate::R<I2SRCNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SRCNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SRCNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SRCNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SRCNTR` writer"]
pub struct W(crate::W<I2SRCNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SRCNTR_SPEC>;
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
impl From<crate::W<I2SRCNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SRCNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCNTR` reader - RCNTR"]
pub type RCNTR_R = crate::FieldReader<u32>;
#[doc = "Field `RCNTR` writer - RCNTR"]
pub type RCNTR_W<'a, const O: u8> = crate::FieldWriter<'a, I2SRCNTR_SPEC, 20, O, u32>;
impl R {
    #[doc = "Bits 0:19 - RCNTR"]
    #[inline(always)]
    pub fn rcntr(&self) -> RCNTR_R {
        RCNTR_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - RCNTR"]
    #[inline(always)]
    #[must_use]
    pub fn rcntr(&mut self) -> RCNTR_W<0> {
        RCNTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2SRCNTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2srcntr](index.html) module"]
pub struct I2SRCNTR_SPEC;
impl crate::RegisterSpec for I2SRCNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2srcntr::R](R) reader structure"]
impl crate::Readable for I2SRCNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2srcntr::W](W) writer structure"]
impl crate::Writable for I2SRCNTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SRCNTR to value 0"]
impl crate::Resettable for I2SRCNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
