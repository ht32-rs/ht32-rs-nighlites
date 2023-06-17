#[doc = "Register `ADCLST1` reader"]
pub struct R(crate::R<ADCLST1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCLST1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCLST1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCLST1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCLST1` writer"]
pub struct W(crate::W<ADCLST1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCLST1_SPEC>;
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
impl From<crate::W<ADCLST1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCLST1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADSEQ4` reader - ADSEQ4"]
pub type ADSEQ4_R = crate::FieldReader;
#[doc = "Field `ADSEQ4` writer - ADSEQ4"]
pub type ADSEQ4_W<'a, const O: u8> = crate::FieldWriter<'a, ADCLST1_SPEC, 5, O>;
#[doc = "Field `ADSEQ5` reader - ADSEQ5"]
pub type ADSEQ5_R = crate::FieldReader;
#[doc = "Field `ADSEQ5` writer - ADSEQ5"]
pub type ADSEQ5_W<'a, const O: u8> = crate::FieldWriter<'a, ADCLST1_SPEC, 5, O>;
#[doc = "Field `ADSEQ6` reader - ADSEQ6"]
pub type ADSEQ6_R = crate::FieldReader;
#[doc = "Field `ADSEQ6` writer - ADSEQ6"]
pub type ADSEQ6_W<'a, const O: u8> = crate::FieldWriter<'a, ADCLST1_SPEC, 5, O>;
#[doc = "Field `ADSEQ7` reader - ADSEQ7"]
pub type ADSEQ7_R = crate::FieldReader;
#[doc = "Field `ADSEQ7` writer - ADSEQ7"]
pub type ADSEQ7_W<'a, const O: u8> = crate::FieldWriter<'a, ADCLST1_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - ADSEQ4"]
    #[inline(always)]
    pub fn adseq4(&self) -> ADSEQ4_R {
        ADSEQ4_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - ADSEQ5"]
    #[inline(always)]
    pub fn adseq5(&self) -> ADSEQ5_R {
        ADSEQ5_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADSEQ6"]
    #[inline(always)]
    pub fn adseq6(&self) -> ADSEQ6_R {
        ADSEQ6_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADSEQ7"]
    #[inline(always)]
    pub fn adseq7(&self) -> ADSEQ7_R {
        ADSEQ7_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADSEQ4"]
    #[inline(always)]
    #[must_use]
    pub fn adseq4(&mut self) -> ADSEQ4_W<0> {
        ADSEQ4_W::new(self)
    }
    #[doc = "Bits 8:12 - ADSEQ5"]
    #[inline(always)]
    #[must_use]
    pub fn adseq5(&mut self) -> ADSEQ5_W<8> {
        ADSEQ5_W::new(self)
    }
    #[doc = "Bits 16:20 - ADSEQ6"]
    #[inline(always)]
    #[must_use]
    pub fn adseq6(&mut self) -> ADSEQ6_W<16> {
        ADSEQ6_W::new(self)
    }
    #[doc = "Bits 24:28 - ADSEQ7"]
    #[inline(always)]
    #[must_use]
    pub fn adseq7(&mut self) -> ADSEQ7_W<24> {
        ADSEQ7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADCLST1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adclst1](index.html) module"]
pub struct ADCLST1_SPEC;
impl crate::RegisterSpec for ADCLST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adclst1::R](R) reader structure"]
impl crate::Readable for ADCLST1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adclst1::W](W) writer structure"]
impl crate::Writable for ADCLST1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCLST1 to value 0"]
impl crate::Resettable for ADCLST1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
