#[doc = "Register `EBI_PR1` reader"]
pub struct R(crate::R<EBI_PR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBI_PR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EBI_PR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EBI_PR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EBI_PR1` writer"]
pub struct W(crate::W<EBI_PR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBI_PR1_SPEC>;
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
impl From<crate::W<EBI_PR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EBI_PR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSPOL` reader - CSPOL"]
pub type CSPOL_R = crate::BitReader;
#[doc = "Field `CSPOL` writer - CSPOL"]
pub type CSPOL_W<'a, const O: u8> = crate::BitWriter<'a, EBI_PR1_SPEC, O>;
#[doc = "Field `OEPOL` reader - OEPOL"]
pub type OEPOL_R = crate::BitReader;
#[doc = "Field `OEPOL` writer - OEPOL"]
pub type OEPOL_W<'a, const O: u8> = crate::BitWriter<'a, EBI_PR1_SPEC, O>;
#[doc = "Field `WEPOL` reader - WEPOL"]
pub type WEPOL_R = crate::BitReader;
#[doc = "Field `WEPOL` writer - WEPOL"]
pub type WEPOL_W<'a, const O: u8> = crate::BitWriter<'a, EBI_PR1_SPEC, O>;
#[doc = "Field `ALEPOL` reader - ALEPOL"]
pub type ALEPOL_R = crate::BitReader;
#[doc = "Field `ALEPOL` writer - ALEPOL"]
pub type ALEPOL_W<'a, const O: u8> = crate::BitWriter<'a, EBI_PR1_SPEC, O>;
#[doc = "Field `ARDYPOL` reader - ARDYPOL"]
pub type ARDYPOL_R = crate::BitReader;
#[doc = "Field `ARDYPOL` writer - ARDYPOL"]
pub type ARDYPOL_W<'a, const O: u8> = crate::BitWriter<'a, EBI_PR1_SPEC, O>;
#[doc = "Field `BLPOL` reader - BLPOL"]
pub type BLPOL_R = crate::BitReader;
#[doc = "Field `BLPOL` writer - BLPOL"]
pub type BLPOL_W<'a, const O: u8> = crate::BitWriter<'a, EBI_PR1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CSPOL"]
    #[inline(always)]
    pub fn cspol(&self) -> CSPOL_R {
        CSPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OEPOL"]
    #[inline(always)]
    pub fn oepol(&self) -> OEPOL_R {
        OEPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WEPOL"]
    #[inline(always)]
    pub fn wepol(&self) -> WEPOL_R {
        WEPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ALEPOL"]
    #[inline(always)]
    pub fn alepol(&self) -> ALEPOL_R {
        ALEPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARDYPOL"]
    #[inline(always)]
    pub fn ardypol(&self) -> ARDYPOL_R {
        ARDYPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BLPOL"]
    #[inline(always)]
    pub fn blpol(&self) -> BLPOL_R {
        BLPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSPOL"]
    #[inline(always)]
    #[must_use]
    pub fn cspol(&mut self) -> CSPOL_W<0> {
        CSPOL_W::new(self)
    }
    #[doc = "Bit 1 - OEPOL"]
    #[inline(always)]
    #[must_use]
    pub fn oepol(&mut self) -> OEPOL_W<1> {
        OEPOL_W::new(self)
    }
    #[doc = "Bit 2 - WEPOL"]
    #[inline(always)]
    #[must_use]
    pub fn wepol(&mut self) -> WEPOL_W<2> {
        WEPOL_W::new(self)
    }
    #[doc = "Bit 3 - ALEPOL"]
    #[inline(always)]
    #[must_use]
    pub fn alepol(&mut self) -> ALEPOL_W<3> {
        ALEPOL_W::new(self)
    }
    #[doc = "Bit 4 - ARDYPOL"]
    #[inline(always)]
    #[must_use]
    pub fn ardypol(&mut self) -> ARDYPOL_W<4> {
        ARDYPOL_W::new(self)
    }
    #[doc = "Bit 5 - BLPOL"]
    #[inline(always)]
    #[must_use]
    pub fn blpol(&mut self) -> BLPOL_W<5> {
        BLPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBI_PR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_pr1](index.html) module"]
pub struct EBI_PR1_SPEC;
impl crate::RegisterSpec for EBI_PR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ebi_pr1::R](R) reader structure"]
impl crate::Readable for EBI_PR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ebi_pr1::W](W) writer structure"]
impl crate::Writable for EBI_PR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EBI_PR1 to value 0"]
impl crate::Resettable for EBI_PR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
