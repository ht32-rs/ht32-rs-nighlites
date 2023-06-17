#[doc = "Register `PARR` reader"]
pub struct R(crate::R<PARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PARR` writer"]
pub struct W(crate::W<PARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PARR_SPEC>;
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
impl From<crate::W<PARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PARR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARST0` reader - PARST0"]
pub type PARST0_R = crate::BitReader;
#[doc = "Field `PARST0` writer - PARST0"]
pub type PARST0_W<'a, const O: u8> = crate::BitWriter<'a, PARR_SPEC, O>;
#[doc = "Field `PARST1` reader - PARST1"]
pub type PARST1_R = crate::BitReader;
#[doc = "Field `PARST1` writer - PARST1"]
pub type PARST1_W<'a, const O: u8> = crate::BitWriter<'a, PARR_SPEC, O>;
#[doc = "Field `PARST2` reader - PARST2"]
pub type PARST2_R = crate::BitReader;
#[doc = "Field `PARST2` writer - PARST2"]
pub type PARST2_W<'a, const O: u8> = crate::BitWriter<'a, PARR_SPEC, O>;
#[doc = "Field `PARST3` reader - PARST3"]
pub type PARST3_R = crate::BitReader;
#[doc = "Field `PARST3` writer - PARST3"]
pub type PARST3_W<'a, const O: u8> = crate::BitWriter<'a, PARR_SPEC, O>;
#[doc = "Field `PARST4` reader - PARST4"]
pub type PARST4_R = crate::BitReader;
#[doc = "Field `PARST4` writer - PARST4"]
pub type PARST4_W<'a, const O: u8> = crate::BitWriter<'a, PARR_SPEC, O>;
#[doc = "Field `PARST5` reader - PARST5"]
pub type PARST5_R = crate::BitReader;
#[doc = "Field `PARST5` writer - PARST5"]
pub type PARST5_W<'a, const O: u8> = crate::BitWriter<'a, PARR_SPEC, O>;
#[doc = "Field `PARST6` reader - PARST6"]
pub type PARST6_R = crate::BitReader;
#[doc = "Field `PARST6` writer - PARST6"]
pub type PARST6_W<'a, const O: u8> = crate::BitWriter<'a, PARR_SPEC, O>;
#[doc = "Field `PARST7` reader - PARST7"]
pub type PARST7_R = crate::BitReader;
#[doc = "Field `PARST7` writer - PARST7"]
pub type PARST7_W<'a, const O: u8> = crate::BitWriter<'a, PARR_SPEC, O>;
#[doc = "Field `PARST8` reader - PARST8"]
pub type PARST8_R = crate::BitReader;
#[doc = "Field `PARST8` writer - PARST8"]
pub type PARST8_W<'a, const O: u8> = crate::BitWriter<'a, PARR_SPEC, O>;
#[doc = "Field `PARST9` reader - PARST9"]
pub type PARST9_R = crate::BitReader;
#[doc = "Field `PARST9` writer - PARST9"]
pub type PARST9_W<'a, const O: u8> = crate::BitWriter<'a, PARR_SPEC, O>;
#[doc = "Field `PARST10` reader - PARST10"]
pub type PARST10_R = crate::BitReader;
#[doc = "Field `PARST10` writer - PARST10"]
pub type PARST10_W<'a, const O: u8> = crate::BitWriter<'a, PARR_SPEC, O>;
#[doc = "Field `PARST11` reader - PARST11"]
pub type PARST11_R = crate::BitReader;
#[doc = "Field `PARST11` writer - PARST11"]
pub type PARST11_W<'a, const O: u8> = crate::BitWriter<'a, PARR_SPEC, O>;
#[doc = "Field `PARST12` reader - PARST12"]
pub type PARST12_R = crate::BitReader;
#[doc = "Field `PARST12` writer - PARST12"]
pub type PARST12_W<'a, const O: u8> = crate::BitWriter<'a, PARR_SPEC, O>;
#[doc = "Field `PARST13` reader - PARST13"]
pub type PARST13_R = crate::BitReader;
#[doc = "Field `PARST13` writer - PARST13"]
pub type PARST13_W<'a, const O: u8> = crate::BitWriter<'a, PARR_SPEC, O>;
#[doc = "Field `PARST14` reader - PARST14"]
pub type PARST14_R = crate::BitReader;
#[doc = "Field `PARST14` writer - PARST14"]
pub type PARST14_W<'a, const O: u8> = crate::BitWriter<'a, PARR_SPEC, O>;
#[doc = "Field `PARST15` reader - PARST15"]
pub type PARST15_R = crate::BitReader;
#[doc = "Field `PARST15` writer - PARST15"]
pub type PARST15_W<'a, const O: u8> = crate::BitWriter<'a, PARR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PARST0"]
    #[inline(always)]
    pub fn parst0(&self) -> PARST0_R {
        PARST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PARST1"]
    #[inline(always)]
    pub fn parst1(&self) -> PARST1_R {
        PARST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PARST2"]
    #[inline(always)]
    pub fn parst2(&self) -> PARST2_R {
        PARST2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PARST3"]
    #[inline(always)]
    pub fn parst3(&self) -> PARST3_R {
        PARST3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PARST4"]
    #[inline(always)]
    pub fn parst4(&self) -> PARST4_R {
        PARST4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PARST5"]
    #[inline(always)]
    pub fn parst5(&self) -> PARST5_R {
        PARST5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PARST6"]
    #[inline(always)]
    pub fn parst6(&self) -> PARST6_R {
        PARST6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PARST7"]
    #[inline(always)]
    pub fn parst7(&self) -> PARST7_R {
        PARST7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PARST8"]
    #[inline(always)]
    pub fn parst8(&self) -> PARST8_R {
        PARST8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PARST9"]
    #[inline(always)]
    pub fn parst9(&self) -> PARST9_R {
        PARST9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PARST10"]
    #[inline(always)]
    pub fn parst10(&self) -> PARST10_R {
        PARST10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PARST11"]
    #[inline(always)]
    pub fn parst11(&self) -> PARST11_R {
        PARST11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PARST12"]
    #[inline(always)]
    pub fn parst12(&self) -> PARST12_R {
        PARST12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PARST13"]
    #[inline(always)]
    pub fn parst13(&self) -> PARST13_R {
        PARST13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PARST14"]
    #[inline(always)]
    pub fn parst14(&self) -> PARST14_R {
        PARST14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PARST15"]
    #[inline(always)]
    pub fn parst15(&self) -> PARST15_R {
        PARST15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PARST0"]
    #[inline(always)]
    #[must_use]
    pub fn parst0(&mut self) -> PARST0_W<0> {
        PARST0_W::new(self)
    }
    #[doc = "Bit 1 - PARST1"]
    #[inline(always)]
    #[must_use]
    pub fn parst1(&mut self) -> PARST1_W<1> {
        PARST1_W::new(self)
    }
    #[doc = "Bit 2 - PARST2"]
    #[inline(always)]
    #[must_use]
    pub fn parst2(&mut self) -> PARST2_W<2> {
        PARST2_W::new(self)
    }
    #[doc = "Bit 3 - PARST3"]
    #[inline(always)]
    #[must_use]
    pub fn parst3(&mut self) -> PARST3_W<3> {
        PARST3_W::new(self)
    }
    #[doc = "Bit 4 - PARST4"]
    #[inline(always)]
    #[must_use]
    pub fn parst4(&mut self) -> PARST4_W<4> {
        PARST4_W::new(self)
    }
    #[doc = "Bit 5 - PARST5"]
    #[inline(always)]
    #[must_use]
    pub fn parst5(&mut self) -> PARST5_W<5> {
        PARST5_W::new(self)
    }
    #[doc = "Bit 6 - PARST6"]
    #[inline(always)]
    #[must_use]
    pub fn parst6(&mut self) -> PARST6_W<6> {
        PARST6_W::new(self)
    }
    #[doc = "Bit 7 - PARST7"]
    #[inline(always)]
    #[must_use]
    pub fn parst7(&mut self) -> PARST7_W<7> {
        PARST7_W::new(self)
    }
    #[doc = "Bit 8 - PARST8"]
    #[inline(always)]
    #[must_use]
    pub fn parst8(&mut self) -> PARST8_W<8> {
        PARST8_W::new(self)
    }
    #[doc = "Bit 9 - PARST9"]
    #[inline(always)]
    #[must_use]
    pub fn parst9(&mut self) -> PARST9_W<9> {
        PARST9_W::new(self)
    }
    #[doc = "Bit 10 - PARST10"]
    #[inline(always)]
    #[must_use]
    pub fn parst10(&mut self) -> PARST10_W<10> {
        PARST10_W::new(self)
    }
    #[doc = "Bit 11 - PARST11"]
    #[inline(always)]
    #[must_use]
    pub fn parst11(&mut self) -> PARST11_W<11> {
        PARST11_W::new(self)
    }
    #[doc = "Bit 12 - PARST12"]
    #[inline(always)]
    #[must_use]
    pub fn parst12(&mut self) -> PARST12_W<12> {
        PARST12_W::new(self)
    }
    #[doc = "Bit 13 - PARST13"]
    #[inline(always)]
    #[must_use]
    pub fn parst13(&mut self) -> PARST13_W<13> {
        PARST13_W::new(self)
    }
    #[doc = "Bit 14 - PARST14"]
    #[inline(always)]
    #[must_use]
    pub fn parst14(&mut self) -> PARST14_W<14> {
        PARST14_W::new(self)
    }
    #[doc = "Bit 15 - PARST15"]
    #[inline(always)]
    #[must_use]
    pub fn parst15(&mut self) -> PARST15_W<15> {
        PARST15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PARR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [parr](index.html) module"]
pub struct PARR_SPEC;
impl crate::RegisterSpec for PARR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [parr::R](R) reader structure"]
impl crate::Readable for PARR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [parr::W](W) writer structure"]
impl crate::Writable for PARR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PARR to value 0"]
impl crate::Resettable for PARR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
