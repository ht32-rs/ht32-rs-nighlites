#[doc = "Register `RSTCU_AHBPRSTR` reader"]
pub struct R(crate::R<RSTCU_AHBPRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCU_AHBPRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCU_AHBPRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCU_AHBPRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCU_AHBPRSTR` writer"]
pub struct W(crate::W<RSTCU_AHBPRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCU_AHBPRSTR_SPEC>;
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
impl From<crate::W<RSTCU_AHBPRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCU_AHBPRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMARST` reader - DMARST"]
pub type DMARST_R = crate::BitReader;
#[doc = "Field `DMARST` writer - DMARST"]
pub type DMARST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_AHBPRSTR_SPEC, O>;
#[doc = "Field `CSIFRST` reader - CSIFRST"]
pub type CSIFRST_R = crate::BitReader;
#[doc = "Field `CSIFRST` writer - CSIFRST"]
pub type CSIFRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_AHBPRSTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - DMARST"]
    #[inline(always)]
    pub fn dmarst(&self) -> DMARST_R {
        DMARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CSIFRST"]
    #[inline(always)]
    pub fn csifrst(&self) -> CSIFRST_R {
        CSIFRST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMARST"]
    #[inline(always)]
    #[must_use]
    pub fn dmarst(&mut self) -> DMARST_W<0> {
        DMARST_W::new(self)
    }
    #[doc = "Bit 4 - CSIFRST"]
    #[inline(always)]
    #[must_use]
    pub fn csifrst(&mut self) -> CSIFRST_W<4> {
        CSIFRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RSTCU_AHBPRSTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcu_ahbprstr](index.html) module"]
pub struct RSTCU_AHBPRSTR_SPEC;
impl crate::RegisterSpec for RSTCU_AHBPRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstcu_ahbprstr::R](R) reader structure"]
impl crate::Readable for RSTCU_AHBPRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstcu_ahbprstr::W](W) writer structure"]
impl crate::Writable for RSTCU_AHBPRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTCU_AHBPRSTR to value 0"]
impl crate::Resettable for RSTCU_AHBPRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
