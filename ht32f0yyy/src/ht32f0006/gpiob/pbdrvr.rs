#[doc = "Register `PBDRVR` reader"]
pub struct R(crate::R<PBDRVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBDRVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBDRVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBDRVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBDRVR` writer"]
pub struct W(crate::W<PBDRVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBDRVR_SPEC>;
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
impl From<crate::W<PBDRVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBDRVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBDV0` reader - PBDV0"]
pub type PBDV0_R = crate::FieldReader;
#[doc = "Field `PBDV0` writer - PBDV0"]
pub type PBDV0_W<'a, const O: u8> = crate::FieldWriter<'a, PBDRVR_SPEC, 2, O>;
#[doc = "Field `PBDV1` reader - PBDV1"]
pub type PBDV1_R = crate::FieldReader;
#[doc = "Field `PBDV1` writer - PBDV1"]
pub type PBDV1_W<'a, const O: u8> = crate::FieldWriter<'a, PBDRVR_SPEC, 2, O>;
#[doc = "Field `PBDV2` reader - PBDV2"]
pub type PBDV2_R = crate::FieldReader;
#[doc = "Field `PBDV2` writer - PBDV2"]
pub type PBDV2_W<'a, const O: u8> = crate::FieldWriter<'a, PBDRVR_SPEC, 2, O>;
#[doc = "Field `PBDV3` reader - PBDV3"]
pub type PBDV3_R = crate::FieldReader;
#[doc = "Field `PBDV3` writer - PBDV3"]
pub type PBDV3_W<'a, const O: u8> = crate::FieldWriter<'a, PBDRVR_SPEC, 2, O>;
#[doc = "Field `PBDV4` reader - PBDV4"]
pub type PBDV4_R = crate::FieldReader;
#[doc = "Field `PBDV4` writer - PBDV4"]
pub type PBDV4_W<'a, const O: u8> = crate::FieldWriter<'a, PBDRVR_SPEC, 2, O>;
#[doc = "Field `PBDV5` reader - PBDV5"]
pub type PBDV5_R = crate::FieldReader;
#[doc = "Field `PBDV5` writer - PBDV5"]
pub type PBDV5_W<'a, const O: u8> = crate::FieldWriter<'a, PBDRVR_SPEC, 2, O>;
#[doc = "Field `PBDV6` reader - PBDV6"]
pub type PBDV6_R = crate::FieldReader;
#[doc = "Field `PBDV6` writer - PBDV6"]
pub type PBDV6_W<'a, const O: u8> = crate::FieldWriter<'a, PBDRVR_SPEC, 2, O>;
#[doc = "Field `PBDV7` reader - PBDV7"]
pub type PBDV7_R = crate::FieldReader;
#[doc = "Field `PBDV7` writer - PBDV7"]
pub type PBDV7_W<'a, const O: u8> = crate::FieldWriter<'a, PBDRVR_SPEC, 2, O>;
#[doc = "Field `PBDV8` reader - PBDV8"]
pub type PBDV8_R = crate::FieldReader;
#[doc = "Field `PBDV8` writer - PBDV8"]
pub type PBDV8_W<'a, const O: u8> = crate::FieldWriter<'a, PBDRVR_SPEC, 2, O>;
#[doc = "Field `PBDV9` reader - PBDV9"]
pub type PBDV9_R = crate::FieldReader;
#[doc = "Field `PBDV9` writer - PBDV9"]
pub type PBDV9_W<'a, const O: u8> = crate::FieldWriter<'a, PBDRVR_SPEC, 2, O>;
#[doc = "Field `PBDV10` reader - PBDV10"]
pub type PBDV10_R = crate::FieldReader;
#[doc = "Field `PBDV10` writer - PBDV10"]
pub type PBDV10_W<'a, const O: u8> = crate::FieldWriter<'a, PBDRVR_SPEC, 2, O>;
#[doc = "Field `PBDV11` reader - PBDV11"]
pub type PBDV11_R = crate::FieldReader;
#[doc = "Field `PBDV11` writer - PBDV11"]
pub type PBDV11_W<'a, const O: u8> = crate::FieldWriter<'a, PBDRVR_SPEC, 2, O>;
#[doc = "Field `PBDV12` reader - PBDV12"]
pub type PBDV12_R = crate::FieldReader;
#[doc = "Field `PBDV12` writer - PBDV12"]
pub type PBDV12_W<'a, const O: u8> = crate::FieldWriter<'a, PBDRVR_SPEC, 2, O>;
#[doc = "Field `PBDV13` reader - PBDV13"]
pub type PBDV13_R = crate::FieldReader;
#[doc = "Field `PBDV13` writer - PBDV13"]
pub type PBDV13_W<'a, const O: u8> = crate::FieldWriter<'a, PBDRVR_SPEC, 2, O>;
#[doc = "Field `PBDV14` reader - PBDV14"]
pub type PBDV14_R = crate::FieldReader;
#[doc = "Field `PBDV14` writer - PBDV14"]
pub type PBDV14_W<'a, const O: u8> = crate::FieldWriter<'a, PBDRVR_SPEC, 2, O>;
#[doc = "Field `PBDV15` reader - PBDV15"]
pub type PBDV15_R = crate::FieldReader;
#[doc = "Field `PBDV15` writer - PBDV15"]
pub type PBDV15_W<'a, const O: u8> = crate::FieldWriter<'a, PBDRVR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - PBDV0"]
    #[inline(always)]
    pub fn pbdv0(&self) -> PBDV0_R {
        PBDV0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PBDV1"]
    #[inline(always)]
    pub fn pbdv1(&self) -> PBDV1_R {
        PBDV1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PBDV2"]
    #[inline(always)]
    pub fn pbdv2(&self) -> PBDV2_R {
        PBDV2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PBDV3"]
    #[inline(always)]
    pub fn pbdv3(&self) -> PBDV3_R {
        PBDV3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PBDV4"]
    #[inline(always)]
    pub fn pbdv4(&self) -> PBDV4_R {
        PBDV4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PBDV5"]
    #[inline(always)]
    pub fn pbdv5(&self) -> PBDV5_R {
        PBDV5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PBDV6"]
    #[inline(always)]
    pub fn pbdv6(&self) -> PBDV6_R {
        PBDV6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PBDV7"]
    #[inline(always)]
    pub fn pbdv7(&self) -> PBDV7_R {
        PBDV7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PBDV8"]
    #[inline(always)]
    pub fn pbdv8(&self) -> PBDV8_R {
        PBDV8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PBDV9"]
    #[inline(always)]
    pub fn pbdv9(&self) -> PBDV9_R {
        PBDV9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PBDV10"]
    #[inline(always)]
    pub fn pbdv10(&self) -> PBDV10_R {
        PBDV10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PBDV11"]
    #[inline(always)]
    pub fn pbdv11(&self) -> PBDV11_R {
        PBDV11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PBDV12"]
    #[inline(always)]
    pub fn pbdv12(&self) -> PBDV12_R {
        PBDV12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PBDV13"]
    #[inline(always)]
    pub fn pbdv13(&self) -> PBDV13_R {
        PBDV13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PBDV14"]
    #[inline(always)]
    pub fn pbdv14(&self) -> PBDV14_R {
        PBDV14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PBDV15"]
    #[inline(always)]
    pub fn pbdv15(&self) -> PBDV15_R {
        PBDV15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PBDV0"]
    #[inline(always)]
    #[must_use]
    pub fn pbdv0(&mut self) -> PBDV0_W<0> {
        PBDV0_W::new(self)
    }
    #[doc = "Bits 2:3 - PBDV1"]
    #[inline(always)]
    #[must_use]
    pub fn pbdv1(&mut self) -> PBDV1_W<2> {
        PBDV1_W::new(self)
    }
    #[doc = "Bits 4:5 - PBDV2"]
    #[inline(always)]
    #[must_use]
    pub fn pbdv2(&mut self) -> PBDV2_W<4> {
        PBDV2_W::new(self)
    }
    #[doc = "Bits 6:7 - PBDV3"]
    #[inline(always)]
    #[must_use]
    pub fn pbdv3(&mut self) -> PBDV3_W<6> {
        PBDV3_W::new(self)
    }
    #[doc = "Bits 8:9 - PBDV4"]
    #[inline(always)]
    #[must_use]
    pub fn pbdv4(&mut self) -> PBDV4_W<8> {
        PBDV4_W::new(self)
    }
    #[doc = "Bits 10:11 - PBDV5"]
    #[inline(always)]
    #[must_use]
    pub fn pbdv5(&mut self) -> PBDV5_W<10> {
        PBDV5_W::new(self)
    }
    #[doc = "Bits 12:13 - PBDV6"]
    #[inline(always)]
    #[must_use]
    pub fn pbdv6(&mut self) -> PBDV6_W<12> {
        PBDV6_W::new(self)
    }
    #[doc = "Bits 14:15 - PBDV7"]
    #[inline(always)]
    #[must_use]
    pub fn pbdv7(&mut self) -> PBDV7_W<14> {
        PBDV7_W::new(self)
    }
    #[doc = "Bits 16:17 - PBDV8"]
    #[inline(always)]
    #[must_use]
    pub fn pbdv8(&mut self) -> PBDV8_W<16> {
        PBDV8_W::new(self)
    }
    #[doc = "Bits 18:19 - PBDV9"]
    #[inline(always)]
    #[must_use]
    pub fn pbdv9(&mut self) -> PBDV9_W<18> {
        PBDV9_W::new(self)
    }
    #[doc = "Bits 20:21 - PBDV10"]
    #[inline(always)]
    #[must_use]
    pub fn pbdv10(&mut self) -> PBDV10_W<20> {
        PBDV10_W::new(self)
    }
    #[doc = "Bits 22:23 - PBDV11"]
    #[inline(always)]
    #[must_use]
    pub fn pbdv11(&mut self) -> PBDV11_W<22> {
        PBDV11_W::new(self)
    }
    #[doc = "Bits 24:25 - PBDV12"]
    #[inline(always)]
    #[must_use]
    pub fn pbdv12(&mut self) -> PBDV12_W<24> {
        PBDV12_W::new(self)
    }
    #[doc = "Bits 26:27 - PBDV13"]
    #[inline(always)]
    #[must_use]
    pub fn pbdv13(&mut self) -> PBDV13_W<26> {
        PBDV13_W::new(self)
    }
    #[doc = "Bits 28:29 - PBDV14"]
    #[inline(always)]
    #[must_use]
    pub fn pbdv14(&mut self) -> PBDV14_W<28> {
        PBDV14_W::new(self)
    }
    #[doc = "Bits 30:31 - PBDV15"]
    #[inline(always)]
    #[must_use]
    pub fn pbdv15(&mut self) -> PBDV15_W<30> {
        PBDV15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBDRVR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbdrvr](index.html) module"]
pub struct PBDRVR_SPEC;
impl crate::RegisterSpec for PBDRVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbdrvr::R](R) reader structure"]
impl crate::Readable for PBDRVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbdrvr::W](W) writer structure"]
impl crate::Writable for PBDRVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBDRVR to value 0"]
impl crate::Resettable for PBDRVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
