#[doc = "Register `APBPCSR1` reader"]
pub struct R(crate::R<APBPCSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBPCSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBPCSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBPCSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBPCSR1` writer"]
pub struct W(crate::W<APBPCSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBPCSR1_SPEC>;
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
impl From<crate::W<APBPCSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBPCSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2CPCLK` reader - I2CPCLK"]
pub type I2CPCLK_R = crate::FieldReader;
#[doc = "Field `I2CPCLK` writer - I2CPCLK"]
pub type I2CPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR1_SPEC, 2, O>;
#[doc = "Field `SPIPCLK` reader - SPIPCLK"]
pub type SPIPCLK_R = crate::FieldReader;
#[doc = "Field `SPIPCLK` writer - SPIPCLK"]
pub type SPIPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR1_SPEC, 2, O>;
#[doc = "Field `BFTMPCLK` reader - BFTMPCLK"]
pub type BFTMPCLK_R = crate::FieldReader;
#[doc = "Field `BFTMPCLK` writer - BFTMPCLK"]
pub type BFTMPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR1_SPEC, 2, O>;
#[doc = "Field `GPTMPCLK` reader - GPTMPCLK"]
pub type GPTMPCLK_R = crate::FieldReader;
#[doc = "Field `GPTMPCLK` writer - GPTMPCLK"]
pub type GPTMPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR1_SPEC, 2, O>;
#[doc = "Field `USRPCLK` reader - USRPCLK"]
pub type USRPCLK_R = crate::FieldReader;
#[doc = "Field `USRPCLK` writer - USRPCLK"]
pub type USRPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR1_SPEC, 2, O>;
#[doc = "Field `URPCLK` reader - URPCLK"]
pub type URPCLK_R = crate::FieldReader;
#[doc = "Field `URPCLK` writer - URPCLK"]
pub type URPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR1_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - I2CPCLK"]
    #[inline(always)]
    pub fn i2cpclk(&self) -> I2CPCLK_R {
        I2CPCLK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - SPIPCLK"]
    #[inline(always)]
    pub fn spipclk(&self) -> SPIPCLK_R {
        SPIPCLK_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:13 - BFTMPCLK"]
    #[inline(always)]
    pub fn bftmpclk(&self) -> BFTMPCLK_R {
        BFTMPCLK_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPTMPCLK"]
    #[inline(always)]
    pub fn gptmpclk(&self) -> GPTMPCLK_R {
        GPTMPCLK_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - USRPCLK"]
    #[inline(always)]
    pub fn usrpclk(&self) -> USRPCLK_R {
        USRPCLK_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - URPCLK"]
    #[inline(always)]
    pub fn urpclk(&self) -> URPCLK_R {
        URPCLK_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - I2CPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn i2cpclk(&mut self) -> I2CPCLK_W<0> {
        I2CPCLK_W::new(self)
    }
    #[doc = "Bits 4:5 - SPIPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn spipclk(&mut self) -> SPIPCLK_W<4> {
        SPIPCLK_W::new(self)
    }
    #[doc = "Bits 12:13 - BFTMPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn bftmpclk(&mut self) -> BFTMPCLK_W<12> {
        BFTMPCLK_W::new(self)
    }
    #[doc = "Bits 20:21 - GPTMPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn gptmpclk(&mut self) -> GPTMPCLK_W<20> {
        GPTMPCLK_W::new(self)
    }
    #[doc = "Bits 24:25 - USRPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn usrpclk(&mut self) -> USRPCLK_W<24> {
        USRPCLK_W::new(self)
    }
    #[doc = "Bits 28:29 - URPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn urpclk(&mut self) -> URPCLK_W<28> {
        URPCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBPCSR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbpcsr1](index.html) module"]
pub struct APBPCSR1_SPEC;
impl crate::RegisterSpec for APBPCSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbpcsr1::R](R) reader structure"]
impl crate::Readable for APBPCSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbpcsr1::W](W) writer structure"]
impl crate::Writable for APBPCSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBPCSR1 to value 0"]
impl crate::Resettable for APBPCSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
