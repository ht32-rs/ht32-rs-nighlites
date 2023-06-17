#[doc = "Register `PDDRVR` reader"]
pub struct R(crate::R<PDDRVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDDRVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDDRVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDDRVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDDRVR` writer"]
pub struct W(crate::W<PDDRVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDDRVR_SPEC>;
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
impl From<crate::W<PDDRVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDDRVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDDV0` reader - PDDV0"]
pub type PDDV0_R = crate::FieldReader;
#[doc = "Field `PDDV0` writer - PDDV0"]
pub type PDDV0_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV1` reader - PDDV1"]
pub type PDDV1_R = crate::FieldReader;
#[doc = "Field `PDDV1` writer - PDDV1"]
pub type PDDV1_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV2` reader - PDDV2"]
pub type PDDV2_R = crate::FieldReader;
#[doc = "Field `PDDV2` writer - PDDV2"]
pub type PDDV2_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV3` reader - PDDV3"]
pub type PDDV3_R = crate::FieldReader;
#[doc = "Field `PDDV3` writer - PDDV3"]
pub type PDDV3_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV4` reader - PDDV4"]
pub type PDDV4_R = crate::FieldReader;
#[doc = "Field `PDDV4` writer - PDDV4"]
pub type PDDV4_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV5` reader - PDDV5"]
pub type PDDV5_R = crate::FieldReader;
#[doc = "Field `PDDV5` writer - PDDV5"]
pub type PDDV5_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV6` reader - PDDV6"]
pub type PDDV6_R = crate::FieldReader;
#[doc = "Field `PDDV6` writer - PDDV6"]
pub type PDDV6_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV7` reader - PDDV7"]
pub type PDDV7_R = crate::FieldReader;
#[doc = "Field `PDDV7` writer - PDDV7"]
pub type PDDV7_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV8` reader - PDDV8"]
pub type PDDV8_R = crate::FieldReader;
#[doc = "Field `PDDV8` writer - PDDV8"]
pub type PDDV8_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV9` reader - PDDV9"]
pub type PDDV9_R = crate::FieldReader;
#[doc = "Field `PDDV9` writer - PDDV9"]
pub type PDDV9_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV10` reader - PDDV10"]
pub type PDDV10_R = crate::FieldReader;
#[doc = "Field `PDDV10` writer - PDDV10"]
pub type PDDV10_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV11` reader - PDDV11"]
pub type PDDV11_R = crate::FieldReader;
#[doc = "Field `PDDV11` writer - PDDV11"]
pub type PDDV11_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV12` reader - PDDV12"]
pub type PDDV12_R = crate::FieldReader;
#[doc = "Field `PDDV12` writer - PDDV12"]
pub type PDDV12_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV13` reader - PDDV13"]
pub type PDDV13_R = crate::FieldReader;
#[doc = "Field `PDDV13` writer - PDDV13"]
pub type PDDV13_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV14` reader - PDDV14"]
pub type PDDV14_R = crate::FieldReader;
#[doc = "Field `PDDV14` writer - PDDV14"]
pub type PDDV14_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV15` reader - PDDV15"]
pub type PDDV15_R = crate::FieldReader;
#[doc = "Field `PDDV15` writer - PDDV15"]
pub type PDDV15_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - PDDV0"]
    #[inline(always)]
    pub fn pddv0(&self) -> PDDV0_R {
        PDDV0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PDDV1"]
    #[inline(always)]
    pub fn pddv1(&self) -> PDDV1_R {
        PDDV1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PDDV2"]
    #[inline(always)]
    pub fn pddv2(&self) -> PDDV2_R {
        PDDV2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PDDV3"]
    #[inline(always)]
    pub fn pddv3(&self) -> PDDV3_R {
        PDDV3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PDDV4"]
    #[inline(always)]
    pub fn pddv4(&self) -> PDDV4_R {
        PDDV4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PDDV5"]
    #[inline(always)]
    pub fn pddv5(&self) -> PDDV5_R {
        PDDV5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PDDV6"]
    #[inline(always)]
    pub fn pddv6(&self) -> PDDV6_R {
        PDDV6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PDDV7"]
    #[inline(always)]
    pub fn pddv7(&self) -> PDDV7_R {
        PDDV7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PDDV8"]
    #[inline(always)]
    pub fn pddv8(&self) -> PDDV8_R {
        PDDV8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PDDV9"]
    #[inline(always)]
    pub fn pddv9(&self) -> PDDV9_R {
        PDDV9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PDDV10"]
    #[inline(always)]
    pub fn pddv10(&self) -> PDDV10_R {
        PDDV10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PDDV11"]
    #[inline(always)]
    pub fn pddv11(&self) -> PDDV11_R {
        PDDV11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PDDV12"]
    #[inline(always)]
    pub fn pddv12(&self) -> PDDV12_R {
        PDDV12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PDDV13"]
    #[inline(always)]
    pub fn pddv13(&self) -> PDDV13_R {
        PDDV13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PDDV14"]
    #[inline(always)]
    pub fn pddv14(&self) -> PDDV14_R {
        PDDV14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PDDV15"]
    #[inline(always)]
    pub fn pddv15(&self) -> PDDV15_R {
        PDDV15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PDDV0"]
    #[inline(always)]
    #[must_use]
    pub fn pddv0(&mut self) -> PDDV0_W<0> {
        PDDV0_W::new(self)
    }
    #[doc = "Bits 2:3 - PDDV1"]
    #[inline(always)]
    #[must_use]
    pub fn pddv1(&mut self) -> PDDV1_W<2> {
        PDDV1_W::new(self)
    }
    #[doc = "Bits 4:5 - PDDV2"]
    #[inline(always)]
    #[must_use]
    pub fn pddv2(&mut self) -> PDDV2_W<4> {
        PDDV2_W::new(self)
    }
    #[doc = "Bits 6:7 - PDDV3"]
    #[inline(always)]
    #[must_use]
    pub fn pddv3(&mut self) -> PDDV3_W<6> {
        PDDV3_W::new(self)
    }
    #[doc = "Bits 8:9 - PDDV4"]
    #[inline(always)]
    #[must_use]
    pub fn pddv4(&mut self) -> PDDV4_W<8> {
        PDDV4_W::new(self)
    }
    #[doc = "Bits 10:11 - PDDV5"]
    #[inline(always)]
    #[must_use]
    pub fn pddv5(&mut self) -> PDDV5_W<10> {
        PDDV5_W::new(self)
    }
    #[doc = "Bits 12:13 - PDDV6"]
    #[inline(always)]
    #[must_use]
    pub fn pddv6(&mut self) -> PDDV6_W<12> {
        PDDV6_W::new(self)
    }
    #[doc = "Bits 14:15 - PDDV7"]
    #[inline(always)]
    #[must_use]
    pub fn pddv7(&mut self) -> PDDV7_W<14> {
        PDDV7_W::new(self)
    }
    #[doc = "Bits 16:17 - PDDV8"]
    #[inline(always)]
    #[must_use]
    pub fn pddv8(&mut self) -> PDDV8_W<16> {
        PDDV8_W::new(self)
    }
    #[doc = "Bits 18:19 - PDDV9"]
    #[inline(always)]
    #[must_use]
    pub fn pddv9(&mut self) -> PDDV9_W<18> {
        PDDV9_W::new(self)
    }
    #[doc = "Bits 20:21 - PDDV10"]
    #[inline(always)]
    #[must_use]
    pub fn pddv10(&mut self) -> PDDV10_W<20> {
        PDDV10_W::new(self)
    }
    #[doc = "Bits 22:23 - PDDV11"]
    #[inline(always)]
    #[must_use]
    pub fn pddv11(&mut self) -> PDDV11_W<22> {
        PDDV11_W::new(self)
    }
    #[doc = "Bits 24:25 - PDDV12"]
    #[inline(always)]
    #[must_use]
    pub fn pddv12(&mut self) -> PDDV12_W<24> {
        PDDV12_W::new(self)
    }
    #[doc = "Bits 26:27 - PDDV13"]
    #[inline(always)]
    #[must_use]
    pub fn pddv13(&mut self) -> PDDV13_W<26> {
        PDDV13_W::new(self)
    }
    #[doc = "Bits 28:29 - PDDV14"]
    #[inline(always)]
    #[must_use]
    pub fn pddv14(&mut self) -> PDDV14_W<28> {
        PDDV14_W::new(self)
    }
    #[doc = "Bits 30:31 - PDDV15"]
    #[inline(always)]
    #[must_use]
    pub fn pddv15(&mut self) -> PDDV15_W<30> {
        PDDV15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDDRVR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddrvr](index.html) module"]
pub struct PDDRVR_SPEC;
impl crate::RegisterSpec for PDDRVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pddrvr::R](R) reader structure"]
impl crate::Readable for PDDRVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pddrvr::W](W) writer structure"]
impl crate::Writable for PDDRVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDDRVR to value 0"]
impl crate::Resettable for PDDRVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
