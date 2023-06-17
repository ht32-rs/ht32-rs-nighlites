#[doc = "Register `PADIRCR` reader"]
pub struct R(crate::R<PADIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADIRCR` writer"]
pub struct W(crate::W<PADIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADIRCR_SPEC>;
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
impl From<crate::W<PADIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADIR0` reader - PADIR0"]
pub type PADIR0_R = crate::BitReader;
#[doc = "Field `PADIR0` writer - PADIR0"]
pub type PADIR0_W<'a, const O: u8> = crate::BitWriter<'a, PADIRCR_SPEC, O>;
#[doc = "Field `PADIR1` reader - PADIR1"]
pub type PADIR1_R = crate::BitReader;
#[doc = "Field `PADIR1` writer - PADIR1"]
pub type PADIR1_W<'a, const O: u8> = crate::BitWriter<'a, PADIRCR_SPEC, O>;
#[doc = "Field `PADIR2` reader - PADIR2"]
pub type PADIR2_R = crate::BitReader;
#[doc = "Field `PADIR2` writer - PADIR2"]
pub type PADIR2_W<'a, const O: u8> = crate::BitWriter<'a, PADIRCR_SPEC, O>;
#[doc = "Field `PADIR3` reader - PADIR3"]
pub type PADIR3_R = crate::BitReader;
#[doc = "Field `PADIR3` writer - PADIR3"]
pub type PADIR3_W<'a, const O: u8> = crate::BitWriter<'a, PADIRCR_SPEC, O>;
#[doc = "Field `PADIR4` reader - PADIR4"]
pub type PADIR4_R = crate::BitReader;
#[doc = "Field `PADIR4` writer - PADIR4"]
pub type PADIR4_W<'a, const O: u8> = crate::BitWriter<'a, PADIRCR_SPEC, O>;
#[doc = "Field `PADIR5` reader - PADIR5"]
pub type PADIR5_R = crate::BitReader;
#[doc = "Field `PADIR5` writer - PADIR5"]
pub type PADIR5_W<'a, const O: u8> = crate::BitWriter<'a, PADIRCR_SPEC, O>;
#[doc = "Field `PADIR6` reader - PADIR6"]
pub type PADIR6_R = crate::BitReader;
#[doc = "Field `PADIR6` writer - PADIR6"]
pub type PADIR6_W<'a, const O: u8> = crate::BitWriter<'a, PADIRCR_SPEC, O>;
#[doc = "Field `PADIR7` reader - PADIR7"]
pub type PADIR7_R = crate::BitReader;
#[doc = "Field `PADIR7` writer - PADIR7"]
pub type PADIR7_W<'a, const O: u8> = crate::BitWriter<'a, PADIRCR_SPEC, O>;
#[doc = "Field `PADIR8` reader - PADIR8"]
pub type PADIR8_R = crate::BitReader;
#[doc = "Field `PADIR8` writer - PADIR8"]
pub type PADIR8_W<'a, const O: u8> = crate::BitWriter<'a, PADIRCR_SPEC, O>;
#[doc = "Field `PADIR9` reader - PADIR9"]
pub type PADIR9_R = crate::BitReader;
#[doc = "Field `PADIR9` writer - PADIR9"]
pub type PADIR9_W<'a, const O: u8> = crate::BitWriter<'a, PADIRCR_SPEC, O>;
#[doc = "Field `PADIR10` reader - PADIR10"]
pub type PADIR10_R = crate::BitReader;
#[doc = "Field `PADIR10` writer - PADIR10"]
pub type PADIR10_W<'a, const O: u8> = crate::BitWriter<'a, PADIRCR_SPEC, O>;
#[doc = "Field `PADIR11` reader - PADIR11"]
pub type PADIR11_R = crate::BitReader;
#[doc = "Field `PADIR11` writer - PADIR11"]
pub type PADIR11_W<'a, const O: u8> = crate::BitWriter<'a, PADIRCR_SPEC, O>;
#[doc = "Field `PADIR12` reader - PADIR12"]
pub type PADIR12_R = crate::BitReader;
#[doc = "Field `PADIR12` writer - PADIR12"]
pub type PADIR12_W<'a, const O: u8> = crate::BitWriter<'a, PADIRCR_SPEC, O>;
#[doc = "Field `PADIR13` reader - PADIR13"]
pub type PADIR13_R = crate::BitReader;
#[doc = "Field `PADIR13` writer - PADIR13"]
pub type PADIR13_W<'a, const O: u8> = crate::BitWriter<'a, PADIRCR_SPEC, O>;
#[doc = "Field `PADIR14` reader - PADIR14"]
pub type PADIR14_R = crate::BitReader;
#[doc = "Field `PADIR14` writer - PADIR14"]
pub type PADIR14_W<'a, const O: u8> = crate::BitWriter<'a, PADIRCR_SPEC, O>;
#[doc = "Field `PADIR15` reader - PADIR15"]
pub type PADIR15_R = crate::BitReader;
#[doc = "Field `PADIR15` writer - PADIR15"]
pub type PADIR15_W<'a, const O: u8> = crate::BitWriter<'a, PADIRCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PADIR0"]
    #[inline(always)]
    pub fn padir0(&self) -> PADIR0_R {
        PADIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PADIR1"]
    #[inline(always)]
    pub fn padir1(&self) -> PADIR1_R {
        PADIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PADIR2"]
    #[inline(always)]
    pub fn padir2(&self) -> PADIR2_R {
        PADIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PADIR3"]
    #[inline(always)]
    pub fn padir3(&self) -> PADIR3_R {
        PADIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PADIR4"]
    #[inline(always)]
    pub fn padir4(&self) -> PADIR4_R {
        PADIR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PADIR5"]
    #[inline(always)]
    pub fn padir5(&self) -> PADIR5_R {
        PADIR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PADIR6"]
    #[inline(always)]
    pub fn padir6(&self) -> PADIR6_R {
        PADIR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PADIR7"]
    #[inline(always)]
    pub fn padir7(&self) -> PADIR7_R {
        PADIR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PADIR8"]
    #[inline(always)]
    pub fn padir8(&self) -> PADIR8_R {
        PADIR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PADIR9"]
    #[inline(always)]
    pub fn padir9(&self) -> PADIR9_R {
        PADIR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PADIR10"]
    #[inline(always)]
    pub fn padir10(&self) -> PADIR10_R {
        PADIR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PADIR11"]
    #[inline(always)]
    pub fn padir11(&self) -> PADIR11_R {
        PADIR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PADIR12"]
    #[inline(always)]
    pub fn padir12(&self) -> PADIR12_R {
        PADIR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PADIR13"]
    #[inline(always)]
    pub fn padir13(&self) -> PADIR13_R {
        PADIR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PADIR14"]
    #[inline(always)]
    pub fn padir14(&self) -> PADIR14_R {
        PADIR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PADIR15"]
    #[inline(always)]
    pub fn padir15(&self) -> PADIR15_R {
        PADIR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PADIR0"]
    #[inline(always)]
    #[must_use]
    pub fn padir0(&mut self) -> PADIR0_W<0> {
        PADIR0_W::new(self)
    }
    #[doc = "Bit 1 - PADIR1"]
    #[inline(always)]
    #[must_use]
    pub fn padir1(&mut self) -> PADIR1_W<1> {
        PADIR1_W::new(self)
    }
    #[doc = "Bit 2 - PADIR2"]
    #[inline(always)]
    #[must_use]
    pub fn padir2(&mut self) -> PADIR2_W<2> {
        PADIR2_W::new(self)
    }
    #[doc = "Bit 3 - PADIR3"]
    #[inline(always)]
    #[must_use]
    pub fn padir3(&mut self) -> PADIR3_W<3> {
        PADIR3_W::new(self)
    }
    #[doc = "Bit 4 - PADIR4"]
    #[inline(always)]
    #[must_use]
    pub fn padir4(&mut self) -> PADIR4_W<4> {
        PADIR4_W::new(self)
    }
    #[doc = "Bit 5 - PADIR5"]
    #[inline(always)]
    #[must_use]
    pub fn padir5(&mut self) -> PADIR5_W<5> {
        PADIR5_W::new(self)
    }
    #[doc = "Bit 6 - PADIR6"]
    #[inline(always)]
    #[must_use]
    pub fn padir6(&mut self) -> PADIR6_W<6> {
        PADIR6_W::new(self)
    }
    #[doc = "Bit 7 - PADIR7"]
    #[inline(always)]
    #[must_use]
    pub fn padir7(&mut self) -> PADIR7_W<7> {
        PADIR7_W::new(self)
    }
    #[doc = "Bit 8 - PADIR8"]
    #[inline(always)]
    #[must_use]
    pub fn padir8(&mut self) -> PADIR8_W<8> {
        PADIR8_W::new(self)
    }
    #[doc = "Bit 9 - PADIR9"]
    #[inline(always)]
    #[must_use]
    pub fn padir9(&mut self) -> PADIR9_W<9> {
        PADIR9_W::new(self)
    }
    #[doc = "Bit 10 - PADIR10"]
    #[inline(always)]
    #[must_use]
    pub fn padir10(&mut self) -> PADIR10_W<10> {
        PADIR10_W::new(self)
    }
    #[doc = "Bit 11 - PADIR11"]
    #[inline(always)]
    #[must_use]
    pub fn padir11(&mut self) -> PADIR11_W<11> {
        PADIR11_W::new(self)
    }
    #[doc = "Bit 12 - PADIR12"]
    #[inline(always)]
    #[must_use]
    pub fn padir12(&mut self) -> PADIR12_W<12> {
        PADIR12_W::new(self)
    }
    #[doc = "Bit 13 - PADIR13"]
    #[inline(always)]
    #[must_use]
    pub fn padir13(&mut self) -> PADIR13_W<13> {
        PADIR13_W::new(self)
    }
    #[doc = "Bit 14 - PADIR14"]
    #[inline(always)]
    #[must_use]
    pub fn padir14(&mut self) -> PADIR14_W<14> {
        PADIR14_W::new(self)
    }
    #[doc = "Bit 15 - PADIR15"]
    #[inline(always)]
    #[must_use]
    pub fn padir15(&mut self) -> PADIR15_W<15> {
        PADIR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PADIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padircr](index.html) module"]
pub struct PADIRCR_SPEC;
impl crate::RegisterSpec for PADIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padircr::R](R) reader structure"]
impl crate::Readable for PADIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padircr::W](W) writer structure"]
impl crate::Writable for PADIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADIRCR to value 0"]
impl crate::Resettable for PADIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
