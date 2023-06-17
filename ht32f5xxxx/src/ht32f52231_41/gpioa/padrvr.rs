#[doc = "Register `PADRVR` reader"]
pub struct R(crate::R<PADRVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADRVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADRVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADRVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADRVR` writer"]
pub struct W(crate::W<PADRVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADRVR_SPEC>;
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
impl From<crate::W<PADRVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADRVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADV0` reader - PADV0"]
pub type PADV0_R = crate::FieldReader;
#[doc = "Field `PADV0` writer - PADV0"]
pub type PADV0_W<'a, const O: u8> = crate::FieldWriter<'a, PADRVR_SPEC, 2, O>;
#[doc = "Field `PADV1` reader - PADV1"]
pub type PADV1_R = crate::FieldReader;
#[doc = "Field `PADV1` writer - PADV1"]
pub type PADV1_W<'a, const O: u8> = crate::FieldWriter<'a, PADRVR_SPEC, 2, O>;
#[doc = "Field `PADV2` reader - PADV2"]
pub type PADV2_R = crate::FieldReader;
#[doc = "Field `PADV2` writer - PADV2"]
pub type PADV2_W<'a, const O: u8> = crate::FieldWriter<'a, PADRVR_SPEC, 2, O>;
#[doc = "Field `PADV3` reader - PADV3"]
pub type PADV3_R = crate::FieldReader;
#[doc = "Field `PADV3` writer - PADV3"]
pub type PADV3_W<'a, const O: u8> = crate::FieldWriter<'a, PADRVR_SPEC, 2, O>;
#[doc = "Field `PADV4` reader - PADV4"]
pub type PADV4_R = crate::FieldReader;
#[doc = "Field `PADV4` writer - PADV4"]
pub type PADV4_W<'a, const O: u8> = crate::FieldWriter<'a, PADRVR_SPEC, 2, O>;
#[doc = "Field `PADV5` reader - PADV5"]
pub type PADV5_R = crate::FieldReader;
#[doc = "Field `PADV5` writer - PADV5"]
pub type PADV5_W<'a, const O: u8> = crate::FieldWriter<'a, PADRVR_SPEC, 2, O>;
#[doc = "Field `PADV6` reader - PADV6"]
pub type PADV6_R = crate::FieldReader;
#[doc = "Field `PADV6` writer - PADV6"]
pub type PADV6_W<'a, const O: u8> = crate::FieldWriter<'a, PADRVR_SPEC, 2, O>;
#[doc = "Field `PADV7` reader - PADV7"]
pub type PADV7_R = crate::FieldReader;
#[doc = "Field `PADV7` writer - PADV7"]
pub type PADV7_W<'a, const O: u8> = crate::FieldWriter<'a, PADRVR_SPEC, 2, O>;
#[doc = "Field `PADV8` reader - PADV8"]
pub type PADV8_R = crate::FieldReader;
#[doc = "Field `PADV8` writer - PADV8"]
pub type PADV8_W<'a, const O: u8> = crate::FieldWriter<'a, PADRVR_SPEC, 2, O>;
#[doc = "Field `PADV9` reader - PADV9"]
pub type PADV9_R = crate::FieldReader;
#[doc = "Field `PADV9` writer - PADV9"]
pub type PADV9_W<'a, const O: u8> = crate::FieldWriter<'a, PADRVR_SPEC, 2, O>;
#[doc = "Field `PADV10` reader - PADV10"]
pub type PADV10_R = crate::FieldReader;
#[doc = "Field `PADV10` writer - PADV10"]
pub type PADV10_W<'a, const O: u8> = crate::FieldWriter<'a, PADRVR_SPEC, 2, O>;
#[doc = "Field `PADV11` reader - PADV11"]
pub type PADV11_R = crate::FieldReader;
#[doc = "Field `PADV11` writer - PADV11"]
pub type PADV11_W<'a, const O: u8> = crate::FieldWriter<'a, PADRVR_SPEC, 2, O>;
#[doc = "Field `PADV12` reader - PADV12"]
pub type PADV12_R = crate::FieldReader;
#[doc = "Field `PADV12` writer - PADV12"]
pub type PADV12_W<'a, const O: u8> = crate::FieldWriter<'a, PADRVR_SPEC, 2, O>;
#[doc = "Field `PADV13` reader - PADV13"]
pub type PADV13_R = crate::FieldReader;
#[doc = "Field `PADV13` writer - PADV13"]
pub type PADV13_W<'a, const O: u8> = crate::FieldWriter<'a, PADRVR_SPEC, 2, O>;
#[doc = "Field `PADV14` reader - PADV14"]
pub type PADV14_R = crate::FieldReader;
#[doc = "Field `PADV14` writer - PADV14"]
pub type PADV14_W<'a, const O: u8> = crate::FieldWriter<'a, PADRVR_SPEC, 2, O>;
#[doc = "Field `PADV15` reader - PADV15"]
pub type PADV15_R = crate::FieldReader;
#[doc = "Field `PADV15` writer - PADV15"]
pub type PADV15_W<'a, const O: u8> = crate::FieldWriter<'a, PADRVR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - PADV0"]
    #[inline(always)]
    pub fn padv0(&self) -> PADV0_R {
        PADV0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PADV1"]
    #[inline(always)]
    pub fn padv1(&self) -> PADV1_R {
        PADV1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PADV2"]
    #[inline(always)]
    pub fn padv2(&self) -> PADV2_R {
        PADV2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PADV3"]
    #[inline(always)]
    pub fn padv3(&self) -> PADV3_R {
        PADV3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PADV4"]
    #[inline(always)]
    pub fn padv4(&self) -> PADV4_R {
        PADV4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PADV5"]
    #[inline(always)]
    pub fn padv5(&self) -> PADV5_R {
        PADV5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PADV6"]
    #[inline(always)]
    pub fn padv6(&self) -> PADV6_R {
        PADV6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PADV7"]
    #[inline(always)]
    pub fn padv7(&self) -> PADV7_R {
        PADV7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PADV8"]
    #[inline(always)]
    pub fn padv8(&self) -> PADV8_R {
        PADV8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PADV9"]
    #[inline(always)]
    pub fn padv9(&self) -> PADV9_R {
        PADV9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PADV10"]
    #[inline(always)]
    pub fn padv10(&self) -> PADV10_R {
        PADV10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PADV11"]
    #[inline(always)]
    pub fn padv11(&self) -> PADV11_R {
        PADV11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PADV12"]
    #[inline(always)]
    pub fn padv12(&self) -> PADV12_R {
        PADV12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PADV13"]
    #[inline(always)]
    pub fn padv13(&self) -> PADV13_R {
        PADV13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PADV14"]
    #[inline(always)]
    pub fn padv14(&self) -> PADV14_R {
        PADV14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PADV15"]
    #[inline(always)]
    pub fn padv15(&self) -> PADV15_R {
        PADV15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PADV0"]
    #[inline(always)]
    #[must_use]
    pub fn padv0(&mut self) -> PADV0_W<0> {
        PADV0_W::new(self)
    }
    #[doc = "Bits 2:3 - PADV1"]
    #[inline(always)]
    #[must_use]
    pub fn padv1(&mut self) -> PADV1_W<2> {
        PADV1_W::new(self)
    }
    #[doc = "Bits 4:5 - PADV2"]
    #[inline(always)]
    #[must_use]
    pub fn padv2(&mut self) -> PADV2_W<4> {
        PADV2_W::new(self)
    }
    #[doc = "Bits 6:7 - PADV3"]
    #[inline(always)]
    #[must_use]
    pub fn padv3(&mut self) -> PADV3_W<6> {
        PADV3_W::new(self)
    }
    #[doc = "Bits 8:9 - PADV4"]
    #[inline(always)]
    #[must_use]
    pub fn padv4(&mut self) -> PADV4_W<8> {
        PADV4_W::new(self)
    }
    #[doc = "Bits 10:11 - PADV5"]
    #[inline(always)]
    #[must_use]
    pub fn padv5(&mut self) -> PADV5_W<10> {
        PADV5_W::new(self)
    }
    #[doc = "Bits 12:13 - PADV6"]
    #[inline(always)]
    #[must_use]
    pub fn padv6(&mut self) -> PADV6_W<12> {
        PADV6_W::new(self)
    }
    #[doc = "Bits 14:15 - PADV7"]
    #[inline(always)]
    #[must_use]
    pub fn padv7(&mut self) -> PADV7_W<14> {
        PADV7_W::new(self)
    }
    #[doc = "Bits 16:17 - PADV8"]
    #[inline(always)]
    #[must_use]
    pub fn padv8(&mut self) -> PADV8_W<16> {
        PADV8_W::new(self)
    }
    #[doc = "Bits 18:19 - PADV9"]
    #[inline(always)]
    #[must_use]
    pub fn padv9(&mut self) -> PADV9_W<18> {
        PADV9_W::new(self)
    }
    #[doc = "Bits 20:21 - PADV10"]
    #[inline(always)]
    #[must_use]
    pub fn padv10(&mut self) -> PADV10_W<20> {
        PADV10_W::new(self)
    }
    #[doc = "Bits 22:23 - PADV11"]
    #[inline(always)]
    #[must_use]
    pub fn padv11(&mut self) -> PADV11_W<22> {
        PADV11_W::new(self)
    }
    #[doc = "Bits 24:25 - PADV12"]
    #[inline(always)]
    #[must_use]
    pub fn padv12(&mut self) -> PADV12_W<24> {
        PADV12_W::new(self)
    }
    #[doc = "Bits 26:27 - PADV13"]
    #[inline(always)]
    #[must_use]
    pub fn padv13(&mut self) -> PADV13_W<26> {
        PADV13_W::new(self)
    }
    #[doc = "Bits 28:29 - PADV14"]
    #[inline(always)]
    #[must_use]
    pub fn padv14(&mut self) -> PADV14_W<28> {
        PADV14_W::new(self)
    }
    #[doc = "Bits 30:31 - PADV15"]
    #[inline(always)]
    #[must_use]
    pub fn padv15(&mut self) -> PADV15_W<30> {
        PADV15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PADRVR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padrvr](index.html) module"]
pub struct PADRVR_SPEC;
impl crate::RegisterSpec for PADRVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padrvr::R](R) reader structure"]
impl crate::Readable for PADRVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padrvr::W](W) writer structure"]
impl crate::Writable for PADRVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADRVR to value 0"]
impl crate::Resettable for PADRVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
