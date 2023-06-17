#[doc = "Register `GPIOE_DOUTR` reader"]
pub struct R(crate::R<GPIOE_DOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOE_DOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOE_DOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOE_DOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOE_DOUTR` writer"]
pub struct W(crate::W<GPIOE_DOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOE_DOUTR_SPEC>;
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
impl From<crate::W<GPIOE_DOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOE_DOUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT0` reader - DOUT0"]
pub type DOUT0_R = crate::BitReader;
#[doc = "Field `DOUT0` writer - DOUT0"]
pub type DOUT0_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_DOUTR_SPEC, O>;
#[doc = "Field `DOUT1` reader - DOUT1"]
pub type DOUT1_R = crate::BitReader;
#[doc = "Field `DOUT1` writer - DOUT1"]
pub type DOUT1_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_DOUTR_SPEC, O>;
#[doc = "Field `DOUT2` reader - DOUT2"]
pub type DOUT2_R = crate::BitReader;
#[doc = "Field `DOUT2` writer - DOUT2"]
pub type DOUT2_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_DOUTR_SPEC, O>;
#[doc = "Field `DOUT3` reader - DOUT3"]
pub type DOUT3_R = crate::BitReader;
#[doc = "Field `DOUT3` writer - DOUT3"]
pub type DOUT3_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_DOUTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - DOUT0"]
    #[inline(always)]
    pub fn dout0(&self) -> DOUT0_R {
        DOUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DOUT1"]
    #[inline(always)]
    pub fn dout1(&self) -> DOUT1_R {
        DOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DOUT2"]
    #[inline(always)]
    pub fn dout2(&self) -> DOUT2_R {
        DOUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DOUT3"]
    #[inline(always)]
    pub fn dout3(&self) -> DOUT3_R {
        DOUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DOUT0"]
    #[inline(always)]
    #[must_use]
    pub fn dout0(&mut self) -> DOUT0_W<0> {
        DOUT0_W::new(self)
    }
    #[doc = "Bit 1 - DOUT1"]
    #[inline(always)]
    #[must_use]
    pub fn dout1(&mut self) -> DOUT1_W<1> {
        DOUT1_W::new(self)
    }
    #[doc = "Bit 2 - DOUT2"]
    #[inline(always)]
    #[must_use]
    pub fn dout2(&mut self) -> DOUT2_W<2> {
        DOUT2_W::new(self)
    }
    #[doc = "Bit 3 - DOUT3"]
    #[inline(always)]
    #[must_use]
    pub fn dout3(&mut self) -> DOUT3_W<3> {
        DOUT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOE_DOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_doutr](index.html) module"]
pub struct GPIOE_DOUTR_SPEC;
impl crate::RegisterSpec for GPIOE_DOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioe_doutr::R](R) reader structure"]
impl crate::Readable for GPIOE_DOUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioe_doutr::W](W) writer structure"]
impl crate::Writable for GPIOE_DOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOE_DOUTR to value 0"]
impl crate::Resettable for GPIOE_DOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
