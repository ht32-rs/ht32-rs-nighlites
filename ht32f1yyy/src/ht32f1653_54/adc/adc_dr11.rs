#[doc = "Register `ADC_DR11` reader"]
pub struct R(crate::R<ADC_DR11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_DR11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_DR11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_DR11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_DR11` writer"]
pub struct W(crate::W<ADC_DR11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_DR11_SPEC>;
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
impl From<crate::W<ADC_DR11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_DR11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD11` reader - ADD11"]
pub type ADD11_R = crate::FieldReader<u16>;
#[doc = "Field `ADD11` writer - ADD11"]
pub type ADD11_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_DR11_SPEC, 16, O, u16>;
#[doc = "Field `ADVLD11` reader - ADVLD11"]
pub type ADVLD11_R = crate::BitReader;
#[doc = "Field `ADVLD11` writer - ADVLD11"]
pub type ADVLD11_W<'a, const O: u8> = crate::BitWriter<'a, ADC_DR11_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - ADD11"]
    #[inline(always)]
    pub fn add11(&self) -> ADD11_R {
        ADD11_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD11"]
    #[inline(always)]
    pub fn advld11(&self) -> ADVLD11_R {
        ADVLD11_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD11"]
    #[inline(always)]
    #[must_use]
    pub fn add11(&mut self) -> ADD11_W<0> {
        ADD11_W::new(self)
    }
    #[doc = "Bit 31 - ADVLD11"]
    #[inline(always)]
    #[must_use]
    pub fn advld11(&mut self) -> ADVLD11_W<31> {
        ADVLD11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_DR11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr11](index.html) module"]
pub struct ADC_DR11_SPEC;
impl crate::RegisterSpec for ADC_DR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_dr11::R](R) reader structure"]
impl crate::Readable for ADC_DR11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_dr11::W](W) writer structure"]
impl crate::Writable for ADC_DR11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_DR11 to value 0"]
impl crate::Resettable for ADC_DR11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
