#[doc = "Register `INTSR` reader"]
pub struct R(crate::R<INTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSR` writer"]
pub struct W(crate::W<INTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSR_SPEC>;
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
impl From<crate::W<INTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0CIF` reader - CH0CIF"]
pub type CH0CIF_R = crate::BitReader;
#[doc = "Field `CH0CIF` writer - CH0CIF"]
pub type CH0CIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `CH1CIF` reader - CH1CIF"]
pub type CH1CIF_R = crate::BitReader;
#[doc = "Field `CH1CIF` writer - CH1CIF"]
pub type CH1CIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `CH2CIF` reader - CH2CIF"]
pub type CH2CIF_R = crate::BitReader;
#[doc = "Field `CH2CIF` writer - CH2CIF"]
pub type CH2CIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `CH3CIF` reader - CH3CIF"]
pub type CH3CIF_R = crate::BitReader;
#[doc = "Field `CH3CIF` writer - CH3CIF"]
pub type CH3CIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `UEVIF` reader - UEVIF"]
pub type UEVIF_R = crate::BitReader;
#[doc = "Field `UEVIF` writer - UEVIF"]
pub type UEVIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `TEVIF` reader - TEVIF"]
pub type TEVIF_R = crate::BitReader;
#[doc = "Field `TEVIF` writer - TEVIF"]
pub type TEVIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CH0CIF"]
    #[inline(always)]
    pub fn ch0cif(&self) -> CH0CIF_R {
        CH0CIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1CIF"]
    #[inline(always)]
    pub fn ch1cif(&self) -> CH1CIF_R {
        CH1CIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2CIF"]
    #[inline(always)]
    pub fn ch2cif(&self) -> CH2CIF_R {
        CH2CIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3CIF"]
    #[inline(always)]
    pub fn ch3cif(&self) -> CH3CIF_R {
        CH3CIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - UEVIF"]
    #[inline(always)]
    pub fn uevif(&self) -> UEVIF_R {
        UEVIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - TEVIF"]
    #[inline(always)]
    pub fn tevif(&self) -> TEVIF_R {
        TEVIF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cif(&mut self) -> CH0CIF_W<0> {
        CH0CIF_W::new(self)
    }
    #[doc = "Bit 1 - CH1CIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cif(&mut self) -> CH1CIF_W<1> {
        CH1CIF_W::new(self)
    }
    #[doc = "Bit 2 - CH2CIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch2cif(&mut self) -> CH2CIF_W<2> {
        CH2CIF_W::new(self)
    }
    #[doc = "Bit 3 - CH3CIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch3cif(&mut self) -> CH3CIF_W<3> {
        CH3CIF_W::new(self)
    }
    #[doc = "Bit 8 - UEVIF"]
    #[inline(always)]
    #[must_use]
    pub fn uevif(&mut self) -> UEVIF_W<8> {
        UEVIF_W::new(self)
    }
    #[doc = "Bit 10 - TEVIF"]
    #[inline(always)]
    #[must_use]
    pub fn tevif(&mut self) -> TEVIF_W<10> {
        TEVIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intsr](index.html) module"]
pub struct INTSR_SPEC;
impl crate::RegisterSpec for INTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intsr::R](R) reader structure"]
impl crate::Readable for INTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intsr::W](W) writer structure"]
impl crate::Writable for INTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTSR to value 0"]
impl crate::Resettable for INTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
