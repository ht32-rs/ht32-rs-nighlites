#[doc = "Register `PWRSR` reader"]
pub struct R(crate::R<PWRSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRSR` writer"]
pub struct W(crate::W<PWRSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRSR_SPEC>;
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
impl From<crate::W<PWRSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUPF` reader - WUPF"]
pub type WUPF_R = crate::BitReader;
#[doc = "Field `WUPF` writer - WUPF"]
pub type WUPF_W<'a, const O: u8> = crate::BitWriter<'a, PWRSR_SPEC, O>;
impl R {
    #[doc = "Bit 8 - WUPF"]
    #[inline(always)]
    pub fn wupf(&self) -> WUPF_R {
        WUPF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - WUPF"]
    #[inline(always)]
    #[must_use]
    pub fn wupf(&mut self) -> WUPF_W<8> {
        WUPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWRSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrsr](index.html) module"]
pub struct PWRSR_SPEC;
impl crate::RegisterSpec for PWRSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrsr::R](R) reader structure"]
impl crate::Readable for PWRSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrsr::W](W) writer structure"]
impl crate::Writable for PWRSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRSR to value 0"]
impl crate::Resettable for PWRSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
