#[doc = "Register `CKCU_APBPCSR1` reader"]
pub struct R(crate::R<CKCU_APBPCSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_APBPCSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_APBPCSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_APBPCSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_APBPCSR1` writer"]
pub struct W(crate::W<CKCU_APBPCSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_APBPCSR1_SPEC>;
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
impl From<crate::W<CKCU_APBPCSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_APBPCSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFIOPCLK` reader - AFIOPCLK"]
pub type AFIOPCLK_R = crate::FieldReader;
#[doc = "Field `AFIOPCLK` writer - AFIOPCLK"]
pub type AFIOPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR1_SPEC, 2, O>;
#[doc = "Field `EXTIPCLK` reader - EXTIPCLK"]
pub type EXTIPCLK_R = crate::FieldReader;
#[doc = "Field `EXTIPCLK` writer - EXTIPCLK"]
pub type EXTIPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR1_SPEC, 2, O>;
#[doc = "Field `ADCCPCLK` reader - ADCCPCLK"]
pub type ADCCPCLK_R = crate::FieldReader;
#[doc = "Field `ADCCPCLK` writer - ADCCPCLK"]
pub type ADCCPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR1_SPEC, 2, O>;
#[doc = "Field `WDTRPCLK` reader - WDTRPCLK"]
pub type WDTRPCLK_R = crate::FieldReader;
#[doc = "Field `WDTRPCLK` writer - WDTRPCLK"]
pub type WDTRPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR1_SPEC, 2, O>;
#[doc = "Field `VDDRCLK` reader - VDDRCLK"]
pub type VDDRCLK_R = crate::FieldReader;
#[doc = "Field `VDDRCLK` writer - VDDRCLK"]
pub type VDDRCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR1_SPEC, 2, O>;
#[doc = "Field `I2SPCLK` reader - I2SPCLK"]
pub type I2SPCLK_R = crate::FieldReader;
#[doc = "Field `I2SPCLK` writer - I2SPCLK"]
pub type I2SPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR1_SPEC, 2, O>;
#[doc = "Field `SCTM0PCLK` reader - SCTM0PCLK"]
pub type SCTM0PCLK_R = crate::FieldReader;
#[doc = "Field `SCTM0PCLK` writer - SCTM0PCLK"]
pub type SCTM0PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR1_SPEC, 2, O>;
#[doc = "Field `SCTM1PCLK` reader - SCTM1PCLK"]
pub type SCTM1PCLK_R = crate::FieldReader;
#[doc = "Field `SCTM1PCLK` writer - SCTM1PCLK"]
pub type SCTM1PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR1_SPEC, 2, O>;
#[doc = "Field `SCTM2PCLK` reader - SCTM2PCLK"]
pub type SCTM2PCLK_R = crate::FieldReader;
#[doc = "Field `SCTM2PCLK` writer - SCTM2PCLK"]
pub type SCTM2PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR1_SPEC, 2, O>;
#[doc = "Field `SCTM3PCLK` reader - SCTM3PCLK"]
pub type SCTM3PCLK_R = crate::FieldReader;
#[doc = "Field `SCTM3PCLK` writer - SCTM3PCLK"]
pub type SCTM3PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR1_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - AFIOPCLK"]
    #[inline(always)]
    pub fn afiopclk(&self) -> AFIOPCLK_R {
        AFIOPCLK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - EXTIPCLK"]
    #[inline(always)]
    pub fn extipclk(&self) -> EXTIPCLK_R {
        EXTIPCLK_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - ADCCPCLK"]
    #[inline(always)]
    pub fn adccpclk(&self) -> ADCCPCLK_R {
        ADCCPCLK_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:13 - WDTRPCLK"]
    #[inline(always)]
    pub fn wdtrpclk(&self) -> WDTRPCLK_R {
        WDTRPCLK_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - VDDRCLK"]
    #[inline(always)]
    pub fn vddrclk(&self) -> VDDRCLK_R {
        VDDRCLK_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 20:21 - I2SPCLK"]
    #[inline(always)]
    pub fn i2spclk(&self) -> I2SPCLK_R {
        I2SPCLK_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - SCTM0PCLK"]
    #[inline(always)]
    pub fn sctm0pclk(&self) -> SCTM0PCLK_R {
        SCTM0PCLK_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - SCTM1PCLK"]
    #[inline(always)]
    pub fn sctm1pclk(&self) -> SCTM1PCLK_R {
        SCTM1PCLK_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - SCTM2PCLK"]
    #[inline(always)]
    pub fn sctm2pclk(&self) -> SCTM2PCLK_R {
        SCTM2PCLK_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - SCTM3PCLK"]
    #[inline(always)]
    pub fn sctm3pclk(&self) -> SCTM3PCLK_R {
        SCTM3PCLK_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AFIOPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn afiopclk(&mut self) -> AFIOPCLK_W<0> {
        AFIOPCLK_W::new(self)
    }
    #[doc = "Bits 2:3 - EXTIPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn extipclk(&mut self) -> EXTIPCLK_W<2> {
        EXTIPCLK_W::new(self)
    }
    #[doc = "Bits 4:5 - ADCCPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn adccpclk(&mut self) -> ADCCPCLK_W<4> {
        ADCCPCLK_W::new(self)
    }
    #[doc = "Bits 12:13 - WDTRPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrpclk(&mut self) -> WDTRPCLK_W<12> {
        WDTRPCLK_W::new(self)
    }
    #[doc = "Bits 14:15 - VDDRCLK"]
    #[inline(always)]
    #[must_use]
    pub fn vddrclk(&mut self) -> VDDRCLK_W<14> {
        VDDRCLK_W::new(self)
    }
    #[doc = "Bits 20:21 - I2SPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn i2spclk(&mut self) -> I2SPCLK_W<20> {
        I2SPCLK_W::new(self)
    }
    #[doc = "Bits 24:25 - SCTM0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn sctm0pclk(&mut self) -> SCTM0PCLK_W<24> {
        SCTM0PCLK_W::new(self)
    }
    #[doc = "Bits 26:27 - SCTM1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn sctm1pclk(&mut self) -> SCTM1PCLK_W<26> {
        SCTM1PCLK_W::new(self)
    }
    #[doc = "Bits 28:29 - SCTM2PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn sctm2pclk(&mut self) -> SCTM2PCLK_W<28> {
        SCTM2PCLK_W::new(self)
    }
    #[doc = "Bits 30:31 - SCTM3PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn sctm3pclk(&mut self) -> SCTM3PCLK_W<30> {
        SCTM3PCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_APBPCSR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_apbpcsr1](index.html) module"]
pub struct CKCU_APBPCSR1_SPEC;
impl crate::RegisterSpec for CKCU_APBPCSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_apbpcsr1::R](R) reader structure"]
impl crate::Readable for CKCU_APBPCSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_apbpcsr1::W](W) writer structure"]
impl crate::Writable for CKCU_APBPCSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_APBPCSR1 to value 0"]
impl crate::Resettable for CKCU_APBPCSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
