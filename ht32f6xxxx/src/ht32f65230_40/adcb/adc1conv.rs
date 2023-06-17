#[doc = "Register `ADC1CONV` reader"]
pub struct R(crate::R<ADC1CONV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1CONV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1CONV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1CONV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1CONV` writer"]
pub struct W(crate::W<ADC1CONV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1CONV_SPEC>;
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
impl From<crate::W<ADC1CONV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1CONV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1MODE` reader - AD1MODE"]
pub type AD1MODE_R = crate::FieldReader;
#[doc = "Field `AD1MODE` writer - AD1MODE"]
pub type AD1MODE_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1CONV_SPEC, 2, O>;
#[doc = "Field `ADC1EN` reader - ADC1EN"]
pub type ADC1EN_R = crate::BitReader;
#[doc = "Field `ADC1EN` writer - ADC1EN"]
pub type ADC1EN_W<'a, const O: u8> = crate::BitWriter<'a, ADC1CONV_SPEC, O>;
#[doc = "Field `AD1SEQL` reader - AD1SEQL"]
pub type AD1SEQL_R = crate::FieldReader;
#[doc = "Field `AD1SEQL` writer - AD1SEQL"]
pub type AD1SEQL_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1CONV_SPEC, 4, O>;
#[doc = "Field `AD1SUBL` reader - AD1SUBL"]
pub type AD1SUBL_R = crate::FieldReader;
#[doc = "Field `AD1SUBL` writer - AD1SUBL"]
pub type AD1SUBL_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1CONV_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:1 - AD1MODE"]
    #[inline(always)]
    pub fn ad1mode(&self) -> AD1MODE_R {
        AD1MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - ADC1EN"]
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - AD1SEQL"]
    #[inline(always)]
    pub fn ad1seql(&self) -> AD1SEQL_R {
        AD1SEQL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AD1SUBL"]
    #[inline(always)]
    pub fn ad1subl(&self) -> AD1SUBL_R {
        AD1SUBL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AD1MODE"]
    #[inline(always)]
    #[must_use]
    pub fn ad1mode(&mut self) -> AD1MODE_W<0> {
        AD1MODE_W::new(self)
    }
    #[doc = "Bit 7 - ADC1EN"]
    #[inline(always)]
    #[must_use]
    pub fn adc1en(&mut self) -> ADC1EN_W<7> {
        ADC1EN_W::new(self)
    }
    #[doc = "Bits 8:11 - AD1SEQL"]
    #[inline(always)]
    #[must_use]
    pub fn ad1seql(&mut self) -> AD1SEQL_W<8> {
        AD1SEQL_W::new(self)
    }
    #[doc = "Bits 16:19 - AD1SUBL"]
    #[inline(always)]
    #[must_use]
    pub fn ad1subl(&mut self) -> AD1SUBL_W<16> {
        AD1SUBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1CONV\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1conv](index.html) module"]
pub struct ADC1CONV_SPEC;
impl crate::RegisterSpec for ADC1CONV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1conv::R](R) reader structure"]
impl crate::Readable for ADC1CONV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1conv::W](W) writer structure"]
impl crate::Writable for ADC1CONV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1CONV to value 0"]
impl crate::Resettable for ADC1CONV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
