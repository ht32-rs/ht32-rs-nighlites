#[doc = "Register `GPIOE_RR` reader"]
pub struct R(crate::R<GPIOE_RR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOE_RR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOE_RR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOE_RR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOE_RR` writer"]
pub struct W(crate::W<GPIOE_RR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOE_RR_SPEC>;
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
impl From<crate::W<GPIOE_RR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOE_RR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST0` reader - RST0"]
pub type RST0_R = crate::BitReader;
#[doc = "Field `RST0` writer - RST0"]
pub type RST0_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_RR_SPEC, O>;
#[doc = "Field `RST1` reader - RST1"]
pub type RST1_R = crate::BitReader;
#[doc = "Field `RST1` writer - RST1"]
pub type RST1_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_RR_SPEC, O>;
#[doc = "Field `RST2` reader - RST2"]
pub type RST2_R = crate::BitReader;
#[doc = "Field `RST2` writer - RST2"]
pub type RST2_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_RR_SPEC, O>;
#[doc = "Field `RST3` reader - RST3"]
pub type RST3_R = crate::BitReader;
#[doc = "Field `RST3` writer - RST3"]
pub type RST3_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_RR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - RST0"]
    #[inline(always)]
    pub fn rst0(&self) -> RST0_R {
        RST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RST1"]
    #[inline(always)]
    pub fn rst1(&self) -> RST1_R {
        RST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RST2"]
    #[inline(always)]
    pub fn rst2(&self) -> RST2_R {
        RST2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RST3"]
    #[inline(always)]
    pub fn rst3(&self) -> RST3_R {
        RST3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RST0"]
    #[inline(always)]
    #[must_use]
    pub fn rst0(&mut self) -> RST0_W<0> {
        RST0_W::new(self)
    }
    #[doc = "Bit 1 - RST1"]
    #[inline(always)]
    #[must_use]
    pub fn rst1(&mut self) -> RST1_W<1> {
        RST1_W::new(self)
    }
    #[doc = "Bit 2 - RST2"]
    #[inline(always)]
    #[must_use]
    pub fn rst2(&mut self) -> RST2_W<2> {
        RST2_W::new(self)
    }
    #[doc = "Bit 3 - RST3"]
    #[inline(always)]
    #[must_use]
    pub fn rst3(&mut self) -> RST3_W<3> {
        RST3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOE_RR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_rr](index.html) module"]
pub struct GPIOE_RR_SPEC;
impl crate::RegisterSpec for GPIOE_RR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioe_rr::R](R) reader structure"]
impl crate::Readable for GPIOE_RR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioe_rr::W](W) writer structure"]
impl crate::Writable for GPIOE_RR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOE_RR to value 0"]
impl crate::Resettable for GPIOE_RR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
