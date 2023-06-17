#[doc = "Register `PBDIRCR` reader"]
pub struct R(crate::R<PBDIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBDIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBDIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBDIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBDIRCR` writer"]
pub struct W(crate::W<PBDIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBDIRCR_SPEC>;
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
impl From<crate::W<PBDIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBDIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBDIR0` reader - PBDIR0"]
pub type PBDIR0_R = crate::BitReader;
#[doc = "Field `PBDIR0` writer - PBDIR0"]
pub type PBDIR0_W<'a, const O: u8> = crate::BitWriter<'a, PBDIRCR_SPEC, O>;
#[doc = "Field `PBDIR1` reader - PBDIR1"]
pub type PBDIR1_R = crate::BitReader;
#[doc = "Field `PBDIR1` writer - PBDIR1"]
pub type PBDIR1_W<'a, const O: u8> = crate::BitWriter<'a, PBDIRCR_SPEC, O>;
#[doc = "Field `PBDIR2` reader - PBDIR2"]
pub type PBDIR2_R = crate::BitReader;
#[doc = "Field `PBDIR2` writer - PBDIR2"]
pub type PBDIR2_W<'a, const O: u8> = crate::BitWriter<'a, PBDIRCR_SPEC, O>;
#[doc = "Field `PBDIR3` reader - PBDIR3"]
pub type PBDIR3_R = crate::BitReader;
#[doc = "Field `PBDIR3` writer - PBDIR3"]
pub type PBDIR3_W<'a, const O: u8> = crate::BitWriter<'a, PBDIRCR_SPEC, O>;
#[doc = "Field `PBDIR4` reader - PBDIR4"]
pub type PBDIR4_R = crate::BitReader;
#[doc = "Field `PBDIR4` writer - PBDIR4"]
pub type PBDIR4_W<'a, const O: u8> = crate::BitWriter<'a, PBDIRCR_SPEC, O>;
#[doc = "Field `PBDIR5` reader - PBDIR5"]
pub type PBDIR5_R = crate::BitReader;
#[doc = "Field `PBDIR5` writer - PBDIR5"]
pub type PBDIR5_W<'a, const O: u8> = crate::BitWriter<'a, PBDIRCR_SPEC, O>;
#[doc = "Field `PBDIR6` reader - PBDIR6"]
pub type PBDIR6_R = crate::BitReader;
#[doc = "Field `PBDIR6` writer - PBDIR6"]
pub type PBDIR6_W<'a, const O: u8> = crate::BitWriter<'a, PBDIRCR_SPEC, O>;
#[doc = "Field `PBDIR7` reader - PBDIR7"]
pub type PBDIR7_R = crate::BitReader;
#[doc = "Field `PBDIR7` writer - PBDIR7"]
pub type PBDIR7_W<'a, const O: u8> = crate::BitWriter<'a, PBDIRCR_SPEC, O>;
#[doc = "Field `PBDIR8` reader - PBDIR8"]
pub type PBDIR8_R = crate::BitReader;
#[doc = "Field `PBDIR8` writer - PBDIR8"]
pub type PBDIR8_W<'a, const O: u8> = crate::BitWriter<'a, PBDIRCR_SPEC, O>;
#[doc = "Field `PBDIR9` reader - PBDIR9"]
pub type PBDIR9_R = crate::BitReader;
#[doc = "Field `PBDIR9` writer - PBDIR9"]
pub type PBDIR9_W<'a, const O: u8> = crate::BitWriter<'a, PBDIRCR_SPEC, O>;
#[doc = "Field `PBDIR10` reader - PBDIR10"]
pub type PBDIR10_R = crate::BitReader;
#[doc = "Field `PBDIR10` writer - PBDIR10"]
pub type PBDIR10_W<'a, const O: u8> = crate::BitWriter<'a, PBDIRCR_SPEC, O>;
#[doc = "Field `PBDIR11` reader - PBDIR11"]
pub type PBDIR11_R = crate::BitReader;
#[doc = "Field `PBDIR11` writer - PBDIR11"]
pub type PBDIR11_W<'a, const O: u8> = crate::BitWriter<'a, PBDIRCR_SPEC, O>;
#[doc = "Field `PBDIR12` reader - PBDIR12"]
pub type PBDIR12_R = crate::BitReader;
#[doc = "Field `PBDIR12` writer - PBDIR12"]
pub type PBDIR12_W<'a, const O: u8> = crate::BitWriter<'a, PBDIRCR_SPEC, O>;
#[doc = "Field `PBDIR13` reader - PBDIR13"]
pub type PBDIR13_R = crate::BitReader;
#[doc = "Field `PBDIR13` writer - PBDIR13"]
pub type PBDIR13_W<'a, const O: u8> = crate::BitWriter<'a, PBDIRCR_SPEC, O>;
#[doc = "Field `PBDIR14` reader - PBDIR14"]
pub type PBDIR14_R = crate::BitReader;
#[doc = "Field `PBDIR14` writer - PBDIR14"]
pub type PBDIR14_W<'a, const O: u8> = crate::BitWriter<'a, PBDIRCR_SPEC, O>;
#[doc = "Field `PBDIR15` reader - PBDIR15"]
pub type PBDIR15_R = crate::BitReader;
#[doc = "Field `PBDIR15` writer - PBDIR15"]
pub type PBDIR15_W<'a, const O: u8> = crate::BitWriter<'a, PBDIRCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PBDIR0"]
    #[inline(always)]
    pub fn pbdir0(&self) -> PBDIR0_R {
        PBDIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PBDIR1"]
    #[inline(always)]
    pub fn pbdir1(&self) -> PBDIR1_R {
        PBDIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBDIR2"]
    #[inline(always)]
    pub fn pbdir2(&self) -> PBDIR2_R {
        PBDIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PBDIR3"]
    #[inline(always)]
    pub fn pbdir3(&self) -> PBDIR3_R {
        PBDIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PBDIR4"]
    #[inline(always)]
    pub fn pbdir4(&self) -> PBDIR4_R {
        PBDIR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PBDIR5"]
    #[inline(always)]
    pub fn pbdir5(&self) -> PBDIR5_R {
        PBDIR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PBDIR6"]
    #[inline(always)]
    pub fn pbdir6(&self) -> PBDIR6_R {
        PBDIR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PBDIR7"]
    #[inline(always)]
    pub fn pbdir7(&self) -> PBDIR7_R {
        PBDIR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PBDIR8"]
    #[inline(always)]
    pub fn pbdir8(&self) -> PBDIR8_R {
        PBDIR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PBDIR9"]
    #[inline(always)]
    pub fn pbdir9(&self) -> PBDIR9_R {
        PBDIR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PBDIR10"]
    #[inline(always)]
    pub fn pbdir10(&self) -> PBDIR10_R {
        PBDIR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PBDIR11"]
    #[inline(always)]
    pub fn pbdir11(&self) -> PBDIR11_R {
        PBDIR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PBDIR12"]
    #[inline(always)]
    pub fn pbdir12(&self) -> PBDIR12_R {
        PBDIR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PBDIR13"]
    #[inline(always)]
    pub fn pbdir13(&self) -> PBDIR13_R {
        PBDIR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PBDIR14"]
    #[inline(always)]
    pub fn pbdir14(&self) -> PBDIR14_R {
        PBDIR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PBDIR15"]
    #[inline(always)]
    pub fn pbdir15(&self) -> PBDIR15_R {
        PBDIR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PBDIR0"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir0(&mut self) -> PBDIR0_W<0> {
        PBDIR0_W::new(self)
    }
    #[doc = "Bit 1 - PBDIR1"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir1(&mut self) -> PBDIR1_W<1> {
        PBDIR1_W::new(self)
    }
    #[doc = "Bit 2 - PBDIR2"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir2(&mut self) -> PBDIR2_W<2> {
        PBDIR2_W::new(self)
    }
    #[doc = "Bit 3 - PBDIR3"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir3(&mut self) -> PBDIR3_W<3> {
        PBDIR3_W::new(self)
    }
    #[doc = "Bit 4 - PBDIR4"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir4(&mut self) -> PBDIR4_W<4> {
        PBDIR4_W::new(self)
    }
    #[doc = "Bit 5 - PBDIR5"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir5(&mut self) -> PBDIR5_W<5> {
        PBDIR5_W::new(self)
    }
    #[doc = "Bit 6 - PBDIR6"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir6(&mut self) -> PBDIR6_W<6> {
        PBDIR6_W::new(self)
    }
    #[doc = "Bit 7 - PBDIR7"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir7(&mut self) -> PBDIR7_W<7> {
        PBDIR7_W::new(self)
    }
    #[doc = "Bit 8 - PBDIR8"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir8(&mut self) -> PBDIR8_W<8> {
        PBDIR8_W::new(self)
    }
    #[doc = "Bit 9 - PBDIR9"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir9(&mut self) -> PBDIR9_W<9> {
        PBDIR9_W::new(self)
    }
    #[doc = "Bit 10 - PBDIR10"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir10(&mut self) -> PBDIR10_W<10> {
        PBDIR10_W::new(self)
    }
    #[doc = "Bit 11 - PBDIR11"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir11(&mut self) -> PBDIR11_W<11> {
        PBDIR11_W::new(self)
    }
    #[doc = "Bit 12 - PBDIR12"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir12(&mut self) -> PBDIR12_W<12> {
        PBDIR12_W::new(self)
    }
    #[doc = "Bit 13 - PBDIR13"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir13(&mut self) -> PBDIR13_W<13> {
        PBDIR13_W::new(self)
    }
    #[doc = "Bit 14 - PBDIR14"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir14(&mut self) -> PBDIR14_W<14> {
        PBDIR14_W::new(self)
    }
    #[doc = "Bit 15 - PBDIR15"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir15(&mut self) -> PBDIR15_W<15> {
        PBDIR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBDIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbdircr](index.html) module"]
pub struct PBDIRCR_SPEC;
impl crate::RegisterSpec for PBDIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbdircr::R](R) reader structure"]
impl crate::Readable for PBDIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbdircr::W](W) writer structure"]
impl crate::Writable for PBDIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBDIRCR to value 0"]
impl crate::Resettable for PBDIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
