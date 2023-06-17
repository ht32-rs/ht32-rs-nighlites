#[doc = "Register `DAC0DHR` reader"]
pub struct R(crate::R<DAC0DHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC0DHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC0DHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC0DHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC0DHR` writer"]
pub struct W(crate::W<DAC0DHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC0DHR_SPEC>;
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
impl From<crate::W<DAC0DHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC0DHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC0DATA` reader - DAC0DATA"]
pub type DAC0DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DAC0DATA` writer - DAC0DATA"]
pub type DAC0DATA_W<'a, const O: u8> = crate::FieldWriter<'a, DAC0DHR_SPEC, 28, O, u32>;
impl R {
    #[doc = "Bits 0:27 - DAC0DATA"]
    #[inline(always)]
    pub fn dac0data(&self) -> DAC0DATA_R {
        DAC0DATA_R::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - DAC0DATA"]
    #[inline(always)]
    #[must_use]
    pub fn dac0data(&mut self) -> DAC0DATA_W<0> {
        DAC0DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC0DHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac0dhr](index.html) module"]
pub struct DAC0DHR_SPEC;
impl crate::RegisterSpec for DAC0DHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac0dhr::R](R) reader structure"]
impl crate::Readable for DAC0DHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac0dhr::W](W) writer structure"]
impl crate::Writable for DAC0DHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC0DHR to value 0"]
impl crate::Resettable for DAC0DHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
