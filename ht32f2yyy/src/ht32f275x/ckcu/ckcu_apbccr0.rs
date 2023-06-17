#[doc = "Register `CKCU_APBCCR0` reader"]
pub struct R(crate::R<CKCU_APBCCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_APBCCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_APBCCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_APBCCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_APBCCR0` writer"]
pub struct W(crate::W<CKCU_APBCCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_APBCCR0_SPEC>;
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
impl From<crate::W<CKCU_APBCCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_APBCCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C0EN` reader - I2C0EN"]
pub type I2C0EN_R = crate::BitReader;
#[doc = "Field `I2C0EN` writer - I2C0EN"]
pub type I2C0EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `I2C1EN` reader - I2C1EN"]
pub type I2C1EN_R = crate::BitReader;
#[doc = "Field `I2C1EN` writer - I2C1EN"]
pub type I2C1EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `SPI0EN` reader - SPI0EN"]
pub type SPI0EN_R = crate::BitReader;
#[doc = "Field `SPI0EN` writer - SPI0EN"]
pub type SPI0EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `SPI1EN` reader - SPI1EN"]
pub type SPI1EN_R = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1EN"]
pub type SPI1EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `UR0EN` reader - UR0EN"]
pub type UR0EN_R = crate::BitReader;
#[doc = "Field `UR0EN` writer - UR0EN"]
pub type UR0EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `UR1EN` reader - UR1EN"]
pub type UR1EN_R = crate::BitReader;
#[doc = "Field `UR1EN` writer - UR1EN"]
pub type UR1EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `AFIOEN` reader - AFIOEN"]
pub type AFIOEN_R = crate::BitReader;
#[doc = "Field `AFIOEN` writer - AFIOEN"]
pub type AFIOEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `EXTIEN` reader - EXTIEN"]
pub type EXTIEN_R = crate::BitReader;
#[doc = "Field `EXTIEN` writer - EXTIEN"]
pub type EXTIEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `PAEN` reader - PAEN"]
pub type PAEN_R = crate::BitReader;
#[doc = "Field `PAEN` writer - PAEN"]
pub type PAEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `PBEN` reader - PBEN"]
pub type PBEN_R = crate::BitReader;
#[doc = "Field `PBEN` writer - PBEN"]
pub type PBEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `PCEN` reader - PCEN"]
pub type PCEN_R = crate::BitReader;
#[doc = "Field `PCEN` writer - PCEN"]
pub type PCEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `PDEN` reader - PDEN"]
pub type PDEN_R = crate::BitReader;
#[doc = "Field `PDEN` writer - PDEN"]
pub type PDEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `PEEN` reader - PEEN"]
pub type PEEN_R = crate::BitReader;
#[doc = "Field `PEEN` writer - PEEN"]
pub type PEEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `SCIEN` reader - SCIEN"]
pub type SCIEN_R = crate::BitReader;
#[doc = "Field `SCIEN` writer - SCIEN"]
pub type SCIEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
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
    #[doc = "Bit 8 - UR0EN"]
    #[inline(always)]
    pub fn ur0en(&self) -> UR0EN_R {
        UR0EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UR1EN"]
    #[inline(always)]
    pub fn ur1en(&self) -> UR1EN_R {
        UR1EN_R::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 16 - PAEN"]
    #[inline(always)]
    pub fn paen(&self) -> PAEN_R {
        PAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PBEN"]
    #[inline(always)]
    pub fn pben(&self) -> PBEN_R {
        PBEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PCEN"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PDEN"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PEEN"]
    #[inline(always)]
    pub fn peen(&self) -> PEEN_R {
        PEEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - SCIEN"]
    #[inline(always)]
    pub fn scien(&self) -> SCIEN_R {
        SCIEN_R::new(((self.bits >> 24) & 1) != 0)
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
    #[doc = "Bit 8 - UR0EN"]
    #[inline(always)]
    #[must_use]
    pub fn ur0en(&mut self) -> UR0EN_W<8> {
        UR0EN_W::new(self)
    }
    #[doc = "Bit 9 - UR1EN"]
    #[inline(always)]
    #[must_use]
    pub fn ur1en(&mut self) -> UR1EN_W<9> {
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
    #[doc = "Bit 16 - PAEN"]
    #[inline(always)]
    #[must_use]
    pub fn paen(&mut self) -> PAEN_W<16> {
        PAEN_W::new(self)
    }
    #[doc = "Bit 17 - PBEN"]
    #[inline(always)]
    #[must_use]
    pub fn pben(&mut self) -> PBEN_W<17> {
        PBEN_W::new(self)
    }
    #[doc = "Bit 18 - PCEN"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PCEN_W<18> {
        PCEN_W::new(self)
    }
    #[doc = "Bit 19 - PDEN"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<19> {
        PDEN_W::new(self)
    }
    #[doc = "Bit 20 - PEEN"]
    #[inline(always)]
    #[must_use]
    pub fn peen(&mut self) -> PEEN_W<20> {
        PEEN_W::new(self)
    }
    #[doc = "Bit 24 - SCIEN"]
    #[inline(always)]
    #[must_use]
    pub fn scien(&mut self) -> SCIEN_W<24> {
        SCIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_APBCCR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_apbccr0](index.html) module"]
pub struct CKCU_APBCCR0_SPEC;
impl crate::RegisterSpec for CKCU_APBCCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_apbccr0::R](R) reader structure"]
impl crate::Readable for CKCU_APBCCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_apbccr0::W](W) writer structure"]
impl crate::Writable for CKCU_APBCCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_APBCCR0 to value 0"]
impl crate::Resettable for CKCU_APBCCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
