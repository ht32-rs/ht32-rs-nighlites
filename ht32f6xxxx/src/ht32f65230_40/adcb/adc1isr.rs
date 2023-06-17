#[doc = "Register `ADC1ISR` reader"]
pub struct R(crate::R<ADC1ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1ISR` writer"]
pub struct W(crate::W<ADC1ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1ISR_SPEC>;
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
impl From<crate::W<ADC1ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1ISRS` reader - AD1ISRS"]
pub type AD1ISRS_R = crate::BitReader;
#[doc = "Field `AD1ISRS` writer - AD1ISRS"]
pub type AD1ISRS_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ISR_SPEC, O>;
#[doc = "Field `AD1ISRG` reader - AD1ISRG"]
pub type AD1ISRG_R = crate::BitReader;
#[doc = "Field `AD1ISRG` writer - AD1ISRG"]
pub type AD1ISRG_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ISR_SPEC, O>;
#[doc = "Field `AD1ISRC` reader - AD1ISRC"]
pub type AD1ISRC_R = crate::BitReader;
#[doc = "Field `AD1ISRC` writer - AD1ISRC"]
pub type AD1ISRC_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ISR_SPEC, O>;
#[doc = "Field `AD1ISRHS` reader - AD1ISRHS"]
pub type AD1ISRHS_R = crate::BitReader;
#[doc = "Field `AD1ISRHS` writer - AD1ISRHS"]
pub type AD1ISRHS_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ISR_SPEC, O>;
#[doc = "Field `AD1ISRHG` reader - AD1ISRHG"]
pub type AD1ISRHG_R = crate::BitReader;
#[doc = "Field `AD1ISRHG` writer - AD1ISRHG"]
pub type AD1ISRHG_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ISR_SPEC, O>;
#[doc = "Field `AD1ISRHC` reader - AD1ISRHC"]
pub type AD1ISRHC_R = crate::BitReader;
#[doc = "Field `AD1ISRHC` writer - AD1ISRHC"]
pub type AD1ISRHC_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ISR_SPEC, O>;
#[doc = "Field `AD1ISRL` reader - AD1ISRL"]
pub type AD1ISRL_R = crate::BitReader;
#[doc = "Field `AD1ISRL` writer - AD1ISRL"]
pub type AD1ISRL_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ISR_SPEC, O>;
#[doc = "Field `AD1ISRU` reader - AD1ISRU"]
pub type AD1ISRU_R = crate::BitReader;
#[doc = "Field `AD1ISRU` writer - AD1ISRU"]
pub type AD1ISRU_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ISR_SPEC, O>;
#[doc = "Field `AD1ISRO` reader - AD1ISRO"]
pub type AD1ISRO_R = crate::BitReader;
#[doc = "Field `AD1ISRO` writer - AD1ISRO"]
pub type AD1ISRO_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ISR_SPEC, O>;
#[doc = "Field `AD1ISRHO` reader - AD1ISRHO"]
pub type AD1ISRHO_R = crate::BitReader;
#[doc = "Field `AD1ISRHO` writer - AD1ISRHO"]
pub type AD1ISRHO_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ISR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AD1ISRS"]
    #[inline(always)]
    pub fn ad1isrs(&self) -> AD1ISRS_R {
        AD1ISRS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD1ISRG"]
    #[inline(always)]
    pub fn ad1isrg(&self) -> AD1ISRG_R {
        AD1ISRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD1ISRC"]
    #[inline(always)]
    pub fn ad1isrc(&self) -> AD1ISRC_R {
        AD1ISRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - AD1ISRHS"]
    #[inline(always)]
    pub fn ad1isrhs(&self) -> AD1ISRHS_R {
        AD1ISRHS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AD1ISRHG"]
    #[inline(always)]
    pub fn ad1isrhg(&self) -> AD1ISRHG_R {
        AD1ISRHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AD1ISRHC"]
    #[inline(always)]
    pub fn ad1isrhc(&self) -> AD1ISRHC_R {
        AD1ISRHC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - AD1ISRL"]
    #[inline(always)]
    pub fn ad1isrl(&self) -> AD1ISRL_R {
        AD1ISRL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AD1ISRU"]
    #[inline(always)]
    pub fn ad1isru(&self) -> AD1ISRU_R {
        AD1ISRU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - AD1ISRO"]
    #[inline(always)]
    pub fn ad1isro(&self) -> AD1ISRO_R {
        AD1ISRO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AD1ISRHO"]
    #[inline(always)]
    pub fn ad1isrho(&self) -> AD1ISRHO_R {
        AD1ISRHO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD1ISRS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1isrs(&mut self) -> AD1ISRS_W<0> {
        AD1ISRS_W::new(self)
    }
    #[doc = "Bit 1 - AD1ISRG"]
    #[inline(always)]
    #[must_use]
    pub fn ad1isrg(&mut self) -> AD1ISRG_W<1> {
        AD1ISRG_W::new(self)
    }
    #[doc = "Bit 2 - AD1ISRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad1isrc(&mut self) -> AD1ISRC_W<2> {
        AD1ISRC_W::new(self)
    }
    #[doc = "Bit 8 - AD1ISRHS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1isrhs(&mut self) -> AD1ISRHS_W<8> {
        AD1ISRHS_W::new(self)
    }
    #[doc = "Bit 9 - AD1ISRHG"]
    #[inline(always)]
    #[must_use]
    pub fn ad1isrhg(&mut self) -> AD1ISRHG_W<9> {
        AD1ISRHG_W::new(self)
    }
    #[doc = "Bit 10 - AD1ISRHC"]
    #[inline(always)]
    #[must_use]
    pub fn ad1isrhc(&mut self) -> AD1ISRHC_W<10> {
        AD1ISRHC_W::new(self)
    }
    #[doc = "Bit 16 - AD1ISRL"]
    #[inline(always)]
    #[must_use]
    pub fn ad1isrl(&mut self) -> AD1ISRL_W<16> {
        AD1ISRL_W::new(self)
    }
    #[doc = "Bit 17 - AD1ISRU"]
    #[inline(always)]
    #[must_use]
    pub fn ad1isru(&mut self) -> AD1ISRU_W<17> {
        AD1ISRU_W::new(self)
    }
    #[doc = "Bit 24 - AD1ISRO"]
    #[inline(always)]
    #[must_use]
    pub fn ad1isro(&mut self) -> AD1ISRO_W<24> {
        AD1ISRO_W::new(self)
    }
    #[doc = "Bit 25 - AD1ISRHO"]
    #[inline(always)]
    #[must_use]
    pub fn ad1isrho(&mut self) -> AD1ISRHO_W<25> {
        AD1ISRHO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1isr](index.html) module"]
pub struct ADC1ISR_SPEC;
impl crate::RegisterSpec for ADC1ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1isr::R](R) reader structure"]
impl crate::Readable for ADC1ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1isr::W](W) writer structure"]
impl crate::Writable for ADC1ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1ISR to value 0"]
impl crate::Resettable for ADC1ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
