#[doc = "Register `DAC1DHR` reader"]
pub struct R(crate::R<DAC1DHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC1DHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC1DHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC1DHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC1DHR` writer"]
pub struct W(crate::W<DAC1DHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC1DHR_SPEC>;
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
impl From<crate::W<DAC1DHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC1DHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC1DATA` reader - DAC1DATA"]
pub type DAC1DATA_R = crate::FieldReader<u16>;
#[doc = "Field `DAC1DATA` writer - DAC1DATA"]
pub type DAC1DATA_W<'a, const O: u8> = crate::FieldWriter<'a, DAC1DHR_SPEC, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC1DATA"]
    #[inline(always)]
    pub fn dac1data(&self) -> DAC1DATA_R {
        DAC1DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC1DATA"]
    #[inline(always)]
    #[must_use]
    pub fn dac1data(&mut self) -> DAC1DATA_W<0> {
        DAC1DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC1DHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac1dhr](index.html) module"]
pub struct DAC1DHR_SPEC;
impl crate::RegisterSpec for DAC1DHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac1dhr::R](R) reader structure"]
impl crate::Readable for DAC1DHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac1dhr::W](W) writer structure"]
impl crate::Writable for DAC1DHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC1DHR to value 0"]
impl crate::Resettable for DAC1DHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
