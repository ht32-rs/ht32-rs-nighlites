#[doc = "Register `ADC_ICLR` reader"]
pub struct R(crate::R<ADC_ICLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_ICLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_ICLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_ICLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_ICLR` writer"]
pub struct W(crate::W<ADC_ICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_ICLR_SPEC>;
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
impl From<crate::W<ADC_ICLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_ICLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADICLRS` reader - ADICLRS"]
pub type ADICLRS_R = crate::BitReader;
#[doc = "Field `ADICLRS` writer - ADICLRS"]
pub type ADICLRS_W<'a, const O: u8> = crate::BitWriter<'a, ADC_ICLR_SPEC, O>;
#[doc = "Field `ADICLRG` reader - ADICLRG"]
pub type ADICLRG_R = crate::BitReader;
#[doc = "Field `ADICLRG` writer - ADICLRG"]
pub type ADICLRG_W<'a, const O: u8> = crate::BitWriter<'a, ADC_ICLR_SPEC, O>;
#[doc = "Field `ADICLRC` reader - ADICLRC"]
pub type ADICLRC_R = crate::BitReader;
#[doc = "Field `ADICLRC` writer - ADICLRC"]
pub type ADICLRC_W<'a, const O: u8> = crate::BitWriter<'a, ADC_ICLR_SPEC, O>;
#[doc = "Field `ADICLRHS` reader - ADICLRHS"]
pub type ADICLRHS_R = crate::BitReader;
#[doc = "Field `ADICLRHS` writer - ADICLRHS"]
pub type ADICLRHS_W<'a, const O: u8> = crate::BitWriter<'a, ADC_ICLR_SPEC, O>;
#[doc = "Field `ADICLRHG` reader - ADICLRHG"]
pub type ADICLRHG_R = crate::BitReader;
#[doc = "Field `ADICLRHG` writer - ADICLRHG"]
pub type ADICLRHG_W<'a, const O: u8> = crate::BitWriter<'a, ADC_ICLR_SPEC, O>;
#[doc = "Field `ADICLRHC` reader - ADICLRHC"]
pub type ADICLRHC_R = crate::BitReader;
#[doc = "Field `ADICLRHC` writer - ADICLRHC"]
pub type ADICLRHC_W<'a, const O: u8> = crate::BitWriter<'a, ADC_ICLR_SPEC, O>;
#[doc = "Field `ADICLRL` reader - ADICLRL"]
pub type ADICLRL_R = crate::BitReader;
#[doc = "Field `ADICLRL` writer - ADICLRL"]
pub type ADICLRL_W<'a, const O: u8> = crate::BitWriter<'a, ADC_ICLR_SPEC, O>;
#[doc = "Field `ADICLRU` reader - ADICLRU"]
pub type ADICLRU_R = crate::BitReader;
#[doc = "Field `ADICLRU` writer - ADICLRU"]
pub type ADICLRU_W<'a, const O: u8> = crate::BitWriter<'a, ADC_ICLR_SPEC, O>;
#[doc = "Field `ADICLRO` reader - ADICLRO"]
pub type ADICLRO_R = crate::BitReader;
#[doc = "Field `ADICLRO` writer - ADICLRO"]
pub type ADICLRO_W<'a, const O: u8> = crate::BitWriter<'a, ADC_ICLR_SPEC, O>;
#[doc = "Field `ADICLRHO` reader - ADICLRHO"]
pub type ADICLRHO_R = crate::BitReader;
#[doc = "Field `ADICLRHO` writer - ADICLRHO"]
pub type ADICLRHO_W<'a, const O: u8> = crate::BitWriter<'a, ADC_ICLR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ADICLRS"]
    #[inline(always)]
    pub fn adiclrs(&self) -> ADICLRS_R {
        ADICLRS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADICLRG"]
    #[inline(always)]
    pub fn adiclrg(&self) -> ADICLRG_R {
        ADICLRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADICLRC"]
    #[inline(always)]
    pub fn adiclrc(&self) -> ADICLRC_R {
        ADICLRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - ADICLRHS"]
    #[inline(always)]
    pub fn adiclrhs(&self) -> ADICLRHS_R {
        ADICLRHS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADICLRHG"]
    #[inline(always)]
    pub fn adiclrhg(&self) -> ADICLRHG_R {
        ADICLRHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADICLRHC"]
    #[inline(always)]
    pub fn adiclrhc(&self) -> ADICLRHC_R {
        ADICLRHC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - ADICLRL"]
    #[inline(always)]
    pub fn adiclrl(&self) -> ADICLRL_R {
        ADICLRL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADICLRU"]
    #[inline(always)]
    pub fn adiclru(&self) -> ADICLRU_R {
        ADICLRU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - ADICLRO"]
    #[inline(always)]
    pub fn adiclro(&self) -> ADICLRO_R {
        ADICLRO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADICLRHO"]
    #[inline(always)]
    pub fn adiclrho(&self) -> ADICLRHO_R {
        ADICLRHO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADICLRS"]
    #[inline(always)]
    #[must_use]
    pub fn adiclrs(&mut self) -> ADICLRS_W<0> {
        ADICLRS_W::new(self)
    }
    #[doc = "Bit 1 - ADICLRG"]
    #[inline(always)]
    #[must_use]
    pub fn adiclrg(&mut self) -> ADICLRG_W<1> {
        ADICLRG_W::new(self)
    }
    #[doc = "Bit 2 - ADICLRC"]
    #[inline(always)]
    #[must_use]
    pub fn adiclrc(&mut self) -> ADICLRC_W<2> {
        ADICLRC_W::new(self)
    }
    #[doc = "Bit 8 - ADICLRHS"]
    #[inline(always)]
    #[must_use]
    pub fn adiclrhs(&mut self) -> ADICLRHS_W<8> {
        ADICLRHS_W::new(self)
    }
    #[doc = "Bit 9 - ADICLRHG"]
    #[inline(always)]
    #[must_use]
    pub fn adiclrhg(&mut self) -> ADICLRHG_W<9> {
        ADICLRHG_W::new(self)
    }
    #[doc = "Bit 10 - ADICLRHC"]
    #[inline(always)]
    #[must_use]
    pub fn adiclrhc(&mut self) -> ADICLRHC_W<10> {
        ADICLRHC_W::new(self)
    }
    #[doc = "Bit 16 - ADICLRL"]
    #[inline(always)]
    #[must_use]
    pub fn adiclrl(&mut self) -> ADICLRL_W<16> {
        ADICLRL_W::new(self)
    }
    #[doc = "Bit 17 - ADICLRU"]
    #[inline(always)]
    #[must_use]
    pub fn adiclru(&mut self) -> ADICLRU_W<17> {
        ADICLRU_W::new(self)
    }
    #[doc = "Bit 24 - ADICLRO"]
    #[inline(always)]
    #[must_use]
    pub fn adiclro(&mut self) -> ADICLRO_W<24> {
        ADICLRO_W::new(self)
    }
    #[doc = "Bit 25 - ADICLRHO"]
    #[inline(always)]
    #[must_use]
    pub fn adiclrho(&mut self) -> ADICLRHO_W<25> {
        ADICLRHO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_ICLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_iclr](index.html) module"]
pub struct ADC_ICLR_SPEC;
impl crate::RegisterSpec for ADC_ICLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_iclr::R](R) reader structure"]
impl crate::Readable for ADC_ICLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_iclr::W](W) writer structure"]
impl crate::Writable for ADC_ICLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_ICLR to value 0"]
impl crate::Resettable for ADC_ICLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
