#[doc = "Register `ADC0OFR4` reader"]
pub struct R(crate::R<ADC0OFR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0OFR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0OFR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0OFR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0OFR4` writer"]
pub struct W(crate::W<ADC0OFR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0OFR4_SPEC>;
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
impl From<crate::W<ADC0OFR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0OFR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0OF` reader - AD0OF"]
pub type AD0OF_R = crate::FieldReader<u16>;
#[doc = "Field `AD0OF` writer - AD0OF"]
pub type AD0OF_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0OFR4_SPEC, 12, O, u16>;
#[doc = "Field `AD0AL` reader - AD0AL"]
pub type AD0AL_R = crate::BitReader;
#[doc = "Field `AD0AL` writer - AD0AL"]
pub type AD0AL_W<'a, const O: u8> = crate::BitWriter<'a, ADC0OFR4_SPEC, O>;
#[doc = "Field `AD0OFE` reader - AD0OFE"]
pub type AD0OFE_R = crate::BitReader;
#[doc = "Field `AD0OFE` writer - AD0OFE"]
pub type AD0OFE_W<'a, const O: u8> = crate::BitWriter<'a, ADC0OFR4_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - AD0OF"]
    #[inline(always)]
    pub fn ad0of(&self) -> AD0OF_R {
        AD0OF_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - AD0AL"]
    #[inline(always)]
    pub fn ad0al(&self) -> AD0AL_R {
        AD0AL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AD0OFE"]
    #[inline(always)]
    pub fn ad0ofe(&self) -> AD0OFE_R {
        AD0OFE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - AD0OF"]
    #[inline(always)]
    #[must_use]
    pub fn ad0of(&mut self) -> AD0OF_W<0> {
        AD0OF_W::new(self)
    }
    #[doc = "Bit 14 - AD0AL"]
    #[inline(always)]
    #[must_use]
    pub fn ad0al(&mut self) -> AD0AL_W<14> {
        AD0AL_W::new(self)
    }
    #[doc = "Bit 15 - AD0OFE"]
    #[inline(always)]
    #[must_use]
    pub fn ad0ofe(&mut self) -> AD0OFE_W<15> {
        AD0OFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0OFR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0ofr4](index.html) module"]
pub struct ADC0OFR4_SPEC;
impl crate::RegisterSpec for ADC0OFR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0ofr4::R](R) reader structure"]
impl crate::Readable for ADC0OFR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0ofr4::W](W) writer structure"]
impl crate::Writable for ADC0OFR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0OFR4 to value 0"]
impl crate::Resettable for ADC0OFR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
