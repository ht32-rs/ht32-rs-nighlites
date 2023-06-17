#[doc = "Register `PWRCU_PWRLDOSR` reader"]
pub struct R(crate::R<PWRCU_PWRLDOSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCU_PWRLDOSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCU_PWRLDOSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCU_PWRLDOSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCU_PWRLDOSR` writer"]
pub struct W(crate::W<PWRCU_PWRLDOSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCU_PWRLDOSR_SPEC>;
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
impl From<crate::W<PWRCU_PWRLDOSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCU_PWRLDOSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULDOPST` reader - ULDOPST"]
pub type ULDOPST_R = crate::FieldReader;
#[doc = "Field `ULDOPST` writer - ULDOPST"]
pub type ULDOPST_W<'a, const O: u8> = crate::FieldWriter<'a, PWRCU_PWRLDOSR_SPEC, 2, O>;
#[doc = "Field `LDOPST` reader - LDOPST"]
pub type LDOPST_R = crate::FieldReader;
#[doc = "Field `LDOPST` writer - LDOPST"]
pub type LDOPST_W<'a, const O: u8> = crate::FieldWriter<'a, PWRCU_PWRLDOSR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 24:25 - ULDOPST"]
    #[inline(always)]
    pub fn uldopst(&self) -> ULDOPST_R {
        ULDOPST_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - LDOPST"]
    #[inline(always)]
    pub fn ldopst(&self) -> LDOPST_R {
        LDOPST_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 24:25 - ULDOPST"]
    #[inline(always)]
    #[must_use]
    pub fn uldopst(&mut self) -> ULDOPST_W<24> {
        ULDOPST_W::new(self)
    }
    #[doc = "Bits 28:29 - LDOPST"]
    #[inline(always)]
    #[must_use]
    pub fn ldopst(&mut self) -> LDOPST_W<28> {
        LDOPST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWRCU_PWRLDOSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_pwrldosr](index.html) module"]
pub struct PWRCU_PWRLDOSR_SPEC;
impl crate::RegisterSpec for PWRCU_PWRLDOSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrcu_pwrldosr::R](R) reader structure"]
impl crate::Readable for PWRCU_PWRLDOSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrcu_pwrldosr::W](W) writer structure"]
impl crate::Writable for PWRCU_PWRLDOSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCU_PWRLDOSR to value 0"]
impl crate::Resettable for PWRCU_PWRLDOSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
