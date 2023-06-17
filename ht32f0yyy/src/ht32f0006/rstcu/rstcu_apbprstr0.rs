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
#[doc = "Field `USRRST` reader - USRRST"]
pub type USRRST_R = crate::BitReader;
#[doc = "Field `USRRST` writer - USRRST"]
pub type USRRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
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
#[doc = "Field `I2SRST` reader - I2SRST"]
pub type I2SRST_R = crate::BitReader;
#[doc = "Field `I2SRST` writer - I2SRST"]
pub type I2SRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
#[doc = "Field `MIDIRST` reader - MIDIRST"]
pub type MIDIRST_R = crate::BitReader;
#[doc = "Field `MIDIRST` writer - MIDIRST"]
pub type MIDIRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR0_SPEC, O>;
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
    #[doc = "Bit 8 - USRRST"]
    #[inline(always)]
    pub fn usrrst(&self) -> USRRST_R {
        USRRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - URRST"]
    #[inline(always)]
    pub fn urrst(&self) -> URRST_R {
        URRST_R::new(((self.bits >> 10) & 1) != 0)
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
    #[doc = "Bit 25 - I2SRST"]
    #[inline(always)]
    pub fn i2srst(&self) -> I2SRST_R {
        I2SRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - MIDIRST"]
    #[inline(always)]
    pub fn midirst(&self) -> MIDIRST_R {
        MIDIRST_R::new(((self.bits >> 28) & 1) != 0)
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
    #[doc = "Bit 8 - USRRST"]
    #[inline(always)]
    #[must_use]
    pub fn usrrst(&mut self) -> USRRST_W<8> {
        USRRST_W::new(self)
    }
    #[doc = "Bit 10 - URRST"]
    #[inline(always)]
    #[must_use]
    pub fn urrst(&mut self) -> URRST_W<10> {
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
    #[doc = "Bit 25 - I2SRST"]
    #[inline(always)]
    #[must_use]
    pub fn i2srst(&mut self) -> I2SRST_W<25> {
        I2SRST_W::new(self)
    }
    #[doc = "Bit 28 - MIDIRST"]
    #[inline(always)]
    #[must_use]
    pub fn midirst(&mut self) -> MIDIRST_W<28> {
        MIDIRST_W::new(self)
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
