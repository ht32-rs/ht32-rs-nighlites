#[doc = "Register `ADC0TSR` reader"]
pub struct R(crate::R<ADC0TSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0TSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0TSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0TSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0TSR` writer"]
pub struct W(crate::W<ADC0TSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0TSR_SPEC>;
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
impl From<crate::W<ADC0TSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0TSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0SC` reader - AD0SC"]
pub type AD0SC_R = crate::BitReader;
#[doc = "Field `AD0SC` writer - AD0SC"]
pub type AD0SC_W<'a, const O: u8> = crate::BitWriter<'a, ADC0TSR_SPEC, O>;
#[doc = "Field `AD0EXTIS` reader - AD0EXTIS"]
pub type AD0EXTIS_R = crate::FieldReader;
#[doc = "Field `AD0EXTIS` writer - AD0EXTIS"]
pub type AD0EXTIS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0TSR_SPEC, 4, O>;
#[doc = "Field `AD0TMS` reader - AD0TMS"]
pub type AD0TMS_R = crate::FieldReader;
#[doc = "Field `AD0TMS` writer - AD0TMS"]
pub type AD0TMS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0TSR_SPEC, 3, O>;
#[doc = "Field `AD0BFTMS` reader - AD0BFTMS"]
pub type AD0BFTMS_R = crate::BitReader;
#[doc = "Field `AD0BFTMS` writer - AD0BFTMS"]
pub type AD0BFTMS_W<'a, const O: u8> = crate::BitWriter<'a, ADC0TSR_SPEC, O>;
#[doc = "Field `AD0CMPS` reader - AD0CMPS"]
pub type AD0CMPS_R = crate::FieldReader;
#[doc = "Field `AD0CMPS` writer - AD0CMPS"]
pub type AD0CMPS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0TSR_SPEC, 2, O>;
#[doc = "Field `AD0TME` reader - AD0TME"]
pub type AD0TME_R = crate::FieldReader;
#[doc = "Field `AD0TME` writer - AD0TME"]
pub type AD0TME_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0TSR_SPEC, 4, O>;
#[doc = "Field `AD0MCMS` reader - AD0MCMS"]
pub type AD0MCMS_R = crate::FieldReader;
#[doc = "Field `AD0MCMS` writer - AD0MCMS"]
pub type AD0MCMS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0TSR_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0 - AD0SC"]
    #[inline(always)]
    pub fn ad0sc(&self) -> AD0SC_R {
        AD0SC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - AD0EXTIS"]
    #[inline(always)]
    pub fn ad0extis(&self) -> AD0EXTIS_R {
        AD0EXTIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - AD0TMS"]
    #[inline(always)]
    pub fn ad0tms(&self) -> AD0TMS_R {
        AD0TMS_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - AD0BFTMS"]
    #[inline(always)]
    pub fn ad0bftms(&self) -> AD0BFTMS_R {
        AD0BFTMS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - AD0CMPS"]
    #[inline(always)]
    pub fn ad0cmps(&self) -> AD0CMPS_R {
        AD0CMPS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27 - AD0TME"]
    #[inline(always)]
    pub fn ad0tme(&self) -> AD0TME_R {
        AD0TME_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - AD0MCMS"]
    #[inline(always)]
    pub fn ad0mcms(&self) -> AD0MCMS_R {
        AD0MCMS_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AD0SC"]
    #[inline(always)]
    #[must_use]
    pub fn ad0sc(&mut self) -> AD0SC_W<0> {
        AD0SC_W::new(self)
    }
    #[doc = "Bits 8:11 - AD0EXTIS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0extis(&mut self) -> AD0EXTIS_W<8> {
        AD0EXTIS_W::new(self)
    }
    #[doc = "Bits 16:18 - AD0TMS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0tms(&mut self) -> AD0TMS_W<16> {
        AD0TMS_W::new(self)
    }
    #[doc = "Bit 19 - AD0BFTMS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0bftms(&mut self) -> AD0BFTMS_W<19> {
        AD0BFTMS_W::new(self)
    }
    #[doc = "Bits 20:21 - AD0CMPS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0cmps(&mut self) -> AD0CMPS_W<20> {
        AD0CMPS_W::new(self)
    }
    #[doc = "Bits 24:27 - AD0TME"]
    #[inline(always)]
    #[must_use]
    pub fn ad0tme(&mut self) -> AD0TME_W<24> {
        AD0TME_W::new(self)
    }
    #[doc = "Bits 28:29 - AD0MCMS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0mcms(&mut self) -> AD0MCMS_W<28> {
        AD0MCMS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0tsr](index.html) module"]
pub struct ADC0TSR_SPEC;
impl crate::RegisterSpec for ADC0TSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0tsr::R](R) reader structure"]
impl crate::Readable for ADC0TSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0tsr::W](W) writer structure"]
impl crate::Writable for ADC0TSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0TSR to value 0"]
impl crate::Resettable for ADC0TSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
