#[doc = "Register `QSPI_MIDICR1` reader"]
pub struct R(crate::R<QSPI_MIDICR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_MIDICR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_MIDICR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_MIDICR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_MIDICR1` writer"]
pub struct W(crate::W<QSPI_MIDICR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_MIDICR1_SPEC>;
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
impl From<crate::W<QSPI_MIDICR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_MIDICR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDVALUE` reader - MDVALUE"]
pub type MDVALUE_R = crate::FieldReader;
#[doc = "Field `MDVALUE` writer - MDVALUE"]
pub type MDVALUE_W<'a, const O: u8> = crate::FieldWriter<'a, QSPI_MIDICR1_SPEC, 8, O>;
#[doc = "Field `CMDVALUE` reader - CMDVALUE"]
pub type CMDVALUE_R = crate::FieldReader;
#[doc = "Field `CMDVALUE` writer - CMDVALUE"]
pub type CMDVALUE_W<'a, const O: u8> = crate::FieldWriter<'a, QSPI_MIDICR1_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - MDVALUE"]
    #[inline(always)]
    pub fn mdvalue(&self) -> MDVALUE_R {
        MDVALUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CMDVALUE"]
    #[inline(always)]
    pub fn cmdvalue(&self) -> CMDVALUE_R {
        CMDVALUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MDVALUE"]
    #[inline(always)]
    #[must_use]
    pub fn mdvalue(&mut self) -> MDVALUE_W<0> {
        MDVALUE_W::new(self)
    }
    #[doc = "Bits 8:15 - CMDVALUE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdvalue(&mut self) -> CMDVALUE_W<8> {
        CMDVALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_MIDICR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_midicr1](index.html) module"]
pub struct QSPI_MIDICR1_SPEC;
impl crate::RegisterSpec for QSPI_MIDICR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_midicr1::R](R) reader structure"]
impl crate::Readable for QSPI_MIDICR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_midicr1::W](W) writer structure"]
impl crate::Writable for QSPI_MIDICR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QSPI_MIDICR1 to value 0"]
impl crate::Resettable for QSPI_MIDICR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
