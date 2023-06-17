#[doc = "Register `MCTM_CH1OCFR` reader"]
pub struct R(crate::R<MCTM_CH1OCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTM_CH1OCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTM_CH1OCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTM_CH1OCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTM_CH1OCFR` writer"]
pub struct W(crate::W<MCTM_CH1OCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTM_CH1OCFR_SPEC>;
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
impl From<crate::W<MCTM_CH1OCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTM_CH1OCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH1OM20` reader - CH1OM\\[2:0\\]"]
pub type CH1OM20_R = crate::FieldReader;
#[doc = "Field `CH1OM20` writer - CH1OM\\[2:0\\]"]
pub type CH1OM20_W<'a, const O: u8> = crate::FieldWriter<'a, MCTM_CH1OCFR_SPEC, 3, O>;
#[doc = "Field `CH1PRE` reader - CH1PRE"]
pub type CH1PRE_R = crate::BitReader;
#[doc = "Field `CH1PRE` writer - CH1PRE"]
pub type CH1PRE_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CH1OCFR_SPEC, O>;
#[doc = "Field `CH1IMAE` reader - CH1IMAE"]
pub type CH1IMAE_R = crate::BitReader;
#[doc = "Field `CH1IMAE` writer - CH1IMAE"]
pub type CH1IMAE_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CH1OCFR_SPEC, O>;
#[doc = "Field `CH1OM3` reader - CH1OM3"]
pub type CH1OM3_R = crate::BitReader;
#[doc = "Field `CH1OM3` writer - CH1OM3"]
pub type CH1OM3_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CH1OCFR_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - CH1OM\\[2:0\\]"]
    #[inline(always)]
    pub fn ch1om20(&self) -> CH1OM20_R {
        CH1OM20_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - CH1PRE"]
    #[inline(always)]
    pub fn ch1pre(&self) -> CH1PRE_R {
        CH1PRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH1IMAE"]
    #[inline(always)]
    pub fn ch1imae(&self) -> CH1IMAE_R {
        CH1IMAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CH1OM3"]
    #[inline(always)]
    pub fn ch1om3(&self) -> CH1OM3_R {
        CH1OM3_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH1OM\\[2:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch1om20(&mut self) -> CH1OM20_W<0> {
        CH1OM20_W::new(self)
    }
    #[doc = "Bit 4 - CH1PRE"]
    #[inline(always)]
    #[must_use]
    pub fn ch1pre(&mut self) -> CH1PRE_W<4> {
        CH1PRE_W::new(self)
    }
    #[doc = "Bit 5 - CH1IMAE"]
    #[inline(always)]
    #[must_use]
    pub fn ch1imae(&mut self) -> CH1IMAE_W<5> {
        CH1IMAE_W::new(self)
    }
    #[doc = "Bit 8 - CH1OM3"]
    #[inline(always)]
    #[must_use]
    pub fn ch1om3(&mut self) -> CH1OM3_W<8> {
        CH1OM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCTM_CH1OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch1ocfr](index.html) module"]
pub struct MCTM_CH1OCFR_SPEC;
impl crate::RegisterSpec for MCTM_CH1OCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctm_ch1ocfr::R](R) reader structure"]
impl crate::Readable for MCTM_CH1OCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctm_ch1ocfr::W](W) writer structure"]
impl crate::Writable for MCTM_CH1OCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTM_CH1OCFR to value 0"]
impl crate::Resettable for MCTM_CH1OCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
