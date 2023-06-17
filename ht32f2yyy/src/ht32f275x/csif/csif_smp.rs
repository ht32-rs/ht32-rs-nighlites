#[doc = "Register `CSIF_SMP` reader"]
pub struct R(crate::R<CSIF_SMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIF_SMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIF_SMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIF_SMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSIF_SMP` writer"]
pub struct W(crate::W<CSIF_SMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIF_SMP_SPEC>;
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
impl From<crate::W<CSIF_SMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIF_SMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSML` reader - CSML"]
pub type CSML_R = crate::FieldReader;
#[doc = "Field `CSML` writer - CSML"]
pub type CSML_W<'a, const O: u8> = crate::FieldWriter<'a, CSIF_SMP_SPEC, 5, O>;
#[doc = "Field `RSML` reader - RSML"]
pub type RSML_R = crate::FieldReader;
#[doc = "Field `RSML` writer - RSML"]
pub type RSML_W<'a, const O: u8> = crate::FieldWriter<'a, CSIF_SMP_SPEC, 5, O>;
#[doc = "Field `SMP_EN` reader - SMP_EN"]
pub type SMP_EN_R = crate::BitReader;
#[doc = "Field `SMP_EN` writer - SMP_EN"]
pub type SMP_EN_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_SMP_SPEC, O>;
impl R {
    #[doc = "Bits 8:12 - CSML"]
    #[inline(always)]
    pub fn csml(&self) -> CSML_R {
        CSML_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - RSML"]
    #[inline(always)]
    pub fn rsml(&self) -> RSML_R {
        RSML_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - SMP_EN"]
    #[inline(always)]
    pub fn smp_en(&self) -> SMP_EN_R {
        SMP_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:12 - CSML"]
    #[inline(always)]
    #[must_use]
    pub fn csml(&mut self) -> CSML_W<8> {
        CSML_W::new(self)
    }
    #[doc = "Bits 16:20 - RSML"]
    #[inline(always)]
    #[must_use]
    pub fn rsml(&mut self) -> RSML_W<16> {
        RSML_W::new(self)
    }
    #[doc = "Bit 31 - SMP_EN"]
    #[inline(always)]
    #[must_use]
    pub fn smp_en(&mut self) -> SMP_EN_W<31> {
        SMP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIF_SMP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_smp](index.html) module"]
pub struct CSIF_SMP_SPEC;
impl crate::RegisterSpec for CSIF_SMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csif_smp::R](R) reader structure"]
impl crate::Readable for CSIF_SMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csif_smp::W](W) writer structure"]
impl crate::Writable for CSIF_SMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSIF_SMP to value 0"]
impl crate::Resettable for CSIF_SMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
