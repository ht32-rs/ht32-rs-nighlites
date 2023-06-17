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
#[doc = "Field `I2CRST` reader - I2CRST"]
pub type I2CRST_R = crate::BitReader;
#[doc = "Field `I2CRST` writer - I2CRST"]
pub type I2CRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
#[doc = "Field `SPIRST` reader - SPIRST"]
pub type SPIRST_R = crate::BitReader;
#[doc = "Field `SPIRST` writer - SPIRST"]
pub type SPIRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
#[doc = "Field `URRST` reader - URRST"]
pub type URRST_R = crate::BitReader;
#[doc = "Field `URRST` writer - URRST"]
pub type URRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
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
impl R {
    #[doc = "Bit 0 - I2CRST"]
    #[inline(always)]
    pub fn i2crst(&self) -> I2CRST_R {
        I2CRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - SPIRST"]
    #[inline(always)]
    pub fn spirst(&self) -> SPIRST_R {
        SPIRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - URRST"]
    #[inline(always)]
    pub fn urrst(&self) -> URRST_R {
        URRST_R::new(((self.bits >> 8) & 1) != 0)
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
}
impl W {
    #[doc = "Bit 0 - I2CRST"]
    #[inline(always)]
    #[must_use]
    pub fn i2crst(&mut self) -> I2CRST_W<0> {
        I2CRST_W::new(self)
    }
    #[doc = "Bit 4 - SPIRST"]
    #[inline(always)]
    #[must_use]
    pub fn spirst(&mut self) -> SPIRST_W<4> {
        SPIRST_W::new(self)
    }
    #[doc = "Bit 8 - URRST"]
    #[inline(always)]
    #[must_use]
    pub fn urrst(&mut self) -> URRST_W<8> {
        URRST_W::new(self)
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
