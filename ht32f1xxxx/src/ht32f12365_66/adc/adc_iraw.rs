#[doc = "Register `ADC_IRAW` reader"]
pub struct R(crate::R<ADC_IRAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_IRAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_IRAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_IRAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_IRAW` writer"]
pub struct W(crate::W<ADC_IRAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_IRAW_SPEC>;
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
impl From<crate::W<ADC_IRAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_IRAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADIRAWS` reader - ADIRAWS"]
pub type ADIRAWS_R = crate::BitReader;
#[doc = "Field `ADIRAWS` writer - ADIRAWS"]
pub type ADIRAWS_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IRAW_SPEC, O>;
#[doc = "Field `ADIRAWG` reader - ADIRAWG"]
pub type ADIRAWG_R = crate::BitReader;
#[doc = "Field `ADIRAWG` writer - ADIRAWG"]
pub type ADIRAWG_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IRAW_SPEC, O>;
#[doc = "Field `ADIRAWC` reader - ADIRAWC"]
pub type ADIRAWC_R = crate::BitReader;
#[doc = "Field `ADIRAWC` writer - ADIRAWC"]
pub type ADIRAWC_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IRAW_SPEC, O>;
#[doc = "Field `ADIRAWHS` reader - ADIRAWHS"]
pub type ADIRAWHS_R = crate::BitReader;
#[doc = "Field `ADIRAWHS` writer - ADIRAWHS"]
pub type ADIRAWHS_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IRAW_SPEC, O>;
#[doc = "Field `ADIRAWHG` reader - ADIRAWHG"]
pub type ADIRAWHG_R = crate::BitReader;
#[doc = "Field `ADIRAWHG` writer - ADIRAWHG"]
pub type ADIRAWHG_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IRAW_SPEC, O>;
#[doc = "Field `ADIRAWHC` reader - ADIRAWHC"]
pub type ADIRAWHC_R = crate::BitReader;
#[doc = "Field `ADIRAWHC` writer - ADIRAWHC"]
pub type ADIRAWHC_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IRAW_SPEC, O>;
#[doc = "Field `ADIRAWL` reader - ADIRAWL"]
pub type ADIRAWL_R = crate::BitReader;
#[doc = "Field `ADIRAWL` writer - ADIRAWL"]
pub type ADIRAWL_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IRAW_SPEC, O>;
#[doc = "Field `ADIRAWU` reader - ADIRAWU"]
pub type ADIRAWU_R = crate::BitReader;
#[doc = "Field `ADIRAWU` writer - ADIRAWU"]
pub type ADIRAWU_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IRAW_SPEC, O>;
#[doc = "Field `ADIRAWO` reader - ADIRAWO"]
pub type ADIRAWO_R = crate::BitReader;
#[doc = "Field `ADIRAWO` writer - ADIRAWO"]
pub type ADIRAWO_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IRAW_SPEC, O>;
#[doc = "Field `ADIRAWHO` reader - ADIRAWHO"]
pub type ADIRAWHO_R = crate::BitReader;
#[doc = "Field `ADIRAWHO` writer - ADIRAWHO"]
pub type ADIRAWHO_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IRAW_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ADIRAWS"]
    #[inline(always)]
    pub fn adiraws(&self) -> ADIRAWS_R {
        ADIRAWS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADIRAWG"]
    #[inline(always)]
    pub fn adirawg(&self) -> ADIRAWG_R {
        ADIRAWG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADIRAWC"]
    #[inline(always)]
    pub fn adirawc(&self) -> ADIRAWC_R {
        ADIRAWC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - ADIRAWHS"]
    #[inline(always)]
    pub fn adirawhs(&self) -> ADIRAWHS_R {
        ADIRAWHS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADIRAWHG"]
    #[inline(always)]
    pub fn adirawhg(&self) -> ADIRAWHG_R {
        ADIRAWHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADIRAWHC"]
    #[inline(always)]
    pub fn adirawhc(&self) -> ADIRAWHC_R {
        ADIRAWHC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - ADIRAWL"]
    #[inline(always)]
    pub fn adirawl(&self) -> ADIRAWL_R {
        ADIRAWL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADIRAWU"]
    #[inline(always)]
    pub fn adirawu(&self) -> ADIRAWU_R {
        ADIRAWU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - ADIRAWO"]
    #[inline(always)]
    pub fn adirawo(&self) -> ADIRAWO_R {
        ADIRAWO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADIRAWHO"]
    #[inline(always)]
    pub fn adirawho(&self) -> ADIRAWHO_R {
        ADIRAWHO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADIRAWS"]
    #[inline(always)]
    #[must_use]
    pub fn adiraws(&mut self) -> ADIRAWS_W<0> {
        ADIRAWS_W::new(self)
    }
    #[doc = "Bit 1 - ADIRAWG"]
    #[inline(always)]
    #[must_use]
    pub fn adirawg(&mut self) -> ADIRAWG_W<1> {
        ADIRAWG_W::new(self)
    }
    #[doc = "Bit 2 - ADIRAWC"]
    #[inline(always)]
    #[must_use]
    pub fn adirawc(&mut self) -> ADIRAWC_W<2> {
        ADIRAWC_W::new(self)
    }
    #[doc = "Bit 8 - ADIRAWHS"]
    #[inline(always)]
    #[must_use]
    pub fn adirawhs(&mut self) -> ADIRAWHS_W<8> {
        ADIRAWHS_W::new(self)
    }
    #[doc = "Bit 9 - ADIRAWHG"]
    #[inline(always)]
    #[must_use]
    pub fn adirawhg(&mut self) -> ADIRAWHG_W<9> {
        ADIRAWHG_W::new(self)
    }
    #[doc = "Bit 10 - ADIRAWHC"]
    #[inline(always)]
    #[must_use]
    pub fn adirawhc(&mut self) -> ADIRAWHC_W<10> {
        ADIRAWHC_W::new(self)
    }
    #[doc = "Bit 16 - ADIRAWL"]
    #[inline(always)]
    #[must_use]
    pub fn adirawl(&mut self) -> ADIRAWL_W<16> {
        ADIRAWL_W::new(self)
    }
    #[doc = "Bit 17 - ADIRAWU"]
    #[inline(always)]
    #[must_use]
    pub fn adirawu(&mut self) -> ADIRAWU_W<17> {
        ADIRAWU_W::new(self)
    }
    #[doc = "Bit 24 - ADIRAWO"]
    #[inline(always)]
    #[must_use]
    pub fn adirawo(&mut self) -> ADIRAWO_W<24> {
        ADIRAWO_W::new(self)
    }
    #[doc = "Bit 25 - ADIRAWHO"]
    #[inline(always)]
    #[must_use]
    pub fn adirawho(&mut self) -> ADIRAWHO_W<25> {
        ADIRAWHO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_IRAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_iraw](index.html) module"]
pub struct ADC_IRAW_SPEC;
impl crate::RegisterSpec for ADC_IRAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_iraw::R](R) reader structure"]
impl crate::Readable for ADC_IRAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_iraw::W](W) writer structure"]
impl crate::Writable for ADC_IRAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_IRAW to value 0"]
impl crate::Resettable for ADC_IRAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
