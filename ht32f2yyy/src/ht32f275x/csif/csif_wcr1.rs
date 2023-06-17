#[doc = "Register `CSIF_WCR1` reader"]
pub struct R(crate::R<CSIF_WCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIF_WCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIF_WCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIF_WCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSIF_WCR1` writer"]
pub struct W(crate::W<CSIF_WCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIF_WCR1_SPEC>;
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
impl From<crate::W<CSIF_WCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIF_WCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIN_WID` reader - WIN_WID"]
pub type WIN_WID_R = crate::FieldReader<u16>;
#[doc = "Field `WIN_WID` writer - WIN_WID"]
pub type WIN_WID_W<'a, const O: u8> = crate::FieldWriter<'a, CSIF_WCR1_SPEC, 11, O, u16>;
#[doc = "Field `WIN_HGH` reader - WIN_HGH"]
pub type WIN_HGH_R = crate::FieldReader<u16>;
#[doc = "Field `WIN_HGH` writer - WIN_HGH"]
pub type WIN_HGH_W<'a, const O: u8> = crate::FieldWriter<'a, CSIF_WCR1_SPEC, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - WIN_WID"]
    #[inline(always)]
    pub fn win_wid(&self) -> WIN_WID_R {
        WIN_WID_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - WIN_HGH"]
    #[inline(always)]
    pub fn win_hgh(&self) -> WIN_HGH_R {
        WIN_HGH_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - WIN_WID"]
    #[inline(always)]
    #[must_use]
    pub fn win_wid(&mut self) -> WIN_WID_W<0> {
        WIN_WID_W::new(self)
    }
    #[doc = "Bits 16:26 - WIN_HGH"]
    #[inline(always)]
    #[must_use]
    pub fn win_hgh(&mut self) -> WIN_HGH_W<16> {
        WIN_HGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIF_WCR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_wcr1](index.html) module"]
pub struct CSIF_WCR1_SPEC;
impl crate::RegisterSpec for CSIF_WCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csif_wcr1::R](R) reader structure"]
impl crate::Readable for CSIF_WCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csif_wcr1::W](W) writer structure"]
impl crate::Writable for CSIF_WCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSIF_WCR1 to value 0"]
impl crate::Resettable for CSIF_WCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
