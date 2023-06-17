#[doc = "Register `MCTM_CH3ICFR` reader"]
pub struct R(crate::R<MCTM_CH3ICFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTM_CH3ICFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTM_CH3ICFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTM_CH3ICFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTM_CH3ICFR` writer"]
pub struct W(crate::W<MCTM_CH3ICFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTM_CH3ICFR_SPEC>;
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
impl From<crate::W<MCTM_CH3ICFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTM_CH3ICFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI3F` reader - TI3F"]
pub type TI3F_R = crate::FieldReader;
#[doc = "Field `TI3F` writer - TI3F"]
pub type TI3F_W<'a, const O: u8> = crate::FieldWriter<'a, MCTM_CH3ICFR_SPEC, 4, O>;
#[doc = "Field `CH3CCS` reader - CH3CCS"]
pub type CH3CCS_R = crate::FieldReader;
#[doc = "Field `CH3CCS` writer - CH3CCS"]
pub type CH3CCS_W<'a, const O: u8> = crate::FieldWriter<'a, MCTM_CH3ICFR_SPEC, 2, O>;
#[doc = "Field `CH3PSC` reader - CH3PSC"]
pub type CH3PSC_R = crate::FieldReader;
#[doc = "Field `CH3PSC` writer - CH3PSC"]
pub type CH3PSC_W<'a, const O: u8> = crate::FieldWriter<'a, MCTM_CH3ICFR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:3 - TI3F"]
    #[inline(always)]
    pub fn ti3f(&self) -> TI3F_R {
        TI3F_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - CH3CCS"]
    #[inline(always)]
    pub fn ch3ccs(&self) -> CH3CCS_R {
        CH3CCS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - CH3PSC"]
    #[inline(always)]
    pub fn ch3psc(&self) -> CH3PSC_R {
        CH3PSC_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI3F"]
    #[inline(always)]
    #[must_use]
    pub fn ti3f(&mut self) -> TI3F_W<0> {
        TI3F_W::new(self)
    }
    #[doc = "Bits 16:17 - CH3CCS"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ccs(&mut self) -> CH3CCS_W<16> {
        CH3CCS_W::new(self)
    }
    #[doc = "Bits 18:19 - CH3PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ch3psc(&mut self) -> CH3PSC_W<18> {
        CH3PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCTM_CH3ICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch3icfr](index.html) module"]
pub struct MCTM_CH3ICFR_SPEC;
impl crate::RegisterSpec for MCTM_CH3ICFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctm_ch3icfr::R](R) reader structure"]
impl crate::Readable for MCTM_CH3ICFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctm_ch3icfr::W](W) writer structure"]
impl crate::Writable for MCTM_CH3ICFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTM_CH3ICFR to value 0"]
impl crate::Resettable for MCTM_CH3ICFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
