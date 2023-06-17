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
#[doc = "Field `PBCFG0` reader - PBCFG0"]
pub type PBCFG0_R = crate::FieldReader;
#[doc = "Field `PBCFG0` writer - PBCFG0"]
pub type PBCFG0_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGLR_SPEC, 4, O>;
#[doc = "Field `PBCFG1` reader - PBCFG1"]
pub type PBCFG1_R = crate::FieldReader;
#[doc = "Field `PBCFG1` writer - PBCFG1"]
pub type PBCFG1_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGLR_SPEC, 4, O>;
#[doc = "Field `PBCFG2` reader - PBCFG2"]
pub type PBCFG2_R = crate::FieldReader;
#[doc = "Field `PBCFG2` writer - PBCFG2"]
pub type PBCFG2_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGLR_SPEC, 4, O>;
#[doc = "Field `PBCFG3` reader - PBCFG3"]
pub type PBCFG3_R = crate::FieldReader;
#[doc = "Field `PBCFG3` writer - PBCFG3"]
pub type PBCFG3_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGLR_SPEC, 4, O>;
#[doc = "Field `PBCFG4` reader - PBCFG4"]
pub type PBCFG4_R = crate::FieldReader;
#[doc = "Field `PBCFG4` writer - PBCFG4"]
pub type PBCFG4_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGLR_SPEC, 4, O>;
#[doc = "Field `PBCFG5` reader - PBCFG5"]
pub type PBCFG5_R = crate::FieldReader;
#[doc = "Field `PBCFG5` writer - PBCFG5"]
pub type PBCFG5_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGLR_SPEC, 4, O>;
#[doc = "Field `PBCFG6` reader - PBCFG6"]
pub type PBCFG6_R = crate::FieldReader;
#[doc = "Field `PBCFG6` writer - PBCFG6"]
pub type PBCFG6_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGLR_SPEC, 4, O>;
#[doc = "Field `PBCFG7` reader - PBCFG7"]
pub type PBCFG7_R = crate::FieldReader;
#[doc = "Field `PBCFG7` writer - PBCFG7"]
pub type PBCFG7_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGLR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PBCFG0"]
    #[inline(always)]
    pub fn pbcfg0(&self) -> PBCFG0_R {
        PBCFG0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PBCFG1"]
    #[inline(always)]
    pub fn pbcfg1(&self) -> PBCFG1_R {
        PBCFG1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PBCFG2"]
    #[inline(always)]
    pub fn pbcfg2(&self) -> PBCFG2_R {
        PBCFG2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PBCFG3"]
    #[inline(always)]
    pub fn pbcfg3(&self) -> PBCFG3_R {
        PBCFG3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PBCFG4"]
    #[inline(always)]
    pub fn pbcfg4(&self) -> PBCFG4_R {
        PBCFG4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PBCFG5"]
    #[inline(always)]
    pub fn pbcfg5(&self) -> PBCFG5_R {
        PBCFG5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PBCFG6"]
    #[inline(always)]
    pub fn pbcfg6(&self) -> PBCFG6_R {
        PBCFG6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PBCFG7"]
    #[inline(always)]
    pub fn pbcfg7(&self) -> PBCFG7_R {
        PBCFG7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PBCFG0"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg0(&mut self) -> PBCFG0_W<0> {
        PBCFG0_W::new(self)
    }
    #[doc = "Bits 4:7 - PBCFG1"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg1(&mut self) -> PBCFG1_W<4> {
        PBCFG1_W::new(self)
    }
    #[doc = "Bits 8:11 - PBCFG2"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg2(&mut self) -> PBCFG2_W<8> {
        PBCFG2_W::new(self)
    }
    #[doc = "Bits 12:15 - PBCFG3"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg3(&mut self) -> PBCFG3_W<12> {
        PBCFG3_W::new(self)
    }
    #[doc = "Bits 16:19 - PBCFG4"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg4(&mut self) -> PBCFG4_W<16> {
        PBCFG4_W::new(self)
    }
    #[doc = "Bits 20:23 - PBCFG5"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg5(&mut self) -> PBCFG5_W<20> {
        PBCFG5_W::new(self)
    }
    #[doc = "Bits 24:27 - PBCFG6"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg6(&mut self) -> PBCFG6_W<24> {
        PBCFG6_W::new(self)
    }
    #[doc = "Bits 28:31 - PBCFG7"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg7(&mut self) -> PBCFG7_W<28> {
        PBCFG7_W::new(self)
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
