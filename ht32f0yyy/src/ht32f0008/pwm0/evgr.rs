#[doc = "Register `EVGR` reader"]
pub struct R(crate::R<EVGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVGR` writer"]
pub struct W(crate::W<EVGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVGR_SPEC>;
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
impl From<crate::W<EVGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0CG` reader - CH0CG"]
pub type CH0CG_R = crate::BitReader;
#[doc = "Field `CH0CG` writer - CH0CG"]
pub type CH0CG_W<'a, const O: u8> = crate::BitWriter<'a, EVGR_SPEC, O>;
#[doc = "Field `CH1CG` reader - CH1CG"]
pub type CH1CG_R = crate::BitReader;
#[doc = "Field `CH1CG` writer - CH1CG"]
pub type CH1CG_W<'a, const O: u8> = crate::BitWriter<'a, EVGR_SPEC, O>;
#[doc = "Field `CH2CG` reader - CH2CG"]
pub type CH2CG_R = crate::BitReader;
#[doc = "Field `CH2CG` writer - CH2CG"]
pub type CH2CG_W<'a, const O: u8> = crate::BitWriter<'a, EVGR_SPEC, O>;
#[doc = "Field `CH3CG` reader - CH3CG"]
pub type CH3CG_R = crate::BitReader;
#[doc = "Field `CH3CG` writer - CH3CG"]
pub type CH3CG_W<'a, const O: u8> = crate::BitWriter<'a, EVGR_SPEC, O>;
#[doc = "Field `UEVG` reader - UEVG"]
pub type UEVG_R = crate::BitReader;
#[doc = "Field `UEVG` writer - UEVG"]
pub type UEVG_W<'a, const O: u8> = crate::BitWriter<'a, EVGR_SPEC, O>;
#[doc = "Field `TEVG` reader - TEVG"]
pub type TEVG_R = crate::BitReader;
#[doc = "Field `TEVG` writer - TEVG"]
pub type TEVG_W<'a, const O: u8> = crate::BitWriter<'a, EVGR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CH0CG"]
    #[inline(always)]
    pub fn ch0cg(&self) -> CH0CG_R {
        CH0CG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1CG"]
    #[inline(always)]
    pub fn ch1cg(&self) -> CH1CG_R {
        CH1CG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2CG"]
    #[inline(always)]
    pub fn ch2cg(&self) -> CH2CG_R {
        CH2CG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3CG"]
    #[inline(always)]
    pub fn ch3cg(&self) -> CH3CG_R {
        CH3CG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - UEVG"]
    #[inline(always)]
    pub fn uevg(&self) -> UEVG_R {
        UEVG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - TEVG"]
    #[inline(always)]
    pub fn tevg(&self) -> TEVG_R {
        TEVG_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CG"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cg(&mut self) -> CH0CG_W<0> {
        CH0CG_W::new(self)
    }
    #[doc = "Bit 1 - CH1CG"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cg(&mut self) -> CH1CG_W<1> {
        CH1CG_W::new(self)
    }
    #[doc = "Bit 2 - CH2CG"]
    #[inline(always)]
    #[must_use]
    pub fn ch2cg(&mut self) -> CH2CG_W<2> {
        CH2CG_W::new(self)
    }
    #[doc = "Bit 3 - CH3CG"]
    #[inline(always)]
    #[must_use]
    pub fn ch3cg(&mut self) -> CH3CG_W<3> {
        CH3CG_W::new(self)
    }
    #[doc = "Bit 8 - UEVG"]
    #[inline(always)]
    #[must_use]
    pub fn uevg(&mut self) -> UEVG_W<8> {
        UEVG_W::new(self)
    }
    #[doc = "Bit 10 - TEVG"]
    #[inline(always)]
    #[must_use]
    pub fn tevg(&mut self) -> TEVG_W<10> {
        TEVG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EVGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evgr](index.html) module"]
pub struct EVGR_SPEC;
impl crate::RegisterSpec for EVGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evgr::R](R) reader structure"]
impl crate::Readable for EVGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evgr::W](W) writer structure"]
impl crate::Writable for EVGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVGR to value 0"]
impl crate::Resettable for EVGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
