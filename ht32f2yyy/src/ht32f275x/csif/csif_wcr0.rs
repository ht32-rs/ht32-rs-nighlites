#[doc = "Register `CSIF_WCR0` reader"]
pub struct R(crate::R<CSIF_WCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIF_WCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIF_WCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIF_WCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSIF_WCR0` writer"]
pub struct W(crate::W<CSIF_WCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIF_WCR0_SPEC>;
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
impl From<crate::W<CSIF_WCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIF_WCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIN_HSTR` reader - WIN_HSTR"]
pub type WIN_HSTR_R = crate::FieldReader<u16>;
#[doc = "Field `WIN_HSTR` writer - WIN_HSTR"]
pub type WIN_HSTR_W<'a, const O: u8> = crate::FieldWriter<'a, CSIF_WCR0_SPEC, 11, O, u16>;
#[doc = "Field `WIN_VSTR` reader - WIN_VSTR"]
pub type WIN_VSTR_R = crate::FieldReader<u16>;
#[doc = "Field `WIN_VSTR` writer - WIN_VSTR"]
pub type WIN_VSTR_W<'a, const O: u8> = crate::FieldWriter<'a, CSIF_WCR0_SPEC, 11, O, u16>;
#[doc = "Field `WIN_EN` reader - WIN_EN"]
pub type WIN_EN_R = crate::BitReader;
#[doc = "Field `WIN_EN` writer - WIN_EN"]
pub type WIN_EN_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_WCR0_SPEC, O>;
impl R {
    #[doc = "Bits 0:10 - WIN_HSTR"]
    #[inline(always)]
    pub fn win_hstr(&self) -> WIN_HSTR_R {
        WIN_HSTR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - WIN_VSTR"]
    #[inline(always)]
    pub fn win_vstr(&self) -> WIN_VSTR_R {
        WIN_VSTR_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - WIN_EN"]
    #[inline(always)]
    pub fn win_en(&self) -> WIN_EN_R {
        WIN_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - WIN_HSTR"]
    #[inline(always)]
    #[must_use]
    pub fn win_hstr(&mut self) -> WIN_HSTR_W<0> {
        WIN_HSTR_W::new(self)
    }
    #[doc = "Bits 16:26 - WIN_VSTR"]
    #[inline(always)]
    #[must_use]
    pub fn win_vstr(&mut self) -> WIN_VSTR_W<16> {
        WIN_VSTR_W::new(self)
    }
    #[doc = "Bit 31 - WIN_EN"]
    #[inline(always)]
    #[must_use]
    pub fn win_en(&mut self) -> WIN_EN_W<31> {
        WIN_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIF_WCR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_wcr0](index.html) module"]
pub struct CSIF_WCR0_SPEC;
impl crate::RegisterSpec for CSIF_WCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csif_wcr0::R](R) reader structure"]
impl crate::Readable for CSIF_WCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csif_wcr0::W](W) writer structure"]
impl crate::Writable for CSIF_WCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSIF_WCR0 to value 0"]
impl crate::Resettable for CSIF_WCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
