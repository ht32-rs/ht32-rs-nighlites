#[doc = "Register `CKCU_AHBCFGR` reader"]
pub struct R(crate::R<CKCU_AHBCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_AHBCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_AHBCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_AHBCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_AHBCFGR` writer"]
pub struct W(crate::W<CKCU_AHBCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_AHBCFGR_SPEC>;
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
impl From<crate::W<CKCU_AHBCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_AHBCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHBPRE` reader - AHBPRE"]
pub type AHBPRE_R = crate::FieldReader;
#[doc = "Field `AHBPRE` writer - AHBPRE"]
pub type AHBPRE_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_AHBCFGR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - AHBPRE"]
    #[inline(always)]
    pub fn ahbpre(&self) -> AHBPRE_R {
        AHBPRE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AHBPRE"]
    #[inline(always)]
    #[must_use]
    pub fn ahbpre(&mut self) -> AHBPRE_W<0> {
        AHBPRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_AHBCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_ahbcfgr](index.html) module"]
pub struct CKCU_AHBCFGR_SPEC;
impl crate::RegisterSpec for CKCU_AHBCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_ahbcfgr::R](R) reader structure"]
impl crate::Readable for CKCU_AHBCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_ahbcfgr::W](W) writer structure"]
impl crate::Writable for CKCU_AHBCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_AHBCFGR to value 0"]
impl crate::Resettable for CKCU_AHBCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
