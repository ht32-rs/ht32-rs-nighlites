#[doc = "Register `GPIOE_PUR` reader"]
pub struct R(crate::R<GPIOE_PUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOE_PUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOE_PUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOE_PUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOE_PUR` writer"]
pub struct W(crate::W<GPIOE_PUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOE_PUR_SPEC>;
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
impl From<crate::W<GPIOE_PUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOE_PUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PU0` reader - PU0"]
pub type PU0_R = crate::BitReader;
#[doc = "Field `PU0` writer - PU0"]
pub type PU0_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_PUR_SPEC, O>;
#[doc = "Field `PU1` reader - PU1"]
pub type PU1_R = crate::BitReader;
#[doc = "Field `PU1` writer - PU1"]
pub type PU1_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_PUR_SPEC, O>;
#[doc = "Field `PU2` reader - PU2"]
pub type PU2_R = crate::BitReader;
#[doc = "Field `PU2` writer - PU2"]
pub type PU2_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_PUR_SPEC, O>;
#[doc = "Field `PU3` reader - PU3"]
pub type PU3_R = crate::BitReader;
#[doc = "Field `PU3` writer - PU3"]
pub type PU3_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_PUR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PU0"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PU1"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PU2"]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PU3"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PU0"]
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> PU0_W<0> {
        PU0_W::new(self)
    }
    #[doc = "Bit 1 - PU1"]
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> PU1_W<1> {
        PU1_W::new(self)
    }
    #[doc = "Bit 2 - PU2"]
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<2> {
        PU2_W::new(self)
    }
    #[doc = "Bit 3 - PU3"]
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> PU3_W<3> {
        PU3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOE_PUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_pur](index.html) module"]
pub struct GPIOE_PUR_SPEC;
impl crate::RegisterSpec for GPIOE_PUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioe_pur::R](R) reader structure"]
impl crate::Readable for GPIOE_PUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioe_pur::W](W) writer structure"]
impl crate::Writable for GPIOE_PUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOE_PUR to value 0"]
impl crate::Resettable for GPIOE_PUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
