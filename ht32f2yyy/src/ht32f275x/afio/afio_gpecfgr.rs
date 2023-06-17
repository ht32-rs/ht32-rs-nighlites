#[doc = "Register `AFIO_GPECFGR` reader"]
pub struct R(crate::R<AFIO_GPECFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_GPECFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_GPECFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_GPECFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_GPECFGR` writer"]
pub struct W(crate::W<AFIO_GPECFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_GPECFGR_SPEC>;
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
impl From<crate::W<AFIO_GPECFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_GPECFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PECFG0` reader - PECFG0"]
pub type PECFG0_R = crate::FieldReader;
#[doc = "Field `PECFG0` writer - PECFG0"]
pub type PECFG0_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGR_SPEC, 2, O>;
#[doc = "Field `PECFG1` reader - PECFG1"]
pub type PECFG1_R = crate::FieldReader;
#[doc = "Field `PECFG1` writer - PECFG1"]
pub type PECFG1_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGR_SPEC, 2, O>;
#[doc = "Field `PECFG2` reader - PECFG2"]
pub type PECFG2_R = crate::FieldReader;
#[doc = "Field `PECFG2` writer - PECFG2"]
pub type PECFG2_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGR_SPEC, 2, O>;
#[doc = "Field `PECFG3` reader - PECFG3"]
pub type PECFG3_R = crate::FieldReader;
#[doc = "Field `PECFG3` writer - PECFG3"]
pub type PECFG3_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGR_SPEC, 2, O>;
#[doc = "Field `PECFG4` reader - PECFG4"]
pub type PECFG4_R = crate::FieldReader;
#[doc = "Field `PECFG4` writer - PECFG4"]
pub type PECFG4_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGR_SPEC, 2, O>;
#[doc = "Field `PECFG5` reader - PECFG5"]
pub type PECFG5_R = crate::FieldReader;
#[doc = "Field `PECFG5` writer - PECFG5"]
pub type PECFG5_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGR_SPEC, 2, O>;
#[doc = "Field `PECFG6` reader - PECFG6"]
pub type PECFG6_R = crate::FieldReader;
#[doc = "Field `PECFG6` writer - PECFG6"]
pub type PECFG6_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGR_SPEC, 2, O>;
#[doc = "Field `PECFG7` reader - PECFG7"]
pub type PECFG7_R = crate::FieldReader;
#[doc = "Field `PECFG7` writer - PECFG7"]
pub type PECFG7_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGR_SPEC, 2, O>;
#[doc = "Field `PECFG8` reader - PECFG8"]
pub type PECFG8_R = crate::FieldReader;
#[doc = "Field `PECFG8` writer - PECFG8"]
pub type PECFG8_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGR_SPEC, 2, O>;
#[doc = "Field `PECFG9` reader - PECFG9"]
pub type PECFG9_R = crate::FieldReader;
#[doc = "Field `PECFG9` writer - PECFG9"]
pub type PECFG9_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGR_SPEC, 2, O>;
#[doc = "Field `PECFG10` reader - PECFG10"]
pub type PECFG10_R = crate::FieldReader;
#[doc = "Field `PECFG10` writer - PECFG10"]
pub type PECFG10_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGR_SPEC, 2, O>;
#[doc = "Field `PECFG11` reader - PECFG11"]
pub type PECFG11_R = crate::FieldReader;
#[doc = "Field `PECFG11` writer - PECFG11"]
pub type PECFG11_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGR_SPEC, 2, O>;
#[doc = "Field `PECFG12` reader - PECFG12"]
pub type PECFG12_R = crate::FieldReader;
#[doc = "Field `PECFG12` writer - PECFG12"]
pub type PECFG12_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGR_SPEC, 2, O>;
#[doc = "Field `PECFG13` reader - PECFG13"]
pub type PECFG13_R = crate::FieldReader;
#[doc = "Field `PECFG13` writer - PECFG13"]
pub type PECFG13_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGR_SPEC, 2, O>;
#[doc = "Field `PECFG14` reader - PECFG14"]
pub type PECFG14_R = crate::FieldReader;
#[doc = "Field `PECFG14` writer - PECFG14"]
pub type PECFG14_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGR_SPEC, 2, O>;
#[doc = "Field `PECFG15` reader - PECFG15"]
pub type PECFG15_R = crate::FieldReader;
#[doc = "Field `PECFG15` writer - PECFG15"]
pub type PECFG15_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - PECFG0"]
    #[inline(always)]
    pub fn pecfg0(&self) -> PECFG0_R {
        PECFG0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PECFG1"]
    #[inline(always)]
    pub fn pecfg1(&self) -> PECFG1_R {
        PECFG1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PECFG2"]
    #[inline(always)]
    pub fn pecfg2(&self) -> PECFG2_R {
        PECFG2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PECFG3"]
    #[inline(always)]
    pub fn pecfg3(&self) -> PECFG3_R {
        PECFG3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PECFG4"]
    #[inline(always)]
    pub fn pecfg4(&self) -> PECFG4_R {
        PECFG4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PECFG5"]
    #[inline(always)]
    pub fn pecfg5(&self) -> PECFG5_R {
        PECFG5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PECFG6"]
    #[inline(always)]
    pub fn pecfg6(&self) -> PECFG6_R {
        PECFG6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PECFG7"]
    #[inline(always)]
    pub fn pecfg7(&self) -> PECFG7_R {
        PECFG7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PECFG8"]
    #[inline(always)]
    pub fn pecfg8(&self) -> PECFG8_R {
        PECFG8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PECFG9"]
    #[inline(always)]
    pub fn pecfg9(&self) -> PECFG9_R {
        PECFG9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PECFG10"]
    #[inline(always)]
    pub fn pecfg10(&self) -> PECFG10_R {
        PECFG10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PECFG11"]
    #[inline(always)]
    pub fn pecfg11(&self) -> PECFG11_R {
        PECFG11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PECFG12"]
    #[inline(always)]
    pub fn pecfg12(&self) -> PECFG12_R {
        PECFG12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PECFG13"]
    #[inline(always)]
    pub fn pecfg13(&self) -> PECFG13_R {
        PECFG13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PECFG14"]
    #[inline(always)]
    pub fn pecfg14(&self) -> PECFG14_R {
        PECFG14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PECFG15"]
    #[inline(always)]
    pub fn pecfg15(&self) -> PECFG15_R {
        PECFG15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PECFG0"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg0(&mut self) -> PECFG0_W<0> {
        PECFG0_W::new(self)
    }
    #[doc = "Bits 2:3 - PECFG1"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg1(&mut self) -> PECFG1_W<2> {
        PECFG1_W::new(self)
    }
    #[doc = "Bits 4:5 - PECFG2"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg2(&mut self) -> PECFG2_W<4> {
        PECFG2_W::new(self)
    }
    #[doc = "Bits 6:7 - PECFG3"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg3(&mut self) -> PECFG3_W<6> {
        PECFG3_W::new(self)
    }
    #[doc = "Bits 8:9 - PECFG4"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg4(&mut self) -> PECFG4_W<8> {
        PECFG4_W::new(self)
    }
    #[doc = "Bits 10:11 - PECFG5"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg5(&mut self) -> PECFG5_W<10> {
        PECFG5_W::new(self)
    }
    #[doc = "Bits 12:13 - PECFG6"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg6(&mut self) -> PECFG6_W<12> {
        PECFG6_W::new(self)
    }
    #[doc = "Bits 14:15 - PECFG7"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg7(&mut self) -> PECFG7_W<14> {
        PECFG7_W::new(self)
    }
    #[doc = "Bits 16:17 - PECFG8"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg8(&mut self) -> PECFG8_W<16> {
        PECFG8_W::new(self)
    }
    #[doc = "Bits 18:19 - PECFG9"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg9(&mut self) -> PECFG9_W<18> {
        PECFG9_W::new(self)
    }
    #[doc = "Bits 20:21 - PECFG10"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg10(&mut self) -> PECFG10_W<20> {
        PECFG10_W::new(self)
    }
    #[doc = "Bits 22:23 - PECFG11"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg11(&mut self) -> PECFG11_W<22> {
        PECFG11_W::new(self)
    }
    #[doc = "Bits 24:25 - PECFG12"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg12(&mut self) -> PECFG12_W<24> {
        PECFG12_W::new(self)
    }
    #[doc = "Bits 26:27 - PECFG13"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg13(&mut self) -> PECFG13_W<26> {
        PECFG13_W::new(self)
    }
    #[doc = "Bits 28:29 - PECFG14"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg14(&mut self) -> PECFG14_W<28> {
        PECFG14_W::new(self)
    }
    #[doc = "Bits 30:31 - PECFG15"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg15(&mut self) -> PECFG15_W<30> {
        PECFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_GPECFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpecfgr](index.html) module"]
pub struct AFIO_GPECFGR_SPEC;
impl crate::RegisterSpec for AFIO_GPECFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_gpecfgr::R](R) reader structure"]
impl crate::Readable for AFIO_GPECFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_gpecfgr::W](W) writer structure"]
impl crate::Writable for AFIO_GPECFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFIO_GPECFGR to value 0"]
impl crate::Resettable for AFIO_GPECFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
