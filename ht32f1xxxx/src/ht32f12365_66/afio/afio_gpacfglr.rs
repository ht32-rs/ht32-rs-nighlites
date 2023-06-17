#[doc = "Register `AFIO_GPACFGLR` reader"]
pub struct R(crate::R<AFIO_GPACFGLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_GPACFGLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_GPACFGLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_GPACFGLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_GPACFGLR` writer"]
pub struct W(crate::W<AFIO_GPACFGLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_GPACFGLR_SPEC>;
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
impl From<crate::W<AFIO_GPACFGLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_GPACFGLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PACFG0` reader - PACFG0"]
pub type PACFG0_R = crate::FieldReader;
#[doc = "Field `PACFG0` writer - PACFG0"]
pub type PACFG0_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGLR_SPEC, 4, O>;
#[doc = "Field `PACFG1` reader - PACFG1"]
pub type PACFG1_R = crate::FieldReader;
#[doc = "Field `PACFG1` writer - PACFG1"]
pub type PACFG1_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGLR_SPEC, 4, O>;
#[doc = "Field `PACFG2` reader - PACFG2"]
pub type PACFG2_R = crate::FieldReader;
#[doc = "Field `PACFG2` writer - PACFG2"]
pub type PACFG2_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGLR_SPEC, 4, O>;
#[doc = "Field `PACFG3` reader - PACFG3"]
pub type PACFG3_R = crate::FieldReader;
#[doc = "Field `PACFG3` writer - PACFG3"]
pub type PACFG3_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGLR_SPEC, 4, O>;
#[doc = "Field `PACFG4` reader - PACFG4"]
pub type PACFG4_R = crate::FieldReader;
#[doc = "Field `PACFG4` writer - PACFG4"]
pub type PACFG4_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGLR_SPEC, 4, O>;
#[doc = "Field `PACFG5` reader - PACFG5"]
pub type PACFG5_R = crate::FieldReader;
#[doc = "Field `PACFG5` writer - PACFG5"]
pub type PACFG5_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGLR_SPEC, 4, O>;
#[doc = "Field `PACFG6` reader - PACFG6"]
pub type PACFG6_R = crate::FieldReader;
#[doc = "Field `PACFG6` writer - PACFG6"]
pub type PACFG6_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGLR_SPEC, 4, O>;
#[doc = "Field `PACFG7` reader - PACFG7"]
pub type PACFG7_R = crate::FieldReader;
#[doc = "Field `PACFG7` writer - PACFG7"]
pub type PACFG7_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGLR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PACFG0"]
    #[inline(always)]
    pub fn pacfg0(&self) -> PACFG0_R {
        PACFG0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PACFG1"]
    #[inline(always)]
    pub fn pacfg1(&self) -> PACFG1_R {
        PACFG1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PACFG2"]
    #[inline(always)]
    pub fn pacfg2(&self) -> PACFG2_R {
        PACFG2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PACFG3"]
    #[inline(always)]
    pub fn pacfg3(&self) -> PACFG3_R {
        PACFG3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PACFG4"]
    #[inline(always)]
    pub fn pacfg4(&self) -> PACFG4_R {
        PACFG4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PACFG5"]
    #[inline(always)]
    pub fn pacfg5(&self) -> PACFG5_R {
        PACFG5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PACFG6"]
    #[inline(always)]
    pub fn pacfg6(&self) -> PACFG6_R {
        PACFG6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PACFG7"]
    #[inline(always)]
    pub fn pacfg7(&self) -> PACFG7_R {
        PACFG7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PACFG0"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg0(&mut self) -> PACFG0_W<0> {
        PACFG0_W::new(self)
    }
    #[doc = "Bits 4:7 - PACFG1"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg1(&mut self) -> PACFG1_W<4> {
        PACFG1_W::new(self)
    }
    #[doc = "Bits 8:11 - PACFG2"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg2(&mut self) -> PACFG2_W<8> {
        PACFG2_W::new(self)
    }
    #[doc = "Bits 12:15 - PACFG3"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg3(&mut self) -> PACFG3_W<12> {
        PACFG3_W::new(self)
    }
    #[doc = "Bits 16:19 - PACFG4"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg4(&mut self) -> PACFG4_W<16> {
        PACFG4_W::new(self)
    }
    #[doc = "Bits 20:23 - PACFG5"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg5(&mut self) -> PACFG5_W<20> {
        PACFG5_W::new(self)
    }
    #[doc = "Bits 24:27 - PACFG6"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg6(&mut self) -> PACFG6_W<24> {
        PACFG6_W::new(self)
    }
    #[doc = "Bits 28:31 - PACFG7"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg7(&mut self) -> PACFG7_W<28> {
        PACFG7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_GPACFGLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpacfglr](index.html) module"]
pub struct AFIO_GPACFGLR_SPEC;
impl crate::RegisterSpec for AFIO_GPACFGLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_gpacfglr::R](R) reader structure"]
impl crate::Readable for AFIO_GPACFGLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_gpacfglr::W](W) writer structure"]
impl crate::Writable for AFIO_GPACFGLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFIO_GPACFGLR to value 0"]
impl crate::Resettable for AFIO_GPACFGLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
