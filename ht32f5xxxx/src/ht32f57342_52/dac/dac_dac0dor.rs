#[doc = "Register `DAC_DAC0DOR` reader"]
pub struct R(crate::R<DAC_DAC0DOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_DAC0DOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_DAC0DOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_DAC0DOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_DAC0DOR` writer"]
pub struct W(crate::W<DAC_DAC0DOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_DAC0DOR_SPEC>;
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
impl From<crate::W<DAC_DAC0DOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_DAC0DOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC0DOUT` reader - DAC0DOUT"]
pub type DAC0DOUT_R = crate::FieldReader<u16>;
#[doc = "Field `DAC0DOUT` writer - DAC0DOUT"]
pub type DAC0DOUT_W<'a, const O: u8> = crate::FieldWriter<'a, DAC_DAC0DOR_SPEC, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC0DOUT"]
    #[inline(always)]
    pub fn dac0dout(&self) -> DAC0DOUT_R {
        DAC0DOUT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC0DOUT"]
    #[inline(always)]
    #[must_use]
    pub fn dac0dout(&mut self) -> DAC0DOUT_W<0> {
        DAC0DOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC_DAC0DOR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dac0dor](index.html) module"]
pub struct DAC_DAC0DOR_SPEC;
impl crate::RegisterSpec for DAC_DAC0DOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_dac0dor::R](R) reader structure"]
impl crate::Readable for DAC_DAC0DOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_dac0dor::W](W) writer structure"]
impl crate::Writable for DAC_DAC0DOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC_DAC0DOR to value 0"]
impl crate::Resettable for DAC_DAC0DOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
