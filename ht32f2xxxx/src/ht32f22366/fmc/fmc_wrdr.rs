#[doc = "Register `FMC_WRDR` reader"]
pub struct R(crate::R<FMC_WRDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_WRDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_WRDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_WRDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_WRDR` writer"]
pub struct W(crate::W<FMC_WRDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_WRDR_SPEC>;
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
impl From<crate::W<FMC_WRDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_WRDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRDB` reader - WRDB"]
pub type WRDB_R = crate::FieldReader<u32>;
#[doc = "Field `WRDB` writer - WRDB"]
pub type WRDB_W<'a, const O: u8> = crate::FieldWriter<'a, FMC_WRDR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - WRDB"]
    #[inline(always)]
    pub fn wrdb(&self) -> WRDB_R {
        WRDB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WRDB"]
    #[inline(always)]
    #[must_use]
    pub fn wrdb(&mut self) -> WRDB_W<0> {
        WRDB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC_WRDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_wrdr](index.html) module"]
pub struct FMC_WRDR_SPEC;
impl crate::RegisterSpec for FMC_WRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_wrdr::R](R) reader structure"]
impl crate::Readable for FMC_WRDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_wrdr::W](W) writer structure"]
impl crate::Writable for FMC_WRDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_WRDR to value 0"]
impl crate::Resettable for FMC_WRDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
