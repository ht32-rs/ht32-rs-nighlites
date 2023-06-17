#[doc = "Register `AFIO_GPBCFGHR` reader"]
pub struct R(crate::R<AFIO_GPBCFGHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_GPBCFGHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_GPBCFGHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_GPBCFGHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_GPBCFGHR` writer"]
pub struct W(crate::W<AFIO_GPBCFGHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_GPBCFGHR_SPEC>;
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
impl From<crate::W<AFIO_GPBCFGHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_GPBCFGHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBCFG8` reader - PBCFG8"]
pub type PBCFG8_R = crate::FieldReader;
#[doc = "Field `PBCFG8` writer - PBCFG8"]
pub type PBCFG8_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGHR_SPEC, 4, O>;
#[doc = "Field `PBCFG9` reader - PBCFG9"]
pub type PBCFG9_R = crate::FieldReader;
#[doc = "Field `PBCFG9` writer - PBCFG9"]
pub type PBCFG9_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGHR_SPEC, 4, O>;
#[doc = "Field `PBCFG10` reader - PBCFG10"]
pub type PBCFG10_R = crate::FieldReader;
#[doc = "Field `PBCFG10` writer - PBCFG10"]
pub type PBCFG10_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGHR_SPEC, 4, O>;
#[doc = "Field `PBCFG11` reader - PBCFG11"]
pub type PBCFG11_R = crate::FieldReader;
#[doc = "Field `PBCFG11` writer - PBCFG11"]
pub type PBCFG11_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGHR_SPEC, 4, O>;
#[doc = "Field `PBCFG12` reader - PBCFG12"]
pub type PBCFG12_R = crate::FieldReader;
#[doc = "Field `PBCFG12` writer - PBCFG12"]
pub type PBCFG12_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGHR_SPEC, 4, O>;
#[doc = "Field `PBCFG13` reader - PBCFG13"]
pub type PBCFG13_R = crate::FieldReader;
#[doc = "Field `PBCFG13` writer - PBCFG13"]
pub type PBCFG13_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGHR_SPEC, 4, O>;
#[doc = "Field `PBCFG14` reader - PBCFG14"]
pub type PBCFG14_R = crate::FieldReader;
#[doc = "Field `PBCFG14` writer - PBCFG14"]
pub type PBCFG14_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGHR_SPEC, 4, O>;
#[doc = "Field `PBCFG15` reader - PBCFG15"]
pub type PBCFG15_R = crate::FieldReader;
#[doc = "Field `PBCFG15` writer - PBCFG15"]
pub type PBCFG15_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPBCFGHR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PBCFG8"]
    #[inline(always)]
    pub fn pbcfg8(&self) -> PBCFG8_R {
        PBCFG8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PBCFG9"]
    #[inline(always)]
    pub fn pbcfg9(&self) -> PBCFG9_R {
        PBCFG9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PBCFG10"]
    #[inline(always)]
    pub fn pbcfg10(&self) -> PBCFG10_R {
        PBCFG10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PBCFG11"]
    #[inline(always)]
    pub fn pbcfg11(&self) -> PBCFG11_R {
        PBCFG11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PBCFG12"]
    #[inline(always)]
    pub fn pbcfg12(&self) -> PBCFG12_R {
        PBCFG12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PBCFG13"]
    #[inline(always)]
    pub fn pbcfg13(&self) -> PBCFG13_R {
        PBCFG13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PBCFG14"]
    #[inline(always)]
    pub fn pbcfg14(&self) -> PBCFG14_R {
        PBCFG14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PBCFG15"]
    #[inline(always)]
    pub fn pbcfg15(&self) -> PBCFG15_R {
        PBCFG15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PBCFG8"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg8(&mut self) -> PBCFG8_W<0> {
        PBCFG8_W::new(self)
    }
    #[doc = "Bits 4:7 - PBCFG9"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg9(&mut self) -> PBCFG9_W<4> {
        PBCFG9_W::new(self)
    }
    #[doc = "Bits 8:11 - PBCFG10"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg10(&mut self) -> PBCFG10_W<8> {
        PBCFG10_W::new(self)
    }
    #[doc = "Bits 12:15 - PBCFG11"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg11(&mut self) -> PBCFG11_W<12> {
        PBCFG11_W::new(self)
    }
    #[doc = "Bits 16:19 - PBCFG12"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg12(&mut self) -> PBCFG12_W<16> {
        PBCFG12_W::new(self)
    }
    #[doc = "Bits 20:23 - PBCFG13"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg13(&mut self) -> PBCFG13_W<20> {
        PBCFG13_W::new(self)
    }
    #[doc = "Bits 24:27 - PBCFG14"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg14(&mut self) -> PBCFG14_W<24> {
        PBCFG14_W::new(self)
    }
    #[doc = "Bits 28:31 - PBCFG15"]
    #[inline(always)]
    #[must_use]
    pub fn pbcfg15(&mut self) -> PBCFG15_W<28> {
        PBCFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_GPBCFGHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpbcfghr](index.html) module"]
pub struct AFIO_GPBCFGHR_SPEC;
impl crate::RegisterSpec for AFIO_GPBCFGHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_gpbcfghr::R](R) reader structure"]
impl crate::Readable for AFIO_GPBCFGHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_gpbcfghr::W](W) writer structure"]
impl crate::Writable for AFIO_GPBCFGHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFIO_GPBCFGHR to value 0"]
impl crate::Resettable for AFIO_GPBCFGHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
