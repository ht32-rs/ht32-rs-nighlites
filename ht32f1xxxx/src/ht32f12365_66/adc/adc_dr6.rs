#[doc = "Register `ADC_DR6` reader"]
pub struct R(crate::R<ADC_DR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_DR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_DR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_DR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_DR6` writer"]
pub struct W(crate::W<ADC_DR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_DR6_SPEC>;
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
impl From<crate::W<ADC_DR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_DR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD6` reader - ADD6"]
pub type ADD6_R = crate::FieldReader<u16>;
#[doc = "Field `ADD6` writer - ADD6"]
pub type ADD6_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_DR6_SPEC, 16, O, u16>;
#[doc = "Field `ADVLD6` reader - ADVLD6"]
pub type ADVLD6_R = crate::BitReader;
#[doc = "Field `ADVLD6` writer - ADVLD6"]
pub type ADVLD6_W<'a, const O: u8> = crate::BitWriter<'a, ADC_DR6_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - ADD6"]
    #[inline(always)]
    pub fn add6(&self) -> ADD6_R {
        ADD6_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD6"]
    #[inline(always)]
    pub fn advld6(&self) -> ADVLD6_R {
        ADVLD6_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD6"]
    #[inline(always)]
    #[must_use]
    pub fn add6(&mut self) -> ADD6_W<0> {
        ADD6_W::new(self)
    }
    #[doc = "Bit 31 - ADVLD6"]
    #[inline(always)]
    #[must_use]
    pub fn advld6(&mut self) -> ADVLD6_W<31> {
        ADVLD6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_DR6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr6](index.html) module"]
pub struct ADC_DR6_SPEC;
impl crate::RegisterSpec for ADC_DR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_dr6::R](R) reader structure"]
impl crate::Readable for ADC_DR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_dr6::W](W) writer structure"]
impl crate::Writable for ADC_DR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_DR6 to value 0"]
impl crate::Resettable for ADC_DR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
