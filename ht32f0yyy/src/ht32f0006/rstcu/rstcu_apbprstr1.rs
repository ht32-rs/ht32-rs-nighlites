#[doc = "Register `RSTCU_APBPRSTR1` reader"]
pub struct R(crate::R<RSTCU_APBPRSTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCU_APBPRSTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCU_APBPRSTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCU_APBPRSTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCU_APBPRSTR1` writer"]
pub struct W(crate::W<RSTCU_APBPRSTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCU_APBPRSTR1_SPEC>;
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
impl From<crate::W<RSTCU_APBPRSTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCU_APBPRSTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTRST` reader - WDTRST"]
pub type WDTRST_R = crate::BitReader;
#[doc = "Field `WDTRST` writer - WDTRST"]
pub type WDTRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR1_SPEC, O>;
#[doc = "Field `GPTMRST` reader - GPTMRST"]
pub type GPTMRST_R = crate::BitReader;
#[doc = "Field `GPTMRST` writer - GPTMRST"]
pub type GPTMRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR1_SPEC, O>;
#[doc = "Field `BFTM0RST` reader - BFTM0RST"]
pub type BFTM0RST_R = crate::BitReader;
#[doc = "Field `BFTM0RST` writer - BFTM0RST"]
pub type BFTM0RST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR1_SPEC, O>;
#[doc = "Field `BFTM1RST` reader - BFTM1RST"]
pub type BFTM1RST_R = crate::BitReader;
#[doc = "Field `BFTM1RST` writer - BFTM1RST"]
pub type BFTM1RST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR1_SPEC, O>;
#[doc = "Field `ADCRST` reader - ADCRST"]
pub type ADCRST_R = crate::BitReader;
#[doc = "Field `ADCRST` writer - ADCRST"]
pub type ADCRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR1_SPEC, O>;
#[doc = "Field `SCTM0RST` reader - SCTM0RST"]
pub type SCTM0RST_R = crate::BitReader;
#[doc = "Field `SCTM0RST` writer - SCTM0RST"]
pub type SCTM0RST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR1_SPEC, O>;
#[doc = "Field `SCTM1RST` reader - SCTM1RST"]
pub type SCTM1RST_R = crate::BitReader;
#[doc = "Field `SCTM1RST` writer - SCTM1RST"]
pub type SCTM1RST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR1_SPEC, O>;
#[doc = "Field `SCTM2RST` reader - SCTM2RST"]
pub type SCTM2RST_R = crate::BitReader;
#[doc = "Field `SCTM2RST` writer - SCTM2RST"]
pub type SCTM2RST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR1_SPEC, O>;
#[doc = "Field `SCTM3RST` reader - SCTM3RST"]
pub type SCTM3RST_R = crate::BitReader;
#[doc = "Field `SCTM3RST` writer - SCTM3RST"]
pub type SCTM3RST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR1_SPEC, O>;
impl R {
    #[doc = "Bit 4 - WDTRST"]
    #[inline(always)]
    pub fn wdtrst(&self) -> WDTRST_R {
        WDTRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTMRST"]
    #[inline(always)]
    pub fn gptmrst(&self) -> GPTMRST_R {
        GPTMRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - BFTM0RST"]
    #[inline(always)]
    pub fn bftm0rst(&self) -> BFTM0RST_R {
        BFTM0RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BFTM1RST"]
    #[inline(always)]
    pub fn bftm1rst(&self) -> BFTM1RST_R {
        BFTM1RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - ADCRST"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - SCTM0RST"]
    #[inline(always)]
    pub fn sctm0rst(&self) -> SCTM0RST_R {
        SCTM0RST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCTM1RST"]
    #[inline(always)]
    pub fn sctm1rst(&self) -> SCTM1RST_R {
        SCTM1RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SCTM2RST"]
    #[inline(always)]
    pub fn sctm2rst(&self) -> SCTM2RST_R {
        SCTM2RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SCTM3RST"]
    #[inline(always)]
    pub fn sctm3rst(&self) -> SCTM3RST_R {
        SCTM3RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - WDTRST"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrst(&mut self) -> WDTRST_W<4> {
        WDTRST_W::new(self)
    }
    #[doc = "Bit 8 - GPTMRST"]
    #[inline(always)]
    #[must_use]
    pub fn gptmrst(&mut self) -> GPTMRST_W<8> {
        GPTMRST_W::new(self)
    }
    #[doc = "Bit 16 - BFTM0RST"]
    #[inline(always)]
    #[must_use]
    pub fn bftm0rst(&mut self) -> BFTM0RST_W<16> {
        BFTM0RST_W::new(self)
    }
    #[doc = "Bit 17 - BFTM1RST"]
    #[inline(always)]
    #[must_use]
    pub fn bftm1rst(&mut self) -> BFTM1RST_W<17> {
        BFTM1RST_W::new(self)
    }
    #[doc = "Bit 24 - ADCRST"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<24> {
        ADCRST_W::new(self)
    }
    #[doc = "Bit 28 - SCTM0RST"]
    #[inline(always)]
    #[must_use]
    pub fn sctm0rst(&mut self) -> SCTM0RST_W<28> {
        SCTM0RST_W::new(self)
    }
    #[doc = "Bit 29 - SCTM1RST"]
    #[inline(always)]
    #[must_use]
    pub fn sctm1rst(&mut self) -> SCTM1RST_W<29> {
        SCTM1RST_W::new(self)
    }
    #[doc = "Bit 30 - SCTM2RST"]
    #[inline(always)]
    #[must_use]
    pub fn sctm2rst(&mut self) -> SCTM2RST_W<30> {
        SCTM2RST_W::new(self)
    }
    #[doc = "Bit 31 - SCTM3RST"]
    #[inline(always)]
    #[must_use]
    pub fn sctm3rst(&mut self) -> SCTM3RST_W<31> {
        SCTM3RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RSTCU_APBPRSTR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcu_apbprstr1](index.html) module"]
pub struct RSTCU_APBPRSTR1_SPEC;
impl crate::RegisterSpec for RSTCU_APBPRSTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstcu_apbprstr1::R](R) reader structure"]
impl crate::Readable for RSTCU_APBPRSTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstcu_apbprstr1::W](W) writer structure"]
impl crate::Writable for RSTCU_APBPRSTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTCU_APBPRSTR1 to value 0"]
impl crate::Resettable for RSTCU_APBPRSTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
