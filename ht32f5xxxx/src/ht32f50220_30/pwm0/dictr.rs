#[doc = "Register `DICTR` reader"]
pub struct R(crate::R<DICTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DICTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DICTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DICTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DICTR` writer"]
pub struct W(crate::W<DICTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DICTR_SPEC>;
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
impl From<crate::W<DICTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DICTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0CIE` reader - CH0CIE"]
pub type CH0CIE_R = crate::BitReader;
#[doc = "Field `CH0CIE` writer - CH0CIE"]
pub type CH0CIE_W<'a, const O: u8> = crate::BitWriter<'a, DICTR_SPEC, O>;
#[doc = "Field `CH1CIE` reader - CH1CIE"]
pub type CH1CIE_R = crate::BitReader;
#[doc = "Field `CH1CIE` writer - CH1CIE"]
pub type CH1CIE_W<'a, const O: u8> = crate::BitWriter<'a, DICTR_SPEC, O>;
#[doc = "Field `CH2CIE` reader - CH2CIE"]
pub type CH2CIE_R = crate::BitReader;
#[doc = "Field `CH2CIE` writer - CH2CIE"]
pub type CH2CIE_W<'a, const O: u8> = crate::BitWriter<'a, DICTR_SPEC, O>;
#[doc = "Field `CH3CIE` reader - CH3CIE"]
pub type CH3CIE_R = crate::BitReader;
#[doc = "Field `CH3CIE` writer - CH3CIE"]
pub type CH3CIE_W<'a, const O: u8> = crate::BitWriter<'a, DICTR_SPEC, O>;
#[doc = "Field `UEVIE` reader - UEVIE"]
pub type UEVIE_R = crate::BitReader;
#[doc = "Field `UEVIE` writer - UEVIE"]
pub type UEVIE_W<'a, const O: u8> = crate::BitWriter<'a, DICTR_SPEC, O>;
#[doc = "Field `TEVIE` reader - TEVIE"]
pub type TEVIE_R = crate::BitReader;
#[doc = "Field `TEVIE` writer - TEVIE"]
pub type TEVIE_W<'a, const O: u8> = crate::BitWriter<'a, DICTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CH0CIE"]
    #[inline(always)]
    pub fn ch0cie(&self) -> CH0CIE_R {
        CH0CIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1CIE"]
    #[inline(always)]
    pub fn ch1cie(&self) -> CH1CIE_R {
        CH1CIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2CIE"]
    #[inline(always)]
    pub fn ch2cie(&self) -> CH2CIE_R {
        CH2CIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3CIE"]
    #[inline(always)]
    pub fn ch3cie(&self) -> CH3CIE_R {
        CH3CIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - UEVIE"]
    #[inline(always)]
    pub fn uevie(&self) -> UEVIE_R {
        UEVIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - TEVIE"]
    #[inline(always)]
    pub fn tevie(&self) -> TEVIE_R {
        TEVIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cie(&mut self) -> CH0CIE_W<0> {
        CH0CIE_W::new(self)
    }
    #[doc = "Bit 1 - CH1CIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cie(&mut self) -> CH1CIE_W<1> {
        CH1CIE_W::new(self)
    }
    #[doc = "Bit 2 - CH2CIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch2cie(&mut self) -> CH2CIE_W<2> {
        CH2CIE_W::new(self)
    }
    #[doc = "Bit 3 - CH3CIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch3cie(&mut self) -> CH3CIE_W<3> {
        CH3CIE_W::new(self)
    }
    #[doc = "Bit 8 - UEVIE"]
    #[inline(always)]
    #[must_use]
    pub fn uevie(&mut self) -> UEVIE_W<8> {
        UEVIE_W::new(self)
    }
    #[doc = "Bit 10 - TEVIE"]
    #[inline(always)]
    #[must_use]
    pub fn tevie(&mut self) -> TEVIE_W<10> {
        TEVIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DICTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dictr](index.html) module"]
pub struct DICTR_SPEC;
impl crate::RegisterSpec for DICTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dictr::R](R) reader structure"]
impl crate::Readable for DICTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dictr::W](W) writer structure"]
impl crate::Writable for DICTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DICTR to value 0"]
impl crate::Resettable for DICTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
