#[doc = "Register `APBCFGR` reader"]
pub struct R(crate::R<APBCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBCFGR` writer"]
pub struct W(crate::W<APBCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBCFGR_SPEC>;
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
impl From<crate::W<APBCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC0DIV` reader - ADC0DIV"]
pub type ADC0DIV_R = crate::FieldReader;
#[doc = "Field `ADC0DIV` writer - ADC0DIV"]
pub type ADC0DIV_W<'a, const O: u8> = crate::FieldWriter<'a, APBCFGR_SPEC, 3, O>;
#[doc = "Field `ADC1DIV` reader - ADC1DIV"]
pub type ADC1DIV_R = crate::FieldReader;
#[doc = "Field `ADC1DIV` writer - ADC1DIV"]
pub type ADC1DIV_W<'a, const O: u8> = crate::FieldWriter<'a, APBCFGR_SPEC, 3, O>;
impl R {
    #[doc = "Bits 16:18 - ADC0DIV"]
    #[inline(always)]
    pub fn adc0div(&self) -> ADC0DIV_R {
        ADC0DIV_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - ADC1DIV"]
    #[inline(always)]
    pub fn adc1div(&self) -> ADC1DIV_R {
        ADC1DIV_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - ADC0DIV"]
    #[inline(always)]
    #[must_use]
    pub fn adc0div(&mut self) -> ADC0DIV_W<16> {
        ADC0DIV_W::new(self)
    }
    #[doc = "Bits 20:22 - ADC1DIV"]
    #[inline(always)]
    #[must_use]
    pub fn adc1div(&mut self) -> ADC1DIV_W<20> {
        ADC1DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbcfgr](index.html) module"]
pub struct APBCFGR_SPEC;
impl crate::RegisterSpec for APBCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbcfgr::R](R) reader structure"]
impl crate::Readable for APBCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbcfgr::W](W) writer structure"]
impl crate::Writable for APBCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBCFGR to value 0"]
impl crate::Resettable for APBCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
