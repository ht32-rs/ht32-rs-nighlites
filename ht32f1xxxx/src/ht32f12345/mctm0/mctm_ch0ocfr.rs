#[doc = "Register `MCTM_CH0OCFR` reader"]
pub struct R(crate::R<MCTM_CH0OCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTM_CH0OCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTM_CH0OCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTM_CH0OCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTM_CH0OCFR` writer"]
pub struct W(crate::W<MCTM_CH0OCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTM_CH0OCFR_SPEC>;
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
impl From<crate::W<MCTM_CH0OCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTM_CH0OCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0OM` reader - CH0OM"]
pub type CH0OM_R = crate::FieldReader;
#[doc = "Field `CH0OM` writer - CH0OM"]
pub type CH0OM_W<'a, const O: u8> = crate::FieldWriter<'a, MCTM_CH0OCFR_SPEC, 3, O>;
#[doc = "Field `REF0CE` reader - REF0CE"]
pub type REF0CE_R = crate::BitReader;
#[doc = "Field `REF0CE` writer - REF0CE"]
pub type REF0CE_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CH0OCFR_SPEC, O>;
#[doc = "Field `CH0PRE` reader - CH0PRE"]
pub type CH0PRE_R = crate::BitReader;
#[doc = "Field `CH0PRE` writer - CH0PRE"]
pub type CH0PRE_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CH0OCFR_SPEC, O>;
#[doc = "Field `CH0IMAE` reader - CH0IMAE"]
pub type CH0IMAE_R = crate::BitReader;
#[doc = "Field `CH0IMAE` writer - CH0IMAE"]
pub type CH0IMAE_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CH0OCFR_SPEC, O>;
#[doc = "Field `CH0OM3` reader - CH0OM3"]
pub type CH0OM3_R = crate::BitReader;
#[doc = "Field `CH0OM3` writer - CH0OM3"]
pub type CH0OM3_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CH0OCFR_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - CH0OM"]
    #[inline(always)]
    pub fn ch0om(&self) -> CH0OM_R {
        CH0OM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - REF0CE"]
    #[inline(always)]
    pub fn ref0ce(&self) -> REF0CE_R {
        REF0CE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH0PRE"]
    #[inline(always)]
    pub fn ch0pre(&self) -> CH0PRE_R {
        CH0PRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH0IMAE"]
    #[inline(always)]
    pub fn ch0imae(&self) -> CH0IMAE_R {
        CH0IMAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CH0OM3"]
    #[inline(always)]
    pub fn ch0om3(&self) -> CH0OM3_R {
        CH0OM3_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH0OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch0om(&mut self) -> CH0OM_W<0> {
        CH0OM_W::new(self)
    }
    #[doc = "Bit 3 - REF0CE"]
    #[inline(always)]
    #[must_use]
    pub fn ref0ce(&mut self) -> REF0CE_W<3> {
        REF0CE_W::new(self)
    }
    #[doc = "Bit 4 - CH0PRE"]
    #[inline(always)]
    #[must_use]
    pub fn ch0pre(&mut self) -> CH0PRE_W<4> {
        CH0PRE_W::new(self)
    }
    #[doc = "Bit 5 - CH0IMAE"]
    #[inline(always)]
    #[must_use]
    pub fn ch0imae(&mut self) -> CH0IMAE_W<5> {
        CH0IMAE_W::new(self)
    }
    #[doc = "Bit 8 - CH0OM3"]
    #[inline(always)]
    #[must_use]
    pub fn ch0om3(&mut self) -> CH0OM3_W<8> {
        CH0OM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCTM_CH0OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch0ocfr](index.html) module"]
pub struct MCTM_CH0OCFR_SPEC;
impl crate::RegisterSpec for MCTM_CH0OCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctm_ch0ocfr::R](R) reader structure"]
impl crate::Readable for MCTM_CH0OCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctm_ch0ocfr::W](W) writer structure"]
impl crate::Writable for MCTM_CH0OCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTM_CH0OCFR to value 0"]
impl crate::Resettable for MCTM_CH0OCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
