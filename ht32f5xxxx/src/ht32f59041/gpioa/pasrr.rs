#[doc = "Register `PASRR` reader"]
pub struct R(crate::R<PASRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PASRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PASRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PASRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PASRR` writer"]
pub struct W(crate::W<PASRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PASRR_SPEC>;
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
impl From<crate::W<PASRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PASRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PASET0` reader - PASET0"]
pub type PASET0_R = crate::BitReader;
#[doc = "Field `PASET0` writer - PASET0"]
pub type PASET0_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PASET1` reader - PASET1"]
pub type PASET1_R = crate::BitReader;
#[doc = "Field `PASET1` writer - PASET1"]
pub type PASET1_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PASET2` reader - PASET2"]
pub type PASET2_R = crate::BitReader;
#[doc = "Field `PASET2` writer - PASET2"]
pub type PASET2_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PASET3` reader - PASET3"]
pub type PASET3_R = crate::BitReader;
#[doc = "Field `PASET3` writer - PASET3"]
pub type PASET3_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PASET4` reader - PASET4"]
pub type PASET4_R = crate::BitReader;
#[doc = "Field `PASET4` writer - PASET4"]
pub type PASET4_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PASET5` reader - PASET5"]
pub type PASET5_R = crate::BitReader;
#[doc = "Field `PASET5` writer - PASET5"]
pub type PASET5_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PASET6` reader - PASET6"]
pub type PASET6_R = crate::BitReader;
#[doc = "Field `PASET6` writer - PASET6"]
pub type PASET6_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PASET7` reader - PASET7"]
pub type PASET7_R = crate::BitReader;
#[doc = "Field `PASET7` writer - PASET7"]
pub type PASET7_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PASET8` reader - PASET8"]
pub type PASET8_R = crate::BitReader;
#[doc = "Field `PASET8` writer - PASET8"]
pub type PASET8_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PASET9` reader - PASET9"]
pub type PASET9_R = crate::BitReader;
#[doc = "Field `PASET9` writer - PASET9"]
pub type PASET9_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PASET10` reader - PASET10"]
pub type PASET10_R = crate::BitReader;
#[doc = "Field `PASET10` writer - PASET10"]
pub type PASET10_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PASET11` reader - PASET11"]
pub type PASET11_R = crate::BitReader;
#[doc = "Field `PASET11` writer - PASET11"]
pub type PASET11_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PASET12` reader - PASET12"]
pub type PASET12_R = crate::BitReader;
#[doc = "Field `PASET12` writer - PASET12"]
pub type PASET12_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PASET13` reader - PASET13"]
pub type PASET13_R = crate::BitReader;
#[doc = "Field `PASET13` writer - PASET13"]
pub type PASET13_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PASET14` reader - PASET14"]
pub type PASET14_R = crate::BitReader;
#[doc = "Field `PASET14` writer - PASET14"]
pub type PASET14_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PASET15` reader - PASET15"]
pub type PASET15_R = crate::BitReader;
#[doc = "Field `PASET15` writer - PASET15"]
pub type PASET15_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PARST0` reader - PARST0"]
pub type PARST0_R = crate::BitReader;
#[doc = "Field `PARST0` writer - PARST0"]
pub type PARST0_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PARST1` reader - PARST1"]
pub type PARST1_R = crate::BitReader;
#[doc = "Field `PARST1` writer - PARST1"]
pub type PARST1_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PARST2` reader - PARST2"]
pub type PARST2_R = crate::BitReader;
#[doc = "Field `PARST2` writer - PARST2"]
pub type PARST2_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PARST3` reader - PARST3"]
pub type PARST3_R = crate::BitReader;
#[doc = "Field `PARST3` writer - PARST3"]
pub type PARST3_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PARST4` reader - PARST4"]
pub type PARST4_R = crate::BitReader;
#[doc = "Field `PARST4` writer - PARST4"]
pub type PARST4_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PARST5` reader - PARST5"]
pub type PARST5_R = crate::BitReader;
#[doc = "Field `PARST5` writer - PARST5"]
pub type PARST5_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PARST6` reader - PARST6"]
pub type PARST6_R = crate::BitReader;
#[doc = "Field `PARST6` writer - PARST6"]
pub type PARST6_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PARST7` reader - PARST7"]
pub type PARST7_R = crate::BitReader;
#[doc = "Field `PARST7` writer - PARST7"]
pub type PARST7_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PARST8` reader - PARST8"]
pub type PARST8_R = crate::BitReader;
#[doc = "Field `PARST8` writer - PARST8"]
pub type PARST8_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PARST9` reader - PARST9"]
pub type PARST9_R = crate::BitReader;
#[doc = "Field `PARST9` writer - PARST9"]
pub type PARST9_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PARST10` reader - PARST10"]
pub type PARST10_R = crate::BitReader;
#[doc = "Field `PARST10` writer - PARST10"]
pub type PARST10_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PARST11` reader - PARST11"]
pub type PARST11_R = crate::BitReader;
#[doc = "Field `PARST11` writer - PARST11"]
pub type PARST11_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PARST12` reader - PARST12"]
pub type PARST12_R = crate::BitReader;
#[doc = "Field `PARST12` writer - PARST12"]
pub type PARST12_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PARST13` reader - PARST13"]
pub type PARST13_R = crate::BitReader;
#[doc = "Field `PARST13` writer - PARST13"]
pub type PARST13_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PARST14` reader - PARST14"]
pub type PARST14_R = crate::BitReader;
#[doc = "Field `PARST14` writer - PARST14"]
pub type PARST14_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
#[doc = "Field `PARST15` reader - PARST15"]
pub type PARST15_R = crate::BitReader;
#[doc = "Field `PARST15` writer - PARST15"]
pub type PARST15_W<'a, const O: u8> = crate::BitWriter<'a, PASRR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PASET0"]
    #[inline(always)]
    pub fn paset0(&self) -> PASET0_R {
        PASET0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PASET1"]
    #[inline(always)]
    pub fn paset1(&self) -> PASET1_R {
        PASET1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PASET2"]
    #[inline(always)]
    pub fn paset2(&self) -> PASET2_R {
        PASET2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PASET3"]
    #[inline(always)]
    pub fn paset3(&self) -> PASET3_R {
        PASET3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PASET4"]
    #[inline(always)]
    pub fn paset4(&self) -> PASET4_R {
        PASET4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PASET5"]
    #[inline(always)]
    pub fn paset5(&self) -> PASET5_R {
        PASET5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PASET6"]
    #[inline(always)]
    pub fn paset6(&self) -> PASET6_R {
        PASET6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PASET7"]
    #[inline(always)]
    pub fn paset7(&self) -> PASET7_R {
        PASET7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PASET8"]
    #[inline(always)]
    pub fn paset8(&self) -> PASET8_R {
        PASET8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PASET9"]
    #[inline(always)]
    pub fn paset9(&self) -> PASET9_R {
        PASET9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PASET10"]
    #[inline(always)]
    pub fn paset10(&self) -> PASET10_R {
        PASET10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PASET11"]
    #[inline(always)]
    pub fn paset11(&self) -> PASET11_R {
        PASET11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PASET12"]
    #[inline(always)]
    pub fn paset12(&self) -> PASET12_R {
        PASET12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PASET13"]
    #[inline(always)]
    pub fn paset13(&self) -> PASET13_R {
        PASET13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PASET14"]
    #[inline(always)]
    pub fn paset14(&self) -> PASET14_R {
        PASET14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PASET15"]
    #[inline(always)]
    pub fn paset15(&self) -> PASET15_R {
        PASET15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PARST0"]
    #[inline(always)]
    pub fn parst0(&self) -> PARST0_R {
        PARST0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PARST1"]
    #[inline(always)]
    pub fn parst1(&self) -> PARST1_R {
        PARST1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PARST2"]
    #[inline(always)]
    pub fn parst2(&self) -> PARST2_R {
        PARST2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PARST3"]
    #[inline(always)]
    pub fn parst3(&self) -> PARST3_R {
        PARST3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PARST4"]
    #[inline(always)]
    pub fn parst4(&self) -> PARST4_R {
        PARST4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PARST5"]
    #[inline(always)]
    pub fn parst5(&self) -> PARST5_R {
        PARST5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PARST6"]
    #[inline(always)]
    pub fn parst6(&self) -> PARST6_R {
        PARST6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PARST7"]
    #[inline(always)]
    pub fn parst7(&self) -> PARST7_R {
        PARST7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PARST8"]
    #[inline(always)]
    pub fn parst8(&self) -> PARST8_R {
        PARST8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PARST9"]
    #[inline(always)]
    pub fn parst9(&self) -> PARST9_R {
        PARST9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PARST10"]
    #[inline(always)]
    pub fn parst10(&self) -> PARST10_R {
        PARST10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PARST11"]
    #[inline(always)]
    pub fn parst11(&self) -> PARST11_R {
        PARST11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PARST12"]
    #[inline(always)]
    pub fn parst12(&self) -> PARST12_R {
        PARST12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PARST13"]
    #[inline(always)]
    pub fn parst13(&self) -> PARST13_R {
        PARST13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PARST14"]
    #[inline(always)]
    pub fn parst14(&self) -> PARST14_R {
        PARST14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PARST15"]
    #[inline(always)]
    pub fn parst15(&self) -> PARST15_R {
        PARST15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PASET0"]
    #[inline(always)]
    #[must_use]
    pub fn paset0(&mut self) -> PASET0_W<0> {
        PASET0_W::new(self)
    }
    #[doc = "Bit 1 - PASET1"]
    #[inline(always)]
    #[must_use]
    pub fn paset1(&mut self) -> PASET1_W<1> {
        PASET1_W::new(self)
    }
    #[doc = "Bit 2 - PASET2"]
    #[inline(always)]
    #[must_use]
    pub fn paset2(&mut self) -> PASET2_W<2> {
        PASET2_W::new(self)
    }
    #[doc = "Bit 3 - PASET3"]
    #[inline(always)]
    #[must_use]
    pub fn paset3(&mut self) -> PASET3_W<3> {
        PASET3_W::new(self)
    }
    #[doc = "Bit 4 - PASET4"]
    #[inline(always)]
    #[must_use]
    pub fn paset4(&mut self) -> PASET4_W<4> {
        PASET4_W::new(self)
    }
    #[doc = "Bit 5 - PASET5"]
    #[inline(always)]
    #[must_use]
    pub fn paset5(&mut self) -> PASET5_W<5> {
        PASET5_W::new(self)
    }
    #[doc = "Bit 6 - PASET6"]
    #[inline(always)]
    #[must_use]
    pub fn paset6(&mut self) -> PASET6_W<6> {
        PASET6_W::new(self)
    }
    #[doc = "Bit 7 - PASET7"]
    #[inline(always)]
    #[must_use]
    pub fn paset7(&mut self) -> PASET7_W<7> {
        PASET7_W::new(self)
    }
    #[doc = "Bit 8 - PASET8"]
    #[inline(always)]
    #[must_use]
    pub fn paset8(&mut self) -> PASET8_W<8> {
        PASET8_W::new(self)
    }
    #[doc = "Bit 9 - PASET9"]
    #[inline(always)]
    #[must_use]
    pub fn paset9(&mut self) -> PASET9_W<9> {
        PASET9_W::new(self)
    }
    #[doc = "Bit 10 - PASET10"]
    #[inline(always)]
    #[must_use]
    pub fn paset10(&mut self) -> PASET10_W<10> {
        PASET10_W::new(self)
    }
    #[doc = "Bit 11 - PASET11"]
    #[inline(always)]
    #[must_use]
    pub fn paset11(&mut self) -> PASET11_W<11> {
        PASET11_W::new(self)
    }
    #[doc = "Bit 12 - PASET12"]
    #[inline(always)]
    #[must_use]
    pub fn paset12(&mut self) -> PASET12_W<12> {
        PASET12_W::new(self)
    }
    #[doc = "Bit 13 - PASET13"]
    #[inline(always)]
    #[must_use]
    pub fn paset13(&mut self) -> PASET13_W<13> {
        PASET13_W::new(self)
    }
    #[doc = "Bit 14 - PASET14"]
    #[inline(always)]
    #[must_use]
    pub fn paset14(&mut self) -> PASET14_W<14> {
        PASET14_W::new(self)
    }
    #[doc = "Bit 15 - PASET15"]
    #[inline(always)]
    #[must_use]
    pub fn paset15(&mut self) -> PASET15_W<15> {
        PASET15_W::new(self)
    }
    #[doc = "Bit 16 - PARST0"]
    #[inline(always)]
    #[must_use]
    pub fn parst0(&mut self) -> PARST0_W<16> {
        PARST0_W::new(self)
    }
    #[doc = "Bit 17 - PARST1"]
    #[inline(always)]
    #[must_use]
    pub fn parst1(&mut self) -> PARST1_W<17> {
        PARST1_W::new(self)
    }
    #[doc = "Bit 18 - PARST2"]
    #[inline(always)]
    #[must_use]
    pub fn parst2(&mut self) -> PARST2_W<18> {
        PARST2_W::new(self)
    }
    #[doc = "Bit 19 - PARST3"]
    #[inline(always)]
    #[must_use]
    pub fn parst3(&mut self) -> PARST3_W<19> {
        PARST3_W::new(self)
    }
    #[doc = "Bit 20 - PARST4"]
    #[inline(always)]
    #[must_use]
    pub fn parst4(&mut self) -> PARST4_W<20> {
        PARST4_W::new(self)
    }
    #[doc = "Bit 21 - PARST5"]
    #[inline(always)]
    #[must_use]
    pub fn parst5(&mut self) -> PARST5_W<21> {
        PARST5_W::new(self)
    }
    #[doc = "Bit 22 - PARST6"]
    #[inline(always)]
    #[must_use]
    pub fn parst6(&mut self) -> PARST6_W<22> {
        PARST6_W::new(self)
    }
    #[doc = "Bit 23 - PARST7"]
    #[inline(always)]
    #[must_use]
    pub fn parst7(&mut self) -> PARST7_W<23> {
        PARST7_W::new(self)
    }
    #[doc = "Bit 24 - PARST8"]
    #[inline(always)]
    #[must_use]
    pub fn parst8(&mut self) -> PARST8_W<24> {
        PARST8_W::new(self)
    }
    #[doc = "Bit 25 - PARST9"]
    #[inline(always)]
    #[must_use]
    pub fn parst9(&mut self) -> PARST9_W<25> {
        PARST9_W::new(self)
    }
    #[doc = "Bit 26 - PARST10"]
    #[inline(always)]
    #[must_use]
    pub fn parst10(&mut self) -> PARST10_W<26> {
        PARST10_W::new(self)
    }
    #[doc = "Bit 27 - PARST11"]
    #[inline(always)]
    #[must_use]
    pub fn parst11(&mut self) -> PARST11_W<27> {
        PARST11_W::new(self)
    }
    #[doc = "Bit 28 - PARST12"]
    #[inline(always)]
    #[must_use]
    pub fn parst12(&mut self) -> PARST12_W<28> {
        PARST12_W::new(self)
    }
    #[doc = "Bit 29 - PARST13"]
    #[inline(always)]
    #[must_use]
    pub fn parst13(&mut self) -> PARST13_W<29> {
        PARST13_W::new(self)
    }
    #[doc = "Bit 30 - PARST14"]
    #[inline(always)]
    #[must_use]
    pub fn parst14(&mut self) -> PARST14_W<30> {
        PARST14_W::new(self)
    }
    #[doc = "Bit 31 - PARST15"]
    #[inline(always)]
    #[must_use]
    pub fn parst15(&mut self) -> PARST15_W<31> {
        PARST15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PASRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pasrr](index.html) module"]
pub struct PASRR_SPEC;
impl crate::RegisterSpec for PASRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pasrr::R](R) reader structure"]
impl crate::Readable for PASRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pasrr::W](W) writer structure"]
impl crate::Writable for PASRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PASRR to value 0"]
impl crate::Resettable for PASRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
