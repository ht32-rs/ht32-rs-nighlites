#[doc = "Register `MCTM_CHCTR` reader"]
pub struct R(crate::R<MCTM_CHCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTM_CHCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTM_CHCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTM_CHCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTM_CHCTR` writer"]
pub struct W(crate::W<MCTM_CHCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTM_CHCTR_SPEC>;
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
impl From<crate::W<MCTM_CHCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTM_CHCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0E` reader - CH0E"]
pub type CH0E_R = crate::BitReader;
#[doc = "Field `CH0E` writer - CH0E"]
pub type CH0E_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHCTR_SPEC, O>;
#[doc = "Field `CH0NE` reader - CH0NE"]
pub type CH0NE_R = crate::BitReader;
#[doc = "Field `CH0NE` writer - CH0NE"]
pub type CH0NE_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHCTR_SPEC, O>;
#[doc = "Field `CH1E` reader - CH1E"]
pub type CH1E_R = crate::BitReader;
#[doc = "Field `CH1E` writer - CH1E"]
pub type CH1E_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHCTR_SPEC, O>;
#[doc = "Field `CH1NE` reader - CH1NE"]
pub type CH1NE_R = crate::BitReader;
#[doc = "Field `CH1NE` writer - CH1NE"]
pub type CH1NE_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHCTR_SPEC, O>;
#[doc = "Field `CH2E` reader - CH2E"]
pub type CH2E_R = crate::BitReader;
#[doc = "Field `CH2E` writer - CH2E"]
pub type CH2E_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHCTR_SPEC, O>;
#[doc = "Field `CH2NE` reader - CH2NE"]
pub type CH2NE_R = crate::BitReader;
#[doc = "Field `CH2NE` writer - CH2NE"]
pub type CH2NE_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHCTR_SPEC, O>;
#[doc = "Field `CH3E` reader - CH3E"]
pub type CH3E_R = crate::BitReader;
#[doc = "Field `CH3E` writer - CH3E"]
pub type CH3E_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHCTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CH0E"]
    #[inline(always)]
    pub fn ch0e(&self) -> CH0E_R {
        CH0E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH0NE"]
    #[inline(always)]
    pub fn ch0ne(&self) -> CH0NE_R {
        CH0NE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH1E"]
    #[inline(always)]
    pub fn ch1e(&self) -> CH1E_R {
        CH1E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH1NE"]
    #[inline(always)]
    pub fn ch1ne(&self) -> CH1NE_R {
        CH1NE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH2E"]
    #[inline(always)]
    pub fn ch2e(&self) -> CH2E_R {
        CH2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH2NE"]
    #[inline(always)]
    pub fn ch2ne(&self) -> CH2NE_R {
        CH2NE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH3E"]
    #[inline(always)]
    pub fn ch3e(&self) -> CH3E_R {
        CH3E_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0E"]
    #[inline(always)]
    #[must_use]
    pub fn ch0e(&mut self) -> CH0E_W<0> {
        CH0E_W::new(self)
    }
    #[doc = "Bit 1 - CH0NE"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ne(&mut self) -> CH0NE_W<1> {
        CH0NE_W::new(self)
    }
    #[doc = "Bit 2 - CH1E"]
    #[inline(always)]
    #[must_use]
    pub fn ch1e(&mut self) -> CH1E_W<2> {
        CH1E_W::new(self)
    }
    #[doc = "Bit 3 - CH1NE"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ne(&mut self) -> CH1NE_W<3> {
        CH1NE_W::new(self)
    }
    #[doc = "Bit 4 - CH2E"]
    #[inline(always)]
    #[must_use]
    pub fn ch2e(&mut self) -> CH2E_W<4> {
        CH2E_W::new(self)
    }
    #[doc = "Bit 5 - CH2NE"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ne(&mut self) -> CH2NE_W<5> {
        CH2NE_W::new(self)
    }
    #[doc = "Bit 6 - CH3E"]
    #[inline(always)]
    #[must_use]
    pub fn ch3e(&mut self) -> CH3E_W<6> {
        CH3E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCTM_CHCTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_chctr](index.html) module"]
pub struct MCTM_CHCTR_SPEC;
impl crate::RegisterSpec for MCTM_CHCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctm_chctr::R](R) reader structure"]
impl crate::Readable for MCTM_CHCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctm_chctr::W](W) writer structure"]
impl crate::Writable for MCTM_CHCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTM_CHCTR to value 0"]
impl crate::Resettable for MCTM_CHCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
