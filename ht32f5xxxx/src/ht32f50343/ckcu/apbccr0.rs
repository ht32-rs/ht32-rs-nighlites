#[doc = "Register `APBCCR0` reader"]
pub struct R(crate::R<APBCCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBCCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBCCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBCCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBCCR0` writer"]
pub struct W(crate::W<APBCCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBCCR0_SPEC>;
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
impl From<crate::W<APBCCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBCCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C0EN` reader - I2C0EN"]
pub type I2C0EN_R = crate::BitReader;
#[doc = "Field `I2C0EN` writer - I2C0EN"]
pub type I2C0EN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR0_SPEC, O>;
#[doc = "Field `I2C1EN` reader - I2C1EN"]
pub type I2C1EN_R = crate::BitReader;
#[doc = "Field `I2C1EN` writer - I2C1EN"]
pub type I2C1EN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR0_SPEC, O>;
#[doc = "Field `SPI0EN` reader - SPI0EN"]
pub type SPI0EN_R = crate::BitReader;
#[doc = "Field `SPI0EN` writer - SPI0EN"]
pub type SPI0EN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR0_SPEC, O>;
#[doc = "Field `SPI1EN` reader - SPI1EN"]
pub type SPI1EN_R = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1EN"]
pub type SPI1EN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR0_SPEC, O>;
#[doc = "Field `UR0EN` reader - UR0EN"]
pub type UR0EN_R = crate::BitReader;
#[doc = "Field `UR0EN` writer - UR0EN"]
pub type UR0EN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR0_SPEC, O>;
#[doc = "Field `UR1EN` reader - UR1EN"]
pub type UR1EN_R = crate::BitReader;
#[doc = "Field `UR1EN` writer - UR1EN"]
pub type UR1EN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR0_SPEC, O>;
#[doc = "Field `AFIOEN` reader - AFIOEN"]
pub type AFIOEN_R = crate::BitReader;
#[doc = "Field `AFIOEN` writer - AFIOEN"]
pub type AFIOEN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR0_SPEC, O>;
#[doc = "Field `EXTIEN` reader - EXTIEN"]
pub type EXTIEN_R = crate::BitReader;
#[doc = "Field `EXTIEN` writer - EXTIEN"]
pub type EXTIEN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR0_SPEC, O>;
#[doc = "Field `SLED0EN` reader - SLED0EN"]
pub type SLED0EN_R = crate::BitReader;
#[doc = "Field `SLED0EN` writer - SLED0EN"]
pub type SLED0EN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR0_SPEC, O>;
#[doc = "Field `SLED1EN` reader - SLED1EN"]
pub type SLED1EN_R = crate::BitReader;
#[doc = "Field `SLED1EN` writer - SLED1EN"]
pub type SLED1EN_W<'a, const O: u8> = crate::BitWriter<'a, APBCCR0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - I2C0EN"]
    #[inline(always)]
    pub fn i2c0en(&self) -> I2C0EN_R {
        I2C0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C1EN"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 1) & 1) != 0)
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
    #[doc = "Bit 22 - SLED0EN"]
    #[inline(always)]
    pub fn sled0en(&self) -> SLED0EN_R {
        SLED0EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SLED1EN"]
    #[inline(always)]
    pub fn sled1en(&self) -> SLED1EN_R {
        SLED1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C0EN"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0en(&mut self) -> I2C0EN_W<0> {
        I2C0EN_W::new(self)
    }
    #[doc = "Bit 1 - I2C1EN"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<1> {
        I2C1EN_W::new(self)
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
    #[doc = "Bit 22 - SLED0EN"]
    #[inline(always)]
    #[must_use]
    pub fn sled0en(&mut self) -> SLED0EN_W<22> {
        SLED0EN_W::new(self)
    }
    #[doc = "Bit 23 - SLED1EN"]
    #[inline(always)]
    #[must_use]
    pub fn sled1en(&mut self) -> SLED1EN_W<23> {
        SLED1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBCCR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbccr0](index.html) module"]
pub struct APBCCR0_SPEC;
impl crate::RegisterSpec for APBCCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbccr0::R](R) reader structure"]
impl crate::Readable for APBCCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbccr0::W](W) writer structure"]
impl crate::Writable for APBCCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBCCR0 to value 0"]
impl crate::Resettable for APBCCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
