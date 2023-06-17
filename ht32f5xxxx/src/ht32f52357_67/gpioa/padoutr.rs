#[doc = "Register `PADOUTR` reader"]
pub struct R(crate::R<PADOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADOUTR` writer"]
pub struct W(crate::W<PADOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADOUTR_SPEC>;
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
impl From<crate::W<PADOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADOUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADOUT0` reader - PADOUT0"]
pub type PADOUT0_R = crate::BitReader;
#[doc = "Field `PADOUT0` writer - PADOUT0"]
pub type PADOUT0_W<'a, const O: u8> = crate::BitWriter<'a, PADOUTR_SPEC, O>;
#[doc = "Field `PADOUT1` reader - PADOUT1"]
pub type PADOUT1_R = crate::BitReader;
#[doc = "Field `PADOUT1` writer - PADOUT1"]
pub type PADOUT1_W<'a, const O: u8> = crate::BitWriter<'a, PADOUTR_SPEC, O>;
#[doc = "Field `PADOUT2` reader - PADOUT2"]
pub type PADOUT2_R = crate::BitReader;
#[doc = "Field `PADOUT2` writer - PADOUT2"]
pub type PADOUT2_W<'a, const O: u8> = crate::BitWriter<'a, PADOUTR_SPEC, O>;
#[doc = "Field `PADOUT3` reader - PADOUT3"]
pub type PADOUT3_R = crate::BitReader;
#[doc = "Field `PADOUT3` writer - PADOUT3"]
pub type PADOUT3_W<'a, const O: u8> = crate::BitWriter<'a, PADOUTR_SPEC, O>;
#[doc = "Field `PADOUT4` reader - PADOUT4"]
pub type PADOUT4_R = crate::BitReader;
#[doc = "Field `PADOUT4` writer - PADOUT4"]
pub type PADOUT4_W<'a, const O: u8> = crate::BitWriter<'a, PADOUTR_SPEC, O>;
#[doc = "Field `PADOUT5` reader - PADOUT5"]
pub type PADOUT5_R = crate::BitReader;
#[doc = "Field `PADOUT5` writer - PADOUT5"]
pub type PADOUT5_W<'a, const O: u8> = crate::BitWriter<'a, PADOUTR_SPEC, O>;
#[doc = "Field `PADOUT6` reader - PADOUT6"]
pub type PADOUT6_R = crate::BitReader;
#[doc = "Field `PADOUT6` writer - PADOUT6"]
pub type PADOUT6_W<'a, const O: u8> = crate::BitWriter<'a, PADOUTR_SPEC, O>;
#[doc = "Field `PADOUT7` reader - PADOUT7"]
pub type PADOUT7_R = crate::BitReader;
#[doc = "Field `PADOUT7` writer - PADOUT7"]
pub type PADOUT7_W<'a, const O: u8> = crate::BitWriter<'a, PADOUTR_SPEC, O>;
#[doc = "Field `PADOUT8` reader - PADOUT8"]
pub type PADOUT8_R = crate::BitReader;
#[doc = "Field `PADOUT8` writer - PADOUT8"]
pub type PADOUT8_W<'a, const O: u8> = crate::BitWriter<'a, PADOUTR_SPEC, O>;
#[doc = "Field `PADOUT9` reader - PADOUT9"]
pub type PADOUT9_R = crate::BitReader;
#[doc = "Field `PADOUT9` writer - PADOUT9"]
pub type PADOUT9_W<'a, const O: u8> = crate::BitWriter<'a, PADOUTR_SPEC, O>;
#[doc = "Field `PADOUT10` reader - PADOUT10"]
pub type PADOUT10_R = crate::BitReader;
#[doc = "Field `PADOUT10` writer - PADOUT10"]
pub type PADOUT10_W<'a, const O: u8> = crate::BitWriter<'a, PADOUTR_SPEC, O>;
#[doc = "Field `PADOUT11` reader - PADOUT11"]
pub type PADOUT11_R = crate::BitReader;
#[doc = "Field `PADOUT11` writer - PADOUT11"]
pub type PADOUT11_W<'a, const O: u8> = crate::BitWriter<'a, PADOUTR_SPEC, O>;
#[doc = "Field `PADOUT12` reader - PADOUT12"]
pub type PADOUT12_R = crate::BitReader;
#[doc = "Field `PADOUT12` writer - PADOUT12"]
pub type PADOUT12_W<'a, const O: u8> = crate::BitWriter<'a, PADOUTR_SPEC, O>;
#[doc = "Field `PADOUT13` reader - PADOUT13"]
pub type PADOUT13_R = crate::BitReader;
#[doc = "Field `PADOUT13` writer - PADOUT13"]
pub type PADOUT13_W<'a, const O: u8> = crate::BitWriter<'a, PADOUTR_SPEC, O>;
#[doc = "Field `PADOUT14` reader - PADOUT14"]
pub type PADOUT14_R = crate::BitReader;
#[doc = "Field `PADOUT14` writer - PADOUT14"]
pub type PADOUT14_W<'a, const O: u8> = crate::BitWriter<'a, PADOUTR_SPEC, O>;
#[doc = "Field `PADOUT15` reader - PADOUT15"]
pub type PADOUT15_R = crate::BitReader;
#[doc = "Field `PADOUT15` writer - PADOUT15"]
pub type PADOUT15_W<'a, const O: u8> = crate::BitWriter<'a, PADOUTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PADOUT0"]
    #[inline(always)]
    pub fn padout0(&self) -> PADOUT0_R {
        PADOUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PADOUT1"]
    #[inline(always)]
    pub fn padout1(&self) -> PADOUT1_R {
        PADOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PADOUT2"]
    #[inline(always)]
    pub fn padout2(&self) -> PADOUT2_R {
        PADOUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PADOUT3"]
    #[inline(always)]
    pub fn padout3(&self) -> PADOUT3_R {
        PADOUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PADOUT4"]
    #[inline(always)]
    pub fn padout4(&self) -> PADOUT4_R {
        PADOUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PADOUT5"]
    #[inline(always)]
    pub fn padout5(&self) -> PADOUT5_R {
        PADOUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PADOUT6"]
    #[inline(always)]
    pub fn padout6(&self) -> PADOUT6_R {
        PADOUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PADOUT7"]
    #[inline(always)]
    pub fn padout7(&self) -> PADOUT7_R {
        PADOUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PADOUT8"]
    #[inline(always)]
    pub fn padout8(&self) -> PADOUT8_R {
        PADOUT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PADOUT9"]
    #[inline(always)]
    pub fn padout9(&self) -> PADOUT9_R {
        PADOUT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PADOUT10"]
    #[inline(always)]
    pub fn padout10(&self) -> PADOUT10_R {
        PADOUT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PADOUT11"]
    #[inline(always)]
    pub fn padout11(&self) -> PADOUT11_R {
        PADOUT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PADOUT12"]
    #[inline(always)]
    pub fn padout12(&self) -> PADOUT12_R {
        PADOUT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PADOUT13"]
    #[inline(always)]
    pub fn padout13(&self) -> PADOUT13_R {
        PADOUT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PADOUT14"]
    #[inline(always)]
    pub fn padout14(&self) -> PADOUT14_R {
        PADOUT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PADOUT15"]
    #[inline(always)]
    pub fn padout15(&self) -> PADOUT15_R {
        PADOUT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PADOUT0"]
    #[inline(always)]
    #[must_use]
    pub fn padout0(&mut self) -> PADOUT0_W<0> {
        PADOUT0_W::new(self)
    }
    #[doc = "Bit 1 - PADOUT1"]
    #[inline(always)]
    #[must_use]
    pub fn padout1(&mut self) -> PADOUT1_W<1> {
        PADOUT1_W::new(self)
    }
    #[doc = "Bit 2 - PADOUT2"]
    #[inline(always)]
    #[must_use]
    pub fn padout2(&mut self) -> PADOUT2_W<2> {
        PADOUT2_W::new(self)
    }
    #[doc = "Bit 3 - PADOUT3"]
    #[inline(always)]
    #[must_use]
    pub fn padout3(&mut self) -> PADOUT3_W<3> {
        PADOUT3_W::new(self)
    }
    #[doc = "Bit 4 - PADOUT4"]
    #[inline(always)]
    #[must_use]
    pub fn padout4(&mut self) -> PADOUT4_W<4> {
        PADOUT4_W::new(self)
    }
    #[doc = "Bit 5 - PADOUT5"]
    #[inline(always)]
    #[must_use]
    pub fn padout5(&mut self) -> PADOUT5_W<5> {
        PADOUT5_W::new(self)
    }
    #[doc = "Bit 6 - PADOUT6"]
    #[inline(always)]
    #[must_use]
    pub fn padout6(&mut self) -> PADOUT6_W<6> {
        PADOUT6_W::new(self)
    }
    #[doc = "Bit 7 - PADOUT7"]
    #[inline(always)]
    #[must_use]
    pub fn padout7(&mut self) -> PADOUT7_W<7> {
        PADOUT7_W::new(self)
    }
    #[doc = "Bit 8 - PADOUT8"]
    #[inline(always)]
    #[must_use]
    pub fn padout8(&mut self) -> PADOUT8_W<8> {
        PADOUT8_W::new(self)
    }
    #[doc = "Bit 9 - PADOUT9"]
    #[inline(always)]
    #[must_use]
    pub fn padout9(&mut self) -> PADOUT9_W<9> {
        PADOUT9_W::new(self)
    }
    #[doc = "Bit 10 - PADOUT10"]
    #[inline(always)]
    #[must_use]
    pub fn padout10(&mut self) -> PADOUT10_W<10> {
        PADOUT10_W::new(self)
    }
    #[doc = "Bit 11 - PADOUT11"]
    #[inline(always)]
    #[must_use]
    pub fn padout11(&mut self) -> PADOUT11_W<11> {
        PADOUT11_W::new(self)
    }
    #[doc = "Bit 12 - PADOUT12"]
    #[inline(always)]
    #[must_use]
    pub fn padout12(&mut self) -> PADOUT12_W<12> {
        PADOUT12_W::new(self)
    }
    #[doc = "Bit 13 - PADOUT13"]
    #[inline(always)]
    #[must_use]
    pub fn padout13(&mut self) -> PADOUT13_W<13> {
        PADOUT13_W::new(self)
    }
    #[doc = "Bit 14 - PADOUT14"]
    #[inline(always)]
    #[must_use]
    pub fn padout14(&mut self) -> PADOUT14_W<14> {
        PADOUT14_W::new(self)
    }
    #[doc = "Bit 15 - PADOUT15"]
    #[inline(always)]
    #[must_use]
    pub fn padout15(&mut self) -> PADOUT15_W<15> {
        PADOUT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PADOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padoutr](index.html) module"]
pub struct PADOUTR_SPEC;
impl crate::RegisterSpec for PADOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padoutr::R](R) reader structure"]
impl crate::Readable for PADOUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padoutr::W](W) writer structure"]
impl crate::Writable for PADOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADOUTR to value 0"]
impl crate::Resettable for PADOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
