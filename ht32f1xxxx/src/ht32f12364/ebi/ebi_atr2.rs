#[doc = "Register `EBI_ATR2` reader"]
pub struct R(crate::R<EBI_ATR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBI_ATR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EBI_ATR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EBI_ATR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EBI_ATR2` writer"]
pub struct W(crate::W<EBI_ATR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBI_ATR2_SPEC>;
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
impl From<crate::W<EBI_ATR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EBI_ATR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRSETUP` reader - ADDRSETUP"]
pub type ADDRSETUP_R = crate::FieldReader;
#[doc = "Field `ADDRSETUP` writer - ADDRSETUP"]
pub type ADDRSETUP_W<'a, const O: u8> = crate::FieldWriter<'a, EBI_ATR2_SPEC, 4, O>;
#[doc = "Field `ADDRHOLD` reader - ADDRHOLD"]
pub type ADDRHOLD_R = crate::FieldReader;
#[doc = "Field `ADDRHOLD` writer - ADDRHOLD"]
pub type ADDRHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, EBI_ATR2_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - ADDRSETUP"]
    #[inline(always)]
    pub fn addrsetup(&self) -> ADDRSETUP_R {
        ADDRSETUP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADDRHOLD"]
    #[inline(always)]
    pub fn addrhold(&self) -> ADDRHOLD_R {
        ADDRHOLD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRSETUP"]
    #[inline(always)]
    #[must_use]
    pub fn addrsetup(&mut self) -> ADDRSETUP_W<0> {
        ADDRSETUP_W::new(self)
    }
    #[doc = "Bits 8:11 - ADDRHOLD"]
    #[inline(always)]
    #[must_use]
    pub fn addrhold(&mut self) -> ADDRHOLD_W<8> {
        ADDRHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBI_ATR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_atr2](index.html) module"]
pub struct EBI_ATR2_SPEC;
impl crate::RegisterSpec for EBI_ATR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ebi_atr2::R](R) reader structure"]
impl crate::Readable for EBI_ATR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ebi_atr2::W](W) writer structure"]
impl crate::Writable for EBI_ATR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EBI_ATR2 to value 0"]
impl crate::Resettable for EBI_ATR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
