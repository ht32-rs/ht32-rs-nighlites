#[doc = "Register `PDPDR` reader"]
pub struct R(crate::R<PDPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDPDR` writer"]
pub struct W(crate::W<PDPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDPDR_SPEC>;
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
impl From<crate::W<PDPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDPD0` reader - PDPD0"]
pub type PDPD0_R = crate::BitReader;
#[doc = "Field `PDPD0` writer - PDPD0"]
pub type PDPD0_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD1` reader - PDPD1"]
pub type PDPD1_R = crate::BitReader;
#[doc = "Field `PDPD1` writer - PDPD1"]
pub type PDPD1_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD2` reader - PDPD2"]
pub type PDPD2_R = crate::BitReader;
#[doc = "Field `PDPD2` writer - PDPD2"]
pub type PDPD2_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD3` reader - PDPD3"]
pub type PDPD3_R = crate::BitReader;
#[doc = "Field `PDPD3` writer - PDPD3"]
pub type PDPD3_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDPD0"]
    #[inline(always)]
    pub fn pdpd0(&self) -> PDPD0_R {
        PDPD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDPD1"]
    #[inline(always)]
    pub fn pdpd1(&self) -> PDPD1_R {
        PDPD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDPD2"]
    #[inline(always)]
    pub fn pdpd2(&self) -> PDPD2_R {
        PDPD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDPD3"]
    #[inline(always)]
    pub fn pdpd3(&self) -> PDPD3_R {
        PDPD3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDPD0"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd0(&mut self) -> PDPD0_W<0> {
        PDPD0_W::new(self)
    }
    #[doc = "Bit 1 - PDPD1"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd1(&mut self) -> PDPD1_W<1> {
        PDPD1_W::new(self)
    }
    #[doc = "Bit 2 - PDPD2"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd2(&mut self) -> PDPD2_W<2> {
        PDPD2_W::new(self)
    }
    #[doc = "Bit 3 - PDPD3"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd3(&mut self) -> PDPD3_W<3> {
        PDPD3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDPDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdpdr](index.html) module"]
pub struct PDPDR_SPEC;
impl crate::RegisterSpec for PDPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdpdr::R](R) reader structure"]
impl crate::Readable for PDPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdpdr::W](W) writer structure"]
impl crate::Writable for PDPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDPDR to value 0"]
impl crate::Resettable for PDPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
