#[doc = "Register `DRVR` reader"]
pub struct R(crate::R<DRVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DRVR` writer"]
pub struct W(crate::W<DRVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRVR_SPEC>;
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
impl From<crate::W<DRVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DV6` reader - DV6"]
pub type DV6_R = crate::BitReader;
#[doc = "Field `DV6` writer - DV6"]
pub type DV6_W<'a, const O: u8> = crate::BitWriter<'a, DRVR_SPEC, O>;
#[doc = "Field `DV7` reader - DV7"]
pub type DV7_R = crate::BitReader;
#[doc = "Field `DV7` writer - DV7"]
pub type DV7_W<'a, const O: u8> = crate::BitWriter<'a, DRVR_SPEC, O>;
#[doc = "Field `DV8` reader - DV8"]
pub type DV8_R = crate::BitReader;
#[doc = "Field `DV8` writer - DV8"]
pub type DV8_W<'a, const O: u8> = crate::BitWriter<'a, DRVR_SPEC, O>;
#[doc = "Field `DV9` reader - DV9"]
pub type DV9_R = crate::BitReader;
#[doc = "Field `DV9` writer - DV9"]
pub type DV9_W<'a, const O: u8> = crate::BitWriter<'a, DRVR_SPEC, O>;
#[doc = "Field `DV10` reader - DV10"]
pub type DV10_R = crate::BitReader;
#[doc = "Field `DV10` writer - DV10"]
pub type DV10_W<'a, const O: u8> = crate::BitWriter<'a, DRVR_SPEC, O>;
#[doc = "Field `DV11` reader - DV11"]
pub type DV11_R = crate::BitReader;
#[doc = "Field `DV11` writer - DV11"]
pub type DV11_W<'a, const O: u8> = crate::BitWriter<'a, DRVR_SPEC, O>;
#[doc = "Field `DV14` reader - DV14"]
pub type DV14_R = crate::BitReader;
#[doc = "Field `DV14` writer - DV14"]
pub type DV14_W<'a, const O: u8> = crate::BitWriter<'a, DRVR_SPEC, O>;
#[doc = "Field `DV15` reader - DV15"]
pub type DV15_R = crate::BitReader;
#[doc = "Field `DV15` writer - DV15"]
pub type DV15_W<'a, const O: u8> = crate::BitWriter<'a, DRVR_SPEC, O>;
impl R {
    #[doc = "Bit 6 - DV6"]
    #[inline(always)]
    pub fn dv6(&self) -> DV6_R {
        DV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DV7"]
    #[inline(always)]
    pub fn dv7(&self) -> DV7_R {
        DV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DV8"]
    #[inline(always)]
    pub fn dv8(&self) -> DV8_R {
        DV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DV9"]
    #[inline(always)]
    pub fn dv9(&self) -> DV9_R {
        DV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DV10"]
    #[inline(always)]
    pub fn dv10(&self) -> DV10_R {
        DV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DV11"]
    #[inline(always)]
    pub fn dv11(&self) -> DV11_R {
        DV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - DV14"]
    #[inline(always)]
    pub fn dv14(&self) -> DV14_R {
        DV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DV15"]
    #[inline(always)]
    pub fn dv15(&self) -> DV15_R {
        DV15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - DV6"]
    #[inline(always)]
    #[must_use]
    pub fn dv6(&mut self) -> DV6_W<6> {
        DV6_W::new(self)
    }
    #[doc = "Bit 7 - DV7"]
    #[inline(always)]
    #[must_use]
    pub fn dv7(&mut self) -> DV7_W<7> {
        DV7_W::new(self)
    }
    #[doc = "Bit 8 - DV8"]
    #[inline(always)]
    #[must_use]
    pub fn dv8(&mut self) -> DV8_W<8> {
        DV8_W::new(self)
    }
    #[doc = "Bit 9 - DV9"]
    #[inline(always)]
    #[must_use]
    pub fn dv9(&mut self) -> DV9_W<9> {
        DV9_W::new(self)
    }
    #[doc = "Bit 10 - DV10"]
    #[inline(always)]
    #[must_use]
    pub fn dv10(&mut self) -> DV10_W<10> {
        DV10_W::new(self)
    }
    #[doc = "Bit 11 - DV11"]
    #[inline(always)]
    #[must_use]
    pub fn dv11(&mut self) -> DV11_W<11> {
        DV11_W::new(self)
    }
    #[doc = "Bit 14 - DV14"]
    #[inline(always)]
    #[must_use]
    pub fn dv14(&mut self) -> DV14_W<14> {
        DV14_W::new(self)
    }
    #[doc = "Bit 15 - DV15"]
    #[inline(always)]
    #[must_use]
    pub fn dv15(&mut self) -> DV15_W<15> {
        DV15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOB_DRVR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drvr](index.html) module"]
pub struct DRVR_SPEC;
impl crate::RegisterSpec for DRVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [drvr::R](R) reader structure"]
impl crate::Readable for DRVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [drvr::W](W) writer structure"]
impl crate::Writable for DRVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DRVR to value 0"]
impl crate::Resettable for DRVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
