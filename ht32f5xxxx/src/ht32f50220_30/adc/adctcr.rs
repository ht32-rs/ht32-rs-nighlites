#[doc = "Register `ADCTCR` reader"]
pub struct R(crate::R<ADCTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCTCR` writer"]
pub struct W(crate::W<ADCTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCTCR_SPEC>;
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
impl From<crate::W<ADCTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADSW` reader - ADSW"]
pub type ADSW_R = crate::BitReader;
#[doc = "Field `ADSW` writer - ADSW"]
pub type ADSW_W<'a, const O: u8> = crate::BitWriter<'a, ADCTCR_SPEC, O>;
#[doc = "Field `ADEXTI` reader - ADEXTI"]
pub type ADEXTI_R = crate::BitReader;
#[doc = "Field `ADEXTI` writer - ADEXTI"]
pub type ADEXTI_W<'a, const O: u8> = crate::BitWriter<'a, ADCTCR_SPEC, O>;
#[doc = "Field `TM0` reader - TM0"]
pub type TM0_R = crate::BitReader;
#[doc = "Field `TM0` writer - TM0"]
pub type TM0_W<'a, const O: u8> = crate::BitWriter<'a, ADCTCR_SPEC, O>;
#[doc = "Field `TM1` reader - TM1"]
pub type TM1_R = crate::BitReader;
#[doc = "Field `TM1` writer - TM1"]
pub type TM1_W<'a, const O: u8> = crate::BitWriter<'a, ADCTCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ADSW"]
    #[inline(always)]
    pub fn adsw(&self) -> ADSW_R {
        ADSW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADEXTI"]
    #[inline(always)]
    pub fn adexti(&self) -> ADEXTI_R {
        ADEXTI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TM0"]
    #[inline(always)]
    pub fn tm0(&self) -> TM0_R {
        TM0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TM1"]
    #[inline(always)]
    pub fn tm1(&self) -> TM1_R {
        TM1_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADSW"]
    #[inline(always)]
    #[must_use]
    pub fn adsw(&mut self) -> ADSW_W<0> {
        ADSW_W::new(self)
    }
    #[doc = "Bit 1 - ADEXTI"]
    #[inline(always)]
    #[must_use]
    pub fn adexti(&mut self) -> ADEXTI_W<1> {
        ADEXTI_W::new(self)
    }
    #[doc = "Bit 2 - TM0"]
    #[inline(always)]
    #[must_use]
    pub fn tm0(&mut self) -> TM0_W<2> {
        TM0_W::new(self)
    }
    #[doc = "Bit 3 - TM1"]
    #[inline(always)]
    #[must_use]
    pub fn tm1(&mut self) -> TM1_W<3> {
        TM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADCTCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adctcr](index.html) module"]
pub struct ADCTCR_SPEC;
impl crate::RegisterSpec for ADCTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adctcr::R](R) reader structure"]
impl crate::Readable for ADCTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adctcr::W](W) writer structure"]
impl crate::Writable for ADCTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCTCR to value 0"]
impl crate::Resettable for ADCTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
