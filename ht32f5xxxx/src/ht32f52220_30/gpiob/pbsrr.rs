#[doc = "Register `PBSRR` reader"]
pub struct R(crate::R<PBSRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBSRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBSRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBSRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBSRR` writer"]
pub struct W(crate::W<PBSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBSRR_SPEC>;
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
impl From<crate::W<PBSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBSRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBSET0` reader - PBSET0"]
pub type PBSET0_R = crate::BitReader;
#[doc = "Field `PBSET0` writer - PBSET0"]
pub type PBSET0_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBSET1` reader - PBSET1"]
pub type PBSET1_R = crate::BitReader;
#[doc = "Field `PBSET1` writer - PBSET1"]
pub type PBSET1_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBSET2` reader - PBSET2"]
pub type PBSET2_R = crate::BitReader;
#[doc = "Field `PBSET2` writer - PBSET2"]
pub type PBSET2_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBSET3` reader - PBSET3"]
pub type PBSET3_R = crate::BitReader;
#[doc = "Field `PBSET3` writer - PBSET3"]
pub type PBSET3_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBSET4` reader - PBSET4"]
pub type PBSET4_R = crate::BitReader;
#[doc = "Field `PBSET4` writer - PBSET4"]
pub type PBSET4_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBSET5` reader - PBSET5"]
pub type PBSET5_R = crate::BitReader;
#[doc = "Field `PBSET5` writer - PBSET5"]
pub type PBSET5_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBSET6` reader - PBSET6"]
pub type PBSET6_R = crate::BitReader;
#[doc = "Field `PBSET6` writer - PBSET6"]
pub type PBSET6_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBSET7` reader - PBSET7"]
pub type PBSET7_R = crate::BitReader;
#[doc = "Field `PBSET7` writer - PBSET7"]
pub type PBSET7_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBSET8` reader - PBSET8"]
pub type PBSET8_R = crate::BitReader;
#[doc = "Field `PBSET8` writer - PBSET8"]
pub type PBSET8_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBSET9` reader - PBSET9"]
pub type PBSET9_R = crate::BitReader;
#[doc = "Field `PBSET9` writer - PBSET9"]
pub type PBSET9_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBSET10` reader - PBSET10"]
pub type PBSET10_R = crate::BitReader;
#[doc = "Field `PBSET10` writer - PBSET10"]
pub type PBSET10_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBSET11` reader - PBSET11"]
pub type PBSET11_R = crate::BitReader;
#[doc = "Field `PBSET11` writer - PBSET11"]
pub type PBSET11_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBSET12` reader - PBSET12"]
pub type PBSET12_R = crate::BitReader;
#[doc = "Field `PBSET12` writer - PBSET12"]
pub type PBSET12_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBSET13` reader - PBSET13"]
pub type PBSET13_R = crate::BitReader;
#[doc = "Field `PBSET13` writer - PBSET13"]
pub type PBSET13_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBSET14` reader - PBSET14"]
pub type PBSET14_R = crate::BitReader;
#[doc = "Field `PBSET14` writer - PBSET14"]
pub type PBSET14_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBRST0` reader - PBRST0"]
pub type PBRST0_R = crate::BitReader;
#[doc = "Field `PBRST0` writer - PBRST0"]
pub type PBRST0_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBRST1` reader - PBRST1"]
pub type PBRST1_R = crate::BitReader;
#[doc = "Field `PBRST1` writer - PBRST1"]
pub type PBRST1_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBRST2` reader - PBRST2"]
pub type PBRST2_R = crate::BitReader;
#[doc = "Field `PBRST2` writer - PBRST2"]
pub type PBRST2_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBRST3` reader - PBRST3"]
pub type PBRST3_R = crate::BitReader;
#[doc = "Field `PBRST3` writer - PBRST3"]
pub type PBRST3_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBRST4` reader - PBRST4"]
pub type PBRST4_R = crate::BitReader;
#[doc = "Field `PBRST4` writer - PBRST4"]
pub type PBRST4_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBRST5` reader - PBRST5"]
pub type PBRST5_R = crate::BitReader;
#[doc = "Field `PBRST5` writer - PBRST5"]
pub type PBRST5_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBRST6` reader - PBRST6"]
pub type PBRST6_R = crate::BitReader;
#[doc = "Field `PBRST6` writer - PBRST6"]
pub type PBRST6_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBRST7` reader - PBRST7"]
pub type PBRST7_R = crate::BitReader;
#[doc = "Field `PBRST7` writer - PBRST7"]
pub type PBRST7_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBRST8` reader - PBRST8"]
pub type PBRST8_R = crate::BitReader;
#[doc = "Field `PBRST8` writer - PBRST8"]
pub type PBRST8_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBRST9` reader - PBRST9"]
pub type PBRST9_R = crate::BitReader;
#[doc = "Field `PBRST9` writer - PBRST9"]
pub type PBRST9_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBRST10` reader - PBRST10"]
pub type PBRST10_R = crate::BitReader;
#[doc = "Field `PBRST10` writer - PBRST10"]
pub type PBRST10_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBRST11` reader - PBRST11"]
pub type PBRST11_R = crate::BitReader;
#[doc = "Field `PBRST11` writer - PBRST11"]
pub type PBRST11_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBRST12` reader - PBRST12"]
pub type PBRST12_R = crate::BitReader;
#[doc = "Field `PBRST12` writer - PBRST12"]
pub type PBRST12_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBRST13` reader - PBRST13"]
pub type PBRST13_R = crate::BitReader;
#[doc = "Field `PBRST13` writer - PBRST13"]
pub type PBRST13_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
#[doc = "Field `PBRST14` reader - PBRST14"]
pub type PBRST14_R = crate::BitReader;
#[doc = "Field `PBRST14` writer - PBRST14"]
pub type PBRST14_W<'a, const O: u8> = crate::BitWriter<'a, PBSRR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PBSET0"]
    #[inline(always)]
    pub fn pbset0(&self) -> PBSET0_R {
        PBSET0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PBSET1"]
    #[inline(always)]
    pub fn pbset1(&self) -> PBSET1_R {
        PBSET1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBSET2"]
    #[inline(always)]
    pub fn pbset2(&self) -> PBSET2_R {
        PBSET2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PBSET3"]
    #[inline(always)]
    pub fn pbset3(&self) -> PBSET3_R {
        PBSET3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PBSET4"]
    #[inline(always)]
    pub fn pbset4(&self) -> PBSET4_R {
        PBSET4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PBSET5"]
    #[inline(always)]
    pub fn pbset5(&self) -> PBSET5_R {
        PBSET5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PBSET6"]
    #[inline(always)]
    pub fn pbset6(&self) -> PBSET6_R {
        PBSET6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PBSET7"]
    #[inline(always)]
    pub fn pbset7(&self) -> PBSET7_R {
        PBSET7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PBSET8"]
    #[inline(always)]
    pub fn pbset8(&self) -> PBSET8_R {
        PBSET8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PBSET9"]
    #[inline(always)]
    pub fn pbset9(&self) -> PBSET9_R {
        PBSET9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PBSET10"]
    #[inline(always)]
    pub fn pbset10(&self) -> PBSET10_R {
        PBSET10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PBSET11"]
    #[inline(always)]
    pub fn pbset11(&self) -> PBSET11_R {
        PBSET11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PBSET12"]
    #[inline(always)]
    pub fn pbset12(&self) -> PBSET12_R {
        PBSET12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PBSET13"]
    #[inline(always)]
    pub fn pbset13(&self) -> PBSET13_R {
        PBSET13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PBSET14"]
    #[inline(always)]
    pub fn pbset14(&self) -> PBSET14_R {
        PBSET14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - PBRST0"]
    #[inline(always)]
    pub fn pbrst0(&self) -> PBRST0_R {
        PBRST0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PBRST1"]
    #[inline(always)]
    pub fn pbrst1(&self) -> PBRST1_R {
        PBRST1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PBRST2"]
    #[inline(always)]
    pub fn pbrst2(&self) -> PBRST2_R {
        PBRST2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PBRST3"]
    #[inline(always)]
    pub fn pbrst3(&self) -> PBRST3_R {
        PBRST3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PBRST4"]
    #[inline(always)]
    pub fn pbrst4(&self) -> PBRST4_R {
        PBRST4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PBRST5"]
    #[inline(always)]
    pub fn pbrst5(&self) -> PBRST5_R {
        PBRST5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PBRST6"]
    #[inline(always)]
    pub fn pbrst6(&self) -> PBRST6_R {
        PBRST6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PBRST7"]
    #[inline(always)]
    pub fn pbrst7(&self) -> PBRST7_R {
        PBRST7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PBRST8"]
    #[inline(always)]
    pub fn pbrst8(&self) -> PBRST8_R {
        PBRST8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PBRST9"]
    #[inline(always)]
    pub fn pbrst9(&self) -> PBRST9_R {
        PBRST9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PBRST10"]
    #[inline(always)]
    pub fn pbrst10(&self) -> PBRST10_R {
        PBRST10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PBRST11"]
    #[inline(always)]
    pub fn pbrst11(&self) -> PBRST11_R {
        PBRST11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PBRST12"]
    #[inline(always)]
    pub fn pbrst12(&self) -> PBRST12_R {
        PBRST12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PBRST13"]
    #[inline(always)]
    pub fn pbrst13(&self) -> PBRST13_R {
        PBRST13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PBRST14"]
    #[inline(always)]
    pub fn pbrst14(&self) -> PBRST14_R {
        PBRST14_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PBSET0"]
    #[inline(always)]
    #[must_use]
    pub fn pbset0(&mut self) -> PBSET0_W<0> {
        PBSET0_W::new(self)
    }
    #[doc = "Bit 1 - PBSET1"]
    #[inline(always)]
    #[must_use]
    pub fn pbset1(&mut self) -> PBSET1_W<1> {
        PBSET1_W::new(self)
    }
    #[doc = "Bit 2 - PBSET2"]
    #[inline(always)]
    #[must_use]
    pub fn pbset2(&mut self) -> PBSET2_W<2> {
        PBSET2_W::new(self)
    }
    #[doc = "Bit 3 - PBSET3"]
    #[inline(always)]
    #[must_use]
    pub fn pbset3(&mut self) -> PBSET3_W<3> {
        PBSET3_W::new(self)
    }
    #[doc = "Bit 4 - PBSET4"]
    #[inline(always)]
    #[must_use]
    pub fn pbset4(&mut self) -> PBSET4_W<4> {
        PBSET4_W::new(self)
    }
    #[doc = "Bit 5 - PBSET5"]
    #[inline(always)]
    #[must_use]
    pub fn pbset5(&mut self) -> PBSET5_W<5> {
        PBSET5_W::new(self)
    }
    #[doc = "Bit 6 - PBSET6"]
    #[inline(always)]
    #[must_use]
    pub fn pbset6(&mut self) -> PBSET6_W<6> {
        PBSET6_W::new(self)
    }
    #[doc = "Bit 7 - PBSET7"]
    #[inline(always)]
    #[must_use]
    pub fn pbset7(&mut self) -> PBSET7_W<7> {
        PBSET7_W::new(self)
    }
    #[doc = "Bit 8 - PBSET8"]
    #[inline(always)]
    #[must_use]
    pub fn pbset8(&mut self) -> PBSET8_W<8> {
        PBSET8_W::new(self)
    }
    #[doc = "Bit 9 - PBSET9"]
    #[inline(always)]
    #[must_use]
    pub fn pbset9(&mut self) -> PBSET9_W<9> {
        PBSET9_W::new(self)
    }
    #[doc = "Bit 10 - PBSET10"]
    #[inline(always)]
    #[must_use]
    pub fn pbset10(&mut self) -> PBSET10_W<10> {
        PBSET10_W::new(self)
    }
    #[doc = "Bit 11 - PBSET11"]
    #[inline(always)]
    #[must_use]
    pub fn pbset11(&mut self) -> PBSET11_W<11> {
        PBSET11_W::new(self)
    }
    #[doc = "Bit 12 - PBSET12"]
    #[inline(always)]
    #[must_use]
    pub fn pbset12(&mut self) -> PBSET12_W<12> {
        PBSET12_W::new(self)
    }
    #[doc = "Bit 13 - PBSET13"]
    #[inline(always)]
    #[must_use]
    pub fn pbset13(&mut self) -> PBSET13_W<13> {
        PBSET13_W::new(self)
    }
    #[doc = "Bit 14 - PBSET14"]
    #[inline(always)]
    #[must_use]
    pub fn pbset14(&mut self) -> PBSET14_W<14> {
        PBSET14_W::new(self)
    }
    #[doc = "Bit 16 - PBRST0"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst0(&mut self) -> PBRST0_W<16> {
        PBRST0_W::new(self)
    }
    #[doc = "Bit 17 - PBRST1"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst1(&mut self) -> PBRST1_W<17> {
        PBRST1_W::new(self)
    }
    #[doc = "Bit 18 - PBRST2"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst2(&mut self) -> PBRST2_W<18> {
        PBRST2_W::new(self)
    }
    #[doc = "Bit 19 - PBRST3"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst3(&mut self) -> PBRST3_W<19> {
        PBRST3_W::new(self)
    }
    #[doc = "Bit 20 - PBRST4"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst4(&mut self) -> PBRST4_W<20> {
        PBRST4_W::new(self)
    }
    #[doc = "Bit 21 - PBRST5"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst5(&mut self) -> PBRST5_W<21> {
        PBRST5_W::new(self)
    }
    #[doc = "Bit 22 - PBRST6"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst6(&mut self) -> PBRST6_W<22> {
        PBRST6_W::new(self)
    }
    #[doc = "Bit 23 - PBRST7"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst7(&mut self) -> PBRST7_W<23> {
        PBRST7_W::new(self)
    }
    #[doc = "Bit 24 - PBRST8"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst8(&mut self) -> PBRST8_W<24> {
        PBRST8_W::new(self)
    }
    #[doc = "Bit 25 - PBRST9"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst9(&mut self) -> PBRST9_W<25> {
        PBRST9_W::new(self)
    }
    #[doc = "Bit 26 - PBRST10"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst10(&mut self) -> PBRST10_W<26> {
        PBRST10_W::new(self)
    }
    #[doc = "Bit 27 - PBRST11"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst11(&mut self) -> PBRST11_W<27> {
        PBRST11_W::new(self)
    }
    #[doc = "Bit 28 - PBRST12"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst12(&mut self) -> PBRST12_W<28> {
        PBRST12_W::new(self)
    }
    #[doc = "Bit 29 - PBRST13"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst13(&mut self) -> PBRST13_W<29> {
        PBRST13_W::new(self)
    }
    #[doc = "Bit 30 - PBRST14"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst14(&mut self) -> PBRST14_W<30> {
        PBRST14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBSRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbsrr](index.html) module"]
pub struct PBSRR_SPEC;
impl crate::RegisterSpec for PBSRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbsrr::R](R) reader structure"]
impl crate::Readable for PBSRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbsrr::W](W) writer structure"]
impl crate::Writable for PBSRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBSRR to value 0"]
impl crate::Resettable for PBSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
