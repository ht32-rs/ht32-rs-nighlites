#[doc = "Register `ADC0RST` reader"]
pub struct R(crate::R<ADC0RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0RST` writer"]
pub struct W(crate::W<ADC0RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0RST_SPEC>;
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
impl From<crate::W<ADC0RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0RST` reader - AD0RST"]
pub type AD0RST_R = crate::BitReader;
#[doc = "Field `AD0RST` writer - AD0RST"]
pub type AD0RST_W<'a, const O: u8> = crate::BitWriter<'a, ADC0RST_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AD0RST"]
    #[inline(always)]
    pub fn ad0rst(&self) -> AD0RST_R {
        AD0RST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD0RST"]
    #[inline(always)]
    #[must_use]
    pub fn ad0rst(&mut self) -> AD0RST_W<0> {
        AD0RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0RST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0rst](index.html) module"]
pub struct ADC0RST_SPEC;
impl crate::RegisterSpec for ADC0RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0rst::R](R) reader structure"]
impl crate::Readable for ADC0RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0rst::W](W) writer structure"]
impl crate::Writable for ADC0RST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0RST to value 0"]
impl crate::Resettable for ADC0RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
