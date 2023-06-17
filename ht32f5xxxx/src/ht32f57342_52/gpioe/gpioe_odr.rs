#[doc = "Register `GPIOE_ODR` reader"]
pub struct R(crate::R<GPIOE_ODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOE_ODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOE_ODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOE_ODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOE_ODR` writer"]
pub struct W(crate::W<GPIOE_ODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOE_ODR_SPEC>;
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
impl From<crate::W<GPIOE_ODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOE_ODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OD0` reader - OD0"]
pub type OD0_R = crate::BitReader;
#[doc = "Field `OD0` writer - OD0"]
pub type OD0_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_ODR_SPEC, O>;
#[doc = "Field `OD1` reader - OD1"]
pub type OD1_R = crate::BitReader;
#[doc = "Field `OD1` writer - OD1"]
pub type OD1_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_ODR_SPEC, O>;
#[doc = "Field `OD2` reader - OD2"]
pub type OD2_R = crate::BitReader;
#[doc = "Field `OD2` writer - OD2"]
pub type OD2_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_ODR_SPEC, O>;
#[doc = "Field `OD3` reader - OD3"]
pub type OD3_R = crate::BitReader;
#[doc = "Field `OD3` writer - OD3"]
pub type OD3_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_ODR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - OD0"]
    #[inline(always)]
    pub fn od0(&self) -> OD0_R {
        OD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OD1"]
    #[inline(always)]
    pub fn od1(&self) -> OD1_R {
        OD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OD2"]
    #[inline(always)]
    pub fn od2(&self) -> OD2_R {
        OD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OD3"]
    #[inline(always)]
    pub fn od3(&self) -> OD3_R {
        OD3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OD0"]
    #[inline(always)]
    #[must_use]
    pub fn od0(&mut self) -> OD0_W<0> {
        OD0_W::new(self)
    }
    #[doc = "Bit 1 - OD1"]
    #[inline(always)]
    #[must_use]
    pub fn od1(&mut self) -> OD1_W<1> {
        OD1_W::new(self)
    }
    #[doc = "Bit 2 - OD2"]
    #[inline(always)]
    #[must_use]
    pub fn od2(&mut self) -> OD2_W<2> {
        OD2_W::new(self)
    }
    #[doc = "Bit 3 - OD3"]
    #[inline(always)]
    #[must_use]
    pub fn od3(&mut self) -> OD3_W<3> {
        OD3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOE_ODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_odr](index.html) module"]
pub struct GPIOE_ODR_SPEC;
impl crate::RegisterSpec for GPIOE_ODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioe_odr::R](R) reader structure"]
impl crate::Readable for GPIOE_ODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioe_odr::W](W) writer structure"]
impl crate::Writable for GPIOE_ODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOE_ODR to value 0"]
impl crate::Resettable for GPIOE_ODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
