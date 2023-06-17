#[doc = "Register `GPECFGHR` reader"]
pub struct R(crate::R<GPECFGHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPECFGHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPECFGHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPECFGHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPECFGHR` writer"]
pub struct W(crate::W<GPECFGHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPECFGHR_SPEC>;
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
impl From<crate::W<GPECFGHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPECFGHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PxCFG8` reader - PxCFG8"]
pub type PX_CFG8_R = crate::FieldReader;
#[doc = "Field `PxCFG8` writer - PxCFG8"]
pub type PX_CFG8_W<'a, const O: u8> = crate::FieldWriter<'a, GPECFGHR_SPEC, 4, O>;
#[doc = "Field `PxCFG9` reader - PxCFG9"]
pub type PX_CFG9_R = crate::FieldReader;
#[doc = "Field `PxCFG9` writer - PxCFG9"]
pub type PX_CFG9_W<'a, const O: u8> = crate::FieldWriter<'a, GPECFGHR_SPEC, 4, O>;
#[doc = "Field `PxCFG10` reader - PxCFG10"]
pub type PX_CFG10_R = crate::FieldReader;
#[doc = "Field `PxCFG10` writer - PxCFG10"]
pub type PX_CFG10_W<'a, const O: u8> = crate::FieldWriter<'a, GPECFGHR_SPEC, 4, O>;
#[doc = "Field `PxCFG11` reader - PxCFG11"]
pub type PX_CFG11_R = crate::FieldReader;
#[doc = "Field `PxCFG11` writer - PxCFG11"]
pub type PX_CFG11_W<'a, const O: u8> = crate::FieldWriter<'a, GPECFGHR_SPEC, 4, O>;
#[doc = "Field `PxCFG12` reader - PxCFG12"]
pub type PX_CFG12_R = crate::FieldReader;
#[doc = "Field `PxCFG12` writer - PxCFG12"]
pub type PX_CFG12_W<'a, const O: u8> = crate::FieldWriter<'a, GPECFGHR_SPEC, 4, O>;
#[doc = "Field `PxCFG13` reader - PxCFG13"]
pub type PX_CFG13_R = crate::FieldReader;
#[doc = "Field `PxCFG13` writer - PxCFG13"]
pub type PX_CFG13_W<'a, const O: u8> = crate::FieldWriter<'a, GPECFGHR_SPEC, 4, O>;
#[doc = "Field `PxCFG14` reader - PxCFG14"]
pub type PX_CFG14_R = crate::FieldReader;
#[doc = "Field `PxCFG14` writer - PxCFG14"]
pub type PX_CFG14_W<'a, const O: u8> = crate::FieldWriter<'a, GPECFGHR_SPEC, 4, O>;
#[doc = "Field `PxCFG15` reader - PxCFG15"]
pub type PX_CFG15_R = crate::FieldReader;
#[doc = "Field `PxCFG15` writer - PxCFG15"]
pub type PX_CFG15_W<'a, const O: u8> = crate::FieldWriter<'a, GPECFGHR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PxCFG8"]
    #[inline(always)]
    pub fn px_cfg8(&self) -> PX_CFG8_R {
        PX_CFG8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PxCFG9"]
    #[inline(always)]
    pub fn px_cfg9(&self) -> PX_CFG9_R {
        PX_CFG9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PxCFG10"]
    #[inline(always)]
    pub fn px_cfg10(&self) -> PX_CFG10_R {
        PX_CFG10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PxCFG11"]
    #[inline(always)]
    pub fn px_cfg11(&self) -> PX_CFG11_R {
        PX_CFG11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PxCFG12"]
    #[inline(always)]
    pub fn px_cfg12(&self) -> PX_CFG12_R {
        PX_CFG12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PxCFG13"]
    #[inline(always)]
    pub fn px_cfg13(&self) -> PX_CFG13_R {
        PX_CFG13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PxCFG14"]
    #[inline(always)]
    pub fn px_cfg14(&self) -> PX_CFG14_R {
        PX_CFG14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PxCFG15"]
    #[inline(always)]
    pub fn px_cfg15(&self) -> PX_CFG15_R {
        PX_CFG15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PxCFG8"]
    #[inline(always)]
    #[must_use]
    pub fn px_cfg8(&mut self) -> PX_CFG8_W<0> {
        PX_CFG8_W::new(self)
    }
    #[doc = "Bits 4:7 - PxCFG9"]
    #[inline(always)]
    #[must_use]
    pub fn px_cfg9(&mut self) -> PX_CFG9_W<4> {
        PX_CFG9_W::new(self)
    }
    #[doc = "Bits 8:11 - PxCFG10"]
    #[inline(always)]
    #[must_use]
    pub fn px_cfg10(&mut self) -> PX_CFG10_W<8> {
        PX_CFG10_W::new(self)
    }
    #[doc = "Bits 12:15 - PxCFG11"]
    #[inline(always)]
    #[must_use]
    pub fn px_cfg11(&mut self) -> PX_CFG11_W<12> {
        PX_CFG11_W::new(self)
    }
    #[doc = "Bits 16:19 - PxCFG12"]
    #[inline(always)]
    #[must_use]
    pub fn px_cfg12(&mut self) -> PX_CFG12_W<16> {
        PX_CFG12_W::new(self)
    }
    #[doc = "Bits 20:23 - PxCFG13"]
    #[inline(always)]
    #[must_use]
    pub fn px_cfg13(&mut self) -> PX_CFG13_W<20> {
        PX_CFG13_W::new(self)
    }
    #[doc = "Bits 24:27 - PxCFG14"]
    #[inline(always)]
    #[must_use]
    pub fn px_cfg14(&mut self) -> PX_CFG14_W<24> {
        PX_CFG14_W::new(self)
    }
    #[doc = "Bits 28:31 - PxCFG15"]
    #[inline(always)]
    #[must_use]
    pub fn px_cfg15(&mut self) -> PX_CFG15_W<28> {
        PX_CFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPECFGHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpecfghr](index.html) module"]
pub struct GPECFGHR_SPEC;
impl crate::RegisterSpec for GPECFGHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpecfghr::R](R) reader structure"]
impl crate::Readable for GPECFGHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpecfghr::W](W) writer structure"]
impl crate::Writable for GPECFGHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPECFGHR to value 0"]
impl crate::Resettable for GPECFGHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
