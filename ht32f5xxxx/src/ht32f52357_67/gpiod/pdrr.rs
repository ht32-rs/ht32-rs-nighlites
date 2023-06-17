#[doc = "Register `PDRR` reader"]
pub struct R(crate::R<PDRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDRR` writer"]
pub struct W(crate::W<PDRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDRR_SPEC>;
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
impl From<crate::W<PDRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDRST0` reader - PDRST0"]
pub type PDRST0_R = crate::BitReader;
#[doc = "Field `PDRST0` writer - PDRST0"]
pub type PDRST0_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST1` reader - PDRST1"]
pub type PDRST1_R = crate::BitReader;
#[doc = "Field `PDRST1` writer - PDRST1"]
pub type PDRST1_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST2` reader - PDRST2"]
pub type PDRST2_R = crate::BitReader;
#[doc = "Field `PDRST2` writer - PDRST2"]
pub type PDRST2_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST3` reader - PDRST3"]
pub type PDRST3_R = crate::BitReader;
#[doc = "Field `PDRST3` writer - PDRST3"]
pub type PDRST3_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST4` reader - PDRST4"]
pub type PDRST4_R = crate::BitReader;
#[doc = "Field `PDRST4` writer - PDRST4"]
pub type PDRST4_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST5` reader - PDRST5"]
pub type PDRST5_R = crate::BitReader;
#[doc = "Field `PDRST5` writer - PDRST5"]
pub type PDRST5_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST6` reader - PDRST6"]
pub type PDRST6_R = crate::BitReader;
#[doc = "Field `PDRST6` writer - PDRST6"]
pub type PDRST6_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST7` reader - PDRST7"]
pub type PDRST7_R = crate::BitReader;
#[doc = "Field `PDRST7` writer - PDRST7"]
pub type PDRST7_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST8` reader - PDRST8"]
pub type PDRST8_R = crate::BitReader;
#[doc = "Field `PDRST8` writer - PDRST8"]
pub type PDRST8_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST9` reader - PDRST9"]
pub type PDRST9_R = crate::BitReader;
#[doc = "Field `PDRST9` writer - PDRST9"]
pub type PDRST9_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST10` reader - PDRST10"]
pub type PDRST10_R = crate::BitReader;
#[doc = "Field `PDRST10` writer - PDRST10"]
pub type PDRST10_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST11` reader - PDRST11"]
pub type PDRST11_R = crate::BitReader;
#[doc = "Field `PDRST11` writer - PDRST11"]
pub type PDRST11_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST12` reader - PDRST12"]
pub type PDRST12_R = crate::BitReader;
#[doc = "Field `PDRST12` writer - PDRST12"]
pub type PDRST12_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST13` reader - PDRST13"]
pub type PDRST13_R = crate::BitReader;
#[doc = "Field `PDRST13` writer - PDRST13"]
pub type PDRST13_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST14` reader - PDRST14"]
pub type PDRST14_R = crate::BitReader;
#[doc = "Field `PDRST14` writer - PDRST14"]
pub type PDRST14_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST15` reader - PDRST15"]
pub type PDRST15_R = crate::BitReader;
#[doc = "Field `PDRST15` writer - PDRST15"]
pub type PDRST15_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDRST0"]
    #[inline(always)]
    pub fn pdrst0(&self) -> PDRST0_R {
        PDRST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDRST1"]
    #[inline(always)]
    pub fn pdrst1(&self) -> PDRST1_R {
        PDRST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDRST2"]
    #[inline(always)]
    pub fn pdrst2(&self) -> PDRST2_R {
        PDRST2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDRST3"]
    #[inline(always)]
    pub fn pdrst3(&self) -> PDRST3_R {
        PDRST3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PDRST4"]
    #[inline(always)]
    pub fn pdrst4(&self) -> PDRST4_R {
        PDRST4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PDRST5"]
    #[inline(always)]
    pub fn pdrst5(&self) -> PDRST5_R {
        PDRST5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDRST6"]
    #[inline(always)]
    pub fn pdrst6(&self) -> PDRST6_R {
        PDRST6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDRST7"]
    #[inline(always)]
    pub fn pdrst7(&self) -> PDRST7_R {
        PDRST7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PDRST8"]
    #[inline(always)]
    pub fn pdrst8(&self) -> PDRST8_R {
        PDRST8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PDRST9"]
    #[inline(always)]
    pub fn pdrst9(&self) -> PDRST9_R {
        PDRST9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PDRST10"]
    #[inline(always)]
    pub fn pdrst10(&self) -> PDRST10_R {
        PDRST10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PDRST11"]
    #[inline(always)]
    pub fn pdrst11(&self) -> PDRST11_R {
        PDRST11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PDRST12"]
    #[inline(always)]
    pub fn pdrst12(&self) -> PDRST12_R {
        PDRST12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PDRST13"]
    #[inline(always)]
    pub fn pdrst13(&self) -> PDRST13_R {
        PDRST13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PDRST14"]
    #[inline(always)]
    pub fn pdrst14(&self) -> PDRST14_R {
        PDRST14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PDRST15"]
    #[inline(always)]
    pub fn pdrst15(&self) -> PDRST15_R {
        PDRST15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDRST0"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst0(&mut self) -> PDRST0_W<0> {
        PDRST0_W::new(self)
    }
    #[doc = "Bit 1 - PDRST1"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst1(&mut self) -> PDRST1_W<1> {
        PDRST1_W::new(self)
    }
    #[doc = "Bit 2 - PDRST2"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst2(&mut self) -> PDRST2_W<2> {
        PDRST2_W::new(self)
    }
    #[doc = "Bit 3 - PDRST3"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst3(&mut self) -> PDRST3_W<3> {
        PDRST3_W::new(self)
    }
    #[doc = "Bit 4 - PDRST4"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst4(&mut self) -> PDRST4_W<4> {
        PDRST4_W::new(self)
    }
    #[doc = "Bit 5 - PDRST5"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst5(&mut self) -> PDRST5_W<5> {
        PDRST5_W::new(self)
    }
    #[doc = "Bit 6 - PDRST6"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst6(&mut self) -> PDRST6_W<6> {
        PDRST6_W::new(self)
    }
    #[doc = "Bit 7 - PDRST7"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst7(&mut self) -> PDRST7_W<7> {
        PDRST7_W::new(self)
    }
    #[doc = "Bit 8 - PDRST8"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst8(&mut self) -> PDRST8_W<8> {
        PDRST8_W::new(self)
    }
    #[doc = "Bit 9 - PDRST9"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst9(&mut self) -> PDRST9_W<9> {
        PDRST9_W::new(self)
    }
    #[doc = "Bit 10 - PDRST10"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst10(&mut self) -> PDRST10_W<10> {
        PDRST10_W::new(self)
    }
    #[doc = "Bit 11 - PDRST11"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst11(&mut self) -> PDRST11_W<11> {
        PDRST11_W::new(self)
    }
    #[doc = "Bit 12 - PDRST12"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst12(&mut self) -> PDRST12_W<12> {
        PDRST12_W::new(self)
    }
    #[doc = "Bit 13 - PDRST13"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst13(&mut self) -> PDRST13_W<13> {
        PDRST13_W::new(self)
    }
    #[doc = "Bit 14 - PDRST14"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst14(&mut self) -> PDRST14_W<14> {
        PDRST14_W::new(self)
    }
    #[doc = "Bit 15 - PDRST15"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst15(&mut self) -> PDRST15_W<15> {
        PDRST15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdrr](index.html) module"]
pub struct PDRR_SPEC;
impl crate::RegisterSpec for PDRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdrr::R](R) reader structure"]
impl crate::Readable for PDRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdrr::W](W) writer structure"]
impl crate::Writable for PDRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDRR to value 0"]
impl crate::Resettable for PDRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
