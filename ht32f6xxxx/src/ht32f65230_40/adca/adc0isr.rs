#[doc = "Register `ADC0ISR` reader"]
pub struct R(crate::R<ADC0ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0ISR` writer"]
pub struct W(crate::W<ADC0ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0ISR_SPEC>;
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
impl From<crate::W<ADC0ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0ISRS` reader - AD0ISRS"]
pub type AD0ISRS_R = crate::BitReader;
#[doc = "Field `AD0ISRS` writer - AD0ISRS"]
pub type AD0ISRS_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ISR_SPEC, O>;
#[doc = "Field `AD0ISRG` reader - AD0ISRG"]
pub type AD0ISRG_R = crate::BitReader;
#[doc = "Field `AD0ISRG` writer - AD0ISRG"]
pub type AD0ISRG_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ISR_SPEC, O>;
#[doc = "Field `AD0ISRC` reader - AD0ISRC"]
pub type AD0ISRC_R = crate::BitReader;
#[doc = "Field `AD0ISRC` writer - AD0ISRC"]
pub type AD0ISRC_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ISR_SPEC, O>;
#[doc = "Field `AD0ISRHS` reader - AD0ISRHS"]
pub type AD0ISRHS_R = crate::BitReader;
#[doc = "Field `AD0ISRHS` writer - AD0ISRHS"]
pub type AD0ISRHS_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ISR_SPEC, O>;
#[doc = "Field `AD0ISRHG` reader - AD0ISRHG"]
pub type AD0ISRHG_R = crate::BitReader;
#[doc = "Field `AD0ISRHG` writer - AD0ISRHG"]
pub type AD0ISRHG_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ISR_SPEC, O>;
#[doc = "Field `AD0ISRHC` reader - AD0ISRHC"]
pub type AD0ISRHC_R = crate::BitReader;
#[doc = "Field `AD0ISRHC` writer - AD0ISRHC"]
pub type AD0ISRHC_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ISR_SPEC, O>;
#[doc = "Field `AD0ISRL` reader - AD0ISRL"]
pub type AD0ISRL_R = crate::BitReader;
#[doc = "Field `AD0ISRL` writer - AD0ISRL"]
pub type AD0ISRL_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ISR_SPEC, O>;
#[doc = "Field `AD0ISRU` reader - AD0ISRU"]
pub type AD0ISRU_R = crate::BitReader;
#[doc = "Field `AD0ISRU` writer - AD0ISRU"]
pub type AD0ISRU_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ISR_SPEC, O>;
#[doc = "Field `AD0ISRO` reader - AD0ISRO"]
pub type AD0ISRO_R = crate::BitReader;
#[doc = "Field `AD0ISRO` writer - AD0ISRO"]
pub type AD0ISRO_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ISR_SPEC, O>;
#[doc = "Field `AD0ISRHO` reader - AD0ISRHO"]
pub type AD0ISRHO_R = crate::BitReader;
#[doc = "Field `AD0ISRHO` writer - AD0ISRHO"]
pub type AD0ISRHO_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ISR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AD0ISRS"]
    #[inline(always)]
    pub fn ad0isrs(&self) -> AD0ISRS_R {
        AD0ISRS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD0ISRG"]
    #[inline(always)]
    pub fn ad0isrg(&self) -> AD0ISRG_R {
        AD0ISRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD0ISRC"]
    #[inline(always)]
    pub fn ad0isrc(&self) -> AD0ISRC_R {
        AD0ISRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - AD0ISRHS"]
    #[inline(always)]
    pub fn ad0isrhs(&self) -> AD0ISRHS_R {
        AD0ISRHS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AD0ISRHG"]
    #[inline(always)]
    pub fn ad0isrhg(&self) -> AD0ISRHG_R {
        AD0ISRHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AD0ISRHC"]
    #[inline(always)]
    pub fn ad0isrhc(&self) -> AD0ISRHC_R {
        AD0ISRHC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - AD0ISRL"]
    #[inline(always)]
    pub fn ad0isrl(&self) -> AD0ISRL_R {
        AD0ISRL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AD0ISRU"]
    #[inline(always)]
    pub fn ad0isru(&self) -> AD0ISRU_R {
        AD0ISRU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - AD0ISRO"]
    #[inline(always)]
    pub fn ad0isro(&self) -> AD0ISRO_R {
        AD0ISRO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AD0ISRHO"]
    #[inline(always)]
    pub fn ad0isrho(&self) -> AD0ISRHO_R {
        AD0ISRHO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD0ISRS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0isrs(&mut self) -> AD0ISRS_W<0> {
        AD0ISRS_W::new(self)
    }
    #[doc = "Bit 1 - AD0ISRG"]
    #[inline(always)]
    #[must_use]
    pub fn ad0isrg(&mut self) -> AD0ISRG_W<1> {
        AD0ISRG_W::new(self)
    }
    #[doc = "Bit 2 - AD0ISRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad0isrc(&mut self) -> AD0ISRC_W<2> {
        AD0ISRC_W::new(self)
    }
    #[doc = "Bit 8 - AD0ISRHS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0isrhs(&mut self) -> AD0ISRHS_W<8> {
        AD0ISRHS_W::new(self)
    }
    #[doc = "Bit 9 - AD0ISRHG"]
    #[inline(always)]
    #[must_use]
    pub fn ad0isrhg(&mut self) -> AD0ISRHG_W<9> {
        AD0ISRHG_W::new(self)
    }
    #[doc = "Bit 10 - AD0ISRHC"]
    #[inline(always)]
    #[must_use]
    pub fn ad0isrhc(&mut self) -> AD0ISRHC_W<10> {
        AD0ISRHC_W::new(self)
    }
    #[doc = "Bit 16 - AD0ISRL"]
    #[inline(always)]
    #[must_use]
    pub fn ad0isrl(&mut self) -> AD0ISRL_W<16> {
        AD0ISRL_W::new(self)
    }
    #[doc = "Bit 17 - AD0ISRU"]
    #[inline(always)]
    #[must_use]
    pub fn ad0isru(&mut self) -> AD0ISRU_W<17> {
        AD0ISRU_W::new(self)
    }
    #[doc = "Bit 24 - AD0ISRO"]
    #[inline(always)]
    #[must_use]
    pub fn ad0isro(&mut self) -> AD0ISRO_W<24> {
        AD0ISRO_W::new(self)
    }
    #[doc = "Bit 25 - AD0ISRHO"]
    #[inline(always)]
    #[must_use]
    pub fn ad0isrho(&mut self) -> AD0ISRHO_W<25> {
        AD0ISRHO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0isr](index.html) module"]
pub struct ADC0ISR_SPEC;
impl crate::RegisterSpec for ADC0ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0isr::R](R) reader structure"]
impl crate::Readable for ADC0ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0isr::W](W) writer structure"]
impl crate::Writable for ADC0ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0ISR to value 0"]
impl crate::Resettable for ADC0ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
