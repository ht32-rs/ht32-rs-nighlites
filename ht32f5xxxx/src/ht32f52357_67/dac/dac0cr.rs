#[doc = "Register `DAC0CR` reader"]
pub struct R(crate::R<DAC0CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC0CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC0CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC0CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC0CR` writer"]
pub struct W(crate::W<DAC0CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC0CR_SPEC>;
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
impl From<crate::W<DAC0CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC0CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACEN` reader - DACEN"]
pub type DACEN_R = crate::BitReader;
#[doc = "Field `DACEN` writer - DACEN"]
pub type DACEN_W<'a, const O: u8> = crate::BitWriter<'a, DAC0CR_SPEC, O>;
#[doc = "Field `DACRES` reader - DACRES"]
pub type DACRES_R = crate::BitReader;
#[doc = "Field `DACRES` writer - DACRES"]
pub type DACRES_W<'a, const O: u8> = crate::BitWriter<'a, DAC0CR_SPEC, O>;
#[doc = "Field `DON` reader - DON"]
pub type DON_R = crate::BitReader;
#[doc = "Field `DON` writer - DON"]
pub type DON_W<'a, const O: u8> = crate::BitWriter<'a, DAC0CR_SPEC, O>;
#[doc = "Field `DBON` reader - DBON"]
pub type DBON_R = crate::BitReader;
#[doc = "Field `DBON` writer - DBON"]
pub type DBON_W<'a, const O: u8> = crate::BitWriter<'a, DAC0CR_SPEC, O>;
#[doc = "Field `VREFSEL` reader - VREFSEL"]
pub type VREFSEL_R = crate::FieldReader;
#[doc = "Field `VREFSEL` writer - VREFSEL"]
pub type VREFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, DAC0CR_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0 - DACEN"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - DACRES"]
    #[inline(always)]
    pub fn dacres(&self) -> DACRES_R {
        DACRES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - DON"]
    #[inline(always)]
    pub fn don(&self) -> DON_R {
        DON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DBON"]
    #[inline(always)]
    pub fn dbon(&self) -> DBON_R {
        DBON_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 14:15 - VREFSEL"]
    #[inline(always)]
    pub fn vrefsel(&self) -> VREFSEL_R {
        VREFSEL_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DACEN"]
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DACEN_W<0> {
        DACEN_W::new(self)
    }
    #[doc = "Bit 2 - DACRES"]
    #[inline(always)]
    #[must_use]
    pub fn dacres(&mut self) -> DACRES_W<2> {
        DACRES_W::new(self)
    }
    #[doc = "Bit 6 - DON"]
    #[inline(always)]
    #[must_use]
    pub fn don(&mut self) -> DON_W<6> {
        DON_W::new(self)
    }
    #[doc = "Bit 7 - DBON"]
    #[inline(always)]
    #[must_use]
    pub fn dbon(&mut self) -> DBON_W<7> {
        DBON_W::new(self)
    }
    #[doc = "Bits 14:15 - VREFSEL"]
    #[inline(always)]
    #[must_use]
    pub fn vrefsel(&mut self) -> VREFSEL_W<14> {
        VREFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC0CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac0cr](index.html) module"]
pub struct DAC0CR_SPEC;
impl crate::RegisterSpec for DAC0CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac0cr::R](R) reader structure"]
impl crate::Readable for DAC0CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac0cr::W](W) writer structure"]
impl crate::Writable for DAC0CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC0CR to value 0"]
impl crate::Resettable for DAC0CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
