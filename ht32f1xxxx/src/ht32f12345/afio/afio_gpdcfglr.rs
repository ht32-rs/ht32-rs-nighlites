#[doc = "Register `AFIO_GPDCFGLR` reader"]
pub struct R(crate::R<AFIO_GPDCFGLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_GPDCFGLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_GPDCFGLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_GPDCFGLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_GPDCFGLR` writer"]
pub struct W(crate::W<AFIO_GPDCFGLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_GPDCFGLR_SPEC>;
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
impl From<crate::W<AFIO_GPDCFGLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_GPDCFGLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDCFG0` reader - PDCFG0"]
pub type PDCFG0_R = crate::FieldReader;
#[doc = "Field `PDCFG0` writer - PDCFG0"]
pub type PDCFG0_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGLR_SPEC, 4, O>;
#[doc = "Field `PDCFG1` reader - PDCFG1"]
pub type PDCFG1_R = crate::FieldReader;
#[doc = "Field `PDCFG1` writer - PDCFG1"]
pub type PDCFG1_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGLR_SPEC, 4, O>;
#[doc = "Field `PDCFG2` reader - PDCFG2"]
pub type PDCFG2_R = crate::FieldReader;
#[doc = "Field `PDCFG2` writer - PDCFG2"]
pub type PDCFG2_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGLR_SPEC, 4, O>;
#[doc = "Field `PDCFG3` reader - PDCFG3"]
pub type PDCFG3_R = crate::FieldReader;
#[doc = "Field `PDCFG3` writer - PDCFG3"]
pub type PDCFG3_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGLR_SPEC, 4, O>;
#[doc = "Field `PDCFG4` reader - PDCFG4"]
pub type PDCFG4_R = crate::FieldReader;
#[doc = "Field `PDCFG4` writer - PDCFG4"]
pub type PDCFG4_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGLR_SPEC, 4, O>;
#[doc = "Field `PDCFG5` reader - PDCFG5"]
pub type PDCFG5_R = crate::FieldReader;
#[doc = "Field `PDCFG5` writer - PDCFG5"]
pub type PDCFG5_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGLR_SPEC, 4, O>;
#[doc = "Field `PDCFG6` reader - PDCFG6"]
pub type PDCFG6_R = crate::FieldReader;
#[doc = "Field `PDCFG6` writer - PDCFG6"]
pub type PDCFG6_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGLR_SPEC, 4, O>;
#[doc = "Field `PDCFG7` reader - PDCFG7"]
pub type PDCFG7_R = crate::FieldReader;
#[doc = "Field `PDCFG7` writer - PDCFG7"]
pub type PDCFG7_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGLR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PDCFG0"]
    #[inline(always)]
    pub fn pdcfg0(&self) -> PDCFG0_R {
        PDCFG0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PDCFG1"]
    #[inline(always)]
    pub fn pdcfg1(&self) -> PDCFG1_R {
        PDCFG1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PDCFG2"]
    #[inline(always)]
    pub fn pdcfg2(&self) -> PDCFG2_R {
        PDCFG2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PDCFG3"]
    #[inline(always)]
    pub fn pdcfg3(&self) -> PDCFG3_R {
        PDCFG3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PDCFG4"]
    #[inline(always)]
    pub fn pdcfg4(&self) -> PDCFG4_R {
        PDCFG4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PDCFG5"]
    #[inline(always)]
    pub fn pdcfg5(&self) -> PDCFG5_R {
        PDCFG5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PDCFG6"]
    #[inline(always)]
    pub fn pdcfg6(&self) -> PDCFG6_R {
        PDCFG6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PDCFG7"]
    #[inline(always)]
    pub fn pdcfg7(&self) -> PDCFG7_R {
        PDCFG7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDCFG0"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg0(&mut self) -> PDCFG0_W<0> {
        PDCFG0_W::new(self)
    }
    #[doc = "Bits 4:7 - PDCFG1"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg1(&mut self) -> PDCFG1_W<4> {
        PDCFG1_W::new(self)
    }
    #[doc = "Bits 8:11 - PDCFG2"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg2(&mut self) -> PDCFG2_W<8> {
        PDCFG2_W::new(self)
    }
    #[doc = "Bits 12:15 - PDCFG3"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg3(&mut self) -> PDCFG3_W<12> {
        PDCFG3_W::new(self)
    }
    #[doc = "Bits 16:19 - PDCFG4"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg4(&mut self) -> PDCFG4_W<16> {
        PDCFG4_W::new(self)
    }
    #[doc = "Bits 20:23 - PDCFG5"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg5(&mut self) -> PDCFG5_W<20> {
        PDCFG5_W::new(self)
    }
    #[doc = "Bits 24:27 - PDCFG6"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg6(&mut self) -> PDCFG6_W<24> {
        PDCFG6_W::new(self)
    }
    #[doc = "Bits 28:31 - PDCFG7"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg7(&mut self) -> PDCFG7_W<28> {
        PDCFG7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_GPDCFGLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpdcfglr](index.html) module"]
pub struct AFIO_GPDCFGLR_SPEC;
impl crate::RegisterSpec for AFIO_GPDCFGLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_gpdcfglr::R](R) reader structure"]
impl crate::Readable for AFIO_GPDCFGLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_gpdcfglr::W](W) writer structure"]
impl crate::Writable for AFIO_GPDCFGLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFIO_GPDCFGLR to value 0"]
impl crate::Resettable for AFIO_GPDCFGLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
