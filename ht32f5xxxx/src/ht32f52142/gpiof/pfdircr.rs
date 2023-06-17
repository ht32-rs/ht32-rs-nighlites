#[doc = "Register `PFDIRCR` reader"]
pub struct R(crate::R<PFDIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFDIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFDIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFDIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFDIRCR` writer"]
pub struct W(crate::W<PFDIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFDIRCR_SPEC>;
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
impl From<crate::W<PFDIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFDIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFDIR0` reader - PFDIR0"]
pub type PFDIR0_R = crate::BitReader;
#[doc = "Field `PFDIR0` writer - PFDIR0"]
pub type PFDIR0_W<'a, const O: u8> = crate::BitWriter<'a, PFDIRCR_SPEC, O>;
#[doc = "Field `PFDIR1` reader - PFDIR1"]
pub type PFDIR1_R = crate::BitReader;
#[doc = "Field `PFDIR1` writer - PFDIR1"]
pub type PFDIR1_W<'a, const O: u8> = crate::BitWriter<'a, PFDIRCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PFDIR0"]
    #[inline(always)]
    pub fn pfdir0(&self) -> PFDIR0_R {
        PFDIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PFDIR1"]
    #[inline(always)]
    pub fn pfdir1(&self) -> PFDIR1_R {
        PFDIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PFDIR0"]
    #[inline(always)]
    #[must_use]
    pub fn pfdir0(&mut self) -> PFDIR0_W<0> {
        PFDIR0_W::new(self)
    }
    #[doc = "Bit 1 - PFDIR1"]
    #[inline(always)]
    #[must_use]
    pub fn pfdir1(&mut self) -> PFDIR1_W<1> {
        PFDIR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PFDIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfdircr](index.html) module"]
pub struct PFDIRCR_SPEC;
impl crate::RegisterSpec for PFDIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfdircr::R](R) reader structure"]
impl crate::Readable for PFDIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfdircr::W](W) writer structure"]
impl crate::Writable for PFDIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFDIRCR to value 0"]
impl crate::Resettable for PFDIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
