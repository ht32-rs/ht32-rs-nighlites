#[doc = "Register `ADC_DR10` reader"]
pub struct R(crate::R<ADC_DR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_DR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_DR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_DR10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_DR10` writer"]
pub struct W(crate::W<ADC_DR10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_DR10_SPEC>;
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
impl From<crate::W<ADC_DR10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_DR10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD10` reader - ADD10"]
pub type ADD10_R = crate::FieldReader<u16>;
#[doc = "Field `ADD10` writer - ADD10"]
pub type ADD10_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_DR10_SPEC, 16, O, u16>;
#[doc = "Field `ADVLD10` reader - ADVLD10"]
pub type ADVLD10_R = crate::BitReader;
#[doc = "Field `ADVLD10` writer - ADVLD10"]
pub type ADVLD10_W<'a, const O: u8> = crate::BitWriter<'a, ADC_DR10_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - ADD10"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD10"]
    #[inline(always)]
    pub fn advld10(&self) -> ADVLD10_R {
        ADVLD10_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD10"]
    #[inline(always)]
    #[must_use]
    pub fn add10(&mut self) -> ADD10_W<0> {
        ADD10_W::new(self)
    }
    #[doc = "Bit 31 - ADVLD10"]
    #[inline(always)]
    #[must_use]
    pub fn advld10(&mut self) -> ADVLD10_W<31> {
        ADVLD10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_DR10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr10](index.html) module"]
pub struct ADC_DR10_SPEC;
impl crate::RegisterSpec for ADC_DR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_dr10::R](R) reader structure"]
impl crate::Readable for ADC_DR10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_dr10::W](W) writer structure"]
impl crate::Writable for ADC_DR10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_DR10 to value 0"]
impl crate::Resettable for ADC_DR10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
