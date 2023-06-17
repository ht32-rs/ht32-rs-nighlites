#[doc = "Register `APBCCR1` reader"]
pub struct R(crate::R<APBCCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBCCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBCCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBCCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBCCR1` writer"]
pub struct W(crate::W<APBCCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBCCR1_SPEC>;
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
impl From<crate::W<APBCCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBCCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2CEN` reader - I2CEN"]
pub type I2CEN_R = crate::BitReader;
#[doc = "Field `I2CEN` writer - I2CEN"]
pub type I2CEN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR1_SPEC, O>;
#[doc = "Field `SPI0EN` reader - SPI0EN"]
pub type SPI0EN_R = crate::BitReader;
#[doc = "Field `SPI0EN` writer - SPI0EN"]
pub type SPI0EN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR1_SPEC, O>;
#[doc = "Field `SPI1EN` reader - SPI1EN"]
pub type SPI1EN_R = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1EN"]
pub type SPI1EN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR1_SPEC, O>;
#[doc = "Field `UR0EN` reader - UR0EN"]
pub type UR0EN_R = crate::BitReader;
#[doc = "Field `UR0EN` writer - UR0EN"]
pub type UR0EN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR1_SPEC, O>;
#[doc = "Field `UR1EN` reader - UR1EN"]
pub type UR1EN_R = crate::BitReader;
#[doc = "Field `UR1EN` writer - UR1EN"]
pub type UR1EN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR1_SPEC, O>;
#[doc = "Field `AFIOEN` reader - AFIOEN"]
pub type AFIOEN_R = crate::BitReader;
#[doc = "Field `AFIOEN` writer - AFIOEN"]
pub type AFIOEN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR1_SPEC, O>;
#[doc = "Field `EXTIEN` reader - EXTIEN"]
pub type EXTIEN_R = crate::BitReader;
#[doc = "Field `EXTIEN` writer - EXTIEN"]
pub type EXTIEN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - I2CEN"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - SPI0EN"]
    #[inline(always)]
    pub fn spi0en(&self) -> SPI0EN_R {
        SPI0EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI1EN"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - UR0EN"]
    #[inline(always)]
    pub fn ur0en(&self) -> UR0EN_R {
        UR0EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UR1EN"]
    #[inline(always)]
    pub fn ur1en(&self) -> UR1EN_R {
        UR1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - AFIOEN"]
    #[inline(always)]
    pub fn afioen(&self) -> AFIOEN_R {
        AFIOEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTIEN"]
    #[inline(always)]
    pub fn extien(&self) -> EXTIEN_R {
        EXTIEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2CEN"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2CEN_W<0> {
        I2CEN_W::new(self)
    }
    #[doc = "Bit 4 - SPI0EN"]
    #[inline(always)]
    #[must_use]
    pub fn spi0en(&mut self) -> SPI0EN_W<4> {
        SPI0EN_W::new(self)
    }
    #[doc = "Bit 5 - SPI1EN"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<5> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 10 - UR0EN"]
    #[inline(always)]
    #[must_use]
    pub fn ur0en(&mut self) -> UR0EN_W<10> {
        UR0EN_W::new(self)
    }
    #[doc = "Bit 11 - UR1EN"]
    #[inline(always)]
    #[must_use]
    pub fn ur1en(&mut self) -> UR1EN_W<11> {
        UR1EN_W::new(self)
    }
    #[doc = "Bit 14 - AFIOEN"]
    #[inline(always)]
    #[must_use]
    pub fn afioen(&mut self) -> AFIOEN_W<14> {
        AFIOEN_W::new(self)
    }
    #[doc = "Bit 15 - EXTIEN"]
    #[inline(always)]
    #[must_use]
    pub fn extien(&mut self) -> EXTIEN_W<15> {
        EXTIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBCCR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbccr1](index.html) module"]
pub struct APBCCR1_SPEC;
impl crate::RegisterSpec for APBCCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbccr1::R](R) reader structure"]
impl crate::Readable for APBCCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbccr1::W](W) writer structure"]
impl crate::Writable for APBCCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBCCR1 to value 0"]
impl crate::Resettable for APBCCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
