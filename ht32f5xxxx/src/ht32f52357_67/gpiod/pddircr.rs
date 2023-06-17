#[doc = "Register `PDDIRCR` reader"]
pub struct R(crate::R<PDDIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDDIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDDIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDDIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDDIRCR` writer"]
pub struct W(crate::W<PDDIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDDIRCR_SPEC>;
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
impl From<crate::W<PDDIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDDIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDDIR0` reader - PDDIR0"]
pub type PDDIR0_R = crate::BitReader;
#[doc = "Field `PDDIR0` writer - PDDIR0"]
pub type PDDIR0_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR1` reader - PDDIR1"]
pub type PDDIR1_R = crate::BitReader;
#[doc = "Field `PDDIR1` writer - PDDIR1"]
pub type PDDIR1_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR2` reader - PDDIR2"]
pub type PDDIR2_R = crate::BitReader;
#[doc = "Field `PDDIR2` writer - PDDIR2"]
pub type PDDIR2_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR3` reader - PDDIR3"]
pub type PDDIR3_R = crate::BitReader;
#[doc = "Field `PDDIR3` writer - PDDIR3"]
pub type PDDIR3_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR4` reader - PDDIR4"]
pub type PDDIR4_R = crate::BitReader;
#[doc = "Field `PDDIR4` writer - PDDIR4"]
pub type PDDIR4_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR5` reader - PDDIR5"]
pub type PDDIR5_R = crate::BitReader;
#[doc = "Field `PDDIR5` writer - PDDIR5"]
pub type PDDIR5_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR6` reader - PDDIR6"]
pub type PDDIR6_R = crate::BitReader;
#[doc = "Field `PDDIR6` writer - PDDIR6"]
pub type PDDIR6_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR7` reader - PDDIR7"]
pub type PDDIR7_R = crate::BitReader;
#[doc = "Field `PDDIR7` writer - PDDIR7"]
pub type PDDIR7_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR8` reader - PDDIR8"]
pub type PDDIR8_R = crate::BitReader;
#[doc = "Field `PDDIR8` writer - PDDIR8"]
pub type PDDIR8_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR9` reader - PDDIR9"]
pub type PDDIR9_R = crate::BitReader;
#[doc = "Field `PDDIR9` writer - PDDIR9"]
pub type PDDIR9_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR10` reader - PDDIR10"]
pub type PDDIR10_R = crate::BitReader;
#[doc = "Field `PDDIR10` writer - PDDIR10"]
pub type PDDIR10_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR11` reader - PDDIR11"]
pub type PDDIR11_R = crate::BitReader;
#[doc = "Field `PDDIR11` writer - PDDIR11"]
pub type PDDIR11_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR12` reader - PDDIR12"]
pub type PDDIR12_R = crate::BitReader;
#[doc = "Field `PDDIR12` writer - PDDIR12"]
pub type PDDIR12_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR13` reader - PDDIR13"]
pub type PDDIR13_R = crate::BitReader;
#[doc = "Field `PDDIR13` writer - PDDIR13"]
pub type PDDIR13_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR14` reader - PDDIR14"]
pub type PDDIR14_R = crate::BitReader;
#[doc = "Field `PDDIR14` writer - PDDIR14"]
pub type PDDIR14_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR15` reader - PDDIR15"]
pub type PDDIR15_R = crate::BitReader;
#[doc = "Field `PDDIR15` writer - PDDIR15"]
pub type PDDIR15_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDDIR0"]
    #[inline(always)]
    pub fn pddir0(&self) -> PDDIR0_R {
        PDDIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDDIR1"]
    #[inline(always)]
    pub fn pddir1(&self) -> PDDIR1_R {
        PDDIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDDIR2"]
    #[inline(always)]
    pub fn pddir2(&self) -> PDDIR2_R {
        PDDIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDDIR3"]
    #[inline(always)]
    pub fn pddir3(&self) -> PDDIR3_R {
        PDDIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PDDIR4"]
    #[inline(always)]
    pub fn pddir4(&self) -> PDDIR4_R {
        PDDIR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PDDIR5"]
    #[inline(always)]
    pub fn pddir5(&self) -> PDDIR5_R {
        PDDIR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDDIR6"]
    #[inline(always)]
    pub fn pddir6(&self) -> PDDIR6_R {
        PDDIR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDDIR7"]
    #[inline(always)]
    pub fn pddir7(&self) -> PDDIR7_R {
        PDDIR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PDDIR8"]
    #[inline(always)]
    pub fn pddir8(&self) -> PDDIR8_R {
        PDDIR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PDDIR9"]
    #[inline(always)]
    pub fn pddir9(&self) -> PDDIR9_R {
        PDDIR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PDDIR10"]
    #[inline(always)]
    pub fn pddir10(&self) -> PDDIR10_R {
        PDDIR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PDDIR11"]
    #[inline(always)]
    pub fn pddir11(&self) -> PDDIR11_R {
        PDDIR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PDDIR12"]
    #[inline(always)]
    pub fn pddir12(&self) -> PDDIR12_R {
        PDDIR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PDDIR13"]
    #[inline(always)]
    pub fn pddir13(&self) -> PDDIR13_R {
        PDDIR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PDDIR14"]
    #[inline(always)]
    pub fn pddir14(&self) -> PDDIR14_R {
        PDDIR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PDDIR15"]
    #[inline(always)]
    pub fn pddir15(&self) -> PDDIR15_R {
        PDDIR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDDIR0"]
    #[inline(always)]
    #[must_use]
    pub fn pddir0(&mut self) -> PDDIR0_W<0> {
        PDDIR0_W::new(self)
    }
    #[doc = "Bit 1 - PDDIR1"]
    #[inline(always)]
    #[must_use]
    pub fn pddir1(&mut self) -> PDDIR1_W<1> {
        PDDIR1_W::new(self)
    }
    #[doc = "Bit 2 - PDDIR2"]
    #[inline(always)]
    #[must_use]
    pub fn pddir2(&mut self) -> PDDIR2_W<2> {
        PDDIR2_W::new(self)
    }
    #[doc = "Bit 3 - PDDIR3"]
    #[inline(always)]
    #[must_use]
    pub fn pddir3(&mut self) -> PDDIR3_W<3> {
        PDDIR3_W::new(self)
    }
    #[doc = "Bit 4 - PDDIR4"]
    #[inline(always)]
    #[must_use]
    pub fn pddir4(&mut self) -> PDDIR4_W<4> {
        PDDIR4_W::new(self)
    }
    #[doc = "Bit 5 - PDDIR5"]
    #[inline(always)]
    #[must_use]
    pub fn pddir5(&mut self) -> PDDIR5_W<5> {
        PDDIR5_W::new(self)
    }
    #[doc = "Bit 6 - PDDIR6"]
    #[inline(always)]
    #[must_use]
    pub fn pddir6(&mut self) -> PDDIR6_W<6> {
        PDDIR6_W::new(self)
    }
    #[doc = "Bit 7 - PDDIR7"]
    #[inline(always)]
    #[must_use]
    pub fn pddir7(&mut self) -> PDDIR7_W<7> {
        PDDIR7_W::new(self)
    }
    #[doc = "Bit 8 - PDDIR8"]
    #[inline(always)]
    #[must_use]
    pub fn pddir8(&mut self) -> PDDIR8_W<8> {
        PDDIR8_W::new(self)
    }
    #[doc = "Bit 9 - PDDIR9"]
    #[inline(always)]
    #[must_use]
    pub fn pddir9(&mut self) -> PDDIR9_W<9> {
        PDDIR9_W::new(self)
    }
    #[doc = "Bit 10 - PDDIR10"]
    #[inline(always)]
    #[must_use]
    pub fn pddir10(&mut self) -> PDDIR10_W<10> {
        PDDIR10_W::new(self)
    }
    #[doc = "Bit 11 - PDDIR11"]
    #[inline(always)]
    #[must_use]
    pub fn pddir11(&mut self) -> PDDIR11_W<11> {
        PDDIR11_W::new(self)
    }
    #[doc = "Bit 12 - PDDIR12"]
    #[inline(always)]
    #[must_use]
    pub fn pddir12(&mut self) -> PDDIR12_W<12> {
        PDDIR12_W::new(self)
    }
    #[doc = "Bit 13 - PDDIR13"]
    #[inline(always)]
    #[must_use]
    pub fn pddir13(&mut self) -> PDDIR13_W<13> {
        PDDIR13_W::new(self)
    }
    #[doc = "Bit 14 - PDDIR14"]
    #[inline(always)]
    #[must_use]
    pub fn pddir14(&mut self) -> PDDIR14_W<14> {
        PDDIR14_W::new(self)
    }
    #[doc = "Bit 15 - PDDIR15"]
    #[inline(always)]
    #[must_use]
    pub fn pddir15(&mut self) -> PDDIR15_W<15> {
        PDDIR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDDIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddircr](index.html) module"]
pub struct PDDIRCR_SPEC;
impl crate::RegisterSpec for PDDIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pddircr::R](R) reader structure"]
impl crate::Readable for PDDIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pddircr::W](W) writer structure"]
impl crate::Writable for PDDIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDDIRCR to value 0"]
impl crate::Resettable for PDDIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
