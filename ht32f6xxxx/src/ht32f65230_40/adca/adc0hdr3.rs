#[doc = "Register `ADC0HDR3` reader"]
pub struct R(crate::R<ADC0HDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0HDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0HDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0HDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0HDR3` writer"]
pub struct W(crate::W<ADC0HDR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0HDR3_SPEC>;
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
impl From<crate::W<ADC0HDR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0HDR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0HD` reader - AD0HD"]
pub type AD0HD_R = crate::FieldReader<u16>;
#[doc = "Field `AD0HD` writer - AD0HD"]
pub type AD0HD_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0HDR3_SPEC, 16, O, u16>;
#[doc = "Field `AD0HVLD` reader - AD0HVLD"]
pub type AD0HVLD_R = crate::BitReader;
#[doc = "Field `AD0HVLD` writer - AD0HVLD"]
pub type AD0HVLD_W<'a, const O: u8> = crate::BitWriter<'a, ADC0HDR3_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - AD0HD"]
    #[inline(always)]
    pub fn ad0hd(&self) -> AD0HD_R {
        AD0HD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - AD0HVLD"]
    #[inline(always)]
    pub fn ad0hvld(&self) -> AD0HVLD_R {
        AD0HVLD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - AD0HD"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hd(&mut self) -> AD0HD_W<0> {
        AD0HD_W::new(self)
    }
    #[doc = "Bit 31 - AD0HVLD"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hvld(&mut self) -> AD0HVLD_W<31> {
        AD0HVLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0HDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0hdr3](index.html) module"]
pub struct ADC0HDR3_SPEC;
impl crate::RegisterSpec for ADC0HDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0hdr3::R](R) reader structure"]
impl crate::Readable for ADC0HDR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0hdr3::W](W) writer structure"]
impl crate::Writable for ADC0HDR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0HDR3 to value 0"]
impl crate::Resettable for ADC0HDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
