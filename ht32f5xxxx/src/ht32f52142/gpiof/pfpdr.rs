#[doc = "Register `PFPDR` reader"]
pub struct R(crate::R<PFPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFPDR` writer"]
pub struct W(crate::W<PFPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFPDR_SPEC>;
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
impl From<crate::W<PFPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFPD0` reader - PFPD0"]
pub type PFPD0_R = crate::BitReader;
#[doc = "Field `PFPD0` writer - PFPD0"]
pub type PFPD0_W<'a, const O: u8> = crate::BitWriter<'a, PFPDR_SPEC, O>;
#[doc = "Field `PFPD1` reader - PFPD1"]
pub type PFPD1_R = crate::BitReader;
#[doc = "Field `PFPD1` writer - PFPD1"]
pub type PFPD1_W<'a, const O: u8> = crate::BitWriter<'a, PFPDR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PFPD0"]
    #[inline(always)]
    pub fn pfpd0(&self) -> PFPD0_R {
        PFPD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PFPD1"]
    #[inline(always)]
    pub fn pfpd1(&self) -> PFPD1_R {
        PFPD1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PFPD0"]
    #[inline(always)]
    #[must_use]
    pub fn pfpd0(&mut self) -> PFPD0_W<0> {
        PFPD0_W::new(self)
    }
    #[doc = "Bit 1 - PFPD1"]
    #[inline(always)]
    #[must_use]
    pub fn pfpd1(&mut self) -> PFPD1_W<1> {
        PFPD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PFPDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfpdr](index.html) module"]
pub struct PFPDR_SPEC;
impl crate::RegisterSpec for PFPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfpdr::R](R) reader structure"]
impl crate::Readable for PFPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfpdr::W](W) writer structure"]
impl crate::Writable for PFPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFPDR to value 0"]
impl crate::Resettable for PFPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
