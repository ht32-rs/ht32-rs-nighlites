#[doc = "Register `AFIO_GPACFGR` reader"]
pub struct R(crate::R<AFIO_GPACFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_GPACFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_GPACFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_GPACFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_GPACFGR` writer"]
pub struct W(crate::W<AFIO_GPACFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_GPACFGR_SPEC>;
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
impl From<crate::W<AFIO_GPACFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_GPACFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PACFG0` reader - PACFG0"]
pub type PACFG0_R = crate::FieldReader;
#[doc = "Field `PACFG0` writer - PACFG0"]
pub type PACFG0_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGR_SPEC, 2, O>;
#[doc = "Field `PACFG1` reader - PACFG1"]
pub type PACFG1_R = crate::FieldReader;
#[doc = "Field `PACFG1` writer - PACFG1"]
pub type PACFG1_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGR_SPEC, 2, O>;
#[doc = "Field `PACFG2` reader - PACFG2"]
pub type PACFG2_R = crate::FieldReader;
#[doc = "Field `PACFG2` writer - PACFG2"]
pub type PACFG2_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGR_SPEC, 2, O>;
#[doc = "Field `PACFG3` reader - PACFG3"]
pub type PACFG3_R = crate::FieldReader;
#[doc = "Field `PACFG3` writer - PACFG3"]
pub type PACFG3_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGR_SPEC, 2, O>;
#[doc = "Field `PACFG4` reader - PACFG4"]
pub type PACFG4_R = crate::FieldReader;
#[doc = "Field `PACFG4` writer - PACFG4"]
pub type PACFG4_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGR_SPEC, 2, O>;
#[doc = "Field `PACFG5` reader - PACFG5"]
pub type PACFG5_R = crate::FieldReader;
#[doc = "Field `PACFG5` writer - PACFG5"]
pub type PACFG5_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGR_SPEC, 2, O>;
#[doc = "Field `PACFG6` reader - PACFG6"]
pub type PACFG6_R = crate::FieldReader;
#[doc = "Field `PACFG6` writer - PACFG6"]
pub type PACFG6_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGR_SPEC, 2, O>;
#[doc = "Field `PACFG7` reader - PACFG7"]
pub type PACFG7_R = crate::FieldReader;
#[doc = "Field `PACFG7` writer - PACFG7"]
pub type PACFG7_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGR_SPEC, 2, O>;
#[doc = "Field `PACFG8` reader - PACFG8"]
pub type PACFG8_R = crate::FieldReader;
#[doc = "Field `PACFG8` writer - PACFG8"]
pub type PACFG8_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGR_SPEC, 2, O>;
#[doc = "Field `PACFG9` reader - PACFG9"]
pub type PACFG9_R = crate::FieldReader;
#[doc = "Field `PACFG9` writer - PACFG9"]
pub type PACFG9_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGR_SPEC, 2, O>;
#[doc = "Field `PACFG10` reader - PACFG10"]
pub type PACFG10_R = crate::FieldReader;
#[doc = "Field `PACFG10` writer - PACFG10"]
pub type PACFG10_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGR_SPEC, 2, O>;
#[doc = "Field `PACFG11` reader - PACFG11"]
pub type PACFG11_R = crate::FieldReader;
#[doc = "Field `PACFG11` writer - PACFG11"]
pub type PACFG11_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGR_SPEC, 2, O>;
#[doc = "Field `PACFG12` reader - PACFG12"]
pub type PACFG12_R = crate::FieldReader;
#[doc = "Field `PACFG12` writer - PACFG12"]
pub type PACFG12_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGR_SPEC, 2, O>;
#[doc = "Field `PACFG13` reader - PACFG13"]
pub type PACFG13_R = crate::FieldReader;
#[doc = "Field `PACFG13` writer - PACFG13"]
pub type PACFG13_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGR_SPEC, 2, O>;
#[doc = "Field `PACFG14` reader - PACFG14"]
pub type PACFG14_R = crate::FieldReader;
#[doc = "Field `PACFG14` writer - PACFG14"]
pub type PACFG14_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGR_SPEC, 2, O>;
#[doc = "Field `PACFG15` reader - PACFG15"]
pub type PACFG15_R = crate::FieldReader;
#[doc = "Field `PACFG15` writer - PACFG15"]
pub type PACFG15_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - PACFG0"]
    #[inline(always)]
    pub fn pacfg0(&self) -> PACFG0_R {
        PACFG0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PACFG1"]
    #[inline(always)]
    pub fn pacfg1(&self) -> PACFG1_R {
        PACFG1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PACFG2"]
    #[inline(always)]
    pub fn pacfg2(&self) -> PACFG2_R {
        PACFG2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PACFG3"]
    #[inline(always)]
    pub fn pacfg3(&self) -> PACFG3_R {
        PACFG3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PACFG4"]
    #[inline(always)]
    pub fn pacfg4(&self) -> PACFG4_R {
        PACFG4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PACFG5"]
    #[inline(always)]
    pub fn pacfg5(&self) -> PACFG5_R {
        PACFG5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PACFG6"]
    #[inline(always)]
    pub fn pacfg6(&self) -> PACFG6_R {
        PACFG6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PACFG7"]
    #[inline(always)]
    pub fn pacfg7(&self) -> PACFG7_R {
        PACFG7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PACFG8"]
    #[inline(always)]
    pub fn pacfg8(&self) -> PACFG8_R {
        PACFG8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PACFG9"]
    #[inline(always)]
    pub fn pacfg9(&self) -> PACFG9_R {
        PACFG9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PACFG10"]
    #[inline(always)]
    pub fn pacfg10(&self) -> PACFG10_R {
        PACFG10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PACFG11"]
    #[inline(always)]
    pub fn pacfg11(&self) -> PACFG11_R {
        PACFG11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PACFG12"]
    #[inline(always)]
    pub fn pacfg12(&self) -> PACFG12_R {
        PACFG12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PACFG13"]
    #[inline(always)]
    pub fn pacfg13(&self) -> PACFG13_R {
        PACFG13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PACFG14"]
    #[inline(always)]
    pub fn pacfg14(&self) -> PACFG14_R {
        PACFG14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PACFG15"]
    #[inline(always)]
    pub fn pacfg15(&self) -> PACFG15_R {
        PACFG15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PACFG0"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg0(&mut self) -> PACFG0_W<0> {
        PACFG0_W::new(self)
    }
    #[doc = "Bits 2:3 - PACFG1"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg1(&mut self) -> PACFG1_W<2> {
        PACFG1_W::new(self)
    }
    #[doc = "Bits 4:5 - PACFG2"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg2(&mut self) -> PACFG2_W<4> {
        PACFG2_W::new(self)
    }
    #[doc = "Bits 6:7 - PACFG3"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg3(&mut self) -> PACFG3_W<6> {
        PACFG3_W::new(self)
    }
    #[doc = "Bits 8:9 - PACFG4"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg4(&mut self) -> PACFG4_W<8> {
        PACFG4_W::new(self)
    }
    #[doc = "Bits 10:11 - PACFG5"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg5(&mut self) -> PACFG5_W<10> {
        PACFG5_W::new(self)
    }
    #[doc = "Bits 12:13 - PACFG6"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg6(&mut self) -> PACFG6_W<12> {
        PACFG6_W::new(self)
    }
    #[doc = "Bits 14:15 - PACFG7"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg7(&mut self) -> PACFG7_W<14> {
        PACFG7_W::new(self)
    }
    #[doc = "Bits 16:17 - PACFG8"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg8(&mut self) -> PACFG8_W<16> {
        PACFG8_W::new(self)
    }
    #[doc = "Bits 18:19 - PACFG9"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg9(&mut self) -> PACFG9_W<18> {
        PACFG9_W::new(self)
    }
    #[doc = "Bits 20:21 - PACFG10"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg10(&mut self) -> PACFG10_W<20> {
        PACFG10_W::new(self)
    }
    #[doc = "Bits 22:23 - PACFG11"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg11(&mut self) -> PACFG11_W<22> {
        PACFG11_W::new(self)
    }
    #[doc = "Bits 24:25 - PACFG12"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg12(&mut self) -> PACFG12_W<24> {
        PACFG12_W::new(self)
    }
    #[doc = "Bits 26:27 - PACFG13"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg13(&mut self) -> PACFG13_W<26> {
        PACFG13_W::new(self)
    }
    #[doc = "Bits 28:29 - PACFG14"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg14(&mut self) -> PACFG14_W<28> {
        PACFG14_W::new(self)
    }
    #[doc = "Bits 30:31 - PACFG15"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg15(&mut self) -> PACFG15_W<30> {
        PACFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_GPACFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpacfgr](index.html) module"]
pub struct AFIO_GPACFGR_SPEC;
impl crate::RegisterSpec for AFIO_GPACFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_gpacfgr::R](R) reader structure"]
impl crate::Readable for AFIO_GPACFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_gpacfgr::W](W) writer structure"]
impl crate::Writable for AFIO_GPACFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFIO_GPACFGR to value 0"]
impl crate::Resettable for AFIO_GPACFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
