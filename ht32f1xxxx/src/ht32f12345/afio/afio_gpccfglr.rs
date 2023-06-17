#[doc = "Register `AFIO_GPCCFGLR` reader"]
pub struct R(crate::R<AFIO_GPCCFGLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_GPCCFGLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_GPCCFGLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_GPCCFGLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_GPCCFGLR` writer"]
pub struct W(crate::W<AFIO_GPCCFGLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_GPCCFGLR_SPEC>;
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
impl From<crate::W<AFIO_GPCCFGLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_GPCCFGLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCCFG0` reader - PCCFG0"]
pub type PCCFG0_R = crate::FieldReader;
#[doc = "Field `PCCFG0` writer - PCCFG0"]
pub type PCCFG0_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGLR_SPEC, 4, O>;
#[doc = "Field `PCCFG1` reader - PCCFG1"]
pub type PCCFG1_R = crate::FieldReader;
#[doc = "Field `PCCFG1` writer - PCCFG1"]
pub type PCCFG1_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGLR_SPEC, 4, O>;
#[doc = "Field `PCCFG2` reader - PCCFG2"]
pub type PCCFG2_R = crate::FieldReader;
#[doc = "Field `PCCFG2` writer - PCCFG2"]
pub type PCCFG2_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGLR_SPEC, 4, O>;
#[doc = "Field `PCCFG3` reader - PCCFG3"]
pub type PCCFG3_R = crate::FieldReader;
#[doc = "Field `PCCFG3` writer - PCCFG3"]
pub type PCCFG3_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGLR_SPEC, 4, O>;
#[doc = "Field `PCCFG4` reader - PCCFG4"]
pub type PCCFG4_R = crate::FieldReader;
#[doc = "Field `PCCFG4` writer - PCCFG4"]
pub type PCCFG4_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGLR_SPEC, 4, O>;
#[doc = "Field `PCCFG5` reader - PCCFG5"]
pub type PCCFG5_R = crate::FieldReader;
#[doc = "Field `PCCFG5` writer - PCCFG5"]
pub type PCCFG5_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGLR_SPEC, 4, O>;
#[doc = "Field `PCCFG6` reader - PCCFG6"]
pub type PCCFG6_R = crate::FieldReader;
#[doc = "Field `PCCFG6` writer - PCCFG6"]
pub type PCCFG6_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGLR_SPEC, 4, O>;
#[doc = "Field `PCCFG7` reader - PCCFG7"]
pub type PCCFG7_R = crate::FieldReader;
#[doc = "Field `PCCFG7` writer - PCCFG7"]
pub type PCCFG7_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGLR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PCCFG0"]
    #[inline(always)]
    pub fn pccfg0(&self) -> PCCFG0_R {
        PCCFG0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PCCFG1"]
    #[inline(always)]
    pub fn pccfg1(&self) -> PCCFG1_R {
        PCCFG1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PCCFG2"]
    #[inline(always)]
    pub fn pccfg2(&self) -> PCCFG2_R {
        PCCFG2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PCCFG3"]
    #[inline(always)]
    pub fn pccfg3(&self) -> PCCFG3_R {
        PCCFG3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PCCFG4"]
    #[inline(always)]
    pub fn pccfg4(&self) -> PCCFG4_R {
        PCCFG4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PCCFG5"]
    #[inline(always)]
    pub fn pccfg5(&self) -> PCCFG5_R {
        PCCFG5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PCCFG6"]
    #[inline(always)]
    pub fn pccfg6(&self) -> PCCFG6_R {
        PCCFG6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PCCFG7"]
    #[inline(always)]
    pub fn pccfg7(&self) -> PCCFG7_R {
        PCCFG7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PCCFG0"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg0(&mut self) -> PCCFG0_W<0> {
        PCCFG0_W::new(self)
    }
    #[doc = "Bits 4:7 - PCCFG1"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg1(&mut self) -> PCCFG1_W<4> {
        PCCFG1_W::new(self)
    }
    #[doc = "Bits 8:11 - PCCFG2"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg2(&mut self) -> PCCFG2_W<8> {
        PCCFG2_W::new(self)
    }
    #[doc = "Bits 12:15 - PCCFG3"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg3(&mut self) -> PCCFG3_W<12> {
        PCCFG3_W::new(self)
    }
    #[doc = "Bits 16:19 - PCCFG4"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg4(&mut self) -> PCCFG4_W<16> {
        PCCFG4_W::new(self)
    }
    #[doc = "Bits 20:23 - PCCFG5"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg5(&mut self) -> PCCFG5_W<20> {
        PCCFG5_W::new(self)
    }
    #[doc = "Bits 24:27 - PCCFG6"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg6(&mut self) -> PCCFG6_W<24> {
        PCCFG6_W::new(self)
    }
    #[doc = "Bits 28:31 - PCCFG7"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg7(&mut self) -> PCCFG7_W<28> {
        PCCFG7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_GPCCFGLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpccfglr](index.html) module"]
pub struct AFIO_GPCCFGLR_SPEC;
impl crate::RegisterSpec for AFIO_GPCCFGLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_gpccfglr::R](R) reader structure"]
impl crate::Readable for AFIO_GPCCFGLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_gpccfglr::W](W) writer structure"]
impl crate::Writable for AFIO_GPCCFGLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFIO_GPCCFGLR to value 0"]
impl crate::Resettable for AFIO_GPCCFGLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
