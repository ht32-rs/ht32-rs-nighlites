#[doc = "Register `ADC_CONV` reader"]
pub struct R(crate::R<ADC_CONV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CONV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CONV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CONV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CONV` writer"]
pub struct W(crate::W<ADC_CONV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CONV_SPEC>;
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
impl From<crate::W<ADC_CONV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CONV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADMODE` reader - ADMODE"]
pub type ADMODE_R = crate::FieldReader;
#[doc = "Field `ADMODE` writer - ADMODE"]
pub type ADMODE_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_CONV_SPEC, 2, O>;
#[doc = "Field `ADSEQL` reader - ADSEQL"]
pub type ADSEQL_R = crate::FieldReader;
#[doc = "Field `ADSEQL` writer - ADSEQL"]
pub type ADSEQL_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_CONV_SPEC, 3, O>;
#[doc = "Field `ADSUBL` reader - ADSUBL"]
pub type ADSUBL_R = crate::FieldReader;
#[doc = "Field `ADSUBL` writer - ADSUBL"]
pub type ADSUBL_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_CONV_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:1 - ADMODE"]
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:10 - ADSEQL"]
    #[inline(always)]
    pub fn adseql(&self) -> ADSEQL_R {
        ADSEQL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - ADSUBL"]
    #[inline(always)]
    pub fn adsubl(&self) -> ADSUBL_R {
        ADSUBL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADMODE"]
    #[inline(always)]
    #[must_use]
    pub fn admode(&mut self) -> ADMODE_W<0> {
        ADMODE_W::new(self)
    }
    #[doc = "Bits 8:10 - ADSEQL"]
    #[inline(always)]
    #[must_use]
    pub fn adseql(&mut self) -> ADSEQL_W<8> {
        ADSEQL_W::new(self)
    }
    #[doc = "Bits 16:18 - ADSUBL"]
    #[inline(always)]
    #[must_use]
    pub fn adsubl(&mut self) -> ADSUBL_W<16> {
        ADSUBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_CONV\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_conv](index.html) module"]
pub struct ADC_CONV_SPEC;
impl crate::RegisterSpec for ADC_CONV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_conv::R](R) reader structure"]
impl crate::Readable for ADC_CONV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_conv::W](W) writer structure"]
impl crate::Writable for ADC_CONV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_CONV to value 0"]
impl crate::Resettable for ADC_CONV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
