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
#[doc = "Field `CH0CCIE` reader - CH0CCIE"]
pub type CH0CCIE_R = crate::BitReader;
#[doc = "Field `CH0CCIE` writer - CH0CCIE"]
pub type CH0CCIE_W<'a, const O: u8> = crate::BitWriter<'a, DICTR_SPEC, O>;
#[doc = "Field `CH1CCIE` reader - CH1CCIE"]
pub type CH1CCIE_R = crate::BitReader;
#[doc = "Field `CH1CCIE` writer - CH1CCIE"]
pub type CH1CCIE_W<'a, const O: u8> = crate::BitWriter<'a, DICTR_SPEC, O>;
#[doc = "Field `UEVIE` reader - UEVIE"]
pub type UEVIE_R = crate::BitReader;
#[doc = "Field `UEVIE` writer - UEVIE"]
pub type UEVIE_W<'a, const O: u8> = crate::BitWriter<'a, DICTR_SPEC, O>;
#[doc = "Field `TEVIE` reader - TEVIE"]
pub type TEVIE_R = crate::BitReader;
#[doc = "Field `TEVIE` writer - TEVIE"]
pub type TEVIE_W<'a, const O: u8> = crate::BitWriter<'a, DICTR_SPEC, O>;
#[doc = "Field `CH0CCDE` reader - CH0CCDE"]
pub type CH0CCDE_R = crate::BitReader;
#[doc = "Field `CH0CCDE` writer - CH0CCDE"]
pub type CH0CCDE_W<'a, const O: u8> = crate::BitWriter<'a, DICTR_SPEC, O>;
#[doc = "Field `CH1CCDE` reader - CH1CCDE"]
pub type CH1CCDE_R = crate::BitReader;
#[doc = "Field `CH1CCDE` writer - CH1CCDE"]
pub type CH1CCDE_W<'a, const O: u8> = crate::BitWriter<'a, DICTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CH0CCIE"]
    #[inline(always)]
    pub fn ch0ccie(&self) -> CH0CCIE_R {
        CH0CCIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1CCIE"]
    #[inline(always)]
    pub fn ch1ccie(&self) -> CH1CCIE_R {
        CH1CCIE_R::new(((self.bits >> 1) & 1) != 0)
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
    #[doc = "Bit 16 - CH0CCDE"]
    #[inline(always)]
    pub fn ch0ccde(&self) -> CH0CCDE_R {
        CH0CCDE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CH1CCDE"]
    #[inline(always)]
    pub fn ch1ccde(&self) -> CH1CCDE_R {
        CH1CCDE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CCIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccie(&mut self) -> CH0CCIE_W<0> {
        CH0CCIE_W::new(self)
    }
    #[doc = "Bit 1 - CH1CCIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ccie(&mut self) -> CH1CCIE_W<1> {
        CH1CCIE_W::new(self)
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
    #[doc = "Bit 16 - CH0CCDE"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccde(&mut self) -> CH0CCDE_W<16> {
        CH0CCDE_W::new(self)
    }
    #[doc = "Bit 17 - CH1CCDE"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ccde(&mut self) -> CH1CCDE_W<17> {
        CH1CCDE_W::new(self)
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
