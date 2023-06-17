#[doc = "Register `AFIO_GPECFGHR` reader"]
pub struct R(crate::R<AFIO_GPECFGHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_GPECFGHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_GPECFGHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_GPECFGHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_GPECFGHR` writer"]
pub struct W(crate::W<AFIO_GPECFGHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_GPECFGHR_SPEC>;
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
impl From<crate::W<AFIO_GPECFGHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_GPECFGHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PECFG8` reader - PECFG8"]
pub type PECFG8_R = crate::FieldReader;
#[doc = "Field `PECFG8` writer - PECFG8"]
pub type PECFG8_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGHR_SPEC, 4, O>;
#[doc = "Field `PECFG9` reader - PECFG9"]
pub type PECFG9_R = crate::FieldReader;
#[doc = "Field `PECFG9` writer - PECFG9"]
pub type PECFG9_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGHR_SPEC, 4, O>;
#[doc = "Field `PECFG10` reader - PECFG10"]
pub type PECFG10_R = crate::FieldReader;
#[doc = "Field `PECFG10` writer - PECFG10"]
pub type PECFG10_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGHR_SPEC, 4, O>;
#[doc = "Field `PECFG11` reader - PECFG11"]
pub type PECFG11_R = crate::FieldReader;
#[doc = "Field `PECFG11` writer - PECFG11"]
pub type PECFG11_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGHR_SPEC, 4, O>;
#[doc = "Field `PECFG12` reader - PECFG12"]
pub type PECFG12_R = crate::FieldReader;
#[doc = "Field `PECFG12` writer - PECFG12"]
pub type PECFG12_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGHR_SPEC, 4, O>;
#[doc = "Field `PECFG13` reader - PECFG13"]
pub type PECFG13_R = crate::FieldReader;
#[doc = "Field `PECFG13` writer - PECFG13"]
pub type PECFG13_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGHR_SPEC, 4, O>;
#[doc = "Field `PECFG14` reader - PECFG14"]
pub type PECFG14_R = crate::FieldReader;
#[doc = "Field `PECFG14` writer - PECFG14"]
pub type PECFG14_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGHR_SPEC, 4, O>;
#[doc = "Field `PECFG15` reader - PECFG15"]
pub type PECFG15_R = crate::FieldReader;
#[doc = "Field `PECFG15` writer - PECFG15"]
pub type PECFG15_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPECFGHR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PECFG8"]
    #[inline(always)]
    pub fn pecfg8(&self) -> PECFG8_R {
        PECFG8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PECFG9"]
    #[inline(always)]
    pub fn pecfg9(&self) -> PECFG9_R {
        PECFG9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PECFG10"]
    #[inline(always)]
    pub fn pecfg10(&self) -> PECFG10_R {
        PECFG10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PECFG11"]
    #[inline(always)]
    pub fn pecfg11(&self) -> PECFG11_R {
        PECFG11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PECFG12"]
    #[inline(always)]
    pub fn pecfg12(&self) -> PECFG12_R {
        PECFG12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PECFG13"]
    #[inline(always)]
    pub fn pecfg13(&self) -> PECFG13_R {
        PECFG13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PECFG14"]
    #[inline(always)]
    pub fn pecfg14(&self) -> PECFG14_R {
        PECFG14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PECFG15"]
    #[inline(always)]
    pub fn pecfg15(&self) -> PECFG15_R {
        PECFG15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PECFG8"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg8(&mut self) -> PECFG8_W<0> {
        PECFG8_W::new(self)
    }
    #[doc = "Bits 4:7 - PECFG9"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg9(&mut self) -> PECFG9_W<4> {
        PECFG9_W::new(self)
    }
    #[doc = "Bits 8:11 - PECFG10"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg10(&mut self) -> PECFG10_W<8> {
        PECFG10_W::new(self)
    }
    #[doc = "Bits 12:15 - PECFG11"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg11(&mut self) -> PECFG11_W<12> {
        PECFG11_W::new(self)
    }
    #[doc = "Bits 16:19 - PECFG12"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg12(&mut self) -> PECFG12_W<16> {
        PECFG12_W::new(self)
    }
    #[doc = "Bits 20:23 - PECFG13"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg13(&mut self) -> PECFG13_W<20> {
        PECFG13_W::new(self)
    }
    #[doc = "Bits 24:27 - PECFG14"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg14(&mut self) -> PECFG14_W<24> {
        PECFG14_W::new(self)
    }
    #[doc = "Bits 28:31 - PECFG15"]
    #[inline(always)]
    #[must_use]
    pub fn pecfg15(&mut self) -> PECFG15_W<28> {
        PECFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_GPECFGHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpecfghr](index.html) module"]
pub struct AFIO_GPECFGHR_SPEC;
impl crate::RegisterSpec for AFIO_GPECFGHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_gpecfghr::R](R) reader structure"]
impl crate::Readable for AFIO_GPECFGHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_gpecfghr::W](W) writer structure"]
impl crate::Writable for AFIO_GPECFGHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFIO_GPECFGHR to value 0"]
impl crate::Resettable for AFIO_GPECFGHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
