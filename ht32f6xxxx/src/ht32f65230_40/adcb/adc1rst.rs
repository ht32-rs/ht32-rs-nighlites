#[doc = "Register `ADC1RST` reader"]
pub struct R(crate::R<ADC1RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1RST` writer"]
pub struct W(crate::W<ADC1RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1RST_SPEC>;
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
impl From<crate::W<ADC1RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1RST` reader - AD1RST"]
pub type AD1RST_R = crate::BitReader;
#[doc = "Field `AD1RST` writer - AD1RST"]
pub type AD1RST_W<'a, const O: u8> = crate::BitWriter<'a, ADC1RST_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AD1RST"]
    #[inline(always)]
    pub fn ad1rst(&self) -> AD1RST_R {
        AD1RST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD1RST"]
    #[inline(always)]
    #[must_use]
    pub fn ad1rst(&mut self) -> AD1RST_W<0> {
        AD1RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1RST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1rst](index.html) module"]
pub struct ADC1RST_SPEC;
impl crate::RegisterSpec for ADC1RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1rst::R](R) reader structure"]
impl crate::Readable for ADC1RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1rst::W](W) writer structure"]
impl crate::Writable for ADC1RST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1RST to value 0"]
impl crate::Resettable for ADC1RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
