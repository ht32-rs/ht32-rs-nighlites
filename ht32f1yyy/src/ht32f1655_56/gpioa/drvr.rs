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
#[doc = "Field `DV0` reader - DV0"]
pub type DV0_R = crate::BitReader;
#[doc = "Field `DV0` writer - DV0"]
pub type DV0_W<'a, const O: u8> = crate::BitWriter<'a, DRVR_SPEC, O>;
#[doc = "Field `DV1` reader - DV1"]
pub type DV1_R = crate::BitReader;
#[doc = "Field `DV1` writer - DV1"]
pub type DV1_W<'a, const O: u8> = crate::BitWriter<'a, DRVR_SPEC, O>;
#[doc = "Field `DV2` reader - DV2"]
pub type DV2_R = crate::BitReader;
#[doc = "Field `DV2` writer - DV2"]
pub type DV2_W<'a, const O: u8> = crate::BitWriter<'a, DRVR_SPEC, O>;
#[doc = "Field `DV3` reader - DV3"]
pub type DV3_R = crate::BitReader;
#[doc = "Field `DV3` writer - DV3"]
pub type DV3_W<'a, const O: u8> = crate::BitWriter<'a, DRVR_SPEC, O>;
#[doc = "Field `DV4` reader - DV4"]
pub type DV4_R = crate::BitReader;
#[doc = "Field `DV4` writer - DV4"]
pub type DV4_W<'a, const O: u8> = crate::BitWriter<'a, DRVR_SPEC, O>;
#[doc = "Field `DV5` reader - DV5"]
pub type DV5_R = crate::BitReader;
#[doc = "Field `DV5` writer - DV5"]
pub type DV5_W<'a, const O: u8> = crate::BitWriter<'a, DRVR_SPEC, O>;
#[doc = "Field `DV6` reader - DV6"]
pub type DV6_R = crate::BitReader;
#[doc = "Field `DV6` writer - DV6"]
pub type DV6_W<'a, const O: u8> = crate::BitWriter<'a, DRVR_SPEC, O>;
#[doc = "Field `DV7` reader - DV7"]
pub type DV7_R = crate::BitReader;
#[doc = "Field `DV7` writer - DV7"]
pub type DV7_W<'a, const O: u8> = crate::BitWriter<'a, DRVR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - DV0"]
    #[inline(always)]
    pub fn dv0(&self) -> DV0_R {
        DV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DV1"]
    #[inline(always)]
    pub fn dv1(&self) -> DV1_R {
        DV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DV2"]
    #[inline(always)]
    pub fn dv2(&self) -> DV2_R {
        DV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DV3"]
    #[inline(always)]
    pub fn dv3(&self) -> DV3_R {
        DV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DV4"]
    #[inline(always)]
    pub fn dv4(&self) -> DV4_R {
        DV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DV5"]
    #[inline(always)]
    pub fn dv5(&self) -> DV5_R {
        DV5_R::new(((self.bits >> 5) & 1) != 0)
    }
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
}
impl W {
    #[doc = "Bit 0 - DV0"]
    #[inline(always)]
    #[must_use]
    pub fn dv0(&mut self) -> DV0_W<0> {
        DV0_W::new(self)
    }
    #[doc = "Bit 1 - DV1"]
    #[inline(always)]
    #[must_use]
    pub fn dv1(&mut self) -> DV1_W<1> {
        DV1_W::new(self)
    }
    #[doc = "Bit 2 - DV2"]
    #[inline(always)]
    #[must_use]
    pub fn dv2(&mut self) -> DV2_W<2> {
        DV2_W::new(self)
    }
    #[doc = "Bit 3 - DV3"]
    #[inline(always)]
    #[must_use]
    pub fn dv3(&mut self) -> DV3_W<3> {
        DV3_W::new(self)
    }
    #[doc = "Bit 4 - DV4"]
    #[inline(always)]
    #[must_use]
    pub fn dv4(&mut self) -> DV4_W<4> {
        DV4_W::new(self)
    }
    #[doc = "Bit 5 - DV5"]
    #[inline(always)]
    #[must_use]
    pub fn dv5(&mut self) -> DV5_W<5> {
        DV5_W::new(self)
    }
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOA_DRVR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drvr](index.html) module"]
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
