#[doc = "Register `PCSRR` reader"]
pub struct R(crate::R<PCSRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCSRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCSRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCSRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCSRR` writer"]
pub struct W(crate::W<PCSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCSRR_SPEC>;
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
impl From<crate::W<PCSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCSRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCSET0` reader - PCSET0"]
pub type PCSET0_R = crate::BitReader;
#[doc = "Field `PCSET0` writer - PCSET0"]
pub type PCSET0_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET1` reader - PCSET1"]
pub type PCSET1_R = crate::BitReader;
#[doc = "Field `PCSET1` writer - PCSET1"]
pub type PCSET1_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET2` reader - PCSET2"]
pub type PCSET2_R = crate::BitReader;
#[doc = "Field `PCSET2` writer - PCSET2"]
pub type PCSET2_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET3` reader - PCSET3"]
pub type PCSET3_R = crate::BitReader;
#[doc = "Field `PCSET3` writer - PCSET3"]
pub type PCSET3_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET4` reader - PCSET4"]
pub type PCSET4_R = crate::BitReader;
#[doc = "Field `PCSET4` writer - PCSET4"]
pub type PCSET4_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET5` reader - PCSET5"]
pub type PCSET5_R = crate::BitReader;
#[doc = "Field `PCSET5` writer - PCSET5"]
pub type PCSET5_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET6` reader - PCSET6"]
pub type PCSET6_R = crate::BitReader;
#[doc = "Field `PCSET6` writer - PCSET6"]
pub type PCSET6_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET7` reader - PCSET7"]
pub type PCSET7_R = crate::BitReader;
#[doc = "Field `PCSET7` writer - PCSET7"]
pub type PCSET7_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET8` reader - PCSET8"]
pub type PCSET8_R = crate::BitReader;
#[doc = "Field `PCSET8` writer - PCSET8"]
pub type PCSET8_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET9` reader - PCSET9"]
pub type PCSET9_R = crate::BitReader;
#[doc = "Field `PCSET9` writer - PCSET9"]
pub type PCSET9_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET10` reader - PCSET10"]
pub type PCSET10_R = crate::BitReader;
#[doc = "Field `PCSET10` writer - PCSET10"]
pub type PCSET10_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET11` reader - PCSET11"]
pub type PCSET11_R = crate::BitReader;
#[doc = "Field `PCSET11` writer - PCSET11"]
pub type PCSET11_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET12` reader - PCSET12"]
pub type PCSET12_R = crate::BitReader;
#[doc = "Field `PCSET12` writer - PCSET12"]
pub type PCSET12_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET13` reader - PCSET13"]
pub type PCSET13_R = crate::BitReader;
#[doc = "Field `PCSET13` writer - PCSET13"]
pub type PCSET13_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET14` reader - PCSET14"]
pub type PCSET14_R = crate::BitReader;
#[doc = "Field `PCSET14` writer - PCSET14"]
pub type PCSET14_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET15` reader - PCSET15"]
pub type PCSET15_R = crate::BitReader;
#[doc = "Field `PCSET15` writer - PCSET15"]
pub type PCSET15_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST0` reader - PCRST0"]
pub type PCRST0_R = crate::BitReader;
#[doc = "Field `PCRST0` writer - PCRST0"]
pub type PCRST0_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST1` reader - PCRST1"]
pub type PCRST1_R = crate::BitReader;
#[doc = "Field `PCRST1` writer - PCRST1"]
pub type PCRST1_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST2` reader - PCRST2"]
pub type PCRST2_R = crate::BitReader;
#[doc = "Field `PCRST2` writer - PCRST2"]
pub type PCRST2_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST3` reader - PCRST3"]
pub type PCRST3_R = crate::BitReader;
#[doc = "Field `PCRST3` writer - PCRST3"]
pub type PCRST3_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST4` reader - PCRST4"]
pub type PCRST4_R = crate::BitReader;
#[doc = "Field `PCRST4` writer - PCRST4"]
pub type PCRST4_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST5` reader - PCRST5"]
pub type PCRST5_R = crate::BitReader;
#[doc = "Field `PCRST5` writer - PCRST5"]
pub type PCRST5_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST6` reader - PCRST6"]
pub type PCRST6_R = crate::BitReader;
#[doc = "Field `PCRST6` writer - PCRST6"]
pub type PCRST6_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST7` reader - PCRST7"]
pub type PCRST7_R = crate::BitReader;
#[doc = "Field `PCRST7` writer - PCRST7"]
pub type PCRST7_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST8` reader - PCRST8"]
pub type PCRST8_R = crate::BitReader;
#[doc = "Field `PCRST8` writer - PCRST8"]
pub type PCRST8_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST9` reader - PCRST9"]
pub type PCRST9_R = crate::BitReader;
#[doc = "Field `PCRST9` writer - PCRST9"]
pub type PCRST9_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST10` reader - PCRST10"]
pub type PCRST10_R = crate::BitReader;
#[doc = "Field `PCRST10` writer - PCRST10"]
pub type PCRST10_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST11` reader - PCRST11"]
pub type PCRST11_R = crate::BitReader;
#[doc = "Field `PCRST11` writer - PCRST11"]
pub type PCRST11_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST12` reader - PCRST12"]
pub type PCRST12_R = crate::BitReader;
#[doc = "Field `PCRST12` writer - PCRST12"]
pub type PCRST12_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST13` reader - PCRST13"]
pub type PCRST13_R = crate::BitReader;
#[doc = "Field `PCRST13` writer - PCRST13"]
pub type PCRST13_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST14` reader - PCRST14"]
pub type PCRST14_R = crate::BitReader;
#[doc = "Field `PCRST14` writer - PCRST14"]
pub type PCRST14_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST15` reader - PCRST15"]
pub type PCRST15_R = crate::BitReader;
#[doc = "Field `PCRST15` writer - PCRST15"]
pub type PCRST15_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PCSET0"]
    #[inline(always)]
    pub fn pcset0(&self) -> PCSET0_R {
        PCSET0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCSET1"]
    #[inline(always)]
    pub fn pcset1(&self) -> PCSET1_R {
        PCSET1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCSET2"]
    #[inline(always)]
    pub fn pcset2(&self) -> PCSET2_R {
        PCSET2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCSET3"]
    #[inline(always)]
    pub fn pcset3(&self) -> PCSET3_R {
        PCSET3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PCSET4"]
    #[inline(always)]
    pub fn pcset4(&self) -> PCSET4_R {
        PCSET4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCSET5"]
    #[inline(always)]
    pub fn pcset5(&self) -> PCSET5_R {
        PCSET5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCSET6"]
    #[inline(always)]
    pub fn pcset6(&self) -> PCSET6_R {
        PCSET6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PCSET7"]
    #[inline(always)]
    pub fn pcset7(&self) -> PCSET7_R {
        PCSET7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PCSET8"]
    #[inline(always)]
    pub fn pcset8(&self) -> PCSET8_R {
        PCSET8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PCSET9"]
    #[inline(always)]
    pub fn pcset9(&self) -> PCSET9_R {
        PCSET9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCSET10"]
    #[inline(always)]
    pub fn pcset10(&self) -> PCSET10_R {
        PCSET10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PCSET11"]
    #[inline(always)]
    pub fn pcset11(&self) -> PCSET11_R {
        PCSET11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PCSET12"]
    #[inline(always)]
    pub fn pcset12(&self) -> PCSET12_R {
        PCSET12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PCSET13"]
    #[inline(always)]
    pub fn pcset13(&self) -> PCSET13_R {
        PCSET13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PCSET14"]
    #[inline(always)]
    pub fn pcset14(&self) -> PCSET14_R {
        PCSET14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PCSET15"]
    #[inline(always)]
    pub fn pcset15(&self) -> PCSET15_R {
        PCSET15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PCRST0"]
    #[inline(always)]
    pub fn pcrst0(&self) -> PCRST0_R {
        PCRST0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PCRST1"]
    #[inline(always)]
    pub fn pcrst1(&self) -> PCRST1_R {
        PCRST1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PCRST2"]
    #[inline(always)]
    pub fn pcrst2(&self) -> PCRST2_R {
        PCRST2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PCRST3"]
    #[inline(always)]
    pub fn pcrst3(&self) -> PCRST3_R {
        PCRST3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PCRST4"]
    #[inline(always)]
    pub fn pcrst4(&self) -> PCRST4_R {
        PCRST4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PCRST5"]
    #[inline(always)]
    pub fn pcrst5(&self) -> PCRST5_R {
        PCRST5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PCRST6"]
    #[inline(always)]
    pub fn pcrst6(&self) -> PCRST6_R {
        PCRST6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PCRST7"]
    #[inline(always)]
    pub fn pcrst7(&self) -> PCRST7_R {
        PCRST7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PCRST8"]
    #[inline(always)]
    pub fn pcrst8(&self) -> PCRST8_R {
        PCRST8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PCRST9"]
    #[inline(always)]
    pub fn pcrst9(&self) -> PCRST9_R {
        PCRST9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PCRST10"]
    #[inline(always)]
    pub fn pcrst10(&self) -> PCRST10_R {
        PCRST10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PCRST11"]
    #[inline(always)]
    pub fn pcrst11(&self) -> PCRST11_R {
        PCRST11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PCRST12"]
    #[inline(always)]
    pub fn pcrst12(&self) -> PCRST12_R {
        PCRST12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PCRST13"]
    #[inline(always)]
    pub fn pcrst13(&self) -> PCRST13_R {
        PCRST13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PCRST14"]
    #[inline(always)]
    pub fn pcrst14(&self) -> PCRST14_R {
        PCRST14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PCRST15"]
    #[inline(always)]
    pub fn pcrst15(&self) -> PCRST15_R {
        PCRST15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCSET0"]
    #[inline(always)]
    #[must_use]
    pub fn pcset0(&mut self) -> PCSET0_W<0> {
        PCSET0_W::new(self)
    }
    #[doc = "Bit 1 - PCSET1"]
    #[inline(always)]
    #[must_use]
    pub fn pcset1(&mut self) -> PCSET1_W<1> {
        PCSET1_W::new(self)
    }
    #[doc = "Bit 2 - PCSET2"]
    #[inline(always)]
    #[must_use]
    pub fn pcset2(&mut self) -> PCSET2_W<2> {
        PCSET2_W::new(self)
    }
    #[doc = "Bit 3 - PCSET3"]
    #[inline(always)]
    #[must_use]
    pub fn pcset3(&mut self) -> PCSET3_W<3> {
        PCSET3_W::new(self)
    }
    #[doc = "Bit 4 - PCSET4"]
    #[inline(always)]
    #[must_use]
    pub fn pcset4(&mut self) -> PCSET4_W<4> {
        PCSET4_W::new(self)
    }
    #[doc = "Bit 5 - PCSET5"]
    #[inline(always)]
    #[must_use]
    pub fn pcset5(&mut self) -> PCSET5_W<5> {
        PCSET5_W::new(self)
    }
    #[doc = "Bit 6 - PCSET6"]
    #[inline(always)]
    #[must_use]
    pub fn pcset6(&mut self) -> PCSET6_W<6> {
        PCSET6_W::new(self)
    }
    #[doc = "Bit 7 - PCSET7"]
    #[inline(always)]
    #[must_use]
    pub fn pcset7(&mut self) -> PCSET7_W<7> {
        PCSET7_W::new(self)
    }
    #[doc = "Bit 8 - PCSET8"]
    #[inline(always)]
    #[must_use]
    pub fn pcset8(&mut self) -> PCSET8_W<8> {
        PCSET8_W::new(self)
    }
    #[doc = "Bit 9 - PCSET9"]
    #[inline(always)]
    #[must_use]
    pub fn pcset9(&mut self) -> PCSET9_W<9> {
        PCSET9_W::new(self)
    }
    #[doc = "Bit 10 - PCSET10"]
    #[inline(always)]
    #[must_use]
    pub fn pcset10(&mut self) -> PCSET10_W<10> {
        PCSET10_W::new(self)
    }
    #[doc = "Bit 11 - PCSET11"]
    #[inline(always)]
    #[must_use]
    pub fn pcset11(&mut self) -> PCSET11_W<11> {
        PCSET11_W::new(self)
    }
    #[doc = "Bit 12 - PCSET12"]
    #[inline(always)]
    #[must_use]
    pub fn pcset12(&mut self) -> PCSET12_W<12> {
        PCSET12_W::new(self)
    }
    #[doc = "Bit 13 - PCSET13"]
    #[inline(always)]
    #[must_use]
    pub fn pcset13(&mut self) -> PCSET13_W<13> {
        PCSET13_W::new(self)
    }
    #[doc = "Bit 14 - PCSET14"]
    #[inline(always)]
    #[must_use]
    pub fn pcset14(&mut self) -> PCSET14_W<14> {
        PCSET14_W::new(self)
    }
    #[doc = "Bit 15 - PCSET15"]
    #[inline(always)]
    #[must_use]
    pub fn pcset15(&mut self) -> PCSET15_W<15> {
        PCSET15_W::new(self)
    }
    #[doc = "Bit 16 - PCRST0"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst0(&mut self) -> PCRST0_W<16> {
        PCRST0_W::new(self)
    }
    #[doc = "Bit 17 - PCRST1"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst1(&mut self) -> PCRST1_W<17> {
        PCRST1_W::new(self)
    }
    #[doc = "Bit 18 - PCRST2"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst2(&mut self) -> PCRST2_W<18> {
        PCRST2_W::new(self)
    }
    #[doc = "Bit 19 - PCRST3"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst3(&mut self) -> PCRST3_W<19> {
        PCRST3_W::new(self)
    }
    #[doc = "Bit 20 - PCRST4"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst4(&mut self) -> PCRST4_W<20> {
        PCRST4_W::new(self)
    }
    #[doc = "Bit 21 - PCRST5"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst5(&mut self) -> PCRST5_W<21> {
        PCRST5_W::new(self)
    }
    #[doc = "Bit 22 - PCRST6"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst6(&mut self) -> PCRST6_W<22> {
        PCRST6_W::new(self)
    }
    #[doc = "Bit 23 - PCRST7"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst7(&mut self) -> PCRST7_W<23> {
        PCRST7_W::new(self)
    }
    #[doc = "Bit 24 - PCRST8"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst8(&mut self) -> PCRST8_W<24> {
        PCRST8_W::new(self)
    }
    #[doc = "Bit 25 - PCRST9"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst9(&mut self) -> PCRST9_W<25> {
        PCRST9_W::new(self)
    }
    #[doc = "Bit 26 - PCRST10"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst10(&mut self) -> PCRST10_W<26> {
        PCRST10_W::new(self)
    }
    #[doc = "Bit 27 - PCRST11"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst11(&mut self) -> PCRST11_W<27> {
        PCRST11_W::new(self)
    }
    #[doc = "Bit 28 - PCRST12"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst12(&mut self) -> PCRST12_W<28> {
        PCRST12_W::new(self)
    }
    #[doc = "Bit 29 - PCRST13"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst13(&mut self) -> PCRST13_W<29> {
        PCRST13_W::new(self)
    }
    #[doc = "Bit 30 - PCRST14"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst14(&mut self) -> PCRST14_W<30> {
        PCRST14_W::new(self)
    }
    #[doc = "Bit 31 - PCRST15"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst15(&mut self) -> PCRST15_W<31> {
        PCRST15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCSRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsrr](index.html) module"]
pub struct PCSRR_SPEC;
impl crate::RegisterSpec for PCSRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcsrr::R](R) reader structure"]
impl crate::Readable for PCSRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcsrr::W](W) writer structure"]
impl crate::Writable for PCSRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCSRR to value 0"]
impl crate::Resettable for PCSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
