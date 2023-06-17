#[doc = "Register `CHCTR` reader"]
pub struct R(crate::R<CHCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTR` writer"]
pub struct W(crate::W<CHCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTR_SPEC>;
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
impl From<crate::W<CHCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0E` reader - CH0E"]
pub type CH0E_R = crate::BitReader;
#[doc = "Field `CH0E` writer - CH0E"]
pub type CH0E_W<'a, const O: u8> = crate::BitWriter<'a, CHCTR_SPEC, O>;
#[doc = "Field `CH1E` reader - CH1E"]
pub type CH1E_R = crate::BitReader;
#[doc = "Field `CH1E` writer - CH1E"]
pub type CH1E_W<'a, const O: u8> = crate::BitWriter<'a, CHCTR_SPEC, O>;
#[doc = "Field `CH2E` reader - CH2E"]
pub type CH2E_R = crate::BitReader;
#[doc = "Field `CH2E` writer - CH2E"]
pub type CH2E_W<'a, const O: u8> = crate::BitWriter<'a, CHCTR_SPEC, O>;
#[doc = "Field `CH3E` reader - CH3E"]
pub type CH3E_R = crate::BitReader;
#[doc = "Field `CH3E` writer - CH3E"]
pub type CH3E_W<'a, const O: u8> = crate::BitWriter<'a, CHCTR_SPEC, O>;
#[doc = "Field `CH4E` reader - CH4E"]
pub type CH4E_R = crate::BitReader;
#[doc = "Field `CH4E` writer - CH4E"]
pub type CH4E_W<'a, const O: u8> = crate::BitWriter<'a, CHCTR_SPEC, O>;
#[doc = "Field `CH5E` reader - CH5E"]
pub type CH5E_R = crate::BitReader;
#[doc = "Field `CH5E` writer - CH5E"]
pub type CH5E_W<'a, const O: u8> = crate::BitWriter<'a, CHCTR_SPEC, O>;
#[doc = "Field `CH6E` reader - CH6E"]
pub type CH6E_R = crate::BitReader;
#[doc = "Field `CH6E` writer - CH6E"]
pub type CH6E_W<'a, const O: u8> = crate::BitWriter<'a, CHCTR_SPEC, O>;
#[doc = "Field `CH7E` reader - CH7E"]
pub type CH7E_R = crate::BitReader;
#[doc = "Field `CH7E` writer - CH7E"]
pub type CH7E_W<'a, const O: u8> = crate::BitWriter<'a, CHCTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CH0E"]
    #[inline(always)]
    pub fn ch0e(&self) -> CH0E_R {
        CH0E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CH1E"]
    #[inline(always)]
    pub fn ch1e(&self) -> CH1E_R {
        CH1E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CH2E"]
    #[inline(always)]
    pub fn ch2e(&self) -> CH2E_R {
        CH2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CH3E"]
    #[inline(always)]
    pub fn ch3e(&self) -> CH3E_R {
        CH3E_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CH4E"]
    #[inline(always)]
    pub fn ch4e(&self) -> CH4E_R {
        CH4E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - CH5E"]
    #[inline(always)]
    pub fn ch5e(&self) -> CH5E_R {
        CH5E_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - CH6E"]
    #[inline(always)]
    pub fn ch6e(&self) -> CH6E_R {
        CH6E_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CH7E"]
    #[inline(always)]
    pub fn ch7e(&self) -> CH7E_R {
        CH7E_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0E"]
    #[inline(always)]
    #[must_use]
    pub fn ch0e(&mut self) -> CH0E_W<0> {
        CH0E_W::new(self)
    }
    #[doc = "Bit 2 - CH1E"]
    #[inline(always)]
    #[must_use]
    pub fn ch1e(&mut self) -> CH1E_W<2> {
        CH1E_W::new(self)
    }
    #[doc = "Bit 4 - CH2E"]
    #[inline(always)]
    #[must_use]
    pub fn ch2e(&mut self) -> CH2E_W<4> {
        CH2E_W::new(self)
    }
    #[doc = "Bit 6 - CH3E"]
    #[inline(always)]
    #[must_use]
    pub fn ch3e(&mut self) -> CH3E_W<6> {
        CH3E_W::new(self)
    }
    #[doc = "Bit 8 - CH4E"]
    #[inline(always)]
    #[must_use]
    pub fn ch4e(&mut self) -> CH4E_W<8> {
        CH4E_W::new(self)
    }
    #[doc = "Bit 10 - CH5E"]
    #[inline(always)]
    #[must_use]
    pub fn ch5e(&mut self) -> CH5E_W<10> {
        CH5E_W::new(self)
    }
    #[doc = "Bit 12 - CH6E"]
    #[inline(always)]
    #[must_use]
    pub fn ch6e(&mut self) -> CH6E_W<12> {
        CH6E_W::new(self)
    }
    #[doc = "Bit 14 - CH7E"]
    #[inline(always)]
    #[must_use]
    pub fn ch7e(&mut self) -> CH7E_W<14> {
        CH7E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHCTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctr](index.html) module"]
pub struct CHCTR_SPEC;
impl crate::RegisterSpec for CHCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctr::R](R) reader structure"]
impl crate::Readable for CHCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctr::W](W) writer structure"]
impl crate::Writable for CHCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTR to value 0"]
impl crate::Resettable for CHCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
