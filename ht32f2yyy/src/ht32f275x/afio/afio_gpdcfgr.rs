#[doc = "Register `AFIO_GPDCFGR` reader"]
pub struct R(crate::R<AFIO_GPDCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_GPDCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_GPDCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_GPDCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_GPDCFGR` writer"]
pub struct W(crate::W<AFIO_GPDCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_GPDCFGR_SPEC>;
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
impl From<crate::W<AFIO_GPDCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_GPDCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDCFG0` reader - PDCFG0"]
pub type PDCFG0_R = crate::FieldReader;
#[doc = "Field `PDCFG0` writer - PDCFG0"]
pub type PDCFG0_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGR_SPEC, 2, O>;
#[doc = "Field `PDCFG1` reader - PDCFG1"]
pub type PDCFG1_R = crate::FieldReader;
#[doc = "Field `PDCFG1` writer - PDCFG1"]
pub type PDCFG1_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGR_SPEC, 2, O>;
#[doc = "Field `PDCFG2` reader - PDCFG2"]
pub type PDCFG2_R = crate::FieldReader;
#[doc = "Field `PDCFG2` writer - PDCFG2"]
pub type PDCFG2_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGR_SPEC, 2, O>;
#[doc = "Field `PDCFG3` reader - PDCFG3"]
pub type PDCFG3_R = crate::FieldReader;
#[doc = "Field `PDCFG3` writer - PDCFG3"]
pub type PDCFG3_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGR_SPEC, 2, O>;
#[doc = "Field `PDCFG4` reader - PDCFG4"]
pub type PDCFG4_R = crate::FieldReader;
#[doc = "Field `PDCFG4` writer - PDCFG4"]
pub type PDCFG4_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGR_SPEC, 2, O>;
#[doc = "Field `PDCFG5` reader - PDCFG5"]
pub type PDCFG5_R = crate::FieldReader;
#[doc = "Field `PDCFG5` writer - PDCFG5"]
pub type PDCFG5_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGR_SPEC, 2, O>;
#[doc = "Field `PDCFG6` reader - PDCFG6"]
pub type PDCFG6_R = crate::FieldReader;
#[doc = "Field `PDCFG6` writer - PDCFG6"]
pub type PDCFG6_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGR_SPEC, 2, O>;
#[doc = "Field `PDCFG7` reader - PDCFG7"]
pub type PDCFG7_R = crate::FieldReader;
#[doc = "Field `PDCFG7` writer - PDCFG7"]
pub type PDCFG7_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGR_SPEC, 2, O>;
#[doc = "Field `PDCFG8` reader - PDCFG8"]
pub type PDCFG8_R = crate::FieldReader;
#[doc = "Field `PDCFG8` writer - PDCFG8"]
pub type PDCFG8_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGR_SPEC, 2, O>;
#[doc = "Field `PDCFG9` reader - PDCFG9"]
pub type PDCFG9_R = crate::FieldReader;
#[doc = "Field `PDCFG9` writer - PDCFG9"]
pub type PDCFG9_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGR_SPEC, 2, O>;
#[doc = "Field `PDCFG10` reader - PDCFG10"]
pub type PDCFG10_R = crate::FieldReader;
#[doc = "Field `PDCFG10` writer - PDCFG10"]
pub type PDCFG10_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGR_SPEC, 2, O>;
#[doc = "Field `PDCFG11` reader - PDCFG11"]
pub type PDCFG11_R = crate::FieldReader;
#[doc = "Field `PDCFG11` writer - PDCFG11"]
pub type PDCFG11_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGR_SPEC, 2, O>;
#[doc = "Field `PDCFG12` reader - PDCFG12"]
pub type PDCFG12_R = crate::FieldReader;
#[doc = "Field `PDCFG12` writer - PDCFG12"]
pub type PDCFG12_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGR_SPEC, 2, O>;
#[doc = "Field `PDCFG13` reader - PDCFG13"]
pub type PDCFG13_R = crate::FieldReader;
#[doc = "Field `PDCFG13` writer - PDCFG13"]
pub type PDCFG13_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGR_SPEC, 2, O>;
#[doc = "Field `PDCFG14` reader - PDCFG14"]
pub type PDCFG14_R = crate::FieldReader;
#[doc = "Field `PDCFG14` writer - PDCFG14"]
pub type PDCFG14_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGR_SPEC, 2, O>;
#[doc = "Field `PDCFG15` reader - PDCFG15"]
pub type PDCFG15_R = crate::FieldReader;
#[doc = "Field `PDCFG15` writer - PDCFG15"]
pub type PDCFG15_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - PDCFG0"]
    #[inline(always)]
    pub fn pdcfg0(&self) -> PDCFG0_R {
        PDCFG0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PDCFG1"]
    #[inline(always)]
    pub fn pdcfg1(&self) -> PDCFG1_R {
        PDCFG1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PDCFG2"]
    #[inline(always)]
    pub fn pdcfg2(&self) -> PDCFG2_R {
        PDCFG2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PDCFG3"]
    #[inline(always)]
    pub fn pdcfg3(&self) -> PDCFG3_R {
        PDCFG3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PDCFG4"]
    #[inline(always)]
    pub fn pdcfg4(&self) -> PDCFG4_R {
        PDCFG4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PDCFG5"]
    #[inline(always)]
    pub fn pdcfg5(&self) -> PDCFG5_R {
        PDCFG5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PDCFG6"]
    #[inline(always)]
    pub fn pdcfg6(&self) -> PDCFG6_R {
        PDCFG6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PDCFG7"]
    #[inline(always)]
    pub fn pdcfg7(&self) -> PDCFG7_R {
        PDCFG7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PDCFG8"]
    #[inline(always)]
    pub fn pdcfg8(&self) -> PDCFG8_R {
        PDCFG8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PDCFG9"]
    #[inline(always)]
    pub fn pdcfg9(&self) -> PDCFG9_R {
        PDCFG9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PDCFG10"]
    #[inline(always)]
    pub fn pdcfg10(&self) -> PDCFG10_R {
        PDCFG10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PDCFG11"]
    #[inline(always)]
    pub fn pdcfg11(&self) -> PDCFG11_R {
        PDCFG11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PDCFG12"]
    #[inline(always)]
    pub fn pdcfg12(&self) -> PDCFG12_R {
        PDCFG12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PDCFG13"]
    #[inline(always)]
    pub fn pdcfg13(&self) -> PDCFG13_R {
        PDCFG13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PDCFG14"]
    #[inline(always)]
    pub fn pdcfg14(&self) -> PDCFG14_R {
        PDCFG14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PDCFG15"]
    #[inline(always)]
    pub fn pdcfg15(&self) -> PDCFG15_R {
        PDCFG15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PDCFG0"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg0(&mut self) -> PDCFG0_W<0> {
        PDCFG0_W::new(self)
    }
    #[doc = "Bits 2:3 - PDCFG1"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg1(&mut self) -> PDCFG1_W<2> {
        PDCFG1_W::new(self)
    }
    #[doc = "Bits 4:5 - PDCFG2"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg2(&mut self) -> PDCFG2_W<4> {
        PDCFG2_W::new(self)
    }
    #[doc = "Bits 6:7 - PDCFG3"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg3(&mut self) -> PDCFG3_W<6> {
        PDCFG3_W::new(self)
    }
    #[doc = "Bits 8:9 - PDCFG4"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg4(&mut self) -> PDCFG4_W<8> {
        PDCFG4_W::new(self)
    }
    #[doc = "Bits 10:11 - PDCFG5"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg5(&mut self) -> PDCFG5_W<10> {
        PDCFG5_W::new(self)
    }
    #[doc = "Bits 12:13 - PDCFG6"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg6(&mut self) -> PDCFG6_W<12> {
        PDCFG6_W::new(self)
    }
    #[doc = "Bits 14:15 - PDCFG7"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg7(&mut self) -> PDCFG7_W<14> {
        PDCFG7_W::new(self)
    }
    #[doc = "Bits 16:17 - PDCFG8"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg8(&mut self) -> PDCFG8_W<16> {
        PDCFG8_W::new(self)
    }
    #[doc = "Bits 18:19 - PDCFG9"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg9(&mut self) -> PDCFG9_W<18> {
        PDCFG9_W::new(self)
    }
    #[doc = "Bits 20:21 - PDCFG10"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg10(&mut self) -> PDCFG10_W<20> {
        PDCFG10_W::new(self)
    }
    #[doc = "Bits 22:23 - PDCFG11"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg11(&mut self) -> PDCFG11_W<22> {
        PDCFG11_W::new(self)
    }
    #[doc = "Bits 24:25 - PDCFG12"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg12(&mut self) -> PDCFG12_W<24> {
        PDCFG12_W::new(self)
    }
    #[doc = "Bits 26:27 - PDCFG13"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg13(&mut self) -> PDCFG13_W<26> {
        PDCFG13_W::new(self)
    }
    #[doc = "Bits 28:29 - PDCFG14"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg14(&mut self) -> PDCFG14_W<28> {
        PDCFG14_W::new(self)
    }
    #[doc = "Bits 30:31 - PDCFG15"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg15(&mut self) -> PDCFG15_W<30> {
        PDCFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_GPDCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpdcfgr](index.html) module"]
pub struct AFIO_GPDCFGR_SPEC;
impl crate::RegisterSpec for AFIO_GPDCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_gpdcfgr::R](R) reader structure"]
impl crate::Readable for AFIO_GPDCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_gpdcfgr::W](W) writer structure"]
impl crate::Writable for AFIO_GPDCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFIO_GPDCFGR to value 0"]
impl crate::Resettable for AFIO_GPDCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
