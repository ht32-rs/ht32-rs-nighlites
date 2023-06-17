#[doc = "Register `DIDR` reader"]
pub struct R(crate::R<DIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIDR` writer"]
pub struct W(crate::W<DIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIDR_SPEC>;
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
impl From<crate::W<DIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ChipID` reader - ChipID"]
pub type CHIP_ID_R = crate::FieldReader<u32>;
#[doc = "Field `ChipID` writer - ChipID"]
pub type CHIP_ID_W<'a, const O: u8> = crate::FieldWriter<'a, DIDR_SPEC, 20, O, u32>;
impl R {
    #[doc = "Bits 0:19 - ChipID"]
    #[inline(always)]
    pub fn chip_id(&self) -> CHIP_ID_R {
        CHIP_ID_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - ChipID"]
    #[inline(always)]
    #[must_use]
    pub fn chip_id(&mut self) -> CHIP_ID_W<0> {
        CHIP_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DIDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [didr](index.html) module"]
pub struct DIDR_SPEC;
impl crate::RegisterSpec for DIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [didr::R](R) reader structure"]
impl crate::Readable for DIDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [didr::W](W) writer structure"]
impl crate::Writable for DIDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIDR to value 0"]
impl crate::Resettable for DIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
