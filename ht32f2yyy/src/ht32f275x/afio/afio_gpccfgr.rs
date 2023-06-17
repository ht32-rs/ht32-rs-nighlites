#[doc = "Register `AFIO_GPCCFGR` reader"]
pub struct R(crate::R<AFIO_GPCCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_GPCCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_GPCCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_GPCCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_GPCCFGR` writer"]
pub struct W(crate::W<AFIO_GPCCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_GPCCFGR_SPEC>;
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
impl From<crate::W<AFIO_GPCCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_GPCCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCCFG0` reader - PCCFG0"]
pub type PCCFG0_R = crate::FieldReader;
#[doc = "Field `PCCFG0` writer - PCCFG0"]
pub type PCCFG0_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGR_SPEC, 2, O>;
#[doc = "Field `PCCFG1` reader - PCCFG1"]
pub type PCCFG1_R = crate::FieldReader;
#[doc = "Field `PCCFG1` writer - PCCFG1"]
pub type PCCFG1_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGR_SPEC, 2, O>;
#[doc = "Field `PCCFG2` reader - PCCFG2"]
pub type PCCFG2_R = crate::FieldReader;
#[doc = "Field `PCCFG2` writer - PCCFG2"]
pub type PCCFG2_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGR_SPEC, 2, O>;
#[doc = "Field `PCCFG3` reader - PCCFG3"]
pub type PCCFG3_R = crate::FieldReader;
#[doc = "Field `PCCFG3` writer - PCCFG3"]
pub type PCCFG3_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGR_SPEC, 2, O>;
#[doc = "Field `PCCFG4` reader - PCCFG4"]
pub type PCCFG4_R = crate::FieldReader;
#[doc = "Field `PCCFG4` writer - PCCFG4"]
pub type PCCFG4_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGR_SPEC, 2, O>;
#[doc = "Field `PCCFG5` reader - PCCFG5"]
pub type PCCFG5_R = crate::FieldReader;
#[doc = "Field `PCCFG5` writer - PCCFG5"]
pub type PCCFG5_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGR_SPEC, 2, O>;
#[doc = "Field `PCCFG6` reader - PCCFG6"]
pub type PCCFG6_R = crate::FieldReader;
#[doc = "Field `PCCFG6` writer - PCCFG6"]
pub type PCCFG6_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGR_SPEC, 2, O>;
#[doc = "Field `PCCFG7` reader - PCCFG7"]
pub type PCCFG7_R = crate::FieldReader;
#[doc = "Field `PCCFG7` writer - PCCFG7"]
pub type PCCFG7_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGR_SPEC, 2, O>;
#[doc = "Field `PCCFG8` reader - PCCFG8"]
pub type PCCFG8_R = crate::FieldReader;
#[doc = "Field `PCCFG8` writer - PCCFG8"]
pub type PCCFG8_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGR_SPEC, 2, O>;
#[doc = "Field `PCCFG9` reader - PCCFG9"]
pub type PCCFG9_R = crate::FieldReader;
#[doc = "Field `PCCFG9` writer - PCCFG9"]
pub type PCCFG9_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGR_SPEC, 2, O>;
#[doc = "Field `PCCFG10` reader - PCCFG10"]
pub type PCCFG10_R = crate::FieldReader;
#[doc = "Field `PCCFG10` writer - PCCFG10"]
pub type PCCFG10_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGR_SPEC, 2, O>;
#[doc = "Field `PCCFG11` reader - PCCFG11"]
pub type PCCFG11_R = crate::FieldReader;
#[doc = "Field `PCCFG11` writer - PCCFG11"]
pub type PCCFG11_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGR_SPEC, 2, O>;
#[doc = "Field `PCCFG12` reader - PCCFG12"]
pub type PCCFG12_R = crate::FieldReader;
#[doc = "Field `PCCFG12` writer - PCCFG12"]
pub type PCCFG12_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGR_SPEC, 2, O>;
#[doc = "Field `PCCFG13` reader - PCCFG13"]
pub type PCCFG13_R = crate::FieldReader;
#[doc = "Field `PCCFG13` writer - PCCFG13"]
pub type PCCFG13_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGR_SPEC, 2, O>;
#[doc = "Field `PCCFG14` reader - PCCFG14"]
pub type PCCFG14_R = crate::FieldReader;
#[doc = "Field `PCCFG14` writer - PCCFG14"]
pub type PCCFG14_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGR_SPEC, 2, O>;
#[doc = "Field `PCCFG15` reader - PCCFG15"]
pub type PCCFG15_R = crate::FieldReader;
#[doc = "Field `PCCFG15` writer - PCCFG15"]
pub type PCCFG15_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - PCCFG0"]
    #[inline(always)]
    pub fn pccfg0(&self) -> PCCFG0_R {
        PCCFG0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PCCFG1"]
    #[inline(always)]
    pub fn pccfg1(&self) -> PCCFG1_R {
        PCCFG1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PCCFG2"]
    #[inline(always)]
    pub fn pccfg2(&self) -> PCCFG2_R {
        PCCFG2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PCCFG3"]
    #[inline(always)]
    pub fn pccfg3(&self) -> PCCFG3_R {
        PCCFG3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PCCFG4"]
    #[inline(always)]
    pub fn pccfg4(&self) -> PCCFG4_R {
        PCCFG4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PCCFG5"]
    #[inline(always)]
    pub fn pccfg5(&self) -> PCCFG5_R {
        PCCFG5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PCCFG6"]
    #[inline(always)]
    pub fn pccfg6(&self) -> PCCFG6_R {
        PCCFG6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PCCFG7"]
    #[inline(always)]
    pub fn pccfg7(&self) -> PCCFG7_R {
        PCCFG7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PCCFG8"]
    #[inline(always)]
    pub fn pccfg8(&self) -> PCCFG8_R {
        PCCFG8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PCCFG9"]
    #[inline(always)]
    pub fn pccfg9(&self) -> PCCFG9_R {
        PCCFG9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PCCFG10"]
    #[inline(always)]
    pub fn pccfg10(&self) -> PCCFG10_R {
        PCCFG10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PCCFG11"]
    #[inline(always)]
    pub fn pccfg11(&self) -> PCCFG11_R {
        PCCFG11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PCCFG12"]
    #[inline(always)]
    pub fn pccfg12(&self) -> PCCFG12_R {
        PCCFG12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PCCFG13"]
    #[inline(always)]
    pub fn pccfg13(&self) -> PCCFG13_R {
        PCCFG13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PCCFG14"]
    #[inline(always)]
    pub fn pccfg14(&self) -> PCCFG14_R {
        PCCFG14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PCCFG15"]
    #[inline(always)]
    pub fn pccfg15(&self) -> PCCFG15_R {
        PCCFG15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PCCFG0"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg0(&mut self) -> PCCFG0_W<0> {
        PCCFG0_W::new(self)
    }
    #[doc = "Bits 2:3 - PCCFG1"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg1(&mut self) -> PCCFG1_W<2> {
        PCCFG1_W::new(self)
    }
    #[doc = "Bits 4:5 - PCCFG2"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg2(&mut self) -> PCCFG2_W<4> {
        PCCFG2_W::new(self)
    }
    #[doc = "Bits 6:7 - PCCFG3"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg3(&mut self) -> PCCFG3_W<6> {
        PCCFG3_W::new(self)
    }
    #[doc = "Bits 8:9 - PCCFG4"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg4(&mut self) -> PCCFG4_W<8> {
        PCCFG4_W::new(self)
    }
    #[doc = "Bits 10:11 - PCCFG5"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg5(&mut self) -> PCCFG5_W<10> {
        PCCFG5_W::new(self)
    }
    #[doc = "Bits 12:13 - PCCFG6"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg6(&mut self) -> PCCFG6_W<12> {
        PCCFG6_W::new(self)
    }
    #[doc = "Bits 14:15 - PCCFG7"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg7(&mut self) -> PCCFG7_W<14> {
        PCCFG7_W::new(self)
    }
    #[doc = "Bits 16:17 - PCCFG8"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg8(&mut self) -> PCCFG8_W<16> {
        PCCFG8_W::new(self)
    }
    #[doc = "Bits 18:19 - PCCFG9"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg9(&mut self) -> PCCFG9_W<18> {
        PCCFG9_W::new(self)
    }
    #[doc = "Bits 20:21 - PCCFG10"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg10(&mut self) -> PCCFG10_W<20> {
        PCCFG10_W::new(self)
    }
    #[doc = "Bits 22:23 - PCCFG11"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg11(&mut self) -> PCCFG11_W<22> {
        PCCFG11_W::new(self)
    }
    #[doc = "Bits 24:25 - PCCFG12"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg12(&mut self) -> PCCFG12_W<24> {
        PCCFG12_W::new(self)
    }
    #[doc = "Bits 26:27 - PCCFG13"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg13(&mut self) -> PCCFG13_W<26> {
        PCCFG13_W::new(self)
    }
    #[doc = "Bits 28:29 - PCCFG14"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg14(&mut self) -> PCCFG14_W<28> {
        PCCFG14_W::new(self)
    }
    #[doc = "Bits 30:31 - PCCFG15"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg15(&mut self) -> PCCFG15_W<30> {
        PCCFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_GPCCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpccfgr](index.html) module"]
pub struct AFIO_GPCCFGR_SPEC;
impl crate::RegisterSpec for AFIO_GPCCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_gpccfgr::R](R) reader structure"]
impl crate::Readable for AFIO_GPCCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_gpccfgr::W](W) writer structure"]
impl crate::Writable for AFIO_GPCCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFIO_GPCCFGR to value 0"]
impl crate::Resettable for AFIO_GPCCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
