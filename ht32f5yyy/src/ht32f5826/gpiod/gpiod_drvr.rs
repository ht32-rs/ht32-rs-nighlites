#[doc = "Register `GPIOD_DRVR` reader"]
pub struct R(crate::R<GPIOD_DRVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOD_DRVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOD_DRVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOD_DRVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOD_DRVR` writer"]
pub struct W(crate::W<GPIOD_DRVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOD_DRVR_SPEC>;
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
impl From<crate::W<GPIOD_DRVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOD_DRVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DV0` reader - DV0"]
pub type DV0_R = crate::FieldReader;
#[doc = "Field `DV0` writer - DV0"]
pub type DV0_W<'a, const O: u8> = crate::FieldWriter<'a, GPIOD_DRVR_SPEC, 2, O>;
#[doc = "Field `DV1` reader - DV1"]
pub type DV1_R = crate::FieldReader;
#[doc = "Field `DV1` writer - DV1"]
pub type DV1_W<'a, const O: u8> = crate::FieldWriter<'a, GPIOD_DRVR_SPEC, 2, O>;
#[doc = "Field `DV2` reader - DV2"]
pub type DV2_R = crate::FieldReader;
#[doc = "Field `DV2` writer - DV2"]
pub type DV2_W<'a, const O: u8> = crate::FieldWriter<'a, GPIOD_DRVR_SPEC, 2, O>;
#[doc = "Field `DV3` reader - DV3"]
pub type DV3_R = crate::FieldReader;
#[doc = "Field `DV3` writer - DV3"]
pub type DV3_W<'a, const O: u8> = crate::FieldWriter<'a, GPIOD_DRVR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - DV0"]
    #[inline(always)]
    pub fn dv0(&self) -> DV0_R {
        DV0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DV1"]
    #[inline(always)]
    pub fn dv1(&self) -> DV1_R {
        DV1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - DV2"]
    #[inline(always)]
    pub fn dv2(&self) -> DV2_R {
        DV2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - DV3"]
    #[inline(always)]
    pub fn dv3(&self) -> DV3_R {
        DV3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DV0"]
    #[inline(always)]
    #[must_use]
    pub fn dv0(&mut self) -> DV0_W<0> {
        DV0_W::new(self)
    }
    #[doc = "Bits 2:3 - DV1"]
    #[inline(always)]
    #[must_use]
    pub fn dv1(&mut self) -> DV1_W<2> {
        DV1_W::new(self)
    }
    #[doc = "Bits 4:5 - DV2"]
    #[inline(always)]
    #[must_use]
    pub fn dv2(&mut self) -> DV2_W<4> {
        DV2_W::new(self)
    }
    #[doc = "Bits 6:7 - DV3"]
    #[inline(always)]
    #[must_use]
    pub fn dv3(&mut self) -> DV3_W<6> {
        DV3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOD_DRVR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_drvr](index.html) module"]
pub struct GPIOD_DRVR_SPEC;
impl crate::RegisterSpec for GPIOD_DRVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiod_drvr::R](R) reader structure"]
impl crate::Readable for GPIOD_DRVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiod_drvr::W](W) writer structure"]
impl crate::Writable for GPIOD_DRVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOD_DRVR to value 0"]
impl crate::Resettable for GPIOD_DRVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
