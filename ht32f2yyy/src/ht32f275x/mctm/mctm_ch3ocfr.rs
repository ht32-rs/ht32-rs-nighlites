#[doc = "Register `MCTM_CH3OCFR` reader"]
pub struct R(crate::R<MCTM_CH3OCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTM_CH3OCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTM_CH3OCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTM_CH3OCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTM_CH3OCFR` writer"]
pub struct W(crate::W<MCTM_CH3OCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTM_CH3OCFR_SPEC>;
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
impl From<crate::W<MCTM_CH3OCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTM_CH3OCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH3OM` reader - CH3OM"]
pub type CH3OM_R = crate::FieldReader;
#[doc = "Field `CH3OM` writer - CH3OM"]
pub type CH3OM_W<'a, const O: u8> = crate::FieldWriter<'a, MCTM_CH3OCFR_SPEC, 3, O>;
#[doc = "Field `REF3CE` reader - REF3CE"]
pub type REF3CE_R = crate::BitReader;
#[doc = "Field `REF3CE` writer - REF3CE"]
pub type REF3CE_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CH3OCFR_SPEC, O>;
#[doc = "Field `CH3PRE` reader - CH3PRE"]
pub type CH3PRE_R = crate::BitReader;
#[doc = "Field `CH3PRE` writer - CH3PRE"]
pub type CH3PRE_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CH3OCFR_SPEC, O>;
#[doc = "Field `CH3IMAE` reader - CH3IMAE"]
pub type CH3IMAE_R = crate::BitReader;
#[doc = "Field `CH3IMAE` writer - CH3IMAE"]
pub type CH3IMAE_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CH3OCFR_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - CH3OM"]
    #[inline(always)]
    pub fn ch3om(&self) -> CH3OM_R {
        CH3OM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - REF3CE"]
    #[inline(always)]
    pub fn ref3ce(&self) -> REF3CE_R {
        REF3CE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH3PRE"]
    #[inline(always)]
    pub fn ch3pre(&self) -> CH3PRE_R {
        CH3PRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH3IMAE"]
    #[inline(always)]
    pub fn ch3imae(&self) -> CH3IMAE_R {
        CH3IMAE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH3OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch3om(&mut self) -> CH3OM_W<0> {
        CH3OM_W::new(self)
    }
    #[doc = "Bit 3 - REF3CE"]
    #[inline(always)]
    #[must_use]
    pub fn ref3ce(&mut self) -> REF3CE_W<3> {
        REF3CE_W::new(self)
    }
    #[doc = "Bit 4 - CH3PRE"]
    #[inline(always)]
    #[must_use]
    pub fn ch3pre(&mut self) -> CH3PRE_W<4> {
        CH3PRE_W::new(self)
    }
    #[doc = "Bit 5 - CH3IMAE"]
    #[inline(always)]
    #[must_use]
    pub fn ch3imae(&mut self) -> CH3IMAE_W<5> {
        CH3IMAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCTM_CH3OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch3ocfr](index.html) module"]
pub struct MCTM_CH3OCFR_SPEC;
impl crate::RegisterSpec for MCTM_CH3OCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctm_ch3ocfr::R](R) reader structure"]
impl crate::Readable for MCTM_CH3OCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctm_ch3ocfr::W](W) writer structure"]
impl crate::Writable for MCTM_CH3OCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTM_CH3OCFR to value 0"]
impl crate::Resettable for MCTM_CH3OCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
