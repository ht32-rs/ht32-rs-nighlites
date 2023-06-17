#[doc = "Register `PEPDR` reader"]
pub struct R(crate::R<PEPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEPDR` writer"]
pub struct W(crate::W<PEPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEPDR_SPEC>;
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
impl From<crate::W<PEPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEPD0` reader - PEPD0"]
pub type PEPD0_R = crate::BitReader;
#[doc = "Field `PEPD0` writer - PEPD0"]
pub type PEPD0_W<'a, const O: u8> = crate::BitWriter<'a, PEPDR_SPEC, O>;
#[doc = "Field `PEPD1` reader - PEPD1"]
pub type PEPD1_R = crate::BitReader;
#[doc = "Field `PEPD1` writer - PEPD1"]
pub type PEPD1_W<'a, const O: u8> = crate::BitWriter<'a, PEPDR_SPEC, O>;
#[doc = "Field `PEPD2` reader - PEPD2"]
pub type PEPD2_R = crate::BitReader;
#[doc = "Field `PEPD2` writer - PEPD2"]
pub type PEPD2_W<'a, const O: u8> = crate::BitWriter<'a, PEPDR_SPEC, O>;
#[doc = "Field `PEPD3` reader - PEPD3"]
pub type PEPD3_R = crate::BitReader;
#[doc = "Field `PEPD3` writer - PEPD3"]
pub type PEPD3_W<'a, const O: u8> = crate::BitWriter<'a, PEPDR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PEPD0"]
    #[inline(always)]
    pub fn pepd0(&self) -> PEPD0_R {
        PEPD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PEPD1"]
    #[inline(always)]
    pub fn pepd1(&self) -> PEPD1_R {
        PEPD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PEPD2"]
    #[inline(always)]
    pub fn pepd2(&self) -> PEPD2_R {
        PEPD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PEPD3"]
    #[inline(always)]
    pub fn pepd3(&self) -> PEPD3_R {
        PEPD3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PEPD0"]
    #[inline(always)]
    #[must_use]
    pub fn pepd0(&mut self) -> PEPD0_W<0> {
        PEPD0_W::new(self)
    }
    #[doc = "Bit 1 - PEPD1"]
    #[inline(always)]
    #[must_use]
    pub fn pepd1(&mut self) -> PEPD1_W<1> {
        PEPD1_W::new(self)
    }
    #[doc = "Bit 2 - PEPD2"]
    #[inline(always)]
    #[must_use]
    pub fn pepd2(&mut self) -> PEPD2_W<2> {
        PEPD2_W::new(self)
    }
    #[doc = "Bit 3 - PEPD3"]
    #[inline(always)]
    #[must_use]
    pub fn pepd3(&mut self) -> PEPD3_W<3> {
        PEPD3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PEPDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pepdr](index.html) module"]
pub struct PEPDR_SPEC;
impl crate::RegisterSpec for PEPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pepdr::R](R) reader structure"]
impl crate::Readable for PEPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pepdr::W](W) writer structure"]
impl crate::Writable for PEPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PEPDR to value 0"]
impl crate::Resettable for PEPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
