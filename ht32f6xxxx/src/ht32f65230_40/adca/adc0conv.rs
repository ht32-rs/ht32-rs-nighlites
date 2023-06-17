#[doc = "Register `ADC0CONV` reader"]
pub struct R(crate::R<ADC0CONV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0CONV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0CONV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0CONV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0CONV` writer"]
pub struct W(crate::W<ADC0CONV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0CONV_SPEC>;
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
impl From<crate::W<ADC0CONV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0CONV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0MODE` reader - AD0MODE"]
pub type AD0MODE_R = crate::FieldReader;
#[doc = "Field `AD0MODE` writer - AD0MODE"]
pub type AD0MODE_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0CONV_SPEC, 2, O>;
#[doc = "Field `ADC0EN` reader - ADC0EN"]
pub type ADC0EN_R = crate::BitReader;
#[doc = "Field `ADC0EN` writer - ADC0EN"]
pub type ADC0EN_W<'a, const O: u8> = crate::BitWriter<'a, ADC0CONV_SPEC, O>;
#[doc = "Field `AD0SEQL` reader - AD0SEQL"]
pub type AD0SEQL_R = crate::FieldReader;
#[doc = "Field `AD0SEQL` writer - AD0SEQL"]
pub type AD0SEQL_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0CONV_SPEC, 4, O>;
#[doc = "Field `AD0SUBL` reader - AD0SUBL"]
pub type AD0SUBL_R = crate::FieldReader;
#[doc = "Field `AD0SUBL` writer - AD0SUBL"]
pub type AD0SUBL_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0CONV_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:1 - AD0MODE"]
    #[inline(always)]
    pub fn ad0mode(&self) -> AD0MODE_R {
        AD0MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - ADC0EN"]
    #[inline(always)]
    pub fn adc0en(&self) -> ADC0EN_R {
        ADC0EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - AD0SEQL"]
    #[inline(always)]
    pub fn ad0seql(&self) -> AD0SEQL_R {
        AD0SEQL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AD0SUBL"]
    #[inline(always)]
    pub fn ad0subl(&self) -> AD0SUBL_R {
        AD0SUBL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AD0MODE"]
    #[inline(always)]
    #[must_use]
    pub fn ad0mode(&mut self) -> AD0MODE_W<0> {
        AD0MODE_W::new(self)
    }
    #[doc = "Bit 7 - ADC0EN"]
    #[inline(always)]
    #[must_use]
    pub fn adc0en(&mut self) -> ADC0EN_W<7> {
        ADC0EN_W::new(self)
    }
    #[doc = "Bits 8:11 - AD0SEQL"]
    #[inline(always)]
    #[must_use]
    pub fn ad0seql(&mut self) -> AD0SEQL_W<8> {
        AD0SEQL_W::new(self)
    }
    #[doc = "Bits 16:19 - AD0SUBL"]
    #[inline(always)]
    #[must_use]
    pub fn ad0subl(&mut self) -> AD0SUBL_W<16> {
        AD0SUBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0CONV\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0conv](index.html) module"]
pub struct ADC0CONV_SPEC;
impl crate::RegisterSpec for ADC0CONV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0conv::R](R) reader structure"]
impl crate::Readable for ADC0CONV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0conv::W](W) writer structure"]
impl crate::Writable for ADC0CONV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0CONV to value 0"]
impl crate::Resettable for ADC0CONV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
