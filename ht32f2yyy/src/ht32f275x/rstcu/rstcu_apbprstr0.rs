#[doc = "Register `RSTCU_APBPRSTR0` reader"]
pub struct R(crate::R<RSTCU_APBPRSTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCU_APBPRSTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCU_APBPRSTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCU_APBPRSTR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCU_APBPRSTR0` writer"]
pub struct W(crate::W<RSTCU_APBPRSTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCU_APBPRSTR0_SPEC>;
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
impl From<crate::W<RSTCU_APBPRSTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCU_APBPRSTR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C0RST` reader - I2C0RST"]
pub type I2C0RST_R = crate::BitReader;
#[doc = "Field `I2C0RST` writer - I2C0RST"]
pub type I2C0RST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
#[doc = "Field `I2C1RST` reader - I2C1RST"]
pub type I2C1RST_R = crate::BitReader;
#[doc = "Field `I2C1RST` writer - I2C1RST"]
pub type I2C1RST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
#[doc = "Field `SPI0RST` reader - SPI0RST"]
pub type SPI0RST_R = crate::BitReader;
#[doc = "Field `SPI0RST` writer - SPI0RST"]
pub type SPI0RST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
#[doc = "Field `SPI1RST` reader - SPI1RST"]
pub type SPI1RST_R = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1RST"]
pub type SPI1RST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
#[doc = "Field `UR0RST` reader - UR0RST"]
pub type UR0RST_R = crate::BitReader;
#[doc = "Field `UR0RST` writer - UR0RST"]
pub type UR0RST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
#[doc = "Field `UR1RST` reader - UR1RST"]
pub type UR1RST_R = crate::BitReader;
#[doc = "Field `UR1RST` writer - UR1RST"]
pub type UR1RST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
#[doc = "Field `AFIORST` reader - AFIORST"]
pub type AFIORST_R = crate::BitReader;
#[doc = "Field `AFIORST` writer - AFIORST"]
pub type AFIORST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
#[doc = "Field `EXTIRST` reader - EXTIRST"]
pub type EXTIRST_R = crate::BitReader;
#[doc = "Field `EXTIRST` writer - EXTIRST"]
pub type EXTIRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
#[doc = "Field `PARST` reader - PARST"]
pub type PARST_R = crate::BitReader;
#[doc = "Field `PARST` writer - PARST"]
pub type PARST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
#[doc = "Field `PBRST` reader - PBRST"]
pub type PBRST_R = crate::BitReader;
#[doc = "Field `PBRST` writer - PBRST"]
pub type PBRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
#[doc = "Field `PCRST` reader - PCRST"]
pub type PCRST_R = crate::BitReader;
#[doc = "Field `PCRST` writer - PCRST"]
pub type PCRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
#[doc = "Field `PDRST` reader - PDRST"]
pub type PDRST_R = crate::BitReader;
#[doc = "Field `PDRST` writer - PDRST"]
pub type PDRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
#[doc = "Field `PERST` reader - PERST"]
pub type PERST_R = crate::BitReader;
#[doc = "Field `PERST` writer - PERST"]
pub type PERST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
#[doc = "Field `SCIRST` reader - SCIRST"]
pub type SCIRST_R = crate::BitReader;
#[doc = "Field `SCIRST` writer - SCIRST"]
pub type SCIRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - I2C0RST"]
    #[inline(always)]
    pub fn i2c0rst(&self) -> I2C0RST_R {
        I2C0RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C1RST"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI0RST"]
    #[inline(always)]
    pub fn spi0rst(&self) -> SPI0RST_R {
        SPI0RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - UR0RST"]
    #[inline(always)]
    pub fn ur0rst(&self) -> UR0RST_R {
        UR0RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UR1RST"]
    #[inline(always)]
    pub fn ur1rst(&self) -> UR1RST_R {
        UR1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - AFIORST"]
    #[inline(always)]
    pub fn afiorst(&self) -> AFIORST_R {
        AFIORST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTIRST"]
    #[inline(always)]
    pub fn extirst(&self) -> EXTIRST_R {
        EXTIRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PARST"]
    #[inline(always)]
    pub fn parst(&self) -> PARST_R {
        PARST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PBRST"]
    #[inline(always)]
    pub fn pbrst(&self) -> PBRST_R {
        PBRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PCRST"]
    #[inline(always)]
    pub fn pcrst(&self) -> PCRST_R {
        PCRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PDRST"]
    #[inline(always)]
    pub fn pdrst(&self) -> PDRST_R {
        PDRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PERST"]
    #[inline(always)]
    pub fn perst(&self) -> PERST_R {
        PERST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - SCIRST"]
    #[inline(always)]
    pub fn scirst(&self) -> SCIRST_R {
        SCIRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C0RST"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0rst(&mut self) -> I2C0RST_W<0> {
        I2C0RST_W::new(self)
    }
    #[doc = "Bit 1 - I2C1RST"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<1> {
        I2C1RST_W::new(self)
    }
    #[doc = "Bit 4 - SPI0RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi0rst(&mut self) -> SPI0RST_W<4> {
        SPI0RST_W::new(self)
    }
    #[doc = "Bit 5 - SPI1RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<5> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 8 - UR0RST"]
    #[inline(always)]
    #[must_use]
    pub fn ur0rst(&mut self) -> UR0RST_W<8> {
        UR0RST_W::new(self)
    }
    #[doc = "Bit 9 - UR1RST"]
    #[inline(always)]
    #[must_use]
    pub fn ur1rst(&mut self) -> UR1RST_W<9> {
        UR1RST_W::new(self)
    }
    #[doc = "Bit 14 - AFIORST"]
    #[inline(always)]
    #[must_use]
    pub fn afiorst(&mut self) -> AFIORST_W<14> {
        AFIORST_W::new(self)
    }
    #[doc = "Bit 15 - EXTIRST"]
    #[inline(always)]
    #[must_use]
    pub fn extirst(&mut self) -> EXTIRST_W<15> {
        EXTIRST_W::new(self)
    }
    #[doc = "Bit 16 - PARST"]
    #[inline(always)]
    #[must_use]
    pub fn parst(&mut self) -> PARST_W<16> {
        PARST_W::new(self)
    }
    #[doc = "Bit 17 - PBRST"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst(&mut self) -> PBRST_W<17> {
        PBRST_W::new(self)
    }
    #[doc = "Bit 18 - PCRST"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst(&mut self) -> PCRST_W<18> {
        PCRST_W::new(self)
    }
    #[doc = "Bit 19 - PDRST"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst(&mut self) -> PDRST_W<19> {
        PDRST_W::new(self)
    }
    #[doc = "Bit 20 - PERST"]
    #[inline(always)]
    #[must_use]
    pub fn perst(&mut self) -> PERST_W<20> {
        PERST_W::new(self)
    }
    #[doc = "Bit 24 - SCIRST"]
    #[inline(always)]
    #[must_use]
    pub fn scirst(&mut self) -> SCIRST_W<24> {
        SCIRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RSTCU_APBPRSTR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcu_apbprstr0](index.html) module"]
pub struct RSTCU_APBPRSTR0_SPEC;
impl crate::RegisterSpec for RSTCU_APBPRSTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstcu_apbprstr0::R](R) reader structure"]
impl crate::Readable for RSTCU_APBPRSTR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstcu_apbprstr0::W](W) writer structure"]
impl crate::Writable for RSTCU_APBPRSTR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTCU_APBPRSTR0 to value 0"]
impl crate::Resettable for RSTCU_APBPRSTR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
