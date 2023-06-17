#[doc = "Register `APBPCSR2` reader"]
pub struct R(crate::R<APBPCSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBPCSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBPCSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBPCSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBPCSR2` writer"]
pub struct W(crate::W<APBPCSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBPCSR2_SPEC>;
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
impl From<crate::W<APBPCSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBPCSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2CPCLK` reader - I2CPCLK"]
pub type I2CPCLK_R = crate::FieldReader;
#[doc = "Field `I2CPCLK` writer - I2CPCLK"]
pub type I2CPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR2_SPEC, 2, O>;
#[doc = "Field `SPIPCLK` reader - SPIPCLK"]
pub type SPIPCLK_R = crate::FieldReader;
#[doc = "Field `SPIPCLK` writer - SPIPCLK"]
pub type SPIPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR2_SPEC, 2, O>;
#[doc = "Field `BFTM0PCLK` reader - BFTM0PCLK"]
pub type BFTM0PCLK_R = crate::FieldReader;
#[doc = "Field `BFTM0PCLK` writer - BFTM0PCLK"]
pub type BFTM0PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR2_SPEC, 2, O>;
#[doc = "Field `BFTM1PCLK` reader - BFTM1PCLK"]
pub type BFTM1PCLK_R = crate::FieldReader;
#[doc = "Field `BFTM1PCLK` writer - BFTM1PCLK"]
pub type BFTM1PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR2_SPEC, 2, O>;
#[doc = "Field `GPTMPCLK` reader - GPTMPCLK"]
pub type GPTMPCLK_R = crate::FieldReader;
#[doc = "Field `GPTMPCLK` writer - GPTMPCLK"]
pub type GPTMPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR2_SPEC, 2, O>;
#[doc = "Field `USRPCLK` reader - USRPCLK"]
pub type USRPCLK_R = crate::FieldReader;
#[doc = "Field `USRPCLK` writer - USRPCLK"]
pub type USRPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR2_SPEC, 2, O>;
#[doc = "Field `URPCLK` reader - URPCLK"]
pub type URPCLK_R = crate::FieldReader;
#[doc = "Field `URPCLK` writer - URPCLK"]
pub type URPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR2_SPEC, 2, O>;
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
    #[doc = "Bits 12:13 - BFTM0PCLK"]
    #[inline(always)]
    pub fn bftm0pclk(&self) -> BFTM0PCLK_R {
        BFTM0PCLK_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - BFTM1PCLK"]
    #[inline(always)]
    pub fn bftm1pclk(&self) -> BFTM1PCLK_R {
        BFTM1PCLK_R::new(((self.bits >> 14) & 3) as u8)
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
    #[doc = "Bits 12:13 - BFTM0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn bftm0pclk(&mut self) -> BFTM0PCLK_W<12> {
        BFTM0PCLK_W::new(self)
    }
    #[doc = "Bits 14:15 - BFTM1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn bftm1pclk(&mut self) -> BFTM1PCLK_W<14> {
        BFTM1PCLK_W::new(self)
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
#[doc = "APBPCSR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbpcsr2](index.html) module"]
pub struct APBPCSR2_SPEC;
impl crate::RegisterSpec for APBPCSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbpcsr2::R](R) reader structure"]
impl crate::Readable for APBPCSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbpcsr2::W](W) writer structure"]
impl crate::Writable for APBPCSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBPCSR2 to value 0"]
impl crate::Resettable for APBPCSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
