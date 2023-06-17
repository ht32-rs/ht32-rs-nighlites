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
#[doc = "Field `PCDOUT8` reader - PCDOUT8"]
pub type PCDOUT8_R = crate::BitReader;
#[doc = "Field `PCDOUT8` writer - PCDOUT8"]
pub type PCDOUT8_W<'a, const O: u8> = crate::BitWriter<'a, PCDOUTR_SPEC, O>;
#[doc = "Field `PCDOUT9` reader - PCDOUT9"]
pub type PCDOUT9_R = crate::BitReader;
#[doc = "Field `PCDOUT9` writer - PCDOUT9"]
pub type PCDOUT9_W<'a, const O: u8> = crate::BitWriter<'a, PCDOUTR_SPEC, O>;
#[doc = "Field `PCDOUT10` reader - PCDOUT10"]
pub type PCDOUT10_R = crate::BitReader;
#[doc = "Field `PCDOUT10` writer - PCDOUT10"]
pub type PCDOUT10_W<'a, const O: u8> = crate::BitWriter<'a, PCDOUTR_SPEC, O>;
#[doc = "Field `PCDOUT11` reader - PCDOUT11"]
pub type PCDOUT11_R = crate::BitReader;
#[doc = "Field `PCDOUT11` writer - PCDOUT11"]
pub type PCDOUT11_W<'a, const O: u8> = crate::BitWriter<'a, PCDOUTR_SPEC, O>;
#[doc = "Field `PCDOUT12` reader - PCDOUT12"]
pub type PCDOUT12_R = crate::BitReader;
#[doc = "Field `PCDOUT12` writer - PCDOUT12"]
pub type PCDOUT12_W<'a, const O: u8> = crate::BitWriter<'a, PCDOUTR_SPEC, O>;
#[doc = "Field `PCDOUT13` reader - PCDOUT13"]
pub type PCDOUT13_R = crate::BitReader;
#[doc = "Field `PCDOUT13` writer - PCDOUT13"]
pub type PCDOUT13_W<'a, const O: u8> = crate::BitWriter<'a, PCDOUTR_SPEC, O>;
#[doc = "Field `PCDOUT14` reader - PCDOUT14"]
pub type PCDOUT14_R = crate::BitReader;
#[doc = "Field `PCDOUT14` writer - PCDOUT14"]
pub type PCDOUT14_W<'a, const O: u8> = crate::BitWriter<'a, PCDOUTR_SPEC, O>;
#[doc = "Field `PCDOUT15` reader - PCDOUT15"]
pub type PCDOUT15_R = crate::BitReader;
#[doc = "Field `PCDOUT15` writer - PCDOUT15"]
pub type PCDOUT15_W<'a, const O: u8> = crate::BitWriter<'a, PCDOUTR_SPEC, O>;
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
    #[doc = "Bit 8 - PCDOUT8"]
    #[inline(always)]
    pub fn pcdout8(&self) -> PCDOUT8_R {
        PCDOUT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PCDOUT9"]
    #[inline(always)]
    pub fn pcdout9(&self) -> PCDOUT9_R {
        PCDOUT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCDOUT10"]
    #[inline(always)]
    pub fn pcdout10(&self) -> PCDOUT10_R {
        PCDOUT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PCDOUT11"]
    #[inline(always)]
    pub fn pcdout11(&self) -> PCDOUT11_R {
        PCDOUT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PCDOUT12"]
    #[inline(always)]
    pub fn pcdout12(&self) -> PCDOUT12_R {
        PCDOUT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PCDOUT13"]
    #[inline(always)]
    pub fn pcdout13(&self) -> PCDOUT13_R {
        PCDOUT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PCDOUT14"]
    #[inline(always)]
    pub fn pcdout14(&self) -> PCDOUT14_R {
        PCDOUT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PCDOUT15"]
    #[inline(always)]
    pub fn pcdout15(&self) -> PCDOUT15_R {
        PCDOUT15_R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 8 - PCDOUT8"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout8(&mut self) -> PCDOUT8_W<8> {
        PCDOUT8_W::new(self)
    }
    #[doc = "Bit 9 - PCDOUT9"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout9(&mut self) -> PCDOUT9_W<9> {
        PCDOUT9_W::new(self)
    }
    #[doc = "Bit 10 - PCDOUT10"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout10(&mut self) -> PCDOUT10_W<10> {
        PCDOUT10_W::new(self)
    }
    #[doc = "Bit 11 - PCDOUT11"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout11(&mut self) -> PCDOUT11_W<11> {
        PCDOUT11_W::new(self)
    }
    #[doc = "Bit 12 - PCDOUT12"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout12(&mut self) -> PCDOUT12_W<12> {
        PCDOUT12_W::new(self)
    }
    #[doc = "Bit 13 - PCDOUT13"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout13(&mut self) -> PCDOUT13_W<13> {
        PCDOUT13_W::new(self)
    }
    #[doc = "Bit 14 - PCDOUT14"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout14(&mut self) -> PCDOUT14_W<14> {
        PCDOUT14_W::new(self)
    }
    #[doc = "Bit 15 - PCDOUT15"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout15(&mut self) -> PCDOUT15_W<15> {
        PCDOUT15_W::new(self)
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
