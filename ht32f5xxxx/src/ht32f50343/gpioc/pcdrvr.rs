#[doc = "Register `PCDRVR` reader"]
pub struct R(crate::R<PCDRVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCDRVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCDRVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCDRVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCDRVR` writer"]
pub struct W(crate::W<PCDRVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCDRVR_SPEC>;
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
impl From<crate::W<PCDRVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCDRVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCDV0` reader - PCDV0"]
pub type PCDV0_R = crate::FieldReader;
#[doc = "Field `PCDV0` writer - PCDV0"]
pub type PCDV0_W<'a, const O: u8> = crate::FieldWriter<'a, PCDRVR_SPEC, 2, O>;
#[doc = "Field `PCDV1` reader - PCDV1"]
pub type PCDV1_R = crate::FieldReader;
#[doc = "Field `PCDV1` writer - PCDV1"]
pub type PCDV1_W<'a, const O: u8> = crate::FieldWriter<'a, PCDRVR_SPEC, 2, O>;
#[doc = "Field `PCDV2` reader - PCDV2"]
pub type PCDV2_R = crate::FieldReader;
#[doc = "Field `PCDV2` writer - PCDV2"]
pub type PCDV2_W<'a, const O: u8> = crate::FieldWriter<'a, PCDRVR_SPEC, 2, O>;
#[doc = "Field `PCDV3` reader - PCDV3"]
pub type PCDV3_R = crate::FieldReader;
#[doc = "Field `PCDV3` writer - PCDV3"]
pub type PCDV3_W<'a, const O: u8> = crate::FieldWriter<'a, PCDRVR_SPEC, 2, O>;
#[doc = "Field `PCDV4` reader - PCDV4"]
pub type PCDV4_R = crate::FieldReader;
#[doc = "Field `PCDV4` writer - PCDV4"]
pub type PCDV4_W<'a, const O: u8> = crate::FieldWriter<'a, PCDRVR_SPEC, 2, O>;
#[doc = "Field `PCDV5` reader - PCDV5"]
pub type PCDV5_R = crate::FieldReader;
#[doc = "Field `PCDV5` writer - PCDV5"]
pub type PCDV5_W<'a, const O: u8> = crate::FieldWriter<'a, PCDRVR_SPEC, 2, O>;
#[doc = "Field `PCDV6` reader - PCDV6"]
pub type PCDV6_R = crate::FieldReader;
#[doc = "Field `PCDV6` writer - PCDV6"]
pub type PCDV6_W<'a, const O: u8> = crate::FieldWriter<'a, PCDRVR_SPEC, 2, O>;
#[doc = "Field `PCDV7` reader - PCDV7"]
pub type PCDV7_R = crate::FieldReader;
#[doc = "Field `PCDV7` writer - PCDV7"]
pub type PCDV7_W<'a, const O: u8> = crate::FieldWriter<'a, PCDRVR_SPEC, 2, O>;
#[doc = "Field `PCDV8` reader - PCDV8"]
pub type PCDV8_R = crate::FieldReader;
#[doc = "Field `PCDV8` writer - PCDV8"]
pub type PCDV8_W<'a, const O: u8> = crate::FieldWriter<'a, PCDRVR_SPEC, 2, O>;
#[doc = "Field `PCDV9` reader - PCDV9"]
pub type PCDV9_R = crate::FieldReader;
#[doc = "Field `PCDV9` writer - PCDV9"]
pub type PCDV9_W<'a, const O: u8> = crate::FieldWriter<'a, PCDRVR_SPEC, 2, O>;
#[doc = "Field `PCDV10` reader - PCDV10"]
pub type PCDV10_R = crate::FieldReader;
#[doc = "Field `PCDV10` writer - PCDV10"]
pub type PCDV10_W<'a, const O: u8> = crate::FieldWriter<'a, PCDRVR_SPEC, 2, O>;
#[doc = "Field `PCDV11` reader - PCDV11"]
pub type PCDV11_R = crate::FieldReader;
#[doc = "Field `PCDV11` writer - PCDV11"]
pub type PCDV11_W<'a, const O: u8> = crate::FieldWriter<'a, PCDRVR_SPEC, 2, O>;
#[doc = "Field `PCDV12` reader - PCDV12"]
pub type PCDV12_R = crate::FieldReader;
#[doc = "Field `PCDV12` writer - PCDV12"]
pub type PCDV12_W<'a, const O: u8> = crate::FieldWriter<'a, PCDRVR_SPEC, 2, O>;
#[doc = "Field `PCDV13` reader - PCDV13"]
pub type PCDV13_R = crate::FieldReader;
#[doc = "Field `PCDV13` writer - PCDV13"]
pub type PCDV13_W<'a, const O: u8> = crate::FieldWriter<'a, PCDRVR_SPEC, 2, O>;
#[doc = "Field `PCDV14` reader - PCDV14"]
pub type PCDV14_R = crate::FieldReader;
#[doc = "Field `PCDV14` writer - PCDV14"]
pub type PCDV14_W<'a, const O: u8> = crate::FieldWriter<'a, PCDRVR_SPEC, 2, O>;
#[doc = "Field `PCDV15` reader - PCDV15"]
pub type PCDV15_R = crate::FieldReader;
#[doc = "Field `PCDV15` writer - PCDV15"]
pub type PCDV15_W<'a, const O: u8> = crate::FieldWriter<'a, PCDRVR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - PCDV0"]
    #[inline(always)]
    pub fn pcdv0(&self) -> PCDV0_R {
        PCDV0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PCDV1"]
    #[inline(always)]
    pub fn pcdv1(&self) -> PCDV1_R {
        PCDV1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PCDV2"]
    #[inline(always)]
    pub fn pcdv2(&self) -> PCDV2_R {
        PCDV2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PCDV3"]
    #[inline(always)]
    pub fn pcdv3(&self) -> PCDV3_R {
        PCDV3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PCDV4"]
    #[inline(always)]
    pub fn pcdv4(&self) -> PCDV4_R {
        PCDV4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PCDV5"]
    #[inline(always)]
    pub fn pcdv5(&self) -> PCDV5_R {
        PCDV5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PCDV6"]
    #[inline(always)]
    pub fn pcdv6(&self) -> PCDV6_R {
        PCDV6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PCDV7"]
    #[inline(always)]
    pub fn pcdv7(&self) -> PCDV7_R {
        PCDV7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PCDV8"]
    #[inline(always)]
    pub fn pcdv8(&self) -> PCDV8_R {
        PCDV8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PCDV9"]
    #[inline(always)]
    pub fn pcdv9(&self) -> PCDV9_R {
        PCDV9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PCDV10"]
    #[inline(always)]
    pub fn pcdv10(&self) -> PCDV10_R {
        PCDV10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PCDV11"]
    #[inline(always)]
    pub fn pcdv11(&self) -> PCDV11_R {
        PCDV11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PCDV12"]
    #[inline(always)]
    pub fn pcdv12(&self) -> PCDV12_R {
        PCDV12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PCDV13"]
    #[inline(always)]
    pub fn pcdv13(&self) -> PCDV13_R {
        PCDV13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PCDV14"]
    #[inline(always)]
    pub fn pcdv14(&self) -> PCDV14_R {
        PCDV14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PCDV15"]
    #[inline(always)]
    pub fn pcdv15(&self) -> PCDV15_R {
        PCDV15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PCDV0"]
    #[inline(always)]
    #[must_use]
    pub fn pcdv0(&mut self) -> PCDV0_W<0> {
        PCDV0_W::new(self)
    }
    #[doc = "Bits 2:3 - PCDV1"]
    #[inline(always)]
    #[must_use]
    pub fn pcdv1(&mut self) -> PCDV1_W<2> {
        PCDV1_W::new(self)
    }
    #[doc = "Bits 4:5 - PCDV2"]
    #[inline(always)]
    #[must_use]
    pub fn pcdv2(&mut self) -> PCDV2_W<4> {
        PCDV2_W::new(self)
    }
    #[doc = "Bits 6:7 - PCDV3"]
    #[inline(always)]
    #[must_use]
    pub fn pcdv3(&mut self) -> PCDV3_W<6> {
        PCDV3_W::new(self)
    }
    #[doc = "Bits 8:9 - PCDV4"]
    #[inline(always)]
    #[must_use]
    pub fn pcdv4(&mut self) -> PCDV4_W<8> {
        PCDV4_W::new(self)
    }
    #[doc = "Bits 10:11 - PCDV5"]
    #[inline(always)]
    #[must_use]
    pub fn pcdv5(&mut self) -> PCDV5_W<10> {
        PCDV5_W::new(self)
    }
    #[doc = "Bits 12:13 - PCDV6"]
    #[inline(always)]
    #[must_use]
    pub fn pcdv6(&mut self) -> PCDV6_W<12> {
        PCDV6_W::new(self)
    }
    #[doc = "Bits 14:15 - PCDV7"]
    #[inline(always)]
    #[must_use]
    pub fn pcdv7(&mut self) -> PCDV7_W<14> {
        PCDV7_W::new(self)
    }
    #[doc = "Bits 16:17 - PCDV8"]
    #[inline(always)]
    #[must_use]
    pub fn pcdv8(&mut self) -> PCDV8_W<16> {
        PCDV8_W::new(self)
    }
    #[doc = "Bits 18:19 - PCDV9"]
    #[inline(always)]
    #[must_use]
    pub fn pcdv9(&mut self) -> PCDV9_W<18> {
        PCDV9_W::new(self)
    }
    #[doc = "Bits 20:21 - PCDV10"]
    #[inline(always)]
    #[must_use]
    pub fn pcdv10(&mut self) -> PCDV10_W<20> {
        PCDV10_W::new(self)
    }
    #[doc = "Bits 22:23 - PCDV11"]
    #[inline(always)]
    #[must_use]
    pub fn pcdv11(&mut self) -> PCDV11_W<22> {
        PCDV11_W::new(self)
    }
    #[doc = "Bits 24:25 - PCDV12"]
    #[inline(always)]
    #[must_use]
    pub fn pcdv12(&mut self) -> PCDV12_W<24> {
        PCDV12_W::new(self)
    }
    #[doc = "Bits 26:27 - PCDV13"]
    #[inline(always)]
    #[must_use]
    pub fn pcdv13(&mut self) -> PCDV13_W<26> {
        PCDV13_W::new(self)
    }
    #[doc = "Bits 28:29 - PCDV14"]
    #[inline(always)]
    #[must_use]
    pub fn pcdv14(&mut self) -> PCDV14_W<28> {
        PCDV14_W::new(self)
    }
    #[doc = "Bits 30:31 - PCDV15"]
    #[inline(always)]
    #[must_use]
    pub fn pcdv15(&mut self) -> PCDV15_W<30> {
        PCDV15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCDRVR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdrvr](index.html) module"]
pub struct PCDRVR_SPEC;
impl crate::RegisterSpec for PCDRVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcdrvr::R](R) reader structure"]
impl crate::Readable for PCDRVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcdrvr::W](W) writer structure"]
impl crate::Writable for PCDRVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCDRVR to value 0"]
impl crate::Resettable for PCDRVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
