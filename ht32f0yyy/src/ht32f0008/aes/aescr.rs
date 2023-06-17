#[doc = "Register `AESCR` reader"]
pub struct R(crate::R<AESCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESCR` writer"]
pub struct W(crate::W<AESCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESCR_SPEC>;
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
impl From<crate::W<AESCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AESEN` reader - AESEN"]
pub type AESEN_R = crate::BitReader;
#[doc = "Field `AESEN` writer - AESEN"]
pub type AESEN_W<'a, const O: u8> = crate::BitWriter<'a, AESCR_SPEC, O>;
#[doc = "Field `DIR` reader - DIR"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - DIR"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, AESCR_SPEC, O>;
#[doc = "Field `MODE` reader - MODE"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - MODE"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, AESCR_SPEC, 2, O>;
#[doc = "Field `KEYSTART` reader - KEYSTART"]
pub type KEYSTART_R = crate::BitReader;
#[doc = "Field `KEYSTART` writer - KEYSTART"]
pub type KEYSTART_W<'a, const O: u8> = crate::BitWriter<'a, AESCR_SPEC, O>;
#[doc = "Field `KEYSIZE` reader - KEYSIZE"]
pub type KEYSIZE_R = crate::FieldReader;
#[doc = "Field `KEYSIZE` writer - KEYSIZE"]
pub type KEYSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, AESCR_SPEC, 2, O>;
#[doc = "Field `SWAP` reader - SWAP"]
pub type SWAP_R = crate::BitReader;
#[doc = "Field `SWAP` writer - SWAP"]
pub type SWAP_W<'a, const O: u8> = crate::BitWriter<'a, AESCR_SPEC, O>;
#[doc = "Field `FFLUSH` reader - FFLUSH"]
pub type FFLUSH_R = crate::BitReader;
#[doc = "Field `FFLUSH` writer - FFLUSH"]
pub type FFLUSH_W<'a, const O: u8> = crate::BitWriter<'a, AESCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AESEN"]
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - KEYSTART"]
    #[inline(always)]
    pub fn keystart(&self) -> KEYSTART_R {
        KEYSTART_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - KEYSIZE"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - SWAP"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - FFLUSH"]
    #[inline(always)]
    pub fn fflush(&self) -> FFLUSH_R {
        FFLUSH_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AESEN"]
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AESEN_W<0> {
        AESEN_W::new(self)
    }
    #[doc = "Bit 1 - DIR"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<1> {
        DIR_W::new(self)
    }
    #[doc = "Bits 2:3 - MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - KEYSTART"]
    #[inline(always)]
    #[must_use]
    pub fn keystart(&mut self) -> KEYSTART_W<4> {
        KEYSTART_W::new(self)
    }
    #[doc = "Bits 5:6 - KEYSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KEYSIZE_W<5> {
        KEYSIZE_W::new(self)
    }
    #[doc = "Bit 8 - SWAP"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<8> {
        SWAP_W::new(self)
    }
    #[doc = "Bit 10 - FFLUSH"]
    #[inline(always)]
    #[must_use]
    pub fn fflush(&mut self) -> FFLUSH_W<10> {
        FFLUSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AESCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aescr](index.html) module"]
pub struct AESCR_SPEC;
impl crate::RegisterSpec for AESCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aescr::R](R) reader structure"]
impl crate::Readable for AESCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aescr::W](W) writer structure"]
impl crate::Writable for AESCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESCR to value 0"]
impl crate::Resettable for AESCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
