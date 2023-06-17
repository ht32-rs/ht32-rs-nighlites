#[doc = "Register `CKCU_APBCCR1` reader"]
pub struct R(crate::R<CKCU_APBCCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_APBCCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_APBCCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_APBCCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_APBCCR1` writer"]
pub struct W(crate::W<CKCU_APBCCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_APBCCR1_SPEC>;
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
impl From<crate::W<CKCU_APBCCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_APBCCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTREN` reader - WDTREN"]
pub type WDTREN_R = crate::BitReader;
#[doc = "Field `WDTREN` writer - WDTREN"]
pub type WDTREN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `VDDREN` reader - VDDREN"]
pub type VDDREN_R = crate::BitReader;
#[doc = "Field `VDDREN` writer - VDDREN"]
pub type VDDREN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `GPTMEN` reader - GPTMEN"]
pub type GPTMEN_R = crate::BitReader;
#[doc = "Field `GPTMEN` writer - GPTMEN"]
pub type GPTMEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `BFTM0EN` reader - BFTM0EN"]
pub type BFTM0EN_R = crate::BitReader;
#[doc = "Field `BFTM0EN` writer - BFTM0EN"]
pub type BFTM0EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `BFTM1EN` reader - BFTM1EN"]
pub type BFTM1EN_R = crate::BitReader;
#[doc = "Field `BFTM1EN` writer - BFTM1EN"]
pub type BFTM1EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `DACCEN` reader - DACCEN"]
pub type DACCEN_R = crate::BitReader;
#[doc = "Field `DACCEN` writer - DACCEN"]
pub type DACCEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `ADCCEN` reader - ADCCEN"]
pub type ADCCEN_R = crate::BitReader;
#[doc = "Field `ADCCEN` writer - ADCCEN"]
pub type ADCCEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `SCTM0EN` reader - SCTM0EN"]
pub type SCTM0EN_R = crate::BitReader;
#[doc = "Field `SCTM0EN` writer - SCTM0EN"]
pub type SCTM0EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `SCTM1EN` reader - SCTM1EN"]
pub type SCTM1EN_R = crate::BitReader;
#[doc = "Field `SCTM1EN` writer - SCTM1EN"]
pub type SCTM1EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `SCTM2EN` reader - SCTM2EN"]
pub type SCTM2EN_R = crate::BitReader;
#[doc = "Field `SCTM2EN` writer - SCTM2EN"]
pub type SCTM2EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `SCTM3EN` reader - SCTM3EN"]
pub type SCTM3EN_R = crate::BitReader;
#[doc = "Field `SCTM3EN` writer - SCTM3EN"]
pub type SCTM3EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
impl R {
    #[doc = "Bit 4 - WDTREN"]
    #[inline(always)]
    pub fn wdtren(&self) -> WDTREN_R {
        WDTREN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - VDDREN"]
    #[inline(always)]
    pub fn vddren(&self) -> VDDREN_R {
        VDDREN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTMEN"]
    #[inline(always)]
    pub fn gptmen(&self) -> GPTMEN_R {
        GPTMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - BFTM0EN"]
    #[inline(always)]
    pub fn bftm0en(&self) -> BFTM0EN_R {
        BFTM0EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BFTM1EN"]
    #[inline(always)]
    pub fn bftm1en(&self) -> BFTM1EN_R {
        BFTM1EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - DACCEN"]
    #[inline(always)]
    pub fn daccen(&self) -> DACCEN_R {
        DACCEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - ADCCEN"]
    #[inline(always)]
    pub fn adccen(&self) -> ADCCEN_R {
        ADCCEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - SCTM0EN"]
    #[inline(always)]
    pub fn sctm0en(&self) -> SCTM0EN_R {
        SCTM0EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCTM1EN"]
    #[inline(always)]
    pub fn sctm1en(&self) -> SCTM1EN_R {
        SCTM1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SCTM2EN"]
    #[inline(always)]
    pub fn sctm2en(&self) -> SCTM2EN_R {
        SCTM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SCTM3EN"]
    #[inline(always)]
    pub fn sctm3en(&self) -> SCTM3EN_R {
        SCTM3EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - WDTREN"]
    #[inline(always)]
    #[must_use]
    pub fn wdtren(&mut self) -> WDTREN_W<4> {
        WDTREN_W::new(self)
    }
    #[doc = "Bit 6 - VDDREN"]
    #[inline(always)]
    #[must_use]
    pub fn vddren(&mut self) -> VDDREN_W<6> {
        VDDREN_W::new(self)
    }
    #[doc = "Bit 8 - GPTMEN"]
    #[inline(always)]
    #[must_use]
    pub fn gptmen(&mut self) -> GPTMEN_W<8> {
        GPTMEN_W::new(self)
    }
    #[doc = "Bit 16 - BFTM0EN"]
    #[inline(always)]
    #[must_use]
    pub fn bftm0en(&mut self) -> BFTM0EN_W<16> {
        BFTM0EN_W::new(self)
    }
    #[doc = "Bit 17 - BFTM1EN"]
    #[inline(always)]
    #[must_use]
    pub fn bftm1en(&mut self) -> BFTM1EN_W<17> {
        BFTM1EN_W::new(self)
    }
    #[doc = "Bit 21 - DACCEN"]
    #[inline(always)]
    #[must_use]
    pub fn daccen(&mut self) -> DACCEN_W<21> {
        DACCEN_W::new(self)
    }
    #[doc = "Bit 24 - ADCCEN"]
    #[inline(always)]
    #[must_use]
    pub fn adccen(&mut self) -> ADCCEN_W<24> {
        ADCCEN_W::new(self)
    }
    #[doc = "Bit 28 - SCTM0EN"]
    #[inline(always)]
    #[must_use]
    pub fn sctm0en(&mut self) -> SCTM0EN_W<28> {
        SCTM0EN_W::new(self)
    }
    #[doc = "Bit 29 - SCTM1EN"]
    #[inline(always)]
    #[must_use]
    pub fn sctm1en(&mut self) -> SCTM1EN_W<29> {
        SCTM1EN_W::new(self)
    }
    #[doc = "Bit 30 - SCTM2EN"]
    #[inline(always)]
    #[must_use]
    pub fn sctm2en(&mut self) -> SCTM2EN_W<30> {
        SCTM2EN_W::new(self)
    }
    #[doc = "Bit 31 - SCTM3EN"]
    #[inline(always)]
    #[must_use]
    pub fn sctm3en(&mut self) -> SCTM3EN_W<31> {
        SCTM3EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_APBCCR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_apbccr1](index.html) module"]
pub struct CKCU_APBCCR1_SPEC;
impl crate::RegisterSpec for CKCU_APBCCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_apbccr1::R](R) reader structure"]
impl crate::Readable for CKCU_APBCCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_apbccr1::W](W) writer structure"]
impl crate::Writable for CKCU_APBCCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_APBCCR1 to value 0"]
impl crate::Resettable for CKCU_APBCCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
