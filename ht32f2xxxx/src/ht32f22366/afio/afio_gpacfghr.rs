#[doc = "Register `AFIO_GPACFGHR` reader"]
pub struct R(crate::R<AFIO_GPACFGHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_GPACFGHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_GPACFGHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_GPACFGHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_GPACFGHR` writer"]
pub struct W(crate::W<AFIO_GPACFGHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_GPACFGHR_SPEC>;
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
impl From<crate::W<AFIO_GPACFGHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_GPACFGHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PACFG8` reader - PACFG8"]
pub type PACFG8_R = crate::FieldReader;
#[doc = "Field `PACFG8` writer - PACFG8"]
pub type PACFG8_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGHR_SPEC, 4, O>;
#[doc = "Field `PACFG9` reader - PACFG9"]
pub type PACFG9_R = crate::FieldReader;
#[doc = "Field `PACFG9` writer - PACFG9"]
pub type PACFG9_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGHR_SPEC, 4, O>;
#[doc = "Field `PACFG10` reader - PACFG10"]
pub type PACFG10_R = crate::FieldReader;
#[doc = "Field `PACFG10` writer - PACFG10"]
pub type PACFG10_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGHR_SPEC, 4, O>;
#[doc = "Field `PACFG11` reader - PACFG11"]
pub type PACFG11_R = crate::FieldReader;
#[doc = "Field `PACFG11` writer - PACFG11"]
pub type PACFG11_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGHR_SPEC, 4, O>;
#[doc = "Field `PACFG12` reader - PACFG12"]
pub type PACFG12_R = crate::FieldReader;
#[doc = "Field `PACFG12` writer - PACFG12"]
pub type PACFG12_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGHR_SPEC, 4, O>;
#[doc = "Field `PACFG13` reader - PACFG13"]
pub type PACFG13_R = crate::FieldReader;
#[doc = "Field `PACFG13` writer - PACFG13"]
pub type PACFG13_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGHR_SPEC, 4, O>;
#[doc = "Field `PACFG14` reader - PACFG14"]
pub type PACFG14_R = crate::FieldReader;
#[doc = "Field `PACFG14` writer - PACFG14"]
pub type PACFG14_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGHR_SPEC, 4, O>;
#[doc = "Field `PACFG15` reader - PACFG15"]
pub type PACFG15_R = crate::FieldReader;
#[doc = "Field `PACFG15` writer - PACFG15"]
pub type PACFG15_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPACFGHR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PACFG8"]
    #[inline(always)]
    pub fn pacfg8(&self) -> PACFG8_R {
        PACFG8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PACFG9"]
    #[inline(always)]
    pub fn pacfg9(&self) -> PACFG9_R {
        PACFG9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PACFG10"]
    #[inline(always)]
    pub fn pacfg10(&self) -> PACFG10_R {
        PACFG10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PACFG11"]
    #[inline(always)]
    pub fn pacfg11(&self) -> PACFG11_R {
        PACFG11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PACFG12"]
    #[inline(always)]
    pub fn pacfg12(&self) -> PACFG12_R {
        PACFG12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PACFG13"]
    #[inline(always)]
    pub fn pacfg13(&self) -> PACFG13_R {
        PACFG13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PACFG14"]
    #[inline(always)]
    pub fn pacfg14(&self) -> PACFG14_R {
        PACFG14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PACFG15"]
    #[inline(always)]
    pub fn pacfg15(&self) -> PACFG15_R {
        PACFG15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PACFG8"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg8(&mut self) -> PACFG8_W<0> {
        PACFG8_W::new(self)
    }
    #[doc = "Bits 4:7 - PACFG9"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg9(&mut self) -> PACFG9_W<4> {
        PACFG9_W::new(self)
    }
    #[doc = "Bits 8:11 - PACFG10"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg10(&mut self) -> PACFG10_W<8> {
        PACFG10_W::new(self)
    }
    #[doc = "Bits 12:15 - PACFG11"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg11(&mut self) -> PACFG11_W<12> {
        PACFG11_W::new(self)
    }
    #[doc = "Bits 16:19 - PACFG12"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg12(&mut self) -> PACFG12_W<16> {
        PACFG12_W::new(self)
    }
    #[doc = "Bits 20:23 - PACFG13"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg13(&mut self) -> PACFG13_W<20> {
        PACFG13_W::new(self)
    }
    #[doc = "Bits 24:27 - PACFG14"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg14(&mut self) -> PACFG14_W<24> {
        PACFG14_W::new(self)
    }
    #[doc = "Bits 28:31 - PACFG15"]
    #[inline(always)]
    #[must_use]
    pub fn pacfg15(&mut self) -> PACFG15_W<28> {
        PACFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_GPACFGHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpacfghr](index.html) module"]
pub struct AFIO_GPACFGHR_SPEC;
impl crate::RegisterSpec for AFIO_GPACFGHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_gpacfghr::R](R) reader structure"]
impl crate::Readable for AFIO_GPACFGHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_gpacfghr::W](W) writer structure"]
impl crate::Writable for AFIO_GPACFGHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFIO_GPACFGHR to value 0"]
impl crate::Resettable for AFIO_GPACFGHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
