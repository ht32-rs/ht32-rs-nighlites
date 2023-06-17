#[doc = "Register `ADC_DR0` reader"]
pub struct R(crate::R<ADC_DR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_DR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_DR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_DR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_DR0` writer"]
pub struct W(crate::W<ADC_DR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_DR0_SPEC>;
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
impl From<crate::W<ADC_DR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_DR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDy` reader - ADDy"]
pub type ADDY_R = crate::FieldReader<u16>;
#[doc = "Field `ADDy` writer - ADDy"]
pub type ADDY_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_DR0_SPEC, 16, O, u16>;
#[doc = "Field `ADVLDy` reader - ADVLDy"]
pub type ADVLDY_R = crate::BitReader;
#[doc = "Field `ADVLDy` writer - ADVLDy"]
pub type ADVLDY_W<'a, const O: u8> = crate::BitWriter<'a, ADC_DR0_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - ADDy"]
    #[inline(always)]
    pub fn addy(&self) -> ADDY_R {
        ADDY_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLDy"]
    #[inline(always)]
    pub fn advldy(&self) -> ADVLDY_R {
        ADVLDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDy"]
    #[inline(always)]
    #[must_use]
    pub fn addy(&mut self) -> ADDY_W<0> {
        ADDY_W::new(self)
    }
    #[doc = "Bit 31 - ADVLDy"]
    #[inline(always)]
    #[must_use]
    pub fn advldy(&mut self) -> ADVLDY_W<31> {
        ADVLDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_DR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr0](index.html) module"]
pub struct ADC_DR0_SPEC;
impl crate::RegisterSpec for ADC_DR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_dr0::R](R) reader structure"]
impl crate::Readable for ADC_DR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_dr0::W](W) writer structure"]
impl crate::Writable for ADC_DR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_DR0 to value 0"]
impl crate::Resettable for ADC_DR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
