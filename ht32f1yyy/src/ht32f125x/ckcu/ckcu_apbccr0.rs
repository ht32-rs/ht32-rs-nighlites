#[doc = "Register `CKCU_APBCCR0` reader"]
pub struct R(crate::R<CKCU_APBCCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_APBCCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_APBCCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_APBCCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_APBCCR0` writer"]
pub struct W(crate::W<CKCU_APBCCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_APBCCR0_SPEC>;
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
impl From<crate::W<CKCU_APBCCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_APBCCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2CEN` reader - I2CEN"]
pub type I2CEN_R = crate::BitReader;
#[doc = "Field `I2CEN` writer - I2CEN"]
pub type I2CEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `SPIEN` reader - SPIEN"]
pub type SPIEN_R = crate::BitReader;
#[doc = "Field `SPIEN` writer - SPIEN"]
pub type SPIEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `UREN` reader - UREN"]
pub type UREN_R = crate::BitReader;
#[doc = "Field `UREN` writer - UREN"]
pub type UREN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `AFIOEN` reader - AFIOEN"]
pub type AFIOEN_R = crate::BitReader;
#[doc = "Field `AFIOEN` writer - AFIOEN"]
pub type AFIOEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `EXTIEN` reader - EXTIEN"]
pub type EXTIEN_R = crate::BitReader;
#[doc = "Field `EXTIEN` writer - EXTIEN"]
pub type EXTIEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `PAEN` reader - PAEN"]
pub type PAEN_R = crate::BitReader;
#[doc = "Field `PAEN` writer - PAEN"]
pub type PAEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
#[doc = "Field `PBEN` reader - PBEN"]
pub type PBEN_R = crate::BitReader;
#[doc = "Field `PBEN` writer - PBEN"]
pub type PBEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - I2CEN"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - SPIEN"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - UREN"]
    #[inline(always)]
    pub fn uren(&self) -> UREN_R {
        UREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - AFIOEN"]
    #[inline(always)]
    pub fn afioen(&self) -> AFIOEN_R {
        AFIOEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTIEN"]
    #[inline(always)]
    pub fn extien(&self) -> EXTIEN_R {
        EXTIEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PAEN"]
    #[inline(always)]
    pub fn paen(&self) -> PAEN_R {
        PAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PBEN"]
    #[inline(always)]
    pub fn pben(&self) -> PBEN_R {
        PBEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2CEN"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2CEN_W<0> {
        I2CEN_W::new(self)
    }
    #[doc = "Bit 4 - SPIEN"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<4> {
        SPIEN_W::new(self)
    }
    #[doc = "Bit 8 - UREN"]
    #[inline(always)]
    #[must_use]
    pub fn uren(&mut self) -> UREN_W<8> {
        UREN_W::new(self)
    }
    #[doc = "Bit 14 - AFIOEN"]
    #[inline(always)]
    #[must_use]
    pub fn afioen(&mut self) -> AFIOEN_W<14> {
        AFIOEN_W::new(self)
    }
    #[doc = "Bit 15 - EXTIEN"]
    #[inline(always)]
    #[must_use]
    pub fn extien(&mut self) -> EXTIEN_W<15> {
        EXTIEN_W::new(self)
    }
    #[doc = "Bit 16 - PAEN"]
    #[inline(always)]
    #[must_use]
    pub fn paen(&mut self) -> PAEN_W<16> {
        PAEN_W::new(self)
    }
    #[doc = "Bit 17 - PBEN"]
    #[inline(always)]
    #[must_use]
    pub fn pben(&mut self) -> PBEN_W<17> {
        PBEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_APBCCR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_apbccr0](index.html) module"]
pub struct CKCU_APBCCR0_SPEC;
impl crate::RegisterSpec for CKCU_APBCCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_apbccr0::R](R) reader structure"]
impl crate::Readable for CKCU_APBCCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_apbccr0::W](W) writer structure"]
impl crate::Writable for CKCU_APBCCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_APBCCR0 to value 0"]
impl crate::Resettable for CKCU_APBCCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
