#[doc = "Register `AFIO_GPBCFGR` reader"]
pub struct R(crate::R<AFIO_GPBCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_GPBCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_GPBCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_GPBCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_GPBCFGR` writer"]
pub struct W(crate::W<AFIO_GPBCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_GPBCFGR_SPEC>;
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
impl From<crate::W<AFIO_GPBCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_GPBCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBCFG0` reader - PBCFG0"]
pub type PBCFG0_R = crate::FieldReader;
#[doc = "Field `PBCFG0` writer - PBCFG0"]
pub type PBCFG0_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGR_SPEC, 2, O>;
#[doc = "Field `PBCFG1` reader - PBCFG1"]
pub type PBCFG1_R = crate::FieldReader;
#[doc = "Field `PBCFG1` writer - PBCFG1"]
pub type PBCFG1_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGR_SPEC, 2, O>;
#[doc = "Field `PBCFG2` reader - PBCFG2"]
pub type PBCFG2_R = crate::FieldReader;
#[doc = "Field `PBCFG2` writer - PBCFG2"]
pub type PBCFG2_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGR_SPEC, 2, O>;
#[doc = "Field `PBCFG3` reader - PBCFG3"]
pub type PBCFG3_R = crate::FieldReader;
#[doc = "Field `PBCFG3` writer - PBCFG3"]
pub type PBCFG3_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGR_SPEC, 2, O>;
#[doc = "Field `PBCFG4` reader - PBCFG4"]
pub type PBCFG4_R = crate::FieldReader;
#[doc = "Field `PBCFG4` writer - PBCFG4"]
pub type PBCFG4_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGR_SPEC, 2, O>;
#[doc = "Field `PBCFG5` reader - PBCFG5"]
pub type PBCFG5_R = crate::FieldReader;
#[doc = "Field `PBCFG5` writer - PBCFG5"]
pub type PBCFG5_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGR_SPEC, 2, O>;
#[doc = "Field `PBCFG6` reader - PBCFG6"]
pub type PBCFG6_R = crate::FieldReader;
#[doc = "Field `PBCFG6` writer - PBCFG6"]
pub type PBCFG6_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGR_SPEC, 2, O>;
#[doc = "Field `PBCFG7` reader - PBCFG7"]
pub type PBCFG7_R = crate::FieldReader;
#[doc = "Field `PBCFG7` writer - PBCFG7"]
pub type PBCFG7_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGR_SPEC, 2, O>;
#[doc = "Field `PBCFG8` reader - PBCFG8"]
pub type PBCFG8_R = crate::FieldReader;
#[doc = "Field `PBCFG8` writer - PBCFG8"]
pub type PBCFG8_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGR_SPEC, 2, O>;
#[doc = "Field `PBCFG9` reader - PBCFG9"]
pub type PBCFG9_R = crate::FieldReader;
#[doc = "Field `PBCFG9` writer - PBCFG9"]
pub type PBCFG9_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGR_SPEC, 2, O>;
#[doc = "Field `PBCFG10` reader - PBCFG10"]
pub type PBCFG10_R = crate::FieldReader;
#[doc = "Field `PBCFG10` writer - PBCFG10"]
pub type PBCFG10_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGR_SPEC, 2, O>;
#[doc = "Field `PBCFG11` reader - PBCFG11"]
pub type PBCFG11_R = crate::FieldReader;
#[doc = "Field `PBCFG11` writer - PBCFG11"]
pub type PBCFG11_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGR_SPEC, 2, O>;
#[doc = "Field `PBCFG12` reader - PBCFG12"]
pub type PBCFG12_R = crate::FieldReader;
#[doc = "Field `PBCFG12` writer - PBCFG12"]
pub type PBCFG12_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGR_SPEC, 2, O>;
#[doc = "Field `PBCFG13` reader - PBCFG13"]
pub type PBCFG13_R = crate::FieldReader;
#[doc = "Field `PBCFG13` writer - PBCFG13"]
pub type PBCFG13_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGR_SPEC, 2, O>;
#[doc = "Field `PBCFG14` reader - PBCFG14"]
pub type PBCFG14_R = crate::FieldReader;
#[doc = "Field `PBCFG14` writer - PBCFG14"]
pub type PBCFG14_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGR_SPEC, 2, O>;
#[doc = "Field `PBCFG15` reader - PBCFG15"]
pub type PBCFG15_R = crate::FieldReader;
#[doc = "Field `PBCFG15` writer - PBCFG15"]
pub type PBCFG15_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - PBCFG0"]
    #[inline(always)]
    pub fn pbcfg0(&self) -> PBCFG0_R {
        PBCFG0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PBCFG1"]
    #[inline(always)]
    pub fn pbcfg1(&self) -> PBCFG1_R {
        PBCFG1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PBCFG2"]
    #[inline(always)]
    pub fn pbcfg2(&self) -> PBCFG2_R {
        PBCFG2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PBCFG3"]
    #[inline(always)]
    pub fn pbcfg3(&self) -> PBCFG3_R {
        PBCFG3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PBCFG4"]
    #[inline(always)]
    pub fn pbcfg4(&self) -> PBCFG4_R {
        PBCFG4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PBCFG5"]
    #[inline(always)]
    pub fn pbcfg5(&self) -> PBCFG5_R {
        PBCFG5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PBCFG6"]
    #[inline(always)]
    pub fn pbcfg6(&self) -> PBCFG6_R {
        PBCFG6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PBCFG7"]
    #[inline(always)]
    pub fn pbcfg7(&self) -> PBCFG7_R {
        PBCFG7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PBCFG8"]
    #[inline(always)]
    pub fn pbcfg8(&self) -> PBCFG8_R {
        PBCFG8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PBCFG9"]
    #[inline(always)]
    pub fn pbcfg9(&self) -> PBCFG9_R {
        PBCFG9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PBCFG10"]
    #[inline(always)]
    pub fn pbcfg10(&self) -> PBCFG10_R {
        PBCFG10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PBCFG11"]
    #[inline(always)]
    pub fn pbcfg11(&self) -> PBCFG11_R {
        PBCFG11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PBCFG12"]
    #[inline(always)]
    pub fn pbcfg12(&self) -> PBCFG12_R {
        PBCFG12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PBCFG13"]
    #[inline(always)]
    pub fn pbcfg13(&self) -> PBCFG13_R {
        PBCFG13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PBCFG14"]
    #[inline(always)]
    pub fn pbcfg14(&self) -> PBCFG14_R {
        PBCFG14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PBCFG15"]
    #[inline(always)]
    pub fn pbcfg15(&self) -> PBCFG15_R {
        PBCFG15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PBCFG0"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg0(&mut self) -> PBCFG0_W<0> {
        PBCFG0_W::new(self)
    }
    #[doc = "Bits 2:3 - PBCFG1"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg1(&mut self) -> PBCFG1_W<2> {
        PBCFG1_W::new(self)
    }
    #[doc = "Bits 4:5 - PBCFG2"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg2(&mut self) -> PBCFG2_W<4> {
        PBCFG2_W::new(self)
    }
    #[doc = "Bits 6:7 - PBCFG3"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg3(&mut self) -> PBCFG3_W<6> {
        PBCFG3_W::new(self)
    }
    #[doc = "Bits 8:9 - PBCFG4"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg4(&mut self) -> PBCFG4_W<8> {
        PBCFG4_W::new(self)
    }
    #[doc = "Bits 10:11 - PBCFG5"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg5(&mut self) -> PBCFG5_W<10> {
        PBCFG5_W::new(self)
    }
    #[doc = "Bits 12:13 - PBCFG6"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg6(&mut self) -> PBCFG6_W<12> {
        PBCFG6_W::new(self)
    }
    #[doc = "Bits 14:15 - PBCFG7"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg7(&mut self) -> PBCFG7_W<14> {
        PBCFG7_W::new(self)
    }
    #[doc = "Bits 16:17 - PBCFG8"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg8(&mut self) -> PBCFG8_W<16> {
        PBCFG8_W::new(self)
    }
    #[doc = "Bits 18:19 - PBCFG9"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg9(&mut self) -> PBCFG9_W<18> {
        PBCFG9_W::new(self)
    }
    #[doc = "Bits 20:21 - PBCFG10"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg10(&mut self) -> PBCFG10_W<20> {
        PBCFG10_W::new(self)
    }
    #[doc = "Bits 22:23 - PBCFG11"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg11(&mut self) -> PBCFG11_W<22> {
        PBCFG11_W::new(self)
    }
    #[doc = "Bits 24:25 - PBCFG12"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg12(&mut self) -> PBCFG12_W<24> {
        PBCFG12_W::new(self)
    }
    #[doc = "Bits 26:27 - PBCFG13"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg13(&mut self) -> PBCFG13_W<26> {
        PBCFG13_W::new(self)
    }
    #[doc = "Bits 28:29 - PBCFG14"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg14(&mut self) -> PBCFG14_W<28> {
        PBCFG14_W::new(self)
    }
    #[doc = "Bits 30:31 - PBCFG15"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg15(&mut self) -> PBCFG15_W<30> {
        PBCFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_GPBCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpbcfgr](index.html) module"]
pub struct AFIO_GPBCFGR_SPEC;
impl crate::RegisterSpec for AFIO_GPBCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_gpbcfgr::R](R) reader structure"]
impl crate::Readable for AFIO_GPBCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_gpbcfgr::W](W) writer structure"]
impl crate::Writable for AFIO_GPBCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFIO_GPBCFGR to value 0"]
impl crate::Resettable for AFIO_GPBCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
