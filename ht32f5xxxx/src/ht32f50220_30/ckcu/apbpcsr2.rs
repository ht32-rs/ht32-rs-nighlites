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
#[doc = "Field `SPI0PCLK` reader - SPI0PCLK"]
pub type SPI0PCLK_R = crate::FieldReader;
#[doc = "Field `SPI0PCLK` writer - SPI0PCLK"]
pub type SPI0PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR2_SPEC, 2, O>;
#[doc = "Field `SPI1PCLK` reader - SPI1PCLK"]
pub type SPI1PCLK_R = crate::FieldReader;
#[doc = "Field `SPI1PCLK` writer - SPI1PCLK"]
pub type SPI1PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR2_SPEC, 2, O>;
#[doc = "Field `BFTMPCLK` reader - BFTMPCLK"]
pub type BFTMPCLK_R = crate::FieldReader;
#[doc = "Field `BFTMPCLK` writer - BFTMPCLK"]
pub type BFTMPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR2_SPEC, 2, O>;
#[doc = "Field `GPTMPCLK` reader - GPTMPCLK"]
pub type GPTMPCLK_R = crate::FieldReader;
#[doc = "Field `GPTMPCLK` writer - GPTMPCLK"]
pub type GPTMPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR2_SPEC, 2, O>;
#[doc = "Field `UR0PCLK` reader - UR0PCLK"]
pub type UR0PCLK_R = crate::FieldReader;
#[doc = "Field `UR0PCLK` writer - UR0PCLK"]
pub type UR0PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR2_SPEC, 2, O>;
#[doc = "Field `UR1PCLK` reader - UR1PCLK"]
pub type UR1PCLK_R = crate::FieldReader;
#[doc = "Field `UR1PCLK` writer - UR1PCLK"]
pub type UR1PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, APBPCSR2_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - I2CPCLK"]
    #[inline(always)]
    pub fn i2cpclk(&self) -> I2CPCLK_R {
        I2CPCLK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - SPI0PCLK"]
    #[inline(always)]
    pub fn spi0pclk(&self) -> SPI0PCLK_R {
        SPI0PCLK_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SPI1PCLK"]
    #[inline(always)]
    pub fn spi1pclk(&self) -> SPI1PCLK_R {
        SPI1PCLK_R::new(((self.bits >> 6) & 3) as u8)
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
    #[doc = "Bits 28:29 - UR0PCLK"]
    #[inline(always)]
    pub fn ur0pclk(&self) -> UR0PCLK_R {
        UR0PCLK_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - UR1PCLK"]
    #[inline(always)]
    pub fn ur1pclk(&self) -> UR1PCLK_R {
        UR1PCLK_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - I2CPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn i2cpclk(&mut self) -> I2CPCLK_W<0> {
        I2CPCLK_W::new(self)
    }
    #[doc = "Bits 4:5 - SPI0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn spi0pclk(&mut self) -> SPI0PCLK_W<4> {
        SPI0PCLK_W::new(self)
    }
    #[doc = "Bits 6:7 - SPI1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn spi1pclk(&mut self) -> SPI1PCLK_W<6> {
        SPI1PCLK_W::new(self)
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
    #[doc = "Bits 28:29 - UR0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn ur0pclk(&mut self) -> UR0PCLK_W<28> {
        UR0PCLK_W::new(self)
    }
    #[doc = "Bits 30:31 - UR1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn ur1pclk(&mut self) -> UR1PCLK_W<30> {
        UR1PCLK_W::new(self)
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
