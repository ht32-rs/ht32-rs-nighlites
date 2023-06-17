#[doc = "Register `PDSRR` reader"]
pub struct R(crate::R<PDSRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDSRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDSRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDSRR` writer"]
pub struct W(crate::W<PDSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDSRR_SPEC>;
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
impl From<crate::W<PDSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDSRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDSET0` reader - PDSET0"]
pub type PDSET0_R = crate::BitReader;
#[doc = "Field `PDSET0` writer - PDSET0"]
pub type PDSET0_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET1` reader - PDSET1"]
pub type PDSET1_R = crate::BitReader;
#[doc = "Field `PDSET1` writer - PDSET1"]
pub type PDSET1_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET2` reader - PDSET2"]
pub type PDSET2_R = crate::BitReader;
#[doc = "Field `PDSET2` writer - PDSET2"]
pub type PDSET2_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET3` reader - PDSET3"]
pub type PDSET3_R = crate::BitReader;
#[doc = "Field `PDSET3` writer - PDSET3"]
pub type PDSET3_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET4` reader - PDSET4"]
pub type PDSET4_R = crate::BitReader;
#[doc = "Field `PDSET4` writer - PDSET4"]
pub type PDSET4_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET5` reader - PDSET5"]
pub type PDSET5_R = crate::BitReader;
#[doc = "Field `PDSET5` writer - PDSET5"]
pub type PDSET5_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET6` reader - PDSET6"]
pub type PDSET6_R = crate::BitReader;
#[doc = "Field `PDSET6` writer - PDSET6"]
pub type PDSET6_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET7` reader - PDSET7"]
pub type PDSET7_R = crate::BitReader;
#[doc = "Field `PDSET7` writer - PDSET7"]
pub type PDSET7_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET8` reader - PDSET8"]
pub type PDSET8_R = crate::BitReader;
#[doc = "Field `PDSET8` writer - PDSET8"]
pub type PDSET8_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET9` reader - PDSET9"]
pub type PDSET9_R = crate::BitReader;
#[doc = "Field `PDSET9` writer - PDSET9"]
pub type PDSET9_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET10` reader - PDSET10"]
pub type PDSET10_R = crate::BitReader;
#[doc = "Field `PDSET10` writer - PDSET10"]
pub type PDSET10_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET11` reader - PDSET11"]
pub type PDSET11_R = crate::BitReader;
#[doc = "Field `PDSET11` writer - PDSET11"]
pub type PDSET11_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET12` reader - PDSET12"]
pub type PDSET12_R = crate::BitReader;
#[doc = "Field `PDSET12` writer - PDSET12"]
pub type PDSET12_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET13` reader - PDSET13"]
pub type PDSET13_R = crate::BitReader;
#[doc = "Field `PDSET13` writer - PDSET13"]
pub type PDSET13_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET14` reader - PDSET14"]
pub type PDSET14_R = crate::BitReader;
#[doc = "Field `PDSET14` writer - PDSET14"]
pub type PDSET14_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET15` reader - PDSET15"]
pub type PDSET15_R = crate::BitReader;
#[doc = "Field `PDSET15` writer - PDSET15"]
pub type PDSET15_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST0` reader - PDRST0"]
pub type PDRST0_R = crate::BitReader;
#[doc = "Field `PDRST0` writer - PDRST0"]
pub type PDRST0_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST1` reader - PDRST1"]
pub type PDRST1_R = crate::BitReader;
#[doc = "Field `PDRST1` writer - PDRST1"]
pub type PDRST1_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST2` reader - PDRST2"]
pub type PDRST2_R = crate::BitReader;
#[doc = "Field `PDRST2` writer - PDRST2"]
pub type PDRST2_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST3` reader - PDRST3"]
pub type PDRST3_R = crate::BitReader;
#[doc = "Field `PDRST3` writer - PDRST3"]
pub type PDRST3_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST4` reader - PDRST4"]
pub type PDRST4_R = crate::BitReader;
#[doc = "Field `PDRST4` writer - PDRST4"]
pub type PDRST4_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST5` reader - PDRST5"]
pub type PDRST5_R = crate::BitReader;
#[doc = "Field `PDRST5` writer - PDRST5"]
pub type PDRST5_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST6` reader - PDRST6"]
pub type PDRST6_R = crate::BitReader;
#[doc = "Field `PDRST6` writer - PDRST6"]
pub type PDRST6_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST7` reader - PDRST7"]
pub type PDRST7_R = crate::BitReader;
#[doc = "Field `PDRST7` writer - PDRST7"]
pub type PDRST7_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST8` reader - PDRST8"]
pub type PDRST8_R = crate::BitReader;
#[doc = "Field `PDRST8` writer - PDRST8"]
pub type PDRST8_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST9` reader - PDRST9"]
pub type PDRST9_R = crate::BitReader;
#[doc = "Field `PDRST9` writer - PDRST9"]
pub type PDRST9_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST10` reader - PDRST10"]
pub type PDRST10_R = crate::BitReader;
#[doc = "Field `PDRST10` writer - PDRST10"]
pub type PDRST10_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST11` reader - PDRST11"]
pub type PDRST11_R = crate::BitReader;
#[doc = "Field `PDRST11` writer - PDRST11"]
pub type PDRST11_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST12` reader - PDRST12"]
pub type PDRST12_R = crate::BitReader;
#[doc = "Field `PDRST12` writer - PDRST12"]
pub type PDRST12_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST13` reader - PDRST13"]
pub type PDRST13_R = crate::BitReader;
#[doc = "Field `PDRST13` writer - PDRST13"]
pub type PDRST13_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST14` reader - PDRST14"]
pub type PDRST14_R = crate::BitReader;
#[doc = "Field `PDRST14` writer - PDRST14"]
pub type PDRST14_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST15` reader - PDRST15"]
pub type PDRST15_R = crate::BitReader;
#[doc = "Field `PDRST15` writer - PDRST15"]
pub type PDRST15_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDSET0"]
    #[inline(always)]
    pub fn pdset0(&self) -> PDSET0_R {
        PDSET0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDSET1"]
    #[inline(always)]
    pub fn pdset1(&self) -> PDSET1_R {
        PDSET1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDSET2"]
    #[inline(always)]
    pub fn pdset2(&self) -> PDSET2_R {
        PDSET2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDSET3"]
    #[inline(always)]
    pub fn pdset3(&self) -> PDSET3_R {
        PDSET3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PDSET4"]
    #[inline(always)]
    pub fn pdset4(&self) -> PDSET4_R {
        PDSET4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PDSET5"]
    #[inline(always)]
    pub fn pdset5(&self) -> PDSET5_R {
        PDSET5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDSET6"]
    #[inline(always)]
    pub fn pdset6(&self) -> PDSET6_R {
        PDSET6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDSET7"]
    #[inline(always)]
    pub fn pdset7(&self) -> PDSET7_R {
        PDSET7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PDSET8"]
    #[inline(always)]
    pub fn pdset8(&self) -> PDSET8_R {
        PDSET8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PDSET9"]
    #[inline(always)]
    pub fn pdset9(&self) -> PDSET9_R {
        PDSET9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PDSET10"]
    #[inline(always)]
    pub fn pdset10(&self) -> PDSET10_R {
        PDSET10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PDSET11"]
    #[inline(always)]
    pub fn pdset11(&self) -> PDSET11_R {
        PDSET11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PDSET12"]
    #[inline(always)]
    pub fn pdset12(&self) -> PDSET12_R {
        PDSET12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PDSET13"]
    #[inline(always)]
    pub fn pdset13(&self) -> PDSET13_R {
        PDSET13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PDSET14"]
    #[inline(always)]
    pub fn pdset14(&self) -> PDSET14_R {
        PDSET14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PDSET15"]
    #[inline(always)]
    pub fn pdset15(&self) -> PDSET15_R {
        PDSET15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PDRST0"]
    #[inline(always)]
    pub fn pdrst0(&self) -> PDRST0_R {
        PDRST0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PDRST1"]
    #[inline(always)]
    pub fn pdrst1(&self) -> PDRST1_R {
        PDRST1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PDRST2"]
    #[inline(always)]
    pub fn pdrst2(&self) -> PDRST2_R {
        PDRST2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PDRST3"]
    #[inline(always)]
    pub fn pdrst3(&self) -> PDRST3_R {
        PDRST3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PDRST4"]
    #[inline(always)]
    pub fn pdrst4(&self) -> PDRST4_R {
        PDRST4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PDRST5"]
    #[inline(always)]
    pub fn pdrst5(&self) -> PDRST5_R {
        PDRST5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PDRST6"]
    #[inline(always)]
    pub fn pdrst6(&self) -> PDRST6_R {
        PDRST6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PDRST7"]
    #[inline(always)]
    pub fn pdrst7(&self) -> PDRST7_R {
        PDRST7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PDRST8"]
    #[inline(always)]
    pub fn pdrst8(&self) -> PDRST8_R {
        PDRST8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PDRST9"]
    #[inline(always)]
    pub fn pdrst9(&self) -> PDRST9_R {
        PDRST9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PDRST10"]
    #[inline(always)]
    pub fn pdrst10(&self) -> PDRST10_R {
        PDRST10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PDRST11"]
    #[inline(always)]
    pub fn pdrst11(&self) -> PDRST11_R {
        PDRST11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PDRST12"]
    #[inline(always)]
    pub fn pdrst12(&self) -> PDRST12_R {
        PDRST12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PDRST13"]
    #[inline(always)]
    pub fn pdrst13(&self) -> PDRST13_R {
        PDRST13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PDRST14"]
    #[inline(always)]
    pub fn pdrst14(&self) -> PDRST14_R {
        PDRST14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PDRST15"]
    #[inline(always)]
    pub fn pdrst15(&self) -> PDRST15_R {
        PDRST15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDSET0"]
    #[inline(always)]
    #[must_use]
    pub fn pdset0(&mut self) -> PDSET0_W<0> {
        PDSET0_W::new(self)
    }
    #[doc = "Bit 1 - PDSET1"]
    #[inline(always)]
    #[must_use]
    pub fn pdset1(&mut self) -> PDSET1_W<1> {
        PDSET1_W::new(self)
    }
    #[doc = "Bit 2 - PDSET2"]
    #[inline(always)]
    #[must_use]
    pub fn pdset2(&mut self) -> PDSET2_W<2> {
        PDSET2_W::new(self)
    }
    #[doc = "Bit 3 - PDSET3"]
    #[inline(always)]
    #[must_use]
    pub fn pdset3(&mut self) -> PDSET3_W<3> {
        PDSET3_W::new(self)
    }
    #[doc = "Bit 4 - PDSET4"]
    #[inline(always)]
    #[must_use]
    pub fn pdset4(&mut self) -> PDSET4_W<4> {
        PDSET4_W::new(self)
    }
    #[doc = "Bit 5 - PDSET5"]
    #[inline(always)]
    #[must_use]
    pub fn pdset5(&mut self) -> PDSET5_W<5> {
        PDSET5_W::new(self)
    }
    #[doc = "Bit 6 - PDSET6"]
    #[inline(always)]
    #[must_use]
    pub fn pdset6(&mut self) -> PDSET6_W<6> {
        PDSET6_W::new(self)
    }
    #[doc = "Bit 7 - PDSET7"]
    #[inline(always)]
    #[must_use]
    pub fn pdset7(&mut self) -> PDSET7_W<7> {
        PDSET7_W::new(self)
    }
    #[doc = "Bit 8 - PDSET8"]
    #[inline(always)]
    #[must_use]
    pub fn pdset8(&mut self) -> PDSET8_W<8> {
        PDSET8_W::new(self)
    }
    #[doc = "Bit 9 - PDSET9"]
    #[inline(always)]
    #[must_use]
    pub fn pdset9(&mut self) -> PDSET9_W<9> {
        PDSET9_W::new(self)
    }
    #[doc = "Bit 10 - PDSET10"]
    #[inline(always)]
    #[must_use]
    pub fn pdset10(&mut self) -> PDSET10_W<10> {
        PDSET10_W::new(self)
    }
    #[doc = "Bit 11 - PDSET11"]
    #[inline(always)]
    #[must_use]
    pub fn pdset11(&mut self) -> PDSET11_W<11> {
        PDSET11_W::new(self)
    }
    #[doc = "Bit 12 - PDSET12"]
    #[inline(always)]
    #[must_use]
    pub fn pdset12(&mut self) -> PDSET12_W<12> {
        PDSET12_W::new(self)
    }
    #[doc = "Bit 13 - PDSET13"]
    #[inline(always)]
    #[must_use]
    pub fn pdset13(&mut self) -> PDSET13_W<13> {
        PDSET13_W::new(self)
    }
    #[doc = "Bit 14 - PDSET14"]
    #[inline(always)]
    #[must_use]
    pub fn pdset14(&mut self) -> PDSET14_W<14> {
        PDSET14_W::new(self)
    }
    #[doc = "Bit 15 - PDSET15"]
    #[inline(always)]
    #[must_use]
    pub fn pdset15(&mut self) -> PDSET15_W<15> {
        PDSET15_W::new(self)
    }
    #[doc = "Bit 16 - PDRST0"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst0(&mut self) -> PDRST0_W<16> {
        PDRST0_W::new(self)
    }
    #[doc = "Bit 17 - PDRST1"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst1(&mut self) -> PDRST1_W<17> {
        PDRST1_W::new(self)
    }
    #[doc = "Bit 18 - PDRST2"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst2(&mut self) -> PDRST2_W<18> {
        PDRST2_W::new(self)
    }
    #[doc = "Bit 19 - PDRST3"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst3(&mut self) -> PDRST3_W<19> {
        PDRST3_W::new(self)
    }
    #[doc = "Bit 20 - PDRST4"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst4(&mut self) -> PDRST4_W<20> {
        PDRST4_W::new(self)
    }
    #[doc = "Bit 21 - PDRST5"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst5(&mut self) -> PDRST5_W<21> {
        PDRST5_W::new(self)
    }
    #[doc = "Bit 22 - PDRST6"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst6(&mut self) -> PDRST6_W<22> {
        PDRST6_W::new(self)
    }
    #[doc = "Bit 23 - PDRST7"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst7(&mut self) -> PDRST7_W<23> {
        PDRST7_W::new(self)
    }
    #[doc = "Bit 24 - PDRST8"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst8(&mut self) -> PDRST8_W<24> {
        PDRST8_W::new(self)
    }
    #[doc = "Bit 25 - PDRST9"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst9(&mut self) -> PDRST9_W<25> {
        PDRST9_W::new(self)
    }
    #[doc = "Bit 26 - PDRST10"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst10(&mut self) -> PDRST10_W<26> {
        PDRST10_W::new(self)
    }
    #[doc = "Bit 27 - PDRST11"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst11(&mut self) -> PDRST11_W<27> {
        PDRST11_W::new(self)
    }
    #[doc = "Bit 28 - PDRST12"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst12(&mut self) -> PDRST12_W<28> {
        PDRST12_W::new(self)
    }
    #[doc = "Bit 29 - PDRST13"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst13(&mut self) -> PDRST13_W<29> {
        PDRST13_W::new(self)
    }
    #[doc = "Bit 30 - PDRST14"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst14(&mut self) -> PDRST14_W<30> {
        PDRST14_W::new(self)
    }
    #[doc = "Bit 31 - PDRST15"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst15(&mut self) -> PDRST15_W<31> {
        PDRST15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDSRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsrr](index.html) module"]
pub struct PDSRR_SPEC;
impl crate::RegisterSpec for PDSRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdsrr::R](R) reader structure"]
impl crate::Readable for PDSRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdsrr::W](W) writer structure"]
impl crate::Writable for PDSRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDSRR to value 0"]
impl crate::Resettable for PDSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
