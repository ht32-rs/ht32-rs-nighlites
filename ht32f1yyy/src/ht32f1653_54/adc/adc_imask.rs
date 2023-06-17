#[doc = "Register `ADC_IMASK` reader"]
pub struct R(crate::R<ADC_IMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_IMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_IMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_IMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_IMASK` writer"]
pub struct W(crate::W<ADC_IMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_IMASK_SPEC>;
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
impl From<crate::W<ADC_IMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_IMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADIMASKS` reader - ADIMASKS"]
pub type ADIMASKS_R = crate::BitReader;
#[doc = "Field `ADIMASKS` writer - ADIMASKS"]
pub type ADIMASKS_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IMASK_SPEC, O>;
#[doc = "Field `ADIMASKG` reader - ADIMASKG"]
pub type ADIMASKG_R = crate::BitReader;
#[doc = "Field `ADIMASKG` writer - ADIMASKG"]
pub type ADIMASKG_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IMASK_SPEC, O>;
#[doc = "Field `ADIMASKC` reader - ADIMASKC"]
pub type ADIMASKC_R = crate::BitReader;
#[doc = "Field `ADIMASKC` writer - ADIMASKC"]
pub type ADIMASKC_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IMASK_SPEC, O>;
#[doc = "Field `ADIMASKHS` reader - ADIMASKHS"]
pub type ADIMASKHS_R = crate::BitReader;
#[doc = "Field `ADIMASKHS` writer - ADIMASKHS"]
pub type ADIMASKHS_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IMASK_SPEC, O>;
#[doc = "Field `ADIMASKHG` reader - ADIMASKHG"]
pub type ADIMASKHG_R = crate::BitReader;
#[doc = "Field `ADIMASKHG` writer - ADIMASKHG"]
pub type ADIMASKHG_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IMASK_SPEC, O>;
#[doc = "Field `ADIMASKHC` reader - ADIMASKHC"]
pub type ADIMASKHC_R = crate::BitReader;
#[doc = "Field `ADIMASKHC` writer - ADIMASKHC"]
pub type ADIMASKHC_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IMASK_SPEC, O>;
#[doc = "Field `ADIMASKL` reader - ADIMASKL"]
pub type ADIMASKL_R = crate::BitReader;
#[doc = "Field `ADIMASKL` writer - ADIMASKL"]
pub type ADIMASKL_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IMASK_SPEC, O>;
#[doc = "Field `ADIMASKU` reader - ADIMASKU"]
pub type ADIMASKU_R = crate::BitReader;
#[doc = "Field `ADIMASKU` writer - ADIMASKU"]
pub type ADIMASKU_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IMASK_SPEC, O>;
#[doc = "Field `ADIMASKO` reader - ADIMASKO"]
pub type ADIMASKO_R = crate::BitReader;
#[doc = "Field `ADIMASKO` writer - ADIMASKO"]
pub type ADIMASKO_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IMASK_SPEC, O>;
#[doc = "Field `ADIMASKHO` reader - ADIMASKHO"]
pub type ADIMASKHO_R = crate::BitReader;
#[doc = "Field `ADIMASKHO` writer - ADIMASKHO"]
pub type ADIMASKHO_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IMASK_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ADIMASKS"]
    #[inline(always)]
    pub fn adimasks(&self) -> ADIMASKS_R {
        ADIMASKS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADIMASKG"]
    #[inline(always)]
    pub fn adimaskg(&self) -> ADIMASKG_R {
        ADIMASKG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADIMASKC"]
    #[inline(always)]
    pub fn adimaskc(&self) -> ADIMASKC_R {
        ADIMASKC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - ADIMASKHS"]
    #[inline(always)]
    pub fn adimaskhs(&self) -> ADIMASKHS_R {
        ADIMASKHS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADIMASKHG"]
    #[inline(always)]
    pub fn adimaskhg(&self) -> ADIMASKHG_R {
        ADIMASKHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADIMASKHC"]
    #[inline(always)]
    pub fn adimaskhc(&self) -> ADIMASKHC_R {
        ADIMASKHC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - ADIMASKL"]
    #[inline(always)]
    pub fn adimaskl(&self) -> ADIMASKL_R {
        ADIMASKL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADIMASKU"]
    #[inline(always)]
    pub fn adimasku(&self) -> ADIMASKU_R {
        ADIMASKU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - ADIMASKO"]
    #[inline(always)]
    pub fn adimasko(&self) -> ADIMASKO_R {
        ADIMASKO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADIMASKHO"]
    #[inline(always)]
    pub fn adimaskho(&self) -> ADIMASKHO_R {
        ADIMASKHO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADIMASKS"]
    #[inline(always)]
    #[must_use]
    pub fn adimasks(&mut self) -> ADIMASKS_W<0> {
        ADIMASKS_W::new(self)
    }
    #[doc = "Bit 1 - ADIMASKG"]
    #[inline(always)]
    #[must_use]
    pub fn adimaskg(&mut self) -> ADIMASKG_W<1> {
        ADIMASKG_W::new(self)
    }
    #[doc = "Bit 2 - ADIMASKC"]
    #[inline(always)]
    #[must_use]
    pub fn adimaskc(&mut self) -> ADIMASKC_W<2> {
        ADIMASKC_W::new(self)
    }
    #[doc = "Bit 8 - ADIMASKHS"]
    #[inline(always)]
    #[must_use]
    pub fn adimaskhs(&mut self) -> ADIMASKHS_W<8> {
        ADIMASKHS_W::new(self)
    }
    #[doc = "Bit 9 - ADIMASKHG"]
    #[inline(always)]
    #[must_use]
    pub fn adimaskhg(&mut self) -> ADIMASKHG_W<9> {
        ADIMASKHG_W::new(self)
    }
    #[doc = "Bit 10 - ADIMASKHC"]
    #[inline(always)]
    #[must_use]
    pub fn adimaskhc(&mut self) -> ADIMASKHC_W<10> {
        ADIMASKHC_W::new(self)
    }
    #[doc = "Bit 16 - ADIMASKL"]
    #[inline(always)]
    #[must_use]
    pub fn adimaskl(&mut self) -> ADIMASKL_W<16> {
        ADIMASKL_W::new(self)
    }
    #[doc = "Bit 17 - ADIMASKU"]
    #[inline(always)]
    #[must_use]
    pub fn adimasku(&mut self) -> ADIMASKU_W<17> {
        ADIMASKU_W::new(self)
    }
    #[doc = "Bit 24 - ADIMASKO"]
    #[inline(always)]
    #[must_use]
    pub fn adimasko(&mut self) -> ADIMASKO_W<24> {
        ADIMASKO_W::new(self)
    }
    #[doc = "Bit 25 - ADIMASKHO"]
    #[inline(always)]
    #[must_use]
    pub fn adimaskho(&mut self) -> ADIMASKHO_W<25> {
        ADIMASKHO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_IMASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_imask](index.html) module"]
pub struct ADC_IMASK_SPEC;
impl crate::RegisterSpec for ADC_IMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_imask::R](R) reader structure"]
impl crate::Readable for ADC_IMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_imask::W](W) writer structure"]
impl crate::Writable for ADC_IMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_IMASK to value 0"]
impl crate::Resettable for ADC_IMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
