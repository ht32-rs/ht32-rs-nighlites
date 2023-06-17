#[doc = "Register `PCRR` reader"]
pub struct R(crate::R<PCRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCRR` writer"]
pub struct W(crate::W<PCRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCRR_SPEC>;
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
impl From<crate::W<PCRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCRST0` reader - PCRST0"]
pub type PCRST0_R = crate::BitReader;
#[doc = "Field `PCRST0` writer - PCRST0"]
pub type PCRST0_W<'a, const O: u8> = crate::BitWriter<'a, PCRR_SPEC, O>;
#[doc = "Field `PCRST1` reader - PCRST1"]
pub type PCRST1_R = crate::BitReader;
#[doc = "Field `PCRST1` writer - PCRST1"]
pub type PCRST1_W<'a, const O: u8> = crate::BitWriter<'a, PCRR_SPEC, O>;
#[doc = "Field `PCRST2` reader - PCRST2"]
pub type PCRST2_R = crate::BitReader;
#[doc = "Field `PCRST2` writer - PCRST2"]
pub type PCRST2_W<'a, const O: u8> = crate::BitWriter<'a, PCRR_SPEC, O>;
#[doc = "Field `PCRST3` reader - PCRST3"]
pub type PCRST3_R = crate::BitReader;
#[doc = "Field `PCRST3` writer - PCRST3"]
pub type PCRST3_W<'a, const O: u8> = crate::BitWriter<'a, PCRR_SPEC, O>;
#[doc = "Field `PCRST4` reader - PCRST4"]
pub type PCRST4_R = crate::BitReader;
#[doc = "Field `PCRST4` writer - PCRST4"]
pub type PCRST4_W<'a, const O: u8> = crate::BitWriter<'a, PCRR_SPEC, O>;
#[doc = "Field `PCRST5` reader - PCRST5"]
pub type PCRST5_R = crate::BitReader;
#[doc = "Field `PCRST5` writer - PCRST5"]
pub type PCRST5_W<'a, const O: u8> = crate::BitWriter<'a, PCRR_SPEC, O>;
#[doc = "Field `PCRST6` reader - PCRST6"]
pub type PCRST6_R = crate::BitReader;
#[doc = "Field `PCRST6` writer - PCRST6"]
pub type PCRST6_W<'a, const O: u8> = crate::BitWriter<'a, PCRR_SPEC, O>;
#[doc = "Field `PCRST7` reader - PCRST7"]
pub type PCRST7_R = crate::BitReader;
#[doc = "Field `PCRST7` writer - PCRST7"]
pub type PCRST7_W<'a, const O: u8> = crate::BitWriter<'a, PCRR_SPEC, O>;
#[doc = "Field `PCRST8` reader - PCRST8"]
pub type PCRST8_R = crate::BitReader;
#[doc = "Field `PCRST8` writer - PCRST8"]
pub type PCRST8_W<'a, const O: u8> = crate::BitWriter<'a, PCRR_SPEC, O>;
#[doc = "Field `PCRST9` reader - PCRST9"]
pub type PCRST9_R = crate::BitReader;
#[doc = "Field `PCRST9` writer - PCRST9"]
pub type PCRST9_W<'a, const O: u8> = crate::BitWriter<'a, PCRR_SPEC, O>;
#[doc = "Field `PCRST10` reader - PCRST10"]
pub type PCRST10_R = crate::BitReader;
#[doc = "Field `PCRST10` writer - PCRST10"]
pub type PCRST10_W<'a, const O: u8> = crate::BitWriter<'a, PCRR_SPEC, O>;
#[doc = "Field `PCRST11` reader - PCRST11"]
pub type PCRST11_R = crate::BitReader;
#[doc = "Field `PCRST11` writer - PCRST11"]
pub type PCRST11_W<'a, const O: u8> = crate::BitWriter<'a, PCRR_SPEC, O>;
#[doc = "Field `PCRST12` reader - PCRST12"]
pub type PCRST12_R = crate::BitReader;
#[doc = "Field `PCRST12` writer - PCRST12"]
pub type PCRST12_W<'a, const O: u8> = crate::BitWriter<'a, PCRR_SPEC, O>;
#[doc = "Field `PCRST13` reader - PCRST13"]
pub type PCRST13_R = crate::BitReader;
#[doc = "Field `PCRST13` writer - PCRST13"]
pub type PCRST13_W<'a, const O: u8> = crate::BitWriter<'a, PCRR_SPEC, O>;
#[doc = "Field `PCRST14` reader - PCRST14"]
pub type PCRST14_R = crate::BitReader;
#[doc = "Field `PCRST14` writer - PCRST14"]
pub type PCRST14_W<'a, const O: u8> = crate::BitWriter<'a, PCRR_SPEC, O>;
#[doc = "Field `PCRST15` reader - PCRST15"]
pub type PCRST15_R = crate::BitReader;
#[doc = "Field `PCRST15` writer - PCRST15"]
pub type PCRST15_W<'a, const O: u8> = crate::BitWriter<'a, PCRR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PCRST0"]
    #[inline(always)]
    pub fn pcrst0(&self) -> PCRST0_R {
        PCRST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCRST1"]
    #[inline(always)]
    pub fn pcrst1(&self) -> PCRST1_R {
        PCRST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCRST2"]
    #[inline(always)]
    pub fn pcrst2(&self) -> PCRST2_R {
        PCRST2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCRST3"]
    #[inline(always)]
    pub fn pcrst3(&self) -> PCRST3_R {
        PCRST3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PCRST4"]
    #[inline(always)]
    pub fn pcrst4(&self) -> PCRST4_R {
        PCRST4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCRST5"]
    #[inline(always)]
    pub fn pcrst5(&self) -> PCRST5_R {
        PCRST5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCRST6"]
    #[inline(always)]
    pub fn pcrst6(&self) -> PCRST6_R {
        PCRST6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PCRST7"]
    #[inline(always)]
    pub fn pcrst7(&self) -> PCRST7_R {
        PCRST7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PCRST8"]
    #[inline(always)]
    pub fn pcrst8(&self) -> PCRST8_R {
        PCRST8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PCRST9"]
    #[inline(always)]
    pub fn pcrst9(&self) -> PCRST9_R {
        PCRST9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCRST10"]
    #[inline(always)]
    pub fn pcrst10(&self) -> PCRST10_R {
        PCRST10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PCRST11"]
    #[inline(always)]
    pub fn pcrst11(&self) -> PCRST11_R {
        PCRST11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PCRST12"]
    #[inline(always)]
    pub fn pcrst12(&self) -> PCRST12_R {
        PCRST12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PCRST13"]
    #[inline(always)]
    pub fn pcrst13(&self) -> PCRST13_R {
        PCRST13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PCRST14"]
    #[inline(always)]
    pub fn pcrst14(&self) -> PCRST14_R {
        PCRST14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PCRST15"]
    #[inline(always)]
    pub fn pcrst15(&self) -> PCRST15_R {
        PCRST15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCRST0"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst0(&mut self) -> PCRST0_W<0> {
        PCRST0_W::new(self)
    }
    #[doc = "Bit 1 - PCRST1"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst1(&mut self) -> PCRST1_W<1> {
        PCRST1_W::new(self)
    }
    #[doc = "Bit 2 - PCRST2"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst2(&mut self) -> PCRST2_W<2> {
        PCRST2_W::new(self)
    }
    #[doc = "Bit 3 - PCRST3"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst3(&mut self) -> PCRST3_W<3> {
        PCRST3_W::new(self)
    }
    #[doc = "Bit 4 - PCRST4"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst4(&mut self) -> PCRST4_W<4> {
        PCRST4_W::new(self)
    }
    #[doc = "Bit 5 - PCRST5"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst5(&mut self) -> PCRST5_W<5> {
        PCRST5_W::new(self)
    }
    #[doc = "Bit 6 - PCRST6"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst6(&mut self) -> PCRST6_W<6> {
        PCRST6_W::new(self)
    }
    #[doc = "Bit 7 - PCRST7"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst7(&mut self) -> PCRST7_W<7> {
        PCRST7_W::new(self)
    }
    #[doc = "Bit 8 - PCRST8"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst8(&mut self) -> PCRST8_W<8> {
        PCRST8_W::new(self)
    }
    #[doc = "Bit 9 - PCRST9"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst9(&mut self) -> PCRST9_W<9> {
        PCRST9_W::new(self)
    }
    #[doc = "Bit 10 - PCRST10"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst10(&mut self) -> PCRST10_W<10> {
        PCRST10_W::new(self)
    }
    #[doc = "Bit 11 - PCRST11"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst11(&mut self) -> PCRST11_W<11> {
        PCRST11_W::new(self)
    }
    #[doc = "Bit 12 - PCRST12"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst12(&mut self) -> PCRST12_W<12> {
        PCRST12_W::new(self)
    }
    #[doc = "Bit 13 - PCRST13"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst13(&mut self) -> PCRST13_W<13> {
        PCRST13_W::new(self)
    }
    #[doc = "Bit 14 - PCRST14"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst14(&mut self) -> PCRST14_W<14> {
        PCRST14_W::new(self)
    }
    #[doc = "Bit 15 - PCRST15"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst15(&mut self) -> PCRST15_W<15> {
        PCRST15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrr](index.html) module"]
pub struct PCRR_SPEC;
impl crate::RegisterSpec for PCRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcrr::R](R) reader structure"]
impl crate::Readable for PCRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcrr::W](W) writer structure"]
impl crate::Writable for PCRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCRR to value 0"]
impl crate::Resettable for PCRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
