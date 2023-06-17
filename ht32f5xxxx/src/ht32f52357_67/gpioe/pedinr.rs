#[doc = "Register `PEDINR` reader"]
pub struct R(crate::R<PEDINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEDINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEDINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEDINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEDINR` writer"]
pub struct W(crate::W<PEDINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEDINR_SPEC>;
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
impl From<crate::W<PEDINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEDINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEDIN0` reader - PEDIN0"]
pub type PEDIN0_R = crate::BitReader;
#[doc = "Field `PEDIN0` writer - PEDIN0"]
pub type PEDIN0_W<'a, const O: u8> = crate::BitWriter<'a, PEDINR_SPEC, O>;
#[doc = "Field `PEDIN1` reader - PEDIN1"]
pub type PEDIN1_R = crate::BitReader;
#[doc = "Field `PEDIN1` writer - PEDIN1"]
pub type PEDIN1_W<'a, const O: u8> = crate::BitWriter<'a, PEDINR_SPEC, O>;
#[doc = "Field `PEDIN2` reader - PEDIN2"]
pub type PEDIN2_R = crate::BitReader;
#[doc = "Field `PEDIN2` writer - PEDIN2"]
pub type PEDIN2_W<'a, const O: u8> = crate::BitWriter<'a, PEDINR_SPEC, O>;
#[doc = "Field `PEDIN3` reader - PEDIN3"]
pub type PEDIN3_R = crate::BitReader;
#[doc = "Field `PEDIN3` writer - PEDIN3"]
pub type PEDIN3_W<'a, const O: u8> = crate::BitWriter<'a, PEDINR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PEDIN0"]
    #[inline(always)]
    pub fn pedin0(&self) -> PEDIN0_R {
        PEDIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PEDIN1"]
    #[inline(always)]
    pub fn pedin1(&self) -> PEDIN1_R {
        PEDIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PEDIN2"]
    #[inline(always)]
    pub fn pedin2(&self) -> PEDIN2_R {
        PEDIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PEDIN3"]
    #[inline(always)]
    pub fn pedin3(&self) -> PEDIN3_R {
        PEDIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PEDIN0"]
    #[inline(always)]
    #[must_use]
    pub fn pedin0(&mut self) -> PEDIN0_W<0> {
        PEDIN0_W::new(self)
    }
    #[doc = "Bit 1 - PEDIN1"]
    #[inline(always)]
    #[must_use]
    pub fn pedin1(&mut self) -> PEDIN1_W<1> {
        PEDIN1_W::new(self)
    }
    #[doc = "Bit 2 - PEDIN2"]
    #[inline(always)]
    #[must_use]
    pub fn pedin2(&mut self) -> PEDIN2_W<2> {
        PEDIN2_W::new(self)
    }
    #[doc = "Bit 3 - PEDIN3"]
    #[inline(always)]
    #[must_use]
    pub fn pedin3(&mut self) -> PEDIN3_W<3> {
        PEDIN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PEDINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pedinr](index.html) module"]
pub struct PEDINR_SPEC;
impl crate::RegisterSpec for PEDINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pedinr::R](R) reader structure"]
impl crate::Readable for PEDINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pedinr::W](W) writer structure"]
impl crate::Writable for PEDINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PEDINR to value 0"]
impl crate::Resettable for PEDINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
