#[doc = "Register `SCI_ETUR` reader"]
pub struct R(crate::R<SCI_ETUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCI_ETUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCI_ETUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCI_ETUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCI_ETUR` writer"]
pub struct W(crate::W<SCI_ETUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCI_ETUR_SPEC>;
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
impl From<crate::W<SCI_ETUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCI_ETUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETU` reader - ETU"]
pub type ETU_R = crate::FieldReader<u16>;
#[doc = "Field `ETU` writer - ETU"]
pub type ETU_W<'a, const O: u8> = crate::FieldWriter<'a, SCI_ETUR_SPEC, 11, O, u16>;
#[doc = "Field `COMP` reader - COMP"]
pub type COMP_R = crate::BitReader;
#[doc = "Field `COMP` writer - COMP"]
pub type COMP_W<'a, const O: u8> = crate::BitWriter<'a, SCI_ETUR_SPEC, O>;
impl R {
    #[doc = "Bits 0:10 - ETU"]
    #[inline(always)]
    pub fn etu(&self) -> ETU_R {
        ETU_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - COMP"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - ETU"]
    #[inline(always)]
    #[must_use]
    pub fn etu(&mut self) -> ETU_W<0> {
        ETU_W::new(self)
    }
    #[doc = "Bit 15 - COMP"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<15> {
        COMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCI_ETUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_etur](index.html) module"]
pub struct SCI_ETUR_SPEC;
impl crate::RegisterSpec for SCI_ETUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sci_etur::R](R) reader structure"]
impl crate::Readable for SCI_ETUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sci_etur::W](W) writer structure"]
impl crate::Writable for SCI_ETUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCI_ETUR to value 0"]
impl crate::Resettable for SCI_ETUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
