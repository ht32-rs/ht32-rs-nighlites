#[doc = "Register `ADCDR2` reader"]
pub struct R(crate::R<ADCDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCDR2` writer"]
pub struct W(crate::W<ADCDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCDR2_SPEC>;
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
impl From<crate::W<ADCDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD` reader - ADD"]
pub type ADD_R = crate::FieldReader<u16>;
#[doc = "Field `ADD` writer - ADD"]
pub type ADD_W<'a, const O: u8> = crate::FieldWriter<'a, ADCDR2_SPEC, 16, O, u16>;
#[doc = "Field `ADVLD` reader - ADVLD"]
pub type ADVLD_R = crate::BitReader;
#[doc = "Field `ADVLD` writer - ADVLD"]
pub type ADVLD_W<'a, const O: u8> = crate::BitWriter<'a, ADCDR2_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - ADD"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD"]
    #[inline(always)]
    pub fn advld(&self) -> ADVLD_R {
        ADVLD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<0> {
        ADD_W::new(self)
    }
    #[doc = "Bit 31 - ADVLD"]
    #[inline(always)]
    #[must_use]
    pub fn advld(&mut self) -> ADVLD_W<31> {
        ADVLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADCDR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcdr2](index.html) module"]
pub struct ADCDR2_SPEC;
impl crate::RegisterSpec for ADCDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcdr2::R](R) reader structure"]
impl crate::Readable for ADCDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcdr2::W](W) writer structure"]
impl crate::Writable for ADCDR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCDR2 to value 0"]
impl crate::Resettable for ADCDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
