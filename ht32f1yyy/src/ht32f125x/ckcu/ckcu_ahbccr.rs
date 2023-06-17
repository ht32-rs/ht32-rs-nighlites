#[doc = "Register `CKCU_AHBCCR` reader"]
pub struct R(crate::R<CKCU_AHBCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_AHBCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_AHBCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_AHBCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_AHBCCR` writer"]
pub struct W(crate::W<CKCU_AHBCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_AHBCCR_SPEC>;
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
impl From<crate::W<CKCU_AHBCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_AHBCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMCEN` reader - FMCEN"]
pub type FMCEN_R = crate::BitReader;
#[doc = "Field `FMCEN` writer - FMCEN"]
pub type FMCEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_AHBCCR_SPEC, O>;
#[doc = "Field `SRAMEN` reader - SRAMEN"]
pub type SRAMEN_R = crate::BitReader;
#[doc = "Field `SRAMEN` writer - SRAMEN"]
pub type SRAMEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_AHBCCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - FMCEN"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SRAMEN"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FMCEN"]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<0> {
        FMCEN_W::new(self)
    }
    #[doc = "Bit 2 - SRAMEN"]
    #[inline(always)]
    #[must_use]
    pub fn sramen(&mut self) -> SRAMEN_W<2> {
        SRAMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_AHBCCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_ahbccr](index.html) module"]
pub struct CKCU_AHBCCR_SPEC;
impl crate::RegisterSpec for CKCU_AHBCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_ahbccr::R](R) reader structure"]
impl crate::Readable for CKCU_AHBCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_ahbccr::W](W) writer structure"]
impl crate::Writable for CKCU_AHBCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_AHBCCR to value 0"]
impl crate::Resettable for CKCU_AHBCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
