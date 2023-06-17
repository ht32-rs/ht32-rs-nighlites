#[doc = "Register `ADC0IER` reader"]
pub struct R(crate::R<ADC0IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0IER` writer"]
pub struct W(crate::W<ADC0IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0IER_SPEC>;
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
impl From<crate::W<ADC0IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0IES` reader - AD0IES"]
pub type AD0IES_R = crate::BitReader;
#[doc = "Field `AD0IES` writer - AD0IES"]
pub type AD0IES_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IER_SPEC, O>;
#[doc = "Field `AD0IEG` reader - AD0IEG"]
pub type AD0IEG_R = crate::BitReader;
#[doc = "Field `AD0IEG` writer - AD0IEG"]
pub type AD0IEG_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IER_SPEC, O>;
#[doc = "Field `AD0IEC` reader - AD0IEC"]
pub type AD0IEC_R = crate::BitReader;
#[doc = "Field `AD0IEC` writer - AD0IEC"]
pub type AD0IEC_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IER_SPEC, O>;
#[doc = "Field `AD0IEHS` reader - AD0IEHS"]
pub type AD0IEHS_R = crate::BitReader;
#[doc = "Field `AD0IEHS` writer - AD0IEHS"]
pub type AD0IEHS_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IER_SPEC, O>;
#[doc = "Field `AD0IEHG` reader - AD0IEHG"]
pub type AD0IEHG_R = crate::BitReader;
#[doc = "Field `AD0IEHG` writer - AD0IEHG"]
pub type AD0IEHG_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IER_SPEC, O>;
#[doc = "Field `AD0IEHC` reader - AD0IEHC"]
pub type AD0IEHC_R = crate::BitReader;
#[doc = "Field `AD0IEHC` writer - AD0IEHC"]
pub type AD0IEHC_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IER_SPEC, O>;
#[doc = "Field `AD0IEL` reader - AD0IEL"]
pub type AD0IEL_R = crate::BitReader;
#[doc = "Field `AD0IEL` writer - AD0IEL"]
pub type AD0IEL_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IER_SPEC, O>;
#[doc = "Field `AD0IEU` reader - AD0IEU"]
pub type AD0IEU_R = crate::BitReader;
#[doc = "Field `AD0IEU` writer - AD0IEU"]
pub type AD0IEU_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IER_SPEC, O>;
#[doc = "Field `AD0IEO` reader - AD0IEO"]
pub type AD0IEO_R = crate::BitReader;
#[doc = "Field `AD0IEO` writer - AD0IEO"]
pub type AD0IEO_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IER_SPEC, O>;
#[doc = "Field `AD0IEHO` reader - AD0IEHO"]
pub type AD0IEHO_R = crate::BitReader;
#[doc = "Field `AD0IEHO` writer - AD0IEHO"]
pub type AD0IEHO_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AD0IES"]
    #[inline(always)]
    pub fn ad0ies(&self) -> AD0IES_R {
        AD0IES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD0IEG"]
    #[inline(always)]
    pub fn ad0ieg(&self) -> AD0IEG_R {
        AD0IEG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD0IEC"]
    #[inline(always)]
    pub fn ad0iec(&self) -> AD0IEC_R {
        AD0IEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - AD0IEHS"]
    #[inline(always)]
    pub fn ad0iehs(&self) -> AD0IEHS_R {
        AD0IEHS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AD0IEHG"]
    #[inline(always)]
    pub fn ad0iehg(&self) -> AD0IEHG_R {
        AD0IEHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AD0IEHC"]
    #[inline(always)]
    pub fn ad0iehc(&self) -> AD0IEHC_R {
        AD0IEHC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - AD0IEL"]
    #[inline(always)]
    pub fn ad0iel(&self) -> AD0IEL_R {
        AD0IEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AD0IEU"]
    #[inline(always)]
    pub fn ad0ieu(&self) -> AD0IEU_R {
        AD0IEU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - AD0IEO"]
    #[inline(always)]
    pub fn ad0ieo(&self) -> AD0IEO_R {
        AD0IEO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AD0IEHO"]
    #[inline(always)]
    pub fn ad0ieho(&self) -> AD0IEHO_R {
        AD0IEHO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD0IES"]
    #[inline(always)]
    #[must_use]
    pub fn ad0ies(&mut self) -> AD0IES_W<0> {
        AD0IES_W::new(self)
    }
    #[doc = "Bit 1 - AD0IEG"]
    #[inline(always)]
    #[must_use]
    pub fn ad0ieg(&mut self) -> AD0IEG_W<1> {
        AD0IEG_W::new(self)
    }
    #[doc = "Bit 2 - AD0IEC"]
    #[inline(always)]
    #[must_use]
    pub fn ad0iec(&mut self) -> AD0IEC_W<2> {
        AD0IEC_W::new(self)
    }
    #[doc = "Bit 8 - AD0IEHS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0iehs(&mut self) -> AD0IEHS_W<8> {
        AD0IEHS_W::new(self)
    }
    #[doc = "Bit 9 - AD0IEHG"]
    #[inline(always)]
    #[must_use]
    pub fn ad0iehg(&mut self) -> AD0IEHG_W<9> {
        AD0IEHG_W::new(self)
    }
    #[doc = "Bit 10 - AD0IEHC"]
    #[inline(always)]
    #[must_use]
    pub fn ad0iehc(&mut self) -> AD0IEHC_W<10> {
        AD0IEHC_W::new(self)
    }
    #[doc = "Bit 16 - AD0IEL"]
    #[inline(always)]
    #[must_use]
    pub fn ad0iel(&mut self) -> AD0IEL_W<16> {
        AD0IEL_W::new(self)
    }
    #[doc = "Bit 17 - AD0IEU"]
    #[inline(always)]
    #[must_use]
    pub fn ad0ieu(&mut self) -> AD0IEU_W<17> {
        AD0IEU_W::new(self)
    }
    #[doc = "Bit 24 - AD0IEO"]
    #[inline(always)]
    #[must_use]
    pub fn ad0ieo(&mut self) -> AD0IEO_W<24> {
        AD0IEO_W::new(self)
    }
    #[doc = "Bit 25 - AD0IEHO"]
    #[inline(always)]
    #[must_use]
    pub fn ad0ieho(&mut self) -> AD0IEHO_W<25> {
        AD0IEHO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0ier](index.html) module"]
pub struct ADC0IER_SPEC;
impl crate::RegisterSpec for ADC0IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0ier::R](R) reader structure"]
impl crate::Readable for ADC0IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0ier::W](W) writer structure"]
impl crate::Writable for ADC0IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0IER to value 0"]
impl crate::Resettable for ADC0IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
