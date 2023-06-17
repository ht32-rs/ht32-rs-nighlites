#[doc = "Register `GPIOE_DINR` reader"]
pub struct R(crate::R<GPIOE_DINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOE_DINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOE_DINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOE_DINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOE_DINR` writer"]
pub struct W(crate::W<GPIOE_DINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOE_DINR_SPEC>;
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
impl From<crate::W<GPIOE_DINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOE_DINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIN0` reader - DIN0"]
pub type DIN0_R = crate::BitReader;
#[doc = "Field `DIN0` writer - DIN0"]
pub type DIN0_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_DINR_SPEC, O>;
#[doc = "Field `DIN1` reader - DIN1"]
pub type DIN1_R = crate::BitReader;
#[doc = "Field `DIN1` writer - DIN1"]
pub type DIN1_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_DINR_SPEC, O>;
#[doc = "Field `DIN2` reader - DIN2"]
pub type DIN2_R = crate::BitReader;
#[doc = "Field `DIN2` writer - DIN2"]
pub type DIN2_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_DINR_SPEC, O>;
#[doc = "Field `DIN3` reader - DIN3"]
pub type DIN3_R = crate::BitReader;
#[doc = "Field `DIN3` writer - DIN3"]
pub type DIN3_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_DINR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - DIN0"]
    #[inline(always)]
    pub fn din0(&self) -> DIN0_R {
        DIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIN1"]
    #[inline(always)]
    pub fn din1(&self) -> DIN1_R {
        DIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIN2"]
    #[inline(always)]
    pub fn din2(&self) -> DIN2_R {
        DIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIN3"]
    #[inline(always)]
    pub fn din3(&self) -> DIN3_R {
        DIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIN0"]
    #[inline(always)]
    #[must_use]
    pub fn din0(&mut self) -> DIN0_W<0> {
        DIN0_W::new(self)
    }
    #[doc = "Bit 1 - DIN1"]
    #[inline(always)]
    #[must_use]
    pub fn din1(&mut self) -> DIN1_W<1> {
        DIN1_W::new(self)
    }
    #[doc = "Bit 2 - DIN2"]
    #[inline(always)]
    #[must_use]
    pub fn din2(&mut self) -> DIN2_W<2> {
        DIN2_W::new(self)
    }
    #[doc = "Bit 3 - DIN3"]
    #[inline(always)]
    #[must_use]
    pub fn din3(&mut self) -> DIN3_W<3> {
        DIN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOE_DINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_dinr](index.html) module"]
pub struct GPIOE_DINR_SPEC;
impl crate::RegisterSpec for GPIOE_DINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioe_dinr::R](R) reader structure"]
impl crate::Readable for GPIOE_DINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioe_dinr::W](W) writer structure"]
impl crate::Writable for GPIOE_DINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOE_DINR to value 0"]
impl crate::Resettable for GPIOE_DINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
