#[doc = "Register `ADC1OFR6` reader"]
pub struct R(crate::R<ADC1OFR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1OFR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1OFR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1OFR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1OFR6` writer"]
pub struct W(crate::W<ADC1OFR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1OFR6_SPEC>;
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
impl From<crate::W<ADC1OFR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1OFR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1OF` reader - AD1OF"]
pub type AD1OF_R = crate::FieldReader<u16>;
#[doc = "Field `AD1OF` writer - AD1OF"]
pub type AD1OF_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1OFR6_SPEC, 12, O, u16>;
#[doc = "Field `AD1AL` reader - AD1AL"]
pub type AD1AL_R = crate::BitReader;
#[doc = "Field `AD1AL` writer - AD1AL"]
pub type AD1AL_W<'a, const O: u8> = crate::BitWriter<'a, ADC1OFR6_SPEC, O>;
#[doc = "Field `AD1OFE` reader - AD1OFE"]
pub type AD1OFE_R = crate::BitReader;
#[doc = "Field `AD1OFE` writer - AD1OFE"]
pub type AD1OFE_W<'a, const O: u8> = crate::BitWriter<'a, ADC1OFR6_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - AD1OF"]
    #[inline(always)]
    pub fn ad1of(&self) -> AD1OF_R {
        AD1OF_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - AD1AL"]
    #[inline(always)]
    pub fn ad1al(&self) -> AD1AL_R {
        AD1AL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AD1OFE"]
    #[inline(always)]
    pub fn ad1ofe(&self) -> AD1OFE_R {
        AD1OFE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - AD1OF"]
    #[inline(always)]
    #[must_use]
    pub fn ad1of(&mut self) -> AD1OF_W<0> {
        AD1OF_W::new(self)
    }
    #[doc = "Bit 14 - AD1AL"]
    #[inline(always)]
    #[must_use]
    pub fn ad1al(&mut self) -> AD1AL_W<14> {
        AD1AL_W::new(self)
    }
    #[doc = "Bit 15 - AD1OFE"]
    #[inline(always)]
    #[must_use]
    pub fn ad1ofe(&mut self) -> AD1OFE_W<15> {
        AD1OFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1OFR6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1ofr6](index.html) module"]
pub struct ADC1OFR6_SPEC;
impl crate::RegisterSpec for ADC1OFR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1ofr6::R](R) reader structure"]
impl crate::Readable for ADC1OFR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1ofr6::W](W) writer structure"]
impl crate::Writable for ADC1OFR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1OFR6 to value 0"]
impl crate::Resettable for ADC1OFR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
