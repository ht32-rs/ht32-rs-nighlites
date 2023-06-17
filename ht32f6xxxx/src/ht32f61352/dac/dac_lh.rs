#[doc = "Register `DAC_LH` reader"]
pub struct R(crate::R<DAC_LH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_LH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_LH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_LH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_LH` writer"]
pub struct W(crate::W<DAC_LH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_LH_SPEC>;
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
impl From<crate::W<DAC_LH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_LH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DA_L` reader - DA_L"]
pub type DA_L_R = crate::FieldReader<u16>;
#[doc = "Field `DA_L` writer - DA_L"]
pub type DA_L_W<'a, const O: u8> = crate::FieldWriter<'a, DAC_LH_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - DA_L"]
    #[inline(always)]
    pub fn da_l(&self) -> DA_L_R {
        DA_L_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DA_L"]
    #[inline(always)]
    #[must_use]
    pub fn da_l(&mut self) -> DA_L_W<0> {
        DA_L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC_LH\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_lh](index.html) module"]
pub struct DAC_LH_SPEC;
impl crate::RegisterSpec for DAC_LH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_lh::R](R) reader structure"]
impl crate::Readable for DAC_LH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_lh::W](W) writer structure"]
impl crate::Writable for DAC_LH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC_LH to value 0"]
impl crate::Resettable for DAC_LH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
