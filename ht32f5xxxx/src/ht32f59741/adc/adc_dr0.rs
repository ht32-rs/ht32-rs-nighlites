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
#[doc = "Field `ADD0` reader - ADD0"]
pub type ADD0_R = crate::FieldReader<u16>;
#[doc = "Field `ADD0` writer - ADD0"]
pub type ADD0_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_DR0_SPEC, 16, O, u16>;
#[doc = "Field `ADVLD0` reader - ADVLD0"]
pub type ADVLD0_R = crate::BitReader;
#[doc = "Field `ADVLD0` writer - ADVLD0"]
pub type ADVLD0_W<'a, const O: u8> = crate::BitWriter<'a, ADC_DR0_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - ADD0"]
    #[inline(always)]
    pub fn add0(&self) -> ADD0_R {
        ADD0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD0"]
    #[inline(always)]
    pub fn advld0(&self) -> ADVLD0_R {
        ADVLD0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD0"]
    #[inline(always)]
    #[must_use]
    pub fn add0(&mut self) -> ADD0_W<0> {
        ADD0_W::new(self)
    }
    #[doc = "Bit 31 - ADVLD0"]
    #[inline(always)]
    #[must_use]
    pub fn advld0(&mut self) -> ADVLD0_W<31> {
        ADVLD0_W::new(self)
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
