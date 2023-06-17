#[doc = "Register `PBDOUTR` reader"]
pub struct R(crate::R<PBDOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBDOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBDOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBDOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBDOUTR` writer"]
pub struct W(crate::W<PBDOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBDOUTR_SPEC>;
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
impl From<crate::W<PBDOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBDOUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBDOUT0` reader - PBDOUT0"]
pub type PBDOUT0_R = crate::BitReader;
#[doc = "Field `PBDOUT0` writer - PBDOUT0"]
pub type PBDOUT0_W<'a, const O: u8> = crate::BitWriter<'a, PBDOUTR_SPEC, O>;
#[doc = "Field `PBDOUT1` reader - PBDOUT1"]
pub type PBDOUT1_R = crate::BitReader;
#[doc = "Field `PBDOUT1` writer - PBDOUT1"]
pub type PBDOUT1_W<'a, const O: u8> = crate::BitWriter<'a, PBDOUTR_SPEC, O>;
#[doc = "Field `PBDOUT2` reader - PBDOUT2"]
pub type PBDOUT2_R = crate::BitReader;
#[doc = "Field `PBDOUT2` writer - PBDOUT2"]
pub type PBDOUT2_W<'a, const O: u8> = crate::BitWriter<'a, PBDOUTR_SPEC, O>;
#[doc = "Field `PBDOUT3` reader - PBDOUT3"]
pub type PBDOUT3_R = crate::BitReader;
#[doc = "Field `PBDOUT3` writer - PBDOUT3"]
pub type PBDOUT3_W<'a, const O: u8> = crate::BitWriter<'a, PBDOUTR_SPEC, O>;
#[doc = "Field `PBDOUT4` reader - PBDOUT4"]
pub type PBDOUT4_R = crate::BitReader;
#[doc = "Field `PBDOUT4` writer - PBDOUT4"]
pub type PBDOUT4_W<'a, const O: u8> = crate::BitWriter<'a, PBDOUTR_SPEC, O>;
#[doc = "Field `PBDOUT5` reader - PBDOUT5"]
pub type PBDOUT5_R = crate::BitReader;
#[doc = "Field `PBDOUT5` writer - PBDOUT5"]
pub type PBDOUT5_W<'a, const O: u8> = crate::BitWriter<'a, PBDOUTR_SPEC, O>;
#[doc = "Field `PBDOUT6` reader - PBDOUT6"]
pub type PBDOUT6_R = crate::BitReader;
#[doc = "Field `PBDOUT6` writer - PBDOUT6"]
pub type PBDOUT6_W<'a, const O: u8> = crate::BitWriter<'a, PBDOUTR_SPEC, O>;
#[doc = "Field `PBDOUT7` reader - PBDOUT7"]
pub type PBDOUT7_R = crate::BitReader;
#[doc = "Field `PBDOUT7` writer - PBDOUT7"]
pub type PBDOUT7_W<'a, const O: u8> = crate::BitWriter<'a, PBDOUTR_SPEC, O>;
#[doc = "Field `PBDOUT8` reader - PBDOUT8"]
pub type PBDOUT8_R = crate::BitReader;
#[doc = "Field `PBDOUT8` writer - PBDOUT8"]
pub type PBDOUT8_W<'a, const O: u8> = crate::BitWriter<'a, PBDOUTR_SPEC, O>;
#[doc = "Field `PBDOUT9` reader - PBDOUT9"]
pub type PBDOUT9_R = crate::BitReader;
#[doc = "Field `PBDOUT9` writer - PBDOUT9"]
pub type PBDOUT9_W<'a, const O: u8> = crate::BitWriter<'a, PBDOUTR_SPEC, O>;
#[doc = "Field `PBDOUT10` reader - PBDOUT10"]
pub type PBDOUT10_R = crate::BitReader;
#[doc = "Field `PBDOUT10` writer - PBDOUT10"]
pub type PBDOUT10_W<'a, const O: u8> = crate::BitWriter<'a, PBDOUTR_SPEC, O>;
#[doc = "Field `PBDOUT11` reader - PBDOUT11"]
pub type PBDOUT11_R = crate::BitReader;
#[doc = "Field `PBDOUT11` writer - PBDOUT11"]
pub type PBDOUT11_W<'a, const O: u8> = crate::BitWriter<'a, PBDOUTR_SPEC, O>;
#[doc = "Field `PBDOUT12` reader - PBDOUT12"]
pub type PBDOUT12_R = crate::BitReader;
#[doc = "Field `PBDOUT12` writer - PBDOUT12"]
pub type PBDOUT12_W<'a, const O: u8> = crate::BitWriter<'a, PBDOUTR_SPEC, O>;
#[doc = "Field `PBDOUT13` reader - PBDOUT13"]
pub type PBDOUT13_R = crate::BitReader;
#[doc = "Field `PBDOUT13` writer - PBDOUT13"]
pub type PBDOUT13_W<'a, const O: u8> = crate::BitWriter<'a, PBDOUTR_SPEC, O>;
#[doc = "Field `PBDOUT14` reader - PBDOUT14"]
pub type PBDOUT14_R = crate::BitReader;
#[doc = "Field `PBDOUT14` writer - PBDOUT14"]
pub type PBDOUT14_W<'a, const O: u8> = crate::BitWriter<'a, PBDOUTR_SPEC, O>;
#[doc = "Field `PBDOUT15` reader - PBDOUT15"]
pub type PBDOUT15_R = crate::BitReader;
#[doc = "Field `PBDOUT15` writer - PBDOUT15"]
pub type PBDOUT15_W<'a, const O: u8> = crate::BitWriter<'a, PBDOUTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PBDOUT0"]
    #[inline(always)]
    pub fn pbdout0(&self) -> PBDOUT0_R {
        PBDOUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PBDOUT1"]
    #[inline(always)]
    pub fn pbdout1(&self) -> PBDOUT1_R {
        PBDOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBDOUT2"]
    #[inline(always)]
    pub fn pbdout2(&self) -> PBDOUT2_R {
        PBDOUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PBDOUT3"]
    #[inline(always)]
    pub fn pbdout3(&self) -> PBDOUT3_R {
        PBDOUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PBDOUT4"]
    #[inline(always)]
    pub fn pbdout4(&self) -> PBDOUT4_R {
        PBDOUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PBDOUT5"]
    #[inline(always)]
    pub fn pbdout5(&self) -> PBDOUT5_R {
        PBDOUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PBDOUT6"]
    #[inline(always)]
    pub fn pbdout6(&self) -> PBDOUT6_R {
        PBDOUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PBDOUT7"]
    #[inline(always)]
    pub fn pbdout7(&self) -> PBDOUT7_R {
        PBDOUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PBDOUT8"]
    #[inline(always)]
    pub fn pbdout8(&self) -> PBDOUT8_R {
        PBDOUT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PBDOUT9"]
    #[inline(always)]
    pub fn pbdout9(&self) -> PBDOUT9_R {
        PBDOUT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PBDOUT10"]
    #[inline(always)]
    pub fn pbdout10(&self) -> PBDOUT10_R {
        PBDOUT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PBDOUT11"]
    #[inline(always)]
    pub fn pbdout11(&self) -> PBDOUT11_R {
        PBDOUT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PBDOUT12"]
    #[inline(always)]
    pub fn pbdout12(&self) -> PBDOUT12_R {
        PBDOUT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PBDOUT13"]
    #[inline(always)]
    pub fn pbdout13(&self) -> PBDOUT13_R {
        PBDOUT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PBDOUT14"]
    #[inline(always)]
    pub fn pbdout14(&self) -> PBDOUT14_R {
        PBDOUT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PBDOUT15"]
    #[inline(always)]
    pub fn pbdout15(&self) -> PBDOUT15_R {
        PBDOUT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PBDOUT0"]
    #[inline(always)]
    #[must_use]
    pub fn pbdout0(&mut self) -> PBDOUT0_W<0> {
        PBDOUT0_W::new(self)
    }
    #[doc = "Bit 1 - PBDOUT1"]
    #[inline(always)]
    #[must_use]
    pub fn pbdout1(&mut self) -> PBDOUT1_W<1> {
        PBDOUT1_W::new(self)
    }
    #[doc = "Bit 2 - PBDOUT2"]
    #[inline(always)]
    #[must_use]
    pub fn pbdout2(&mut self) -> PBDOUT2_W<2> {
        PBDOUT2_W::new(self)
    }
    #[doc = "Bit 3 - PBDOUT3"]
    #[inline(always)]
    #[must_use]
    pub fn pbdout3(&mut self) -> PBDOUT3_W<3> {
        PBDOUT3_W::new(self)
    }
    #[doc = "Bit 4 - PBDOUT4"]
    #[inline(always)]
    #[must_use]
    pub fn pbdout4(&mut self) -> PBDOUT4_W<4> {
        PBDOUT4_W::new(self)
    }
    #[doc = "Bit 5 - PBDOUT5"]
    #[inline(always)]
    #[must_use]
    pub fn pbdout5(&mut self) -> PBDOUT5_W<5> {
        PBDOUT5_W::new(self)
    }
    #[doc = "Bit 6 - PBDOUT6"]
    #[inline(always)]
    #[must_use]
    pub fn pbdout6(&mut self) -> PBDOUT6_W<6> {
        PBDOUT6_W::new(self)
    }
    #[doc = "Bit 7 - PBDOUT7"]
    #[inline(always)]
    #[must_use]
    pub fn pbdout7(&mut self) -> PBDOUT7_W<7> {
        PBDOUT7_W::new(self)
    }
    #[doc = "Bit 8 - PBDOUT8"]
    #[inline(always)]
    #[must_use]
    pub fn pbdout8(&mut self) -> PBDOUT8_W<8> {
        PBDOUT8_W::new(self)
    }
    #[doc = "Bit 9 - PBDOUT9"]
    #[inline(always)]
    #[must_use]
    pub fn pbdout9(&mut self) -> PBDOUT9_W<9> {
        PBDOUT9_W::new(self)
    }
    #[doc = "Bit 10 - PBDOUT10"]
    #[inline(always)]
    #[must_use]
    pub fn pbdout10(&mut self) -> PBDOUT10_W<10> {
        PBDOUT10_W::new(self)
    }
    #[doc = "Bit 11 - PBDOUT11"]
    #[inline(always)]
    #[must_use]
    pub fn pbdout11(&mut self) -> PBDOUT11_W<11> {
        PBDOUT11_W::new(self)
    }
    #[doc = "Bit 12 - PBDOUT12"]
    #[inline(always)]
    #[must_use]
    pub fn pbdout12(&mut self) -> PBDOUT12_W<12> {
        PBDOUT12_W::new(self)
    }
    #[doc = "Bit 13 - PBDOUT13"]
    #[inline(always)]
    #[must_use]
    pub fn pbdout13(&mut self) -> PBDOUT13_W<13> {
        PBDOUT13_W::new(self)
    }
    #[doc = "Bit 14 - PBDOUT14"]
    #[inline(always)]
    #[must_use]
    pub fn pbdout14(&mut self) -> PBDOUT14_W<14> {
        PBDOUT14_W::new(self)
    }
    #[doc = "Bit 15 - PBDOUT15"]
    #[inline(always)]
    #[must_use]
    pub fn pbdout15(&mut self) -> PBDOUT15_W<15> {
        PBDOUT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBDOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbdoutr](index.html) module"]
pub struct PBDOUTR_SPEC;
impl crate::RegisterSpec for PBDOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbdoutr::R](R) reader structure"]
impl crate::Readable for PBDOUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbdoutr::W](W) writer structure"]
impl crate::Writable for PBDOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBDOUTR to value 0"]
impl crate::Resettable for PBDOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
