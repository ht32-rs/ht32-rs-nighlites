#[doc = "Register `PEDOUTR` reader"]
pub struct R(crate::R<PEDOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEDOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEDOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEDOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEDOUTR` writer"]
pub struct W(crate::W<PEDOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEDOUTR_SPEC>;
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
impl From<crate::W<PEDOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEDOUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEDOUT0` reader - PEDOUT0"]
pub type PEDOUT0_R = crate::BitReader;
#[doc = "Field `PEDOUT0` writer - PEDOUT0"]
pub type PEDOUT0_W<'a, const O: u8> = crate::BitWriter<'a, PEDOUTR_SPEC, O>;
#[doc = "Field `PEDOUT1` reader - PEDOUT1"]
pub type PEDOUT1_R = crate::BitReader;
#[doc = "Field `PEDOUT1` writer - PEDOUT1"]
pub type PEDOUT1_W<'a, const O: u8> = crate::BitWriter<'a, PEDOUTR_SPEC, O>;
#[doc = "Field `PEDOUT2` reader - PEDOUT2"]
pub type PEDOUT2_R = crate::BitReader;
#[doc = "Field `PEDOUT2` writer - PEDOUT2"]
pub type PEDOUT2_W<'a, const O: u8> = crate::BitWriter<'a, PEDOUTR_SPEC, O>;
#[doc = "Field `PEDOUT3` reader - PEDOUT3"]
pub type PEDOUT3_R = crate::BitReader;
#[doc = "Field `PEDOUT3` writer - PEDOUT3"]
pub type PEDOUT3_W<'a, const O: u8> = crate::BitWriter<'a, PEDOUTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PEDOUT0"]
    #[inline(always)]
    pub fn pedout0(&self) -> PEDOUT0_R {
        PEDOUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PEDOUT1"]
    #[inline(always)]
    pub fn pedout1(&self) -> PEDOUT1_R {
        PEDOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PEDOUT2"]
    #[inline(always)]
    pub fn pedout2(&self) -> PEDOUT2_R {
        PEDOUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PEDOUT3"]
    #[inline(always)]
    pub fn pedout3(&self) -> PEDOUT3_R {
        PEDOUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PEDOUT0"]
    #[inline(always)]
    #[must_use]
    pub fn pedout0(&mut self) -> PEDOUT0_W<0> {
        PEDOUT0_W::new(self)
    }
    #[doc = "Bit 1 - PEDOUT1"]
    #[inline(always)]
    #[must_use]
    pub fn pedout1(&mut self) -> PEDOUT1_W<1> {
        PEDOUT1_W::new(self)
    }
    #[doc = "Bit 2 - PEDOUT2"]
    #[inline(always)]
    #[must_use]
    pub fn pedout2(&mut self) -> PEDOUT2_W<2> {
        PEDOUT2_W::new(self)
    }
    #[doc = "Bit 3 - PEDOUT3"]
    #[inline(always)]
    #[must_use]
    pub fn pedout3(&mut self) -> PEDOUT3_W<3> {
        PEDOUT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PEDOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pedoutr](index.html) module"]
pub struct PEDOUTR_SPEC;
impl crate::RegisterSpec for PEDOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pedoutr::R](R) reader structure"]
impl crate::Readable for PEDOUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pedoutr::W](W) writer structure"]
impl crate::Writable for PEDOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PEDOUTR to value 0"]
impl crate::Resettable for PEDOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
