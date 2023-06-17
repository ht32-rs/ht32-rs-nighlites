#[doc = "Register `PDDOUTR` reader"]
pub struct R(crate::R<PDDOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDDOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDDOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDDOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDDOUTR` writer"]
pub struct W(crate::W<PDDOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDDOUTR_SPEC>;
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
impl From<crate::W<PDDOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDDOUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDDOUT0` reader - PDDOUT0"]
pub type PDDOUT0_R = crate::BitReader;
#[doc = "Field `PDDOUT0` writer - PDDOUT0"]
pub type PDDOUT0_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT1` reader - PDDOUT1"]
pub type PDDOUT1_R = crate::BitReader;
#[doc = "Field `PDDOUT1` writer - PDDOUT1"]
pub type PDDOUT1_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT2` reader - PDDOUT2"]
pub type PDDOUT2_R = crate::BitReader;
#[doc = "Field `PDDOUT2` writer - PDDOUT2"]
pub type PDDOUT2_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT3` reader - PDDOUT3"]
pub type PDDOUT3_R = crate::BitReader;
#[doc = "Field `PDDOUT3` writer - PDDOUT3"]
pub type PDDOUT3_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT4` reader - PDDOUT4"]
pub type PDDOUT4_R = crate::BitReader;
#[doc = "Field `PDDOUT4` writer - PDDOUT4"]
pub type PDDOUT4_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT5` reader - PDDOUT5"]
pub type PDDOUT5_R = crate::BitReader;
#[doc = "Field `PDDOUT5` writer - PDDOUT5"]
pub type PDDOUT5_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT6` reader - PDDOUT6"]
pub type PDDOUT6_R = crate::BitReader;
#[doc = "Field `PDDOUT6` writer - PDDOUT6"]
pub type PDDOUT6_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT7` reader - PDDOUT7"]
pub type PDDOUT7_R = crate::BitReader;
#[doc = "Field `PDDOUT7` writer - PDDOUT7"]
pub type PDDOUT7_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT8` reader - PDDOUT8"]
pub type PDDOUT8_R = crate::BitReader;
#[doc = "Field `PDDOUT8` writer - PDDOUT8"]
pub type PDDOUT8_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT9` reader - PDDOUT9"]
pub type PDDOUT9_R = crate::BitReader;
#[doc = "Field `PDDOUT9` writer - PDDOUT9"]
pub type PDDOUT9_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT10` reader - PDDOUT10"]
pub type PDDOUT10_R = crate::BitReader;
#[doc = "Field `PDDOUT10` writer - PDDOUT10"]
pub type PDDOUT10_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT11` reader - PDDOUT11"]
pub type PDDOUT11_R = crate::BitReader;
#[doc = "Field `PDDOUT11` writer - PDDOUT11"]
pub type PDDOUT11_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT12` reader - PDDOUT12"]
pub type PDDOUT12_R = crate::BitReader;
#[doc = "Field `PDDOUT12` writer - PDDOUT12"]
pub type PDDOUT12_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT13` reader - PDDOUT13"]
pub type PDDOUT13_R = crate::BitReader;
#[doc = "Field `PDDOUT13` writer - PDDOUT13"]
pub type PDDOUT13_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT14` reader - PDDOUT14"]
pub type PDDOUT14_R = crate::BitReader;
#[doc = "Field `PDDOUT14` writer - PDDOUT14"]
pub type PDDOUT14_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT15` reader - PDDOUT15"]
pub type PDDOUT15_R = crate::BitReader;
#[doc = "Field `PDDOUT15` writer - PDDOUT15"]
pub type PDDOUT15_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDDOUT0"]
    #[inline(always)]
    pub fn pddout0(&self) -> PDDOUT0_R {
        PDDOUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDDOUT1"]
    #[inline(always)]
    pub fn pddout1(&self) -> PDDOUT1_R {
        PDDOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDDOUT2"]
    #[inline(always)]
    pub fn pddout2(&self) -> PDDOUT2_R {
        PDDOUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDDOUT3"]
    #[inline(always)]
    pub fn pddout3(&self) -> PDDOUT3_R {
        PDDOUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PDDOUT4"]
    #[inline(always)]
    pub fn pddout4(&self) -> PDDOUT4_R {
        PDDOUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PDDOUT5"]
    #[inline(always)]
    pub fn pddout5(&self) -> PDDOUT5_R {
        PDDOUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDDOUT6"]
    #[inline(always)]
    pub fn pddout6(&self) -> PDDOUT6_R {
        PDDOUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDDOUT7"]
    #[inline(always)]
    pub fn pddout7(&self) -> PDDOUT7_R {
        PDDOUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PDDOUT8"]
    #[inline(always)]
    pub fn pddout8(&self) -> PDDOUT8_R {
        PDDOUT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PDDOUT9"]
    #[inline(always)]
    pub fn pddout9(&self) -> PDDOUT9_R {
        PDDOUT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PDDOUT10"]
    #[inline(always)]
    pub fn pddout10(&self) -> PDDOUT10_R {
        PDDOUT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PDDOUT11"]
    #[inline(always)]
    pub fn pddout11(&self) -> PDDOUT11_R {
        PDDOUT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PDDOUT12"]
    #[inline(always)]
    pub fn pddout12(&self) -> PDDOUT12_R {
        PDDOUT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PDDOUT13"]
    #[inline(always)]
    pub fn pddout13(&self) -> PDDOUT13_R {
        PDDOUT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PDDOUT14"]
    #[inline(always)]
    pub fn pddout14(&self) -> PDDOUT14_R {
        PDDOUT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PDDOUT15"]
    #[inline(always)]
    pub fn pddout15(&self) -> PDDOUT15_R {
        PDDOUT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDDOUT0"]
    #[inline(always)]
    #[must_use]
    pub fn pddout0(&mut self) -> PDDOUT0_W<0> {
        PDDOUT0_W::new(self)
    }
    #[doc = "Bit 1 - PDDOUT1"]
    #[inline(always)]
    #[must_use]
    pub fn pddout1(&mut self) -> PDDOUT1_W<1> {
        PDDOUT1_W::new(self)
    }
    #[doc = "Bit 2 - PDDOUT2"]
    #[inline(always)]
    #[must_use]
    pub fn pddout2(&mut self) -> PDDOUT2_W<2> {
        PDDOUT2_W::new(self)
    }
    #[doc = "Bit 3 - PDDOUT3"]
    #[inline(always)]
    #[must_use]
    pub fn pddout3(&mut self) -> PDDOUT3_W<3> {
        PDDOUT3_W::new(self)
    }
    #[doc = "Bit 4 - PDDOUT4"]
    #[inline(always)]
    #[must_use]
    pub fn pddout4(&mut self) -> PDDOUT4_W<4> {
        PDDOUT4_W::new(self)
    }
    #[doc = "Bit 5 - PDDOUT5"]
    #[inline(always)]
    #[must_use]
    pub fn pddout5(&mut self) -> PDDOUT5_W<5> {
        PDDOUT5_W::new(self)
    }
    #[doc = "Bit 6 - PDDOUT6"]
    #[inline(always)]
    #[must_use]
    pub fn pddout6(&mut self) -> PDDOUT6_W<6> {
        PDDOUT6_W::new(self)
    }
    #[doc = "Bit 7 - PDDOUT7"]
    #[inline(always)]
    #[must_use]
    pub fn pddout7(&mut self) -> PDDOUT7_W<7> {
        PDDOUT7_W::new(self)
    }
    #[doc = "Bit 8 - PDDOUT8"]
    #[inline(always)]
    #[must_use]
    pub fn pddout8(&mut self) -> PDDOUT8_W<8> {
        PDDOUT8_W::new(self)
    }
    #[doc = "Bit 9 - PDDOUT9"]
    #[inline(always)]
    #[must_use]
    pub fn pddout9(&mut self) -> PDDOUT9_W<9> {
        PDDOUT9_W::new(self)
    }
    #[doc = "Bit 10 - PDDOUT10"]
    #[inline(always)]
    #[must_use]
    pub fn pddout10(&mut self) -> PDDOUT10_W<10> {
        PDDOUT10_W::new(self)
    }
    #[doc = "Bit 11 - PDDOUT11"]
    #[inline(always)]
    #[must_use]
    pub fn pddout11(&mut self) -> PDDOUT11_W<11> {
        PDDOUT11_W::new(self)
    }
    #[doc = "Bit 12 - PDDOUT12"]
    #[inline(always)]
    #[must_use]
    pub fn pddout12(&mut self) -> PDDOUT12_W<12> {
        PDDOUT12_W::new(self)
    }
    #[doc = "Bit 13 - PDDOUT13"]
    #[inline(always)]
    #[must_use]
    pub fn pddout13(&mut self) -> PDDOUT13_W<13> {
        PDDOUT13_W::new(self)
    }
    #[doc = "Bit 14 - PDDOUT14"]
    #[inline(always)]
    #[must_use]
    pub fn pddout14(&mut self) -> PDDOUT14_W<14> {
        PDDOUT14_W::new(self)
    }
    #[doc = "Bit 15 - PDDOUT15"]
    #[inline(always)]
    #[must_use]
    pub fn pddout15(&mut self) -> PDDOUT15_W<15> {
        PDDOUT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDDOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddoutr](index.html) module"]
pub struct PDDOUTR_SPEC;
impl crate::RegisterSpec for PDDOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pddoutr::R](R) reader structure"]
impl crate::Readable for PDDOUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pddoutr::W](W) writer structure"]
impl crate::Writable for PDDOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDDOUTR to value 0"]
impl crate::Resettable for PDDOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
