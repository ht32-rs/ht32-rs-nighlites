#[doc = "Register `AFIO_GPDCFGHR` reader"]
pub struct R(crate::R<AFIO_GPDCFGHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_GPDCFGHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_GPDCFGHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_GPDCFGHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_GPDCFGHR` writer"]
pub struct W(crate::W<AFIO_GPDCFGHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_GPDCFGHR_SPEC>;
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
impl From<crate::W<AFIO_GPDCFGHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_GPDCFGHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDCFG8` reader - PDCFG8"]
pub type PDCFG8_R = crate::FieldReader;
#[doc = "Field `PDCFG8` writer - PDCFG8"]
pub type PDCFG8_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGHR_SPEC, 4, O>;
#[doc = "Field `PDCFG9` reader - PDCFG9"]
pub type PDCFG9_R = crate::FieldReader;
#[doc = "Field `PDCFG9` writer - PDCFG9"]
pub type PDCFG9_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGHR_SPEC, 4, O>;
#[doc = "Field `PDCFG10` reader - PDCFG10"]
pub type PDCFG10_R = crate::FieldReader;
#[doc = "Field `PDCFG10` writer - PDCFG10"]
pub type PDCFG10_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGHR_SPEC, 4, O>;
#[doc = "Field `PDCFG11` reader - PDCFG11"]
pub type PDCFG11_R = crate::FieldReader;
#[doc = "Field `PDCFG11` writer - PDCFG11"]
pub type PDCFG11_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGHR_SPEC, 4, O>;
#[doc = "Field `PDCFG12` reader - PDCFG12"]
pub type PDCFG12_R = crate::FieldReader;
#[doc = "Field `PDCFG12` writer - PDCFG12"]
pub type PDCFG12_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGHR_SPEC, 4, O>;
#[doc = "Field `PDCFG13` reader - PDCFG13"]
pub type PDCFG13_R = crate::FieldReader;
#[doc = "Field `PDCFG13` writer - PDCFG13"]
pub type PDCFG13_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGHR_SPEC, 4, O>;
#[doc = "Field `PDCFG14` reader - PDCFG14"]
pub type PDCFG14_R = crate::FieldReader;
#[doc = "Field `PDCFG14` writer - PDCFG14"]
pub type PDCFG14_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGHR_SPEC, 4, O>;
#[doc = "Field `PDCFG15` reader - PDCFG15"]
pub type PDCFG15_R = crate::FieldReader;
#[doc = "Field `PDCFG15` writer - PDCFG15"]
pub type PDCFG15_W<'a, const O: u8> = crate::FieldWriter<'a, AFIO_GPDCFGHR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PDCFG8"]
    #[inline(always)]
    pub fn pdcfg8(&self) -> PDCFG8_R {
        PDCFG8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PDCFG9"]
    #[inline(always)]
    pub fn pdcfg9(&self) -> PDCFG9_R {
        PDCFG9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PDCFG10"]
    #[inline(always)]
    pub fn pdcfg10(&self) -> PDCFG10_R {
        PDCFG10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PDCFG11"]
    #[inline(always)]
    pub fn pdcfg11(&self) -> PDCFG11_R {
        PDCFG11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PDCFG12"]
    #[inline(always)]
    pub fn pdcfg12(&self) -> PDCFG12_R {
        PDCFG12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PDCFG13"]
    #[inline(always)]
    pub fn pdcfg13(&self) -> PDCFG13_R {
        PDCFG13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PDCFG14"]
    #[inline(always)]
    pub fn pdcfg14(&self) -> PDCFG14_R {
        PDCFG14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PDCFG15"]
    #[inline(always)]
    pub fn pdcfg15(&self) -> PDCFG15_R {
        PDCFG15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDCFG8"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg8(&mut self) -> PDCFG8_W<0> {
        PDCFG8_W::new(self)
    }
    #[doc = "Bits 4:7 - PDCFG9"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg9(&mut self) -> PDCFG9_W<4> {
        PDCFG9_W::new(self)
    }
    #[doc = "Bits 8:11 - PDCFG10"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg10(&mut self) -> PDCFG10_W<8> {
        PDCFG10_W::new(self)
    }
    #[doc = "Bits 12:15 - PDCFG11"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg11(&mut self) -> PDCFG11_W<12> {
        PDCFG11_W::new(self)
    }
    #[doc = "Bits 16:19 - PDCFG12"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg12(&mut self) -> PDCFG12_W<16> {
        PDCFG12_W::new(self)
    }
    #[doc = "Bits 20:23 - PDCFG13"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg13(&mut self) -> PDCFG13_W<20> {
        PDCFG13_W::new(self)
    }
    #[doc = "Bits 24:27 - PDCFG14"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg14(&mut self) -> PDCFG14_W<24> {
        PDCFG14_W::new(self)
    }
    #[doc = "Bits 28:31 - PDCFG15"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg15(&mut self) -> PDCFG15_W<28> {
        PDCFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_GPDCFGHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpdcfghr](index.html) module"]
pub struct AFIO_GPDCFGHR_SPEC;
impl crate::RegisterSpec for AFIO_GPDCFGHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_gpdcfghr::R](R) reader structure"]
impl crate::Readable for AFIO_GPDCFGHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_gpdcfghr::W](W) writer structure"]
impl crate::Writable for AFIO_GPDCFGHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFIO_GPDCFGHR to value 0"]
impl crate::Resettable for AFIO_GPDCFGHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
