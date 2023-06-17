#[doc = "Register `I2CCR` reader"]
pub struct R(crate::R<I2CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2CCR` writer"]
pub struct W(crate::W<I2CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2CCR_SPEC>;
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
impl From<crate::W<I2CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AA` reader - AA"]
pub type AA_R = crate::BitReader;
#[doc = "Field `AA` writer - AA"]
pub type AA_W<'a, const O: u8> = crate::BitWriter<'a, I2CCR_SPEC, O>;
#[doc = "Field `STOP` reader - STOP"]
pub type STOP_R = crate::BitReader;
#[doc = "Field `STOP` writer - STOP"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, I2CCR_SPEC, O>;
#[doc = "Field `GCEN` reader - GCEN"]
pub type GCEN_R = crate::BitReader;
#[doc = "Field `GCEN` writer - GCEN"]
pub type GCEN_W<'a, const O: u8> = crate::BitWriter<'a, I2CCR_SPEC, O>;
#[doc = "Field `I2CEN` reader - I2CEN"]
pub type I2CEN_R = crate::BitReader;
#[doc = "Field `I2CEN` writer - I2CEN"]
pub type I2CEN_W<'a, const O: u8> = crate::BitWriter<'a, I2CCR_SPEC, O>;
#[doc = "Field `ADRM` reader - ADRM"]
pub type ADRM_R = crate::BitReader;
#[doc = "Field `ADRM` writer - ADRM"]
pub type ADRM_W<'a, const O: u8> = crate::BitWriter<'a, I2CCR_SPEC, O>;
#[doc = "Field `ENTOUT` reader - ENTOUT"]
pub type ENTOUT_R = crate::BitReader;
#[doc = "Field `ENTOUT` writer - ENTOUT"]
pub type ENTOUT_W<'a, const O: u8> = crate::BitWriter<'a, I2CCR_SPEC, O>;
#[doc = "Field `COMBFILTEREN` reader - COMBFILTEREN"]
pub type COMBFILTEREN_R = crate::BitReader;
#[doc = "Field `COMBFILTEREN` writer - COMBFILTEREN"]
pub type COMBFILTEREN_W<'a, const O: u8> = crate::BitWriter<'a, I2CCR_SPEC, O>;
#[doc = "Field `SEQFILTER` reader - SEQFILTER"]
pub type SEQFILTER_R = crate::FieldReader;
#[doc = "Field `SEQFILTER` writer - SEQFILTER"]
pub type SEQFILTER_W<'a, const O: u8> = crate::FieldWriter<'a, I2CCR_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0 - AA"]
    #[inline(always)]
    pub fn aa(&self) -> AA_R {
        AA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STOP"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GCEN"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2CEN"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - ADRM"]
    #[inline(always)]
    pub fn adrm(&self) -> ADRM_R {
        ADRM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - ENTOUT"]
    #[inline(always)]
    pub fn entout(&self) -> ENTOUT_R {
        ENTOUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - COMBFILTEREN"]
    #[inline(always)]
    pub fn combfilteren(&self) -> COMBFILTEREN_R {
        COMBFILTEREN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - SEQFILTER"]
    #[inline(always)]
    pub fn seqfilter(&self) -> SEQFILTER_R {
        SEQFILTER_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AA"]
    #[inline(always)]
    #[must_use]
    pub fn aa(&mut self) -> AA_W<0> {
        AA_W::new(self)
    }
    #[doc = "Bit 1 - STOP"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<1> {
        STOP_W::new(self)
    }
    #[doc = "Bit 2 - GCEN"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GCEN_W<2> {
        GCEN_W::new(self)
    }
    #[doc = "Bit 3 - I2CEN"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2CEN_W<3> {
        I2CEN_W::new(self)
    }
    #[doc = "Bit 7 - ADRM"]
    #[inline(always)]
    #[must_use]
    pub fn adrm(&mut self) -> ADRM_W<7> {
        ADRM_W::new(self)
    }
    #[doc = "Bit 12 - ENTOUT"]
    #[inline(always)]
    #[must_use]
    pub fn entout(&mut self) -> ENTOUT_W<12> {
        ENTOUT_W::new(self)
    }
    #[doc = "Bit 13 - COMBFILTEREN"]
    #[inline(always)]
    #[must_use]
    pub fn combfilteren(&mut self) -> COMBFILTEREN_W<13> {
        COMBFILTEREN_W::new(self)
    }
    #[doc = "Bits 14:15 - SEQFILTER"]
    #[inline(always)]
    #[must_use]
    pub fn seqfilter(&mut self) -> SEQFILTER_W<14> {
        SEQFILTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2ccr](index.html) module"]
pub struct I2CCR_SPEC;
impl crate::RegisterSpec for I2CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2ccr::R](R) reader structure"]
impl crate::Readable for I2CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2ccr::W](W) writer structure"]
impl crate::Writable for I2CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2CCR to value 0"]
impl crate::Resettable for I2CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
