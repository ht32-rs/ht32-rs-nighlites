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
#[doc = "Field `CMPPCLK` reader - CMPPCLK"]
pub type CMPPCLK_R = crate::FieldReader;
#[doc = "Field `CMPPCLK` writer - CMPPCLK"]
pub type CMPPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR1_SPEC, 2, O>;
#[doc = "Field `WDTRPCLK` reader - WDTRPCLK"]
pub type WDTRPCLK_R = crate::FieldReader;
#[doc = "Field `WDTRPCLK` writer - WDTRPCLK"]
pub type WDTRPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR1_SPEC, 2, O>;
#[doc = "Field `BKPRPCLK` reader - BKPRPCLK"]
pub type BKPRPCLK_R = crate::FieldReader;
#[doc = "Field `BKPRPCLK` writer - BKPRPCLK"]
pub type BKPRPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR1_SPEC, 2, O>;
#[doc = "Field `SCIPCLK` reader - SCIPCLK"]
pub type SCIPCLK_R = crate::FieldReader;
#[doc = "Field `SCIPCLK` writer - SCIPCLK"]
pub type SCIPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR1_SPEC, 2, O>;
#[doc = "Field `I2SPCLK` reader - I2SPCLK"]
pub type I2SPCLK_R = crate::FieldReader;
#[doc = "Field `I2SPCLK` writer - I2SPCLK"]
pub type I2SPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR1_SPEC, 2, O>;
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
    #[doc = "Bits 8:9 - CMPPCLK"]
    #[inline(always)]
    pub fn cmppclk(&self) -> CMPPCLK_R {
        CMPPCLK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - WDTRPCLK"]
    #[inline(always)]
    pub fn wdtrpclk(&self) -> WDTRPCLK_R {
        WDTRPCLK_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - BKPRPCLK"]
    #[inline(always)]
    pub fn bkprpclk(&self) -> BKPRPCLK_R {
        BKPRPCLK_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SCIPCLK"]
    #[inline(always)]
    pub fn scipclk(&self) -> SCIPCLK_R {
        SCIPCLK_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - I2SPCLK"]
    #[inline(always)]
    pub fn i2spclk(&self) -> I2SPCLK_R {
        I2SPCLK_R::new(((self.bits >> 20) & 3) as u8)
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
    #[doc = "Bits 8:9 - CMPPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn cmppclk(&mut self) -> CMPPCLK_W<8> {
        CMPPCLK_W::new(self)
    }
    #[doc = "Bits 12:13 - WDTRPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrpclk(&mut self) -> WDTRPCLK_W<12> {
        WDTRPCLK_W::new(self)
    }
    #[doc = "Bits 14:15 - BKPRPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn bkprpclk(&mut self) -> BKPRPCLK_W<14> {
        BKPRPCLK_W::new(self)
    }
    #[doc = "Bits 16:17 - SCIPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn scipclk(&mut self) -> SCIPCLK_W<16> {
        SCIPCLK_W::new(self)
    }
    #[doc = "Bits 20:21 - I2SPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn i2spclk(&mut self) -> I2SPCLK_W<20> {
        I2SPCLK_W::new(self)
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
