#[doc = "Register `ADCTSR` reader"]
pub struct R(crate::R<ADCTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCTSR` writer"]
pub struct W(crate::W<ADCTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCTSR_SPEC>;
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
impl From<crate::W<ADCTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADSC` reader - ADSC"]
pub type ADSC_R = crate::BitReader;
#[doc = "Field `ADSC` writer - ADSC"]
pub type ADSC_W<'a, const O: u8> = crate::BitWriter<'a, ADCTSR_SPEC, O>;
#[doc = "Field `ADEXTIS` reader - ADEXTIS"]
pub type ADEXTIS_R = crate::FieldReader;
#[doc = "Field `ADEXTIS` writer - ADEXTIS"]
pub type ADEXTIS_W<'a, const O: u8> = crate::FieldWriter<'a, ADCTSR_SPEC, 4, O>;
#[doc = "Field `TM0S` reader - TM0S"]
pub type TM0S_R = crate::FieldReader;
#[doc = "Field `TM0S` writer - TM0S"]
pub type TM0S_W<'a, const O: u8> = crate::FieldWriter<'a, ADCTSR_SPEC, 3, O>;
#[doc = "Field `TM1S0` reader - TM1S0"]
pub type TM1S0_R = crate::BitReader;
#[doc = "Field `TM1S0` writer - TM1S0"]
pub type TM1S0_W<'a, const O: u8> = crate::BitWriter<'a, ADCTSR_SPEC, O>;
#[doc = "Field `TM1S12` reader - TM1S12"]
pub type TM1S12_R = crate::FieldReader;
#[doc = "Field `TM1S12` writer - TM1S12"]
pub type TM1S12_W<'a, const O: u8> = crate::FieldWriter<'a, ADCTSR_SPEC, 2, O>;
#[doc = "Field `TM0E` reader - TM0E"]
pub type TM0E_R = crate::FieldReader;
#[doc = "Field `TM0E` writer - TM0E"]
pub type TM0E_W<'a, const O: u8> = crate::FieldWriter<'a, ADCTSR_SPEC, 3, O>;
#[doc = "Field `TM1E` reader - TM1E"]
pub type TM1E_R = crate::FieldReader;
#[doc = "Field `TM1E` writer - TM1E"]
pub type TM1E_W<'a, const O: u8> = crate::FieldWriter<'a, ADCTSR_SPEC, 3, O>;
impl R {
    #[doc = "Bit 0 - ADSC"]
    #[inline(always)]
    pub fn adsc(&self) -> ADSC_R {
        ADSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADEXTIS"]
    #[inline(always)]
    pub fn adextis(&self) -> ADEXTIS_R {
        ADEXTIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - TM0S"]
    #[inline(always)]
    pub fn tm0s(&self) -> TM0S_R {
        TM0S_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - TM1S0"]
    #[inline(always)]
    pub fn tm1s0(&self) -> TM1S0_R {
        TM1S0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 22:23 - TM1S12"]
    #[inline(always)]
    pub fn tm1s12(&self) -> TM1S12_R {
        TM1S12_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - TM0E"]
    #[inline(always)]
    pub fn tm0e(&self) -> TM0E_R {
        TM0E_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - TM1E"]
    #[inline(always)]
    pub fn tm1e(&self) -> TM1E_R {
        TM1E_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADSC"]
    #[inline(always)]
    #[must_use]
    pub fn adsc(&mut self) -> ADSC_W<0> {
        ADSC_W::new(self)
    }
    #[doc = "Bits 8:11 - ADEXTIS"]
    #[inline(always)]
    #[must_use]
    pub fn adextis(&mut self) -> ADEXTIS_W<8> {
        ADEXTIS_W::new(self)
    }
    #[doc = "Bits 16:18 - TM0S"]
    #[inline(always)]
    #[must_use]
    pub fn tm0s(&mut self) -> TM0S_W<16> {
        TM0S_W::new(self)
    }
    #[doc = "Bit 19 - TM1S0"]
    #[inline(always)]
    #[must_use]
    pub fn tm1s0(&mut self) -> TM1S0_W<19> {
        TM1S0_W::new(self)
    }
    #[doc = "Bits 22:23 - TM1S12"]
    #[inline(always)]
    #[must_use]
    pub fn tm1s12(&mut self) -> TM1S12_W<22> {
        TM1S12_W::new(self)
    }
    #[doc = "Bits 24:26 - TM0E"]
    #[inline(always)]
    #[must_use]
    pub fn tm0e(&mut self) -> TM0E_W<24> {
        TM0E_W::new(self)
    }
    #[doc = "Bits 27:29 - TM1E"]
    #[inline(always)]
    #[must_use]
    pub fn tm1e(&mut self) -> TM1E_W<27> {
        TM1E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADCTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adctsr](index.html) module"]
pub struct ADCTSR_SPEC;
impl crate::RegisterSpec for ADCTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adctsr::R](R) reader structure"]
impl crate::Readable for ADCTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adctsr::W](W) writer structure"]
impl crate::Writable for ADCTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCTSR to value 0"]
impl crate::Resettable for ADCTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
