#[doc = "Register `ADC1HDR1` reader"]
pub struct R(crate::R<ADC1HDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1HDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1HDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1HDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1HDR1` writer"]
pub struct W(crate::W<ADC1HDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1HDR1_SPEC>;
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
impl From<crate::W<ADC1HDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1HDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1HD` reader - AD1HD"]
pub type AD1HD_R = crate::FieldReader<u16>;
#[doc = "Field `AD1HD` writer - AD1HD"]
pub type AD1HD_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1HDR1_SPEC, 16, O, u16>;
#[doc = "Field `AD1HVLD` reader - AD1HVLD"]
pub type AD1HVLD_R = crate::BitReader;
#[doc = "Field `AD1HVLD` writer - AD1HVLD"]
pub type AD1HVLD_W<'a, const O: u8> = crate::BitWriter<'a, ADC1HDR1_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - AD1HD"]
    #[inline(always)]
    pub fn ad1hd(&self) -> AD1HD_R {
        AD1HD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - AD1HVLD"]
    #[inline(always)]
    pub fn ad1hvld(&self) -> AD1HVLD_R {
        AD1HVLD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - AD1HD"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hd(&mut self) -> AD1HD_W<0> {
        AD1HD_W::new(self)
    }
    #[doc = "Bit 31 - AD1HVLD"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hvld(&mut self) -> AD1HVLD_W<31> {
        AD1HVLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1HDR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1hdr1](index.html) module"]
pub struct ADC1HDR1_SPEC;
impl crate::RegisterSpec for ADC1HDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1hdr1::R](R) reader structure"]
impl crate::Readable for ADC1HDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1hdr1::W](W) writer structure"]
impl crate::Writable for ADC1HDR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1HDR1 to value 0"]
impl crate::Resettable for ADC1HDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
