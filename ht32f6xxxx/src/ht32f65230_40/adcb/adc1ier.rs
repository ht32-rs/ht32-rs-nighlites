#[doc = "Register `ADC1IER` reader"]
pub struct R(crate::R<ADC1IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1IER` writer"]
pub struct W(crate::W<ADC1IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1IER_SPEC>;
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
impl From<crate::W<ADC1IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1IES` reader - AD1IES"]
pub type AD1IES_R = crate::BitReader;
#[doc = "Field `AD1IES` writer - AD1IES"]
pub type AD1IES_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IER_SPEC, O>;
#[doc = "Field `AD1IEG` reader - AD1IEG"]
pub type AD1IEG_R = crate::BitReader;
#[doc = "Field `AD1IEG` writer - AD1IEG"]
pub type AD1IEG_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IER_SPEC, O>;
#[doc = "Field `AD1IEC` reader - AD1IEC"]
pub type AD1IEC_R = crate::BitReader;
#[doc = "Field `AD1IEC` writer - AD1IEC"]
pub type AD1IEC_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IER_SPEC, O>;
#[doc = "Field `AD1IEHS` reader - AD1IEHS"]
pub type AD1IEHS_R = crate::BitReader;
#[doc = "Field `AD1IEHS` writer - AD1IEHS"]
pub type AD1IEHS_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IER_SPEC, O>;
#[doc = "Field `AD1IEHG` reader - AD1IEHG"]
pub type AD1IEHG_R = crate::BitReader;
#[doc = "Field `AD1IEHG` writer - AD1IEHG"]
pub type AD1IEHG_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IER_SPEC, O>;
#[doc = "Field `AD1IEHC` reader - AD1IEHC"]
pub type AD1IEHC_R = crate::BitReader;
#[doc = "Field `AD1IEHC` writer - AD1IEHC"]
pub type AD1IEHC_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IER_SPEC, O>;
#[doc = "Field `AD1IEL` reader - AD1IEL"]
pub type AD1IEL_R = crate::BitReader;
#[doc = "Field `AD1IEL` writer - AD1IEL"]
pub type AD1IEL_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IER_SPEC, O>;
#[doc = "Field `AD1IEU` reader - AD1IEU"]
pub type AD1IEU_R = crate::BitReader;
#[doc = "Field `AD1IEU` writer - AD1IEU"]
pub type AD1IEU_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IER_SPEC, O>;
#[doc = "Field `AD1IEO` reader - AD1IEO"]
pub type AD1IEO_R = crate::BitReader;
#[doc = "Field `AD1IEO` writer - AD1IEO"]
pub type AD1IEO_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IER_SPEC, O>;
#[doc = "Field `AD1IEHO` reader - AD1IEHO"]
pub type AD1IEHO_R = crate::BitReader;
#[doc = "Field `AD1IEHO` writer - AD1IEHO"]
pub type AD1IEHO_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AD1IES"]
    #[inline(always)]
    pub fn ad1ies(&self) -> AD1IES_R {
        AD1IES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD1IEG"]
    #[inline(always)]
    pub fn ad1ieg(&self) -> AD1IEG_R {
        AD1IEG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD1IEC"]
    #[inline(always)]
    pub fn ad1iec(&self) -> AD1IEC_R {
        AD1IEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - AD1IEHS"]
    #[inline(always)]
    pub fn ad1iehs(&self) -> AD1IEHS_R {
        AD1IEHS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AD1IEHG"]
    #[inline(always)]
    pub fn ad1iehg(&self) -> AD1IEHG_R {
        AD1IEHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AD1IEHC"]
    #[inline(always)]
    pub fn ad1iehc(&self) -> AD1IEHC_R {
        AD1IEHC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - AD1IEL"]
    #[inline(always)]
    pub fn ad1iel(&self) -> AD1IEL_R {
        AD1IEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AD1IEU"]
    #[inline(always)]
    pub fn ad1ieu(&self) -> AD1IEU_R {
        AD1IEU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - AD1IEO"]
    #[inline(always)]
    pub fn ad1ieo(&self) -> AD1IEO_R {
        AD1IEO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AD1IEHO"]
    #[inline(always)]
    pub fn ad1ieho(&self) -> AD1IEHO_R {
        AD1IEHO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD1IES"]
    #[inline(always)]
    #[must_use]
    pub fn ad1ies(&mut self) -> AD1IES_W<0> {
        AD1IES_W::new(self)
    }
    #[doc = "Bit 1 - AD1IEG"]
    #[inline(always)]
    #[must_use]
    pub fn ad1ieg(&mut self) -> AD1IEG_W<1> {
        AD1IEG_W::new(self)
    }
    #[doc = "Bit 2 - AD1IEC"]
    #[inline(always)]
    #[must_use]
    pub fn ad1iec(&mut self) -> AD1IEC_W<2> {
        AD1IEC_W::new(self)
    }
    #[doc = "Bit 8 - AD1IEHS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1iehs(&mut self) -> AD1IEHS_W<8> {
        AD1IEHS_W::new(self)
    }
    #[doc = "Bit 9 - AD1IEHG"]
    #[inline(always)]
    #[must_use]
    pub fn ad1iehg(&mut self) -> AD1IEHG_W<9> {
        AD1IEHG_W::new(self)
    }
    #[doc = "Bit 10 - AD1IEHC"]
    #[inline(always)]
    #[must_use]
    pub fn ad1iehc(&mut self) -> AD1IEHC_W<10> {
        AD1IEHC_W::new(self)
    }
    #[doc = "Bit 16 - AD1IEL"]
    #[inline(always)]
    #[must_use]
    pub fn ad1iel(&mut self) -> AD1IEL_W<16> {
        AD1IEL_W::new(self)
    }
    #[doc = "Bit 17 - AD1IEU"]
    #[inline(always)]
    #[must_use]
    pub fn ad1ieu(&mut self) -> AD1IEU_W<17> {
        AD1IEU_W::new(self)
    }
    #[doc = "Bit 24 - AD1IEO"]
    #[inline(always)]
    #[must_use]
    pub fn ad1ieo(&mut self) -> AD1IEO_W<24> {
        AD1IEO_W::new(self)
    }
    #[doc = "Bit 25 - AD1IEHO"]
    #[inline(always)]
    #[must_use]
    pub fn ad1ieho(&mut self) -> AD1IEHO_W<25> {
        AD1IEHO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1ier](index.html) module"]
pub struct ADC1IER_SPEC;
impl crate::RegisterSpec for ADC1IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1ier::R](R) reader structure"]
impl crate::Readable for ADC1IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1ier::W](W) writer structure"]
impl crate::Writable for ADC1IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1IER to value 0"]
impl crate::Resettable for ADC1IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
