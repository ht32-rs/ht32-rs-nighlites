#[doc = "Register `AFIO_GPECFGLR` reader"]
pub struct R(crate::R<AFIO_GPECFGLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_GPECFGLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_GPECFGLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_GPECFGLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_GPECFGLR` writer"]
pub struct W(crate::W<AFIO_GPECFGLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_GPECFGLR_SPEC>;
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
impl From<crate::W<AFIO_GPECFGLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_GPECFGLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PECFG0` reader - PECFG0"]
pub type PECFG0_R = crate::FieldReader;
#[doc = "Field `PECFG0` writer - PECFG0"]
pub type PECFG0_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGLR_SPEC, 4, O>;
#[doc = "Field `PECFG1` reader - PECFG1"]
pub type PECFG1_R = crate::FieldReader;
#[doc = "Field `PECFG1` writer - PECFG1"]
pub type PECFG1_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGLR_SPEC, 4, O>;
#[doc = "Field `PECFG2` reader - PECFG2"]
pub type PECFG2_R = crate::FieldReader;
#[doc = "Field `PECFG2` writer - PECFG2"]
pub type PECFG2_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGLR_SPEC, 4, O>;
#[doc = "Field `PECFG3` reader - PECFG3"]
pub type PECFG3_R = crate::FieldReader;
#[doc = "Field `PECFG3` writer - PECFG3"]
pub type PECFG3_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGLR_SPEC, 4, O>;
#[doc = "Field `PECFG4` reader - PECFG4"]
pub type PECFG4_R = crate::FieldReader;
#[doc = "Field `PECFG4` writer - PECFG4"]
pub type PECFG4_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGLR_SPEC, 4, O>;
#[doc = "Field `PECFG5` reader - PECFG5"]
pub type PECFG5_R = crate::FieldReader;
#[doc = "Field `PECFG5` writer - PECFG5"]
pub type PECFG5_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGLR_SPEC, 4, O>;
#[doc = "Field `PECFG6` reader - PECFG6"]
pub type PECFG6_R = crate::FieldReader;
#[doc = "Field `PECFG6` writer - PECFG6"]
pub type PECFG6_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGLR_SPEC, 4, O>;
#[doc = "Field `PECFG7` reader - PECFG7"]
pub type PECFG7_R = crate::FieldReader;
#[doc = "Field `PECFG7` writer - PECFG7"]
pub type PECFG7_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGLR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PECFG0"]
    #[inline(always)]
    pub fn pecfg0(&self) -> PECFG0_R {
        PECFG0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PECFG1"]
    #[inline(always)]
    pub fn pecfg1(&self) -> PECFG1_R {
        PECFG1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PECFG2"]
    #[inline(always)]
    pub fn pecfg2(&self) -> PECFG2_R {
        PECFG2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PECFG3"]
    #[inline(always)]
    pub fn pecfg3(&self) -> PECFG3_R {
        PECFG3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PECFG4"]
    #[inline(always)]
    pub fn pecfg4(&self) -> PECFG4_R {
        PECFG4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PECFG5"]
    #[inline(always)]
    pub fn pecfg5(&self) -> PECFG5_R {
        PECFG5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PECFG6"]
    #[inline(always)]
    pub fn pecfg6(&self) -> PECFG6_R {
        PECFG6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PECFG7"]
    #[inline(always)]
    pub fn pecfg7(&self) -> PECFG7_R {
        PECFG7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PECFG0"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg0(&mut self) -> PECFG0_W<0> {
        PECFG0_W::new(self)
    }
    #[doc = "Bits 4:7 - PECFG1"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg1(&mut self) -> PECFG1_W<4> {
        PECFG1_W::new(self)
    }
    #[doc = "Bits 8:11 - PECFG2"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg2(&mut self) -> PECFG2_W<8> {
        PECFG2_W::new(self)
    }
    #[doc = "Bits 12:15 - PECFG3"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg3(&mut self) -> PECFG3_W<12> {
        PECFG3_W::new(self)
    }
    #[doc = "Bits 16:19 - PECFG4"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg4(&mut self) -> PECFG4_W<16> {
        PECFG4_W::new(self)
    }
    #[doc = "Bits 20:23 - PECFG5"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg5(&mut self) -> PECFG5_W<20> {
        PECFG5_W::new(self)
    }
    #[doc = "Bits 24:27 - PECFG6"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg6(&mut self) -> PECFG6_W<24> {
        PECFG6_W::new(self)
    }
    #[doc = "Bits 28:31 - PECFG7"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg7(&mut self) -> PECFG7_W<28> {
        PECFG7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_GPECFGLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpecfglr](index.html) module"]
pub struct AFIO_GPECFGLR_SPEC;
impl crate::RegisterSpec for AFIO_GPECFGLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_gpecfglr::R](R) reader structure"]
impl crate::Readable for AFIO_GPECFGLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_gpecfglr::W](W) writer structure"]
impl crate::Writable for AFIO_GPECFGLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFIO_GPECFGLR to value 0"]
impl crate::Resettable for AFIO_GPECFGLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
