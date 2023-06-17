#[doc = "Register `PFDINR` reader"]
pub struct R(crate::R<PFDINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFDINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFDINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFDINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFDINR` writer"]
pub struct W(crate::W<PFDINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFDINR_SPEC>;
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
impl From<crate::W<PFDINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFDINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFDIN0` reader - PFDIN0"]
pub type PFDIN0_R = crate::BitReader;
#[doc = "Field `PFDIN0` writer - PFDIN0"]
pub type PFDIN0_W<'a, const O: u8> = crate::BitWriter<'a, PFDINR_SPEC, O>;
#[doc = "Field `PFDIN1` reader - PFDIN1"]
pub type PFDIN1_R = crate::BitReader;
#[doc = "Field `PFDIN1` writer - PFDIN1"]
pub type PFDIN1_W<'a, const O: u8> = crate::BitWriter<'a, PFDINR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PFDIN0"]
    #[inline(always)]
    pub fn pfdin0(&self) -> PFDIN0_R {
        PFDIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PFDIN1"]
    #[inline(always)]
    pub fn pfdin1(&self) -> PFDIN1_R {
        PFDIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PFDIN0"]
    #[inline(always)]
    #[must_use]
    pub fn pfdin0(&mut self) -> PFDIN0_W<0> {
        PFDIN0_W::new(self)
    }
    #[doc = "Bit 1 - PFDIN1"]
    #[inline(always)]
    #[must_use]
    pub fn pfdin1(&mut self) -> PFDIN1_W<1> {
        PFDIN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PFDINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfdinr](index.html) module"]
pub struct PFDINR_SPEC;
impl crate::RegisterSpec for PFDINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfdinr::R](R) reader structure"]
impl crate::Readable for PFDINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfdinr::W](W) writer structure"]
impl crate::Writable for PFDINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFDINR to value 0"]
impl crate::Resettable for PFDINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
