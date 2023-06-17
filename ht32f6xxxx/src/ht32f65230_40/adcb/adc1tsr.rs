#[doc = "Register `ADC1TSR` reader"]
pub struct R(crate::R<ADC1TSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1TSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1TSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1TSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1TSR` writer"]
pub struct W(crate::W<ADC1TSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1TSR_SPEC>;
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
impl From<crate::W<ADC1TSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1TSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1SC` reader - AD1SC"]
pub type AD1SC_R = crate::BitReader;
#[doc = "Field `AD1SC` writer - AD1SC"]
pub type AD1SC_W<'a, const O: u8> = crate::BitWriter<'a, ADC1TSR_SPEC, O>;
#[doc = "Field `AD1EXTIS` reader - AD1EXTIS"]
pub type AD1EXTIS_R = crate::FieldReader;
#[doc = "Field `AD1EXTIS` writer - AD1EXTIS"]
pub type AD1EXTIS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1TSR_SPEC, 4, O>;
#[doc = "Field `AD1TMS` reader - AD1TMS"]
pub type AD1TMS_R = crate::FieldReader;
#[doc = "Field `AD1TMS` writer - AD1TMS"]
pub type AD1TMS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1TSR_SPEC, 3, O>;
#[doc = "Field `AD1BFTMS` reader - AD1BFTMS"]
pub type AD1BFTMS_R = crate::BitReader;
#[doc = "Field `AD1BFTMS` writer - AD1BFTMS"]
pub type AD1BFTMS_W<'a, const O: u8> = crate::BitWriter<'a, ADC1TSR_SPEC, O>;
#[doc = "Field `AD1CMPS` reader - AD1CMPS"]
pub type AD1CMPS_R = crate::FieldReader;
#[doc = "Field `AD1CMPS` writer - AD1CMPS"]
pub type AD1CMPS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1TSR_SPEC, 2, O>;
#[doc = "Field `AD1TME` reader - AD1TME"]
pub type AD1TME_R = crate::FieldReader;
#[doc = "Field `AD1TME` writer - AD1TME"]
pub type AD1TME_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1TSR_SPEC, 4, O>;
#[doc = "Field `AD1MCMS` reader - AD1MCMS"]
pub type AD1MCMS_R = crate::FieldReader;
#[doc = "Field `AD1MCMS` writer - AD1MCMS"]
pub type AD1MCMS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1TSR_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0 - AD1SC"]
    #[inline(always)]
    pub fn ad1sc(&self) -> AD1SC_R {
        AD1SC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - AD1EXTIS"]
    #[inline(always)]
    pub fn ad1extis(&self) -> AD1EXTIS_R {
        AD1EXTIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - AD1TMS"]
    #[inline(always)]
    pub fn ad1tms(&self) -> AD1TMS_R {
        AD1TMS_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - AD1BFTMS"]
    #[inline(always)]
    pub fn ad1bftms(&self) -> AD1BFTMS_R {
        AD1BFTMS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - AD1CMPS"]
    #[inline(always)]
    pub fn ad1cmps(&self) -> AD1CMPS_R {
        AD1CMPS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27 - AD1TME"]
    #[inline(always)]
    pub fn ad1tme(&self) -> AD1TME_R {
        AD1TME_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - AD1MCMS"]
    #[inline(always)]
    pub fn ad1mcms(&self) -> AD1MCMS_R {
        AD1MCMS_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AD1SC"]
    #[inline(always)]
    #[must_use]
    pub fn ad1sc(&mut self) -> AD1SC_W<0> {
        AD1SC_W::new(self)
    }
    #[doc = "Bits 8:11 - AD1EXTIS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1extis(&mut self) -> AD1EXTIS_W<8> {
        AD1EXTIS_W::new(self)
    }
    #[doc = "Bits 16:18 - AD1TMS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tms(&mut self) -> AD1TMS_W<16> {
        AD1TMS_W::new(self)
    }
    #[doc = "Bit 19 - AD1BFTMS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1bftms(&mut self) -> AD1BFTMS_W<19> {
        AD1BFTMS_W::new(self)
    }
    #[doc = "Bits 20:21 - AD1CMPS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1cmps(&mut self) -> AD1CMPS_W<20> {
        AD1CMPS_W::new(self)
    }
    #[doc = "Bits 24:27 - AD1TME"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tme(&mut self) -> AD1TME_W<24> {
        AD1TME_W::new(self)
    }
    #[doc = "Bits 28:29 - AD1MCMS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1mcms(&mut self) -> AD1MCMS_W<28> {
        AD1MCMS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1tsr](index.html) module"]
pub struct ADC1TSR_SPEC;
impl crate::RegisterSpec for ADC1TSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1tsr::R](R) reader structure"]
impl crate::Readable for ADC1TSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1tsr::W](W) writer structure"]
impl crate::Writable for ADC1TSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1TSR to value 0"]
impl crate::Resettable for ADC1TSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
