#[doc = "Register `SCTM_CHOCFR` reader"]
pub struct R(crate::R<SCTM_CHOCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTM_CHOCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTM_CHOCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTM_CHOCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTM_CHOCFR` writer"]
pub struct W(crate::W<SCTM_CHOCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTM_CHOCFR_SPEC>;
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
impl From<crate::W<SCTM_CHOCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTM_CHOCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHOM` reader - CHOM"]
pub type CHOM_R = crate::FieldReader;
#[doc = "Field `CHOM` writer - CHOM"]
pub type CHOM_W<'a, const O: u8> = crate::FieldWriter<'a, SCTM_CHOCFR_SPEC, 3, O>;
#[doc = "Field `CHPRE` reader - CHPRE"]
pub type CHPRE_R = crate::BitReader;
#[doc = "Field `CHPRE` writer - CHPRE"]
pub type CHPRE_W<'a, const O: u8> = crate::BitWriter<'a, SCTM_CHOCFR_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - CHOM"]
    #[inline(always)]
    pub fn chom(&self) -> CHOM_R {
        CHOM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - CHPRE"]
    #[inline(always)]
    pub fn chpre(&self) -> CHPRE_R {
        CHPRE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CHOM"]
    #[inline(always)]
    #[must_use]
    pub fn chom(&mut self) -> CHOM_W<0> {
        CHOM_W::new(self)
    }
    #[doc = "Bit 4 - CHPRE"]
    #[inline(always)]
    #[must_use]
    pub fn chpre(&mut self) -> CHPRE_W<4> {
        CHPRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCTM_CHOCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctm_chocfr](index.html) module"]
pub struct SCTM_CHOCFR_SPEC;
impl crate::RegisterSpec for SCTM_CHOCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sctm_chocfr::R](R) reader structure"]
impl crate::Readable for SCTM_CHOCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctm_chocfr::W](W) writer structure"]
impl crate::Writable for SCTM_CHOCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTM_CHOCFR to value 0"]
impl crate::Resettable for SCTM_CHOCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
