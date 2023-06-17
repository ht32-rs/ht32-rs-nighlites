#[doc = "Register `PCDOUTR` reader"]
pub struct R(crate::R<PCDOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCDOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCDOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCDOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCDOUTR` writer"]
pub struct W(crate::W<PCDOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCDOUTR_SPEC>;
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
impl From<crate::W<PCDOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCDOUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCDOUT0` reader - PCDOUT0"]
pub type PCDOUT0_R = crate::BitReader;
#[doc = "Field `PCDOUT0` writer - PCDOUT0"]
pub type PCDOUT0_W<'a, const O: u8> = crate::BitWriter<'a, PCDOUTR_SPEC, O>;
#[doc = "Field `PCDOUT1` reader - PCDOUT1"]
pub type PCDOUT1_R = crate::BitReader;
#[doc = "Field `PCDOUT1` writer - PCDOUT1"]
pub type PCDOUT1_W<'a, const O: u8> = crate::BitWriter<'a, PCDOUTR_SPEC, O>;
#[doc = "Field `PCDOUT2` reader - PCDOUT2"]
pub type PCDOUT2_R = crate::BitReader;
#[doc = "Field `PCDOUT2` writer - PCDOUT2"]
pub type PCDOUT2_W<'a, const O: u8> = crate::BitWriter<'a, PCDOUTR_SPEC, O>;
#[doc = "Field `PCDOUT3` reader - PCDOUT3"]
pub type PCDOUT3_R = crate::BitReader;
#[doc = "Field `PCDOUT3` writer - PCDOUT3"]
pub type PCDOUT3_W<'a, const O: u8> = crate::BitWriter<'a, PCDOUTR_SPEC, O>;
#[doc = "Field `PCDOUT4` reader - PCDOUT4"]
pub type PCDOUT4_R = crate::BitReader;
#[doc = "Field `PCDOUT4` writer - PCDOUT4"]
pub type PCDOUT4_W<'a, const O: u8> = crate::BitWriter<'a, PCDOUTR_SPEC, O>;
#[doc = "Field `PCDOUT5` reader - PCDOUT5"]
pub type PCDOUT5_R = crate::BitReader;
#[doc = "Field `PCDOUT5` writer - PCDOUT5"]
pub type PCDOUT5_W<'a, const O: u8> = crate::BitWriter<'a, PCDOUTR_SPEC, O>;
#[doc = "Field `PCDOUT6` reader - PCDOUT6"]
pub type PCDOUT6_R = crate::BitReader;
#[doc = "Field `PCDOUT6` writer - PCDOUT6"]
pub type PCDOUT6_W<'a, const O: u8> = crate::BitWriter<'a, PCDOUTR_SPEC, O>;
#[doc = "Field `PCDOUT7` reader - PCDOUT7"]
pub type PCDOUT7_R = crate::BitReader;
#[doc = "Field `PCDOUT7` writer - PCDOUT7"]
pub type PCDOUT7_W<'a, const O: u8> = crate::BitWriter<'a, PCDOUTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PCDOUT0"]
    #[inline(always)]
    pub fn pcdout0(&self) -> PCDOUT0_R {
        PCDOUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCDOUT1"]
    #[inline(always)]
    pub fn pcdout1(&self) -> PCDOUT1_R {
        PCDOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCDOUT2"]
    #[inline(always)]
    pub fn pcdout2(&self) -> PCDOUT2_R {
        PCDOUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCDOUT3"]
    #[inline(always)]
    pub fn pcdout3(&self) -> PCDOUT3_R {
        PCDOUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PCDOUT4"]
    #[inline(always)]
    pub fn pcdout4(&self) -> PCDOUT4_R {
        PCDOUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCDOUT5"]
    #[inline(always)]
    pub fn pcdout5(&self) -> PCDOUT5_R {
        PCDOUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCDOUT6"]
    #[inline(always)]
    pub fn pcdout6(&self) -> PCDOUT6_R {
        PCDOUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PCDOUT7"]
    #[inline(always)]
    pub fn pcdout7(&self) -> PCDOUT7_R {
        PCDOUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCDOUT0"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout0(&mut self) -> PCDOUT0_W<0> {
        PCDOUT0_W::new(self)
    }
    #[doc = "Bit 1 - PCDOUT1"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout1(&mut self) -> PCDOUT1_W<1> {
        PCDOUT1_W::new(self)
    }
    #[doc = "Bit 2 - PCDOUT2"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout2(&mut self) -> PCDOUT2_W<2> {
        PCDOUT2_W::new(self)
    }
    #[doc = "Bit 3 - PCDOUT3"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout3(&mut self) -> PCDOUT3_W<3> {
        PCDOUT3_W::new(self)
    }
    #[doc = "Bit 4 - PCDOUT4"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout4(&mut self) -> PCDOUT4_W<4> {
        PCDOUT4_W::new(self)
    }
    #[doc = "Bit 5 - PCDOUT5"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout5(&mut self) -> PCDOUT5_W<5> {
        PCDOUT5_W::new(self)
    }
    #[doc = "Bit 6 - PCDOUT6"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout6(&mut self) -> PCDOUT6_W<6> {
        PCDOUT6_W::new(self)
    }
    #[doc = "Bit 7 - PCDOUT7"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout7(&mut self) -> PCDOUT7_W<7> {
        PCDOUT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCDOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdoutr](index.html) module"]
pub struct PCDOUTR_SPEC;
impl crate::RegisterSpec for PCDOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcdoutr::R](R) reader structure"]
impl crate::Readable for PCDOUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcdoutr::W](W) writer structure"]
impl crate::Writable for PCDOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCDOUTR to value 0"]
impl crate::Resettable for PCDOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
