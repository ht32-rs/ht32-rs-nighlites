#[doc = "Register `ADCCONF` reader"]
pub struct R(crate::R<ADCCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCONF` writer"]
pub struct W(crate::W<ADCCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCONF_SPEC>;
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
impl From<crate::W<ADCCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCONF` reader - ADCONF"]
pub type ADCONF_R = crate::FieldReader;
#[doc = "Field `ADCONF` writer - ADCONF"]
pub type ADCONF_W<'a, const O: u8> = crate::FieldWriter<'a, ADCCONF_SPEC, 2, O>;
#[doc = "Field `SYNDADC` reader - SYNDADC"]
pub type SYNDADC_R = crate::BitReader;
#[doc = "Field `SYNDADC` writer - SYNDADC"]
pub type SYNDADC_W<'a, const O: u8> = crate::BitWriter<'a, ADCCONF_SPEC, O>;
#[doc = "Field `RADTD` reader - RADTD"]
pub type RADTD_R = crate::FieldReader;
#[doc = "Field `RADTD` writer - RADTD"]
pub type RADTD_W<'a, const O: u8> = crate::FieldWriter<'a, ADCCONF_SPEC, 8, O>;
#[doc = "Field `HADTD` reader - HADTD"]
pub type HADTD_R = crate::FieldReader;
#[doc = "Field `HADTD` writer - HADTD"]
pub type HADTD_W<'a, const O: u8> = crate::FieldWriter<'a, ADCCONF_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:1 - ADCONF"]
    #[inline(always)]
    pub fn adconf(&self) -> ADCONF_R {
        ADCONF_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 6 - SYNDADC"]
    #[inline(always)]
    pub fn syndadc(&self) -> SYNDADC_R {
        SYNDADC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - RADTD"]
    #[inline(always)]
    pub fn radtd(&self) -> RADTD_R {
        RADTD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - HADTD"]
    #[inline(always)]
    pub fn hadtd(&self) -> HADTD_R {
        HADTD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADCONF"]
    #[inline(always)]
    #[must_use]
    pub fn adconf(&mut self) -> ADCONF_W<0> {
        ADCONF_W::new(self)
    }
    #[doc = "Bit 6 - SYNDADC"]
    #[inline(always)]
    #[must_use]
    pub fn syndadc(&mut self) -> SYNDADC_W<6> {
        SYNDADC_W::new(self)
    }
    #[doc = "Bits 8:15 - RADTD"]
    #[inline(always)]
    #[must_use]
    pub fn radtd(&mut self) -> RADTD_W<8> {
        RADTD_W::new(self)
    }
    #[doc = "Bits 16:23 - HADTD"]
    #[inline(always)]
    #[must_use]
    pub fn hadtd(&mut self) -> HADTD_W<16> {
        HADTD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADCCONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcconf](index.html) module"]
pub struct ADCCONF_SPEC;
impl crate::RegisterSpec for ADCCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcconf::R](R) reader structure"]
impl crate::Readable for ADCCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcconf::W](W) writer structure"]
impl crate::Writable for ADCCONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCCONF to value 0"]
impl crate::Resettable for ADCCONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
