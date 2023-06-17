#[doc = "Register `PCDIRCR` reader"]
pub struct R(crate::R<PCDIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCDIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCDIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCDIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCDIRCR` writer"]
pub struct W(crate::W<PCDIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCDIRCR_SPEC>;
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
impl From<crate::W<PCDIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCDIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCDIR0` reader - PCDIR0"]
pub type PCDIR0_R = crate::BitReader;
#[doc = "Field `PCDIR0` writer - PCDIR0"]
pub type PCDIR0_W<'a, const O: u8> = crate::BitWriter<'a, PCDIRCR_SPEC, O>;
#[doc = "Field `PCDIR1` reader - PCDIR1"]
pub type PCDIR1_R = crate::BitReader;
#[doc = "Field `PCDIR1` writer - PCDIR1"]
pub type PCDIR1_W<'a, const O: u8> = crate::BitWriter<'a, PCDIRCR_SPEC, O>;
#[doc = "Field `PCDIR2` reader - PCDIR2"]
pub type PCDIR2_R = crate::BitReader;
#[doc = "Field `PCDIR2` writer - PCDIR2"]
pub type PCDIR2_W<'a, const O: u8> = crate::BitWriter<'a, PCDIRCR_SPEC, O>;
#[doc = "Field `PCDIR3` reader - PCDIR3"]
pub type PCDIR3_R = crate::BitReader;
#[doc = "Field `PCDIR3` writer - PCDIR3"]
pub type PCDIR3_W<'a, const O: u8> = crate::BitWriter<'a, PCDIRCR_SPEC, O>;
#[doc = "Field `PCDIR4` reader - PCDIR4"]
pub type PCDIR4_R = crate::BitReader;
#[doc = "Field `PCDIR4` writer - PCDIR4"]
pub type PCDIR4_W<'a, const O: u8> = crate::BitWriter<'a, PCDIRCR_SPEC, O>;
#[doc = "Field `PCDIR5` reader - PCDIR5"]
pub type PCDIR5_R = crate::BitReader;
#[doc = "Field `PCDIR5` writer - PCDIR5"]
pub type PCDIR5_W<'a, const O: u8> = crate::BitWriter<'a, PCDIRCR_SPEC, O>;
#[doc = "Field `PCDIR6` reader - PCDIR6"]
pub type PCDIR6_R = crate::BitReader;
#[doc = "Field `PCDIR6` writer - PCDIR6"]
pub type PCDIR6_W<'a, const O: u8> = crate::BitWriter<'a, PCDIRCR_SPEC, O>;
#[doc = "Field `PCDIR7` reader - PCDIR7"]
pub type PCDIR7_R = crate::BitReader;
#[doc = "Field `PCDIR7` writer - PCDIR7"]
pub type PCDIR7_W<'a, const O: u8> = crate::BitWriter<'a, PCDIRCR_SPEC, O>;
#[doc = "Field `PCDIR8` reader - PCDIR8"]
pub type PCDIR8_R = crate::BitReader;
#[doc = "Field `PCDIR8` writer - PCDIR8"]
pub type PCDIR8_W<'a, const O: u8> = crate::BitWriter<'a, PCDIRCR_SPEC, O>;
#[doc = "Field `PCDIR9` reader - PCDIR9"]
pub type PCDIR9_R = crate::BitReader;
#[doc = "Field `PCDIR9` writer - PCDIR9"]
pub type PCDIR9_W<'a, const O: u8> = crate::BitWriter<'a, PCDIRCR_SPEC, O>;
#[doc = "Field `PCDIR10` reader - PCDIR10"]
pub type PCDIR10_R = crate::BitReader;
#[doc = "Field `PCDIR10` writer - PCDIR10"]
pub type PCDIR10_W<'a, const O: u8> = crate::BitWriter<'a, PCDIRCR_SPEC, O>;
#[doc = "Field `PCDIR11` reader - PCDIR11"]
pub type PCDIR11_R = crate::BitReader;
#[doc = "Field `PCDIR11` writer - PCDIR11"]
pub type PCDIR11_W<'a, const O: u8> = crate::BitWriter<'a, PCDIRCR_SPEC, O>;
#[doc = "Field `PCDIR12` reader - PCDIR12"]
pub type PCDIR12_R = crate::BitReader;
#[doc = "Field `PCDIR12` writer - PCDIR12"]
pub type PCDIR12_W<'a, const O: u8> = crate::BitWriter<'a, PCDIRCR_SPEC, O>;
#[doc = "Field `PCDIR13` reader - PCDIR13"]
pub type PCDIR13_R = crate::BitReader;
#[doc = "Field `PCDIR13` writer - PCDIR13"]
pub type PCDIR13_W<'a, const O: u8> = crate::BitWriter<'a, PCDIRCR_SPEC, O>;
#[doc = "Field `PCDIR14` reader - PCDIR14"]
pub type PCDIR14_R = crate::BitReader;
#[doc = "Field `PCDIR14` writer - PCDIR14"]
pub type PCDIR14_W<'a, const O: u8> = crate::BitWriter<'a, PCDIRCR_SPEC, O>;
#[doc = "Field `PCDIR15` reader - PCDIR15"]
pub type PCDIR15_R = crate::BitReader;
#[doc = "Field `PCDIR15` writer - PCDIR15"]
pub type PCDIR15_W<'a, const O: u8> = crate::BitWriter<'a, PCDIRCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PCDIR0"]
    #[inline(always)]
    pub fn pcdir0(&self) -> PCDIR0_R {
        PCDIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCDIR1"]
    #[inline(always)]
    pub fn pcdir1(&self) -> PCDIR1_R {
        PCDIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCDIR2"]
    #[inline(always)]
    pub fn pcdir2(&self) -> PCDIR2_R {
        PCDIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCDIR3"]
    #[inline(always)]
    pub fn pcdir3(&self) -> PCDIR3_R {
        PCDIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PCDIR4"]
    #[inline(always)]
    pub fn pcdir4(&self) -> PCDIR4_R {
        PCDIR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCDIR5"]
    #[inline(always)]
    pub fn pcdir5(&self) -> PCDIR5_R {
        PCDIR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCDIR6"]
    #[inline(always)]
    pub fn pcdir6(&self) -> PCDIR6_R {
        PCDIR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PCDIR7"]
    #[inline(always)]
    pub fn pcdir7(&self) -> PCDIR7_R {
        PCDIR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PCDIR8"]
    #[inline(always)]
    pub fn pcdir8(&self) -> PCDIR8_R {
        PCDIR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PCDIR9"]
    #[inline(always)]
    pub fn pcdir9(&self) -> PCDIR9_R {
        PCDIR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCDIR10"]
    #[inline(always)]
    pub fn pcdir10(&self) -> PCDIR10_R {
        PCDIR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PCDIR11"]
    #[inline(always)]
    pub fn pcdir11(&self) -> PCDIR11_R {
        PCDIR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PCDIR12"]
    #[inline(always)]
    pub fn pcdir12(&self) -> PCDIR12_R {
        PCDIR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PCDIR13"]
    #[inline(always)]
    pub fn pcdir13(&self) -> PCDIR13_R {
        PCDIR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PCDIR14"]
    #[inline(always)]
    pub fn pcdir14(&self) -> PCDIR14_R {
        PCDIR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PCDIR15"]
    #[inline(always)]
    pub fn pcdir15(&self) -> PCDIR15_R {
        PCDIR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCDIR0"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir0(&mut self) -> PCDIR0_W<0> {
        PCDIR0_W::new(self)
    }
    #[doc = "Bit 1 - PCDIR1"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir1(&mut self) -> PCDIR1_W<1> {
        PCDIR1_W::new(self)
    }
    #[doc = "Bit 2 - PCDIR2"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir2(&mut self) -> PCDIR2_W<2> {
        PCDIR2_W::new(self)
    }
    #[doc = "Bit 3 - PCDIR3"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir3(&mut self) -> PCDIR3_W<3> {
        PCDIR3_W::new(self)
    }
    #[doc = "Bit 4 - PCDIR4"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir4(&mut self) -> PCDIR4_W<4> {
        PCDIR4_W::new(self)
    }
    #[doc = "Bit 5 - PCDIR5"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir5(&mut self) -> PCDIR5_W<5> {
        PCDIR5_W::new(self)
    }
    #[doc = "Bit 6 - PCDIR6"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir6(&mut self) -> PCDIR6_W<6> {
        PCDIR6_W::new(self)
    }
    #[doc = "Bit 7 - PCDIR7"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir7(&mut self) -> PCDIR7_W<7> {
        PCDIR7_W::new(self)
    }
    #[doc = "Bit 8 - PCDIR8"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir8(&mut self) -> PCDIR8_W<8> {
        PCDIR8_W::new(self)
    }
    #[doc = "Bit 9 - PCDIR9"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir9(&mut self) -> PCDIR9_W<9> {
        PCDIR9_W::new(self)
    }
    #[doc = "Bit 10 - PCDIR10"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir10(&mut self) -> PCDIR10_W<10> {
        PCDIR10_W::new(self)
    }
    #[doc = "Bit 11 - PCDIR11"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir11(&mut self) -> PCDIR11_W<11> {
        PCDIR11_W::new(self)
    }
    #[doc = "Bit 12 - PCDIR12"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir12(&mut self) -> PCDIR12_W<12> {
        PCDIR12_W::new(self)
    }
    #[doc = "Bit 13 - PCDIR13"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir13(&mut self) -> PCDIR13_W<13> {
        PCDIR13_W::new(self)
    }
    #[doc = "Bit 14 - PCDIR14"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir14(&mut self) -> PCDIR14_W<14> {
        PCDIR14_W::new(self)
    }
    #[doc = "Bit 15 - PCDIR15"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir15(&mut self) -> PCDIR15_W<15> {
        PCDIR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCDIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdircr](index.html) module"]
pub struct PCDIRCR_SPEC;
impl crate::RegisterSpec for PCDIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcdircr::R](R) reader structure"]
impl crate::Readable for PCDIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcdircr::W](W) writer structure"]
impl crate::Writable for PCDIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCDIRCR to value 0"]
impl crate::Resettable for PCDIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
