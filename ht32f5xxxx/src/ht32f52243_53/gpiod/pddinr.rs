#[doc = "Register `PDDINR` reader"]
pub struct R(crate::R<PDDINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDDINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDDINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDDINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDDINR` writer"]
pub struct W(crate::W<PDDINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDDINR_SPEC>;
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
impl From<crate::W<PDDINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDDINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDDIN0` reader - PDDIN0"]
pub type PDDIN0_R = crate::BitReader;
#[doc = "Field `PDDIN0` writer - PDDIN0"]
pub type PDDIN0_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN1` reader - PDDIN1"]
pub type PDDIN1_R = crate::BitReader;
#[doc = "Field `PDDIN1` writer - PDDIN1"]
pub type PDDIN1_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN2` reader - PDDIN2"]
pub type PDDIN2_R = crate::BitReader;
#[doc = "Field `PDDIN2` writer - PDDIN2"]
pub type PDDIN2_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN3` reader - PDDIN3"]
pub type PDDIN3_R = crate::BitReader;
#[doc = "Field `PDDIN3` writer - PDDIN3"]
pub type PDDIN3_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDDIN0"]
    #[inline(always)]
    pub fn pddin0(&self) -> PDDIN0_R {
        PDDIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDDIN1"]
    #[inline(always)]
    pub fn pddin1(&self) -> PDDIN1_R {
        PDDIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDDIN2"]
    #[inline(always)]
    pub fn pddin2(&self) -> PDDIN2_R {
        PDDIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDDIN3"]
    #[inline(always)]
    pub fn pddin3(&self) -> PDDIN3_R {
        PDDIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDDIN0"]
    #[inline(always)]
    #[must_use]
    pub fn pddin0(&mut self) -> PDDIN0_W<0> {
        PDDIN0_W::new(self)
    }
    #[doc = "Bit 1 - PDDIN1"]
    #[inline(always)]
    #[must_use]
    pub fn pddin1(&mut self) -> PDDIN1_W<1> {
        PDDIN1_W::new(self)
    }
    #[doc = "Bit 2 - PDDIN2"]
    #[inline(always)]
    #[must_use]
    pub fn pddin2(&mut self) -> PDDIN2_W<2> {
        PDDIN2_W::new(self)
    }
    #[doc = "Bit 3 - PDDIN3"]
    #[inline(always)]
    #[must_use]
    pub fn pddin3(&mut self) -> PDDIN3_W<3> {
        PDDIN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDDINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddinr](index.html) module"]
pub struct PDDINR_SPEC;
impl crate::RegisterSpec for PDDINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pddinr::R](R) reader structure"]
impl crate::Readable for PDDINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pddinr::W](W) writer structure"]
impl crate::Writable for PDDINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDDINR to value 0"]
impl crate::Resettable for PDDINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
