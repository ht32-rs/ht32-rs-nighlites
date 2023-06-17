#[doc = "Register `DADCHIR` reader"]
pub struct R(crate::R<DADCHIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DADCHIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DADCHIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DADCHIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DADCHIR` writer"]
pub struct W(crate::W<DADCHIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DADCHIR_SPEC>;
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
impl From<crate::W<DADCHIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DADCHIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DADICLRHO` reader - DADICLRHO"]
pub type DADICLRHO_R = crate::BitReader;
#[doc = "Field `DADICLRHO` writer - DADICLRHO"]
pub type DADICLRHO_W<'a, const O: u8> = crate::BitWriter<'a, DADCHIR_SPEC, O>;
#[doc = "Field `DADISRHO` reader - DADISRHO"]
pub type DADISRHO_R = crate::BitReader;
#[doc = "Field `DADISRHO` writer - DADISRHO"]
pub type DADISRHO_W<'a, const O: u8> = crate::BitWriter<'a, DADCHIR_SPEC, O>;
#[doc = "Field `DADIRAWHO` reader - DADIRAWHO"]
pub type DADIRAWHO_R = crate::BitReader;
#[doc = "Field `DADIRAWHO` writer - DADIRAWHO"]
pub type DADIRAWHO_W<'a, const O: u8> = crate::BitWriter<'a, DADCHIR_SPEC, O>;
#[doc = "Field `DADIEHO` reader - DADIEHO"]
pub type DADIEHO_R = crate::BitReader;
#[doc = "Field `DADIEHO` writer - DADIEHO"]
pub type DADIEHO_W<'a, const O: u8> = crate::BitWriter<'a, DADCHIR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - DADICLRHO"]
    #[inline(always)]
    pub fn dadiclrho(&self) -> DADICLRHO_R {
        DADICLRHO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DADISRHO"]
    #[inline(always)]
    pub fn dadisrho(&self) -> DADISRHO_R {
        DADISRHO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DADIRAWHO"]
    #[inline(always)]
    pub fn dadirawho(&self) -> DADIRAWHO_R {
        DADIRAWHO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DADIEHO"]
    #[inline(always)]
    pub fn dadieho(&self) -> DADIEHO_R {
        DADIEHO_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DADICLRHO"]
    #[inline(always)]
    #[must_use]
    pub fn dadiclrho(&mut self) -> DADICLRHO_W<0> {
        DADICLRHO_W::new(self)
    }
    #[doc = "Bit 1 - DADISRHO"]
    #[inline(always)]
    #[must_use]
    pub fn dadisrho(&mut self) -> DADISRHO_W<1> {
        DADISRHO_W::new(self)
    }
    #[doc = "Bit 2 - DADIRAWHO"]
    #[inline(always)]
    #[must_use]
    pub fn dadirawho(&mut self) -> DADIRAWHO_W<2> {
        DADIRAWHO_W::new(self)
    }
    #[doc = "Bit 3 - DADIEHO"]
    #[inline(always)]
    #[must_use]
    pub fn dadieho(&mut self) -> DADIEHO_W<3> {
        DADIEHO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DADCHIR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dadchir](index.html) module"]
pub struct DADCHIR_SPEC;
impl crate::RegisterSpec for DADCHIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dadchir::R](R) reader structure"]
impl crate::Readable for DADCHIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dadchir::W](W) writer structure"]
impl crate::Writable for DADCHIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DADCHIR to value 0"]
impl crate::Resettable for DADCHIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
