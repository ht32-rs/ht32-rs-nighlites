#[doc = "Register `PFODR` reader"]
pub struct R(crate::R<PFODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFODR` writer"]
pub struct W(crate::W<PFODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFODR_SPEC>;
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
impl From<crate::W<PFODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFOD0` reader - PFOD0"]
pub type PFOD0_R = crate::BitReader;
#[doc = "Field `PFOD0` writer - PFOD0"]
pub type PFOD0_W<'a, const O: u8> = crate::BitWriter<'a, PFODR_SPEC, O>;
#[doc = "Field `PFOD1` reader - PFOD1"]
pub type PFOD1_R = crate::BitReader;
#[doc = "Field `PFOD1` writer - PFOD1"]
pub type PFOD1_W<'a, const O: u8> = crate::BitWriter<'a, PFODR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PFOD0"]
    #[inline(always)]
    pub fn pfod0(&self) -> PFOD0_R {
        PFOD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PFOD1"]
    #[inline(always)]
    pub fn pfod1(&self) -> PFOD1_R {
        PFOD1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PFOD0"]
    #[inline(always)]
    #[must_use]
    pub fn pfod0(&mut self) -> PFOD0_W<0> {
        PFOD0_W::new(self)
    }
    #[doc = "Bit 1 - PFOD1"]
    #[inline(always)]
    #[must_use]
    pub fn pfod1(&mut self) -> PFOD1_W<1> {
        PFOD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PFODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfodr](index.html) module"]
pub struct PFODR_SPEC;
impl crate::RegisterSpec for PFODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfodr::R](R) reader structure"]
impl crate::Readable for PFODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfodr::W](W) writer structure"]
impl crate::Writable for PFODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFODR to value 0"]
impl crate::Resettable for PFODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
