#[doc = "Register `AFIO_GPBCFGLR` reader"]
pub struct R(crate::R<AFIO_GPBCFGLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_GPBCFGLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_GPBCFGLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_GPBCFGLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_GPBCFGLR` writer"]
pub struct W(crate::W<AFIO_GPBCFGLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_GPBCFGLR_SPEC>;
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
impl From<crate::W<AFIO_GPBCFGLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_GPBCFGLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PxCFG0` reader - PxCFG0"]
pub type PX_CFG0_R = crate::FieldReader;
#[doc = "Field `PxCFG0` writer - PxCFG0"]
pub type PX_CFG0_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGLR_SPEC, 4, O>;
#[doc = "Field `PxCFG1` reader - PxCFG1"]
pub type PX_CFG1_R = crate::FieldReader;
#[doc = "Field `PxCFG1` writer - PxCFG1"]
pub type PX_CFG1_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGLR_SPEC, 4, O>;
#[doc = "Field `PxCFG2` reader - PxCFG2"]
pub type PX_CFG2_R = crate::FieldReader;
#[doc = "Field `PxCFG2` writer - PxCFG2"]
pub type PX_CFG2_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGLR_SPEC, 4, O>;
#[doc = "Field `PxCFG3` reader - PxCFG3"]
pub type PX_CFG3_R = crate::FieldReader;
#[doc = "Field `PxCFG3` writer - PxCFG3"]
pub type PX_CFG3_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGLR_SPEC, 4, O>;
#[doc = "Field `PxCFG4` reader - PxCFG4"]
pub type PX_CFG4_R = crate::FieldReader;
#[doc = "Field `PxCFG4` writer - PxCFG4"]
pub type PX_CFG4_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGLR_SPEC, 4, O>;
#[doc = "Field `PxCFG5` reader - PxCFG5"]
pub type PX_CFG5_R = crate::FieldReader;
#[doc = "Field `PxCFG5` writer - PxCFG5"]
pub type PX_CFG5_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGLR_SPEC, 4, O>;
#[doc = "Field `PxCFG6` reader - PxCFG6"]
pub type PX_CFG6_R = crate::FieldReader;
#[doc = "Field `PxCFG6` writer - PxCFG6"]
pub type PX_CFG6_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGLR_SPEC, 4, O>;
#[doc = "Field `PxCFG7` reader - PxCFG7"]
pub type PX_CFG7_R = crate::FieldReader;
#[doc = "Field `PxCFG7` writer - PxCFG7"]
pub type PX_CFG7_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGLR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PxCFG0"]
    #[inline(always)]
    pub fn px_cfg0(&self) -> PX_CFG0_R {
        PX_CFG0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PxCFG1"]
    #[inline(always)]
    pub fn px_cfg1(&self) -> PX_CFG1_R {
        PX_CFG1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PxCFG2"]
    #[inline(always)]
    pub fn px_cfg2(&self) -> PX_CFG2_R {
        PX_CFG2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PxCFG3"]
    #[inline(always)]
    pub fn px_cfg3(&self) -> PX_CFG3_R {
        PX_CFG3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PxCFG4"]
    #[inline(always)]
    pub fn px_cfg4(&self) -> PX_CFG4_R {
        PX_CFG4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PxCFG5"]
    #[inline(always)]
    pub fn px_cfg5(&self) -> PX_CFG5_R {
        PX_CFG5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PxCFG6"]
    #[inline(always)]
    pub fn px_cfg6(&self) -> PX_CFG6_R {
        PX_CFG6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PxCFG7"]
    #[inline(always)]
    pub fn px_cfg7(&self) -> PX_CFG7_R {
        PX_CFG7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PxCFG0"]
    #[inline(always)]
    #[must_use]
    pub fn px_cfg0(&mut self) -> PX_CFG0_W<0> {
        PX_CFG0_W::new(self)
    }
    #[doc = "Bits 4:7 - PxCFG1"]
    #[inline(always)]
    #[must_use]
    pub fn px_cfg1(&mut self) -> PX_CFG1_W<4> {
        PX_CFG1_W::new(self)
    }
    #[doc = "Bits 8:11 - PxCFG2"]
    #[inline(always)]
    #[must_use]
    pub fn px_cfg2(&mut self) -> PX_CFG2_W<8> {
        PX_CFG2_W::new(self)
    }
    #[doc = "Bits 12:15 - PxCFG3"]
    #[inline(always)]
    #[must_use]
    pub fn px_cfg3(&mut self) -> PX_CFG3_W<12> {
        PX_CFG3_W::new(self)
    }
    #[doc = "Bits 16:19 - PxCFG4"]
    #[inline(always)]
    #[must_use]
    pub fn px_cfg4(&mut self) -> PX_CFG4_W<16> {
        PX_CFG4_W::new(self)
    }
    #[doc = "Bits 20:23 - PxCFG5"]
    #[inline(always)]
    #[must_use]
    pub fn px_cfg5(&mut self) -> PX_CFG5_W<20> {
        PX_CFG5_W::new(self)
    }
    #[doc = "Bits 24:27 - PxCFG6"]
    #[inline(always)]
    #[must_use]
    pub fn px_cfg6(&mut self) -> PX_CFG6_W<24> {
        PX_CFG6_W::new(self)
    }
    #[doc = "Bits 28:31 - PxCFG7"]
    #[inline(always)]
    #[must_use]
    pub fn px_cfg7(&mut self) -> PX_CFG7_W<28> {
        PX_CFG7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_GPBCFGLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpbcfglr](index.html) module"]
pub struct AFIO_GPBCFGLR_SPEC;
impl crate::RegisterSpec for AFIO_GPBCFGLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_gpbcfglr::R](R) reader structure"]
impl crate::Readable for AFIO_GPBCFGLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_gpbcfglr::W](W) writer structure"]
impl crate::Writable for AFIO_GPBCFGLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFIO_GPBCFGLR to value 0"]
impl crate::Resettable for AFIO_GPBCFGLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
