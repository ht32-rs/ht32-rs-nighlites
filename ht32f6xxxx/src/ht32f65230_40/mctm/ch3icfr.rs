#[doc = "Register `CH3ICFR` reader"]
pub struct R(crate::R<CH3ICFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3ICFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3ICFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3ICFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3ICFR` writer"]
pub struct W(crate::W<CH3ICFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3ICFR_SPEC>;
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
impl From<crate::W<CH3ICFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH3ICFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI3FN` reader - TI3FN"]
pub type TI3FN_R = crate::FieldReader;
#[doc = "Field `TI3FN` writer - TI3FN"]
pub type TI3FN_W<'a, const O: u8> = crate::FieldWriter<'a, CH3ICFR_SPEC, 3, O>;
#[doc = "Field `TI3FF` reader - TI3FF"]
pub type TI3FF_R = crate::FieldReader;
#[doc = "Field `TI3FF` writer - TI3FF"]
pub type TI3FF_W<'a, const O: u8> = crate::FieldWriter<'a, CH3ICFR_SPEC, 3, O>;
#[doc = "Field `CH3CCS` reader - CH3CCS"]
pub type CH3CCS_R = crate::FieldReader;
#[doc = "Field `CH3CCS` writer - CH3CCS"]
pub type CH3CCS_W<'a, const O: u8> = crate::FieldWriter<'a, CH3ICFR_SPEC, 2, O>;
#[doc = "Field `CH3PSC` reader - CH3PSC"]
pub type CH3PSC_R = crate::FieldReader;
#[doc = "Field `CH3PSC` writer - CH3PSC"]
pub type CH3PSC_W<'a, const O: u8> = crate::FieldWriter<'a, CH3ICFR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:2 - TI3FN"]
    #[inline(always)]
    pub fn ti3fn(&self) -> TI3FN_R {
        TI3FN_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - TI3FF"]
    #[inline(always)]
    pub fn ti3ff(&self) -> TI3FF_R {
        TI3FF_R::new(((self.bits >> 4) & 7) as u8)
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
    #[doc = "Bits 0:2 - TI3FN"]
    #[inline(always)]
    #[must_use]
    pub fn ti3fn(&mut self) -> TI3FN_W<0> {
        TI3FN_W::new(self)
    }
    #[doc = "Bits 4:6 - TI3FF"]
    #[inline(always)]
    #[must_use]
    pub fn ti3ff(&mut self) -> TI3FF_W<4> {
        TI3FF_W::new(self)
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
#[doc = "CH3ICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3icfr](index.html) module"]
pub struct CH3ICFR_SPEC;
impl crate::RegisterSpec for CH3ICFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3icfr::R](R) reader structure"]
impl crate::Readable for CH3ICFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3icfr::W](W) writer structure"]
impl crate::Writable for CH3ICFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3ICFR to value 0"]
impl crate::Resettable for CH3ICFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
