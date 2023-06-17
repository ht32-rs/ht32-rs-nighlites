#[doc = "Register `AESIVR1` reader"]
pub struct R(crate::R<AESIVR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESIVR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESIVR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESIVR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESIVR1` writer"]
pub struct W(crate::W<AESIVR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESIVR1_SPEC>;
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
impl From<crate::W<AESIVR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESIVR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IVDat` reader - IVDat"]
pub type IVDAT_R = crate::FieldReader<u32>;
#[doc = "Field `IVDat` writer - IVDat"]
pub type IVDAT_W<'a, const O: u8> = crate::FieldWriter<'a, AESIVR1_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - IVDat"]
    #[inline(always)]
    pub fn ivdat(&self) -> IVDAT_R {
        IVDAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IVDat"]
    #[inline(always)]
    #[must_use]
    pub fn ivdat(&mut self) -> IVDAT_W<0> {
        IVDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AESIVR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesivr1](index.html) module"]
pub struct AESIVR1_SPEC;
impl crate::RegisterSpec for AESIVR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aesivr1::R](R) reader structure"]
impl crate::Readable for AESIVR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesivr1::W](W) writer structure"]
impl crate::Writable for AESIVR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESIVR1 to value 0"]
impl crate::Resettable for AESIVR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
