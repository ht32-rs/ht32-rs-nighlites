#[doc = "Register `ADC_DR14` reader"]
pub struct R(crate::R<ADC_DR14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_DR14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_DR14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_DR14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_DR14` writer"]
pub struct W(crate::W<ADC_DR14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_DR14_SPEC>;
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
impl From<crate::W<ADC_DR14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_DR14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD14` reader - ADD14"]
pub type ADD14_R = crate::FieldReader<u16>;
#[doc = "Field `ADD14` writer - ADD14"]
pub type ADD14_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_DR14_SPEC, 16, O, u16>;
#[doc = "Field `ADVLD14` reader - ADVLD14"]
pub type ADVLD14_R = crate::BitReader;
#[doc = "Field `ADVLD14` writer - ADVLD14"]
pub type ADVLD14_W<'a, const O: u8> = crate::BitWriter<'a, ADC_DR14_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - ADD14"]
    #[inline(always)]
    pub fn add14(&self) -> ADD14_R {
        ADD14_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD14"]
    #[inline(always)]
    pub fn advld14(&self) -> ADVLD14_R {
        ADVLD14_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD14"]
    #[inline(always)]
    #[must_use]
    pub fn add14(&mut self) -> ADD14_W<0> {
        ADD14_W::new(self)
    }
    #[doc = "Bit 31 - ADVLD14"]
    #[inline(always)]
    #[must_use]
    pub fn advld14(&mut self) -> ADVLD14_W<31> {
        ADVLD14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_DR14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr14](index.html) module"]
pub struct ADC_DR14_SPEC;
impl crate::RegisterSpec for ADC_DR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_dr14::R](R) reader structure"]
impl crate::Readable for ADC_DR14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_dr14::W](W) writer structure"]
impl crate::Writable for ADC_DR14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_DR14 to value 0"]
impl crate::Resettable for ADC_DR14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
