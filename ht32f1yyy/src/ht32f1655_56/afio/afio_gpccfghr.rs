#[doc = "Register `AFIO_GPCCFGHR` reader"]
pub struct R(crate::R<AFIO_GPCCFGHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_GPCCFGHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_GPCCFGHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_GPCCFGHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_GPCCFGHR` writer"]
pub struct W(crate::W<AFIO_GPCCFGHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_GPCCFGHR_SPEC>;
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
impl From<crate::W<AFIO_GPCCFGHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_GPCCFGHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCCFG8` reader - PCCFG8"]
pub type PCCFG8_R = crate::FieldReader;
#[doc = "Field `PCCFG8` writer - PCCFG8"]
pub type PCCFG8_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGHR_SPEC, 4, O>;
#[doc = "Field `PCCFG9` reader - PCCFG9"]
pub type PCCFG9_R = crate::FieldReader;
#[doc = "Field `PCCFG9` writer - PCCFG9"]
pub type PCCFG9_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGHR_SPEC, 4, O>;
#[doc = "Field `PCCFG10` reader - PCCFG10"]
pub type PCCFG10_R = crate::FieldReader;
#[doc = "Field `PCCFG10` writer - PCCFG10"]
pub type PCCFG10_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGHR_SPEC, 4, O>;
#[doc = "Field `PCCFG11` reader - PCCFG11"]
pub type PCCFG11_R = crate::FieldReader;
#[doc = "Field `PCCFG11` writer - PCCFG11"]
pub type PCCFG11_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGHR_SPEC, 4, O>;
#[doc = "Field `PCCFG12` reader - PCCFG12"]
pub type PCCFG12_R = crate::FieldReader;
#[doc = "Field `PCCFG12` writer - PCCFG12"]
pub type PCCFG12_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGHR_SPEC, 4, O>;
#[doc = "Field `PCCFG13` reader - PCCFG13"]
pub type PCCFG13_R = crate::FieldReader;
#[doc = "Field `PCCFG13` writer - PCCFG13"]
pub type PCCFG13_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGHR_SPEC, 4, O>;
#[doc = "Field `PCCFG14` reader - PCCFG14"]
pub type PCCFG14_R = crate::FieldReader;
#[doc = "Field `PCCFG14` writer - PCCFG14"]
pub type PCCFG14_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGHR_SPEC, 4, O>;
#[doc = "Field `PCCFG15` reader - PCCFG15"]
pub type PCCFG15_R = crate::FieldReader;
#[doc = "Field `PCCFG15` writer - PCCFG15"]
pub type PCCFG15_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPCCFGHR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PCCFG8"]
    #[inline(always)]
    pub fn pccfg8(&self) -> PCCFG8_R {
        PCCFG8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PCCFG9"]
    #[inline(always)]
    pub fn pccfg9(&self) -> PCCFG9_R {
        PCCFG9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PCCFG10"]
    #[inline(always)]
    pub fn pccfg10(&self) -> PCCFG10_R {
        PCCFG10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PCCFG11"]
    #[inline(always)]
    pub fn pccfg11(&self) -> PCCFG11_R {
        PCCFG11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PCCFG12"]
    #[inline(always)]
    pub fn pccfg12(&self) -> PCCFG12_R {
        PCCFG12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PCCFG13"]
    #[inline(always)]
    pub fn pccfg13(&self) -> PCCFG13_R {
        PCCFG13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PCCFG14"]
    #[inline(always)]
    pub fn pccfg14(&self) -> PCCFG14_R {
        PCCFG14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PCCFG15"]
    #[inline(always)]
    pub fn pccfg15(&self) -> PCCFG15_R {
        PCCFG15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PCCFG8"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg8(&mut self) -> PCCFG8_W<0> {
        PCCFG8_W::new(self)
    }
    #[doc = "Bits 4:7 - PCCFG9"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg9(&mut self) -> PCCFG9_W<4> {
        PCCFG9_W::new(self)
    }
    #[doc = "Bits 8:11 - PCCFG10"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg10(&mut self) -> PCCFG10_W<8> {
        PCCFG10_W::new(self)
    }
    #[doc = "Bits 12:15 - PCCFG11"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg11(&mut self) -> PCCFG11_W<12> {
        PCCFG11_W::new(self)
    }
    #[doc = "Bits 16:19 - PCCFG12"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg12(&mut self) -> PCCFG12_W<16> {
        PCCFG12_W::new(self)
    }
    #[doc = "Bits 20:23 - PCCFG13"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg13(&mut self) -> PCCFG13_W<20> {
        PCCFG13_W::new(self)
    }
    #[doc = "Bits 24:27 - PCCFG14"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg14(&mut self) -> PCCFG14_W<24> {
        PCCFG14_W::new(self)
    }
    #[doc = "Bits 28:31 - PCCFG15"]
    #[inline(always)]
    #[must_use]
    pub fn pccfg15(&mut self) -> PCCFG15_W<28> {
        PCCFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_GPCCFGHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpccfghr](index.html) module"]
pub struct AFIO_GPCCFGHR_SPEC;
impl crate::RegisterSpec for AFIO_GPCCFGHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_gpccfghr::R](R) reader structure"]
impl crate::Readable for AFIO_GPCCFGHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_gpccfghr::W](W) writer structure"]
impl crate::Writable for AFIO_GPCCFGHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFIO_GPCCFGHR to value 0"]
impl crate::Resettable for AFIO_GPCCFGHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
