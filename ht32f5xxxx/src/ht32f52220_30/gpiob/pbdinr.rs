#[doc = "Register `PBDINR` reader"]
pub struct R(crate::R<PBDINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBDINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBDINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBDINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBDINR` writer"]
pub struct W(crate::W<PBDINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBDINR_SPEC>;
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
impl From<crate::W<PBDINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBDINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBDIN0` reader - PBDIN0"]
pub type PBDIN0_R = crate::BitReader;
#[doc = "Field `PBDIN0` writer - PBDIN0"]
pub type PBDIN0_W<'a, const O: u8> = crate::BitWriter<'a, PBDINR_SPEC, O>;
#[doc = "Field `PBDIN1` reader - PBDIN1"]
pub type PBDIN1_R = crate::BitReader;
#[doc = "Field `PBDIN1` writer - PBDIN1"]
pub type PBDIN1_W<'a, const O: u8> = crate::BitWriter<'a, PBDINR_SPEC, O>;
#[doc = "Field `PBDIN2` reader - PBDIN2"]
pub type PBDIN2_R = crate::BitReader;
#[doc = "Field `PBDIN2` writer - PBDIN2"]
pub type PBDIN2_W<'a, const O: u8> = crate::BitWriter<'a, PBDINR_SPEC, O>;
#[doc = "Field `PBDIN3` reader - PBDIN3"]
pub type PBDIN3_R = crate::BitReader;
#[doc = "Field `PBDIN3` writer - PBDIN3"]
pub type PBDIN3_W<'a, const O: u8> = crate::BitWriter<'a, PBDINR_SPEC, O>;
#[doc = "Field `PBDIN4` reader - PBDIN4"]
pub type PBDIN4_R = crate::BitReader;
#[doc = "Field `PBDIN4` writer - PBDIN4"]
pub type PBDIN4_W<'a, const O: u8> = crate::BitWriter<'a, PBDINR_SPEC, O>;
#[doc = "Field `PBDIN5` reader - PBDIN5"]
pub type PBDIN5_R = crate::BitReader;
#[doc = "Field `PBDIN5` writer - PBDIN5"]
pub type PBDIN5_W<'a, const O: u8> = crate::BitWriter<'a, PBDINR_SPEC, O>;
#[doc = "Field `PBDIN6` reader - PBDIN6"]
pub type PBDIN6_R = crate::BitReader;
#[doc = "Field `PBDIN6` writer - PBDIN6"]
pub type PBDIN6_W<'a, const O: u8> = crate::BitWriter<'a, PBDINR_SPEC, O>;
#[doc = "Field `PBDIN7` reader - PBDIN7"]
pub type PBDIN7_R = crate::BitReader;
#[doc = "Field `PBDIN7` writer - PBDIN7"]
pub type PBDIN7_W<'a, const O: u8> = crate::BitWriter<'a, PBDINR_SPEC, O>;
#[doc = "Field `PBDIN8` reader - PBDIN8"]
pub type PBDIN8_R = crate::BitReader;
#[doc = "Field `PBDIN8` writer - PBDIN8"]
pub type PBDIN8_W<'a, const O: u8> = crate::BitWriter<'a, PBDINR_SPEC, O>;
#[doc = "Field `PBDIN9` reader - PBDIN9"]
pub type PBDIN9_R = crate::BitReader;
#[doc = "Field `PBDIN9` writer - PBDIN9"]
pub type PBDIN9_W<'a, const O: u8> = crate::BitWriter<'a, PBDINR_SPEC, O>;
#[doc = "Field `PBDIN10` reader - PBDIN10"]
pub type PBDIN10_R = crate::BitReader;
#[doc = "Field `PBDIN10` writer - PBDIN10"]
pub type PBDIN10_W<'a, const O: u8> = crate::BitWriter<'a, PBDINR_SPEC, O>;
#[doc = "Field `PBDIN11` reader - PBDIN11"]
pub type PBDIN11_R = crate::BitReader;
#[doc = "Field `PBDIN11` writer - PBDIN11"]
pub type PBDIN11_W<'a, const O: u8> = crate::BitWriter<'a, PBDINR_SPEC, O>;
#[doc = "Field `PBDIN12` reader - PBDIN12"]
pub type PBDIN12_R = crate::BitReader;
#[doc = "Field `PBDIN12` writer - PBDIN12"]
pub type PBDIN12_W<'a, const O: u8> = crate::BitWriter<'a, PBDINR_SPEC, O>;
#[doc = "Field `PBDIN13` reader - PBDIN13"]
pub type PBDIN13_R = crate::BitReader;
#[doc = "Field `PBDIN13` writer - PBDIN13"]
pub type PBDIN13_W<'a, const O: u8> = crate::BitWriter<'a, PBDINR_SPEC, O>;
#[doc = "Field `PBDIN14` reader - PBDIN14"]
pub type PBDIN14_R = crate::BitReader;
#[doc = "Field `PBDIN14` writer - PBDIN14"]
pub type PBDIN14_W<'a, const O: u8> = crate::BitWriter<'a, PBDINR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PBDIN0"]
    #[inline(always)]
    pub fn pbdin0(&self) -> PBDIN0_R {
        PBDIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PBDIN1"]
    #[inline(always)]
    pub fn pbdin1(&self) -> PBDIN1_R {
        PBDIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBDIN2"]
    #[inline(always)]
    pub fn pbdin2(&self) -> PBDIN2_R {
        PBDIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PBDIN3"]
    #[inline(always)]
    pub fn pbdin3(&self) -> PBDIN3_R {
        PBDIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PBDIN4"]
    #[inline(always)]
    pub fn pbdin4(&self) -> PBDIN4_R {
        PBDIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PBDIN5"]
    #[inline(always)]
    pub fn pbdin5(&self) -> PBDIN5_R {
        PBDIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PBDIN6"]
    #[inline(always)]
    pub fn pbdin6(&self) -> PBDIN6_R {
        PBDIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PBDIN7"]
    #[inline(always)]
    pub fn pbdin7(&self) -> PBDIN7_R {
        PBDIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PBDIN8"]
    #[inline(always)]
    pub fn pbdin8(&self) -> PBDIN8_R {
        PBDIN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PBDIN9"]
    #[inline(always)]
    pub fn pbdin9(&self) -> PBDIN9_R {
        PBDIN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PBDIN10"]
    #[inline(always)]
    pub fn pbdin10(&self) -> PBDIN10_R {
        PBDIN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PBDIN11"]
    #[inline(always)]
    pub fn pbdin11(&self) -> PBDIN11_R {
        PBDIN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PBDIN12"]
    #[inline(always)]
    pub fn pbdin12(&self) -> PBDIN12_R {
        PBDIN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PBDIN13"]
    #[inline(always)]
    pub fn pbdin13(&self) -> PBDIN13_R {
        PBDIN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PBDIN14"]
    #[inline(always)]
    pub fn pbdin14(&self) -> PBDIN14_R {
        PBDIN14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PBDIN0"]
    #[inline(always)]
    #[must_use]
    pub fn pbdin0(&mut self) -> PBDIN0_W<0> {
        PBDIN0_W::new(self)
    }
    #[doc = "Bit 1 - PBDIN1"]
    #[inline(always)]
    #[must_use]
    pub fn pbdin1(&mut self) -> PBDIN1_W<1> {
        PBDIN1_W::new(self)
    }
    #[doc = "Bit 2 - PBDIN2"]
    #[inline(always)]
    #[must_use]
    pub fn pbdin2(&mut self) -> PBDIN2_W<2> {
        PBDIN2_W::new(self)
    }
    #[doc = "Bit 3 - PBDIN3"]
    #[inline(always)]
    #[must_use]
    pub fn pbdin3(&mut self) -> PBDIN3_W<3> {
        PBDIN3_W::new(self)
    }
    #[doc = "Bit 4 - PBDIN4"]
    #[inline(always)]
    #[must_use]
    pub fn pbdin4(&mut self) -> PBDIN4_W<4> {
        PBDIN4_W::new(self)
    }
    #[doc = "Bit 5 - PBDIN5"]
    #[inline(always)]
    #[must_use]
    pub fn pbdin5(&mut self) -> PBDIN5_W<5> {
        PBDIN5_W::new(self)
    }
    #[doc = "Bit 6 - PBDIN6"]
    #[inline(always)]
    #[must_use]
    pub fn pbdin6(&mut self) -> PBDIN6_W<6> {
        PBDIN6_W::new(self)
    }
    #[doc = "Bit 7 - PBDIN7"]
    #[inline(always)]
    #[must_use]
    pub fn pbdin7(&mut self) -> PBDIN7_W<7> {
        PBDIN7_W::new(self)
    }
    #[doc = "Bit 8 - PBDIN8"]
    #[inline(always)]
    #[must_use]
    pub fn pbdin8(&mut self) -> PBDIN8_W<8> {
        PBDIN8_W::new(self)
    }
    #[doc = "Bit 9 - PBDIN9"]
    #[inline(always)]
    #[must_use]
    pub fn pbdin9(&mut self) -> PBDIN9_W<9> {
        PBDIN9_W::new(self)
    }
    #[doc = "Bit 10 - PBDIN10"]
    #[inline(always)]
    #[must_use]
    pub fn pbdin10(&mut self) -> PBDIN10_W<10> {
        PBDIN10_W::new(self)
    }
    #[doc = "Bit 11 - PBDIN11"]
    #[inline(always)]
    #[must_use]
    pub fn pbdin11(&mut self) -> PBDIN11_W<11> {
        PBDIN11_W::new(self)
    }
    #[doc = "Bit 12 - PBDIN12"]
    #[inline(always)]
    #[must_use]
    pub fn pbdin12(&mut self) -> PBDIN12_W<12> {
        PBDIN12_W::new(self)
    }
    #[doc = "Bit 13 - PBDIN13"]
    #[inline(always)]
    #[must_use]
    pub fn pbdin13(&mut self) -> PBDIN13_W<13> {
        PBDIN13_W::new(self)
    }
    #[doc = "Bit 14 - PBDIN14"]
    #[inline(always)]
    #[must_use]
    pub fn pbdin14(&mut self) -> PBDIN14_W<14> {
        PBDIN14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBDINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbdinr](index.html) module"]
pub struct PBDINR_SPEC;
impl crate::RegisterSpec for PBDINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbdinr::R](R) reader structure"]
impl crate::Readable for PBDINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbdinr::W](W) writer structure"]
impl crate::Writable for PBDINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBDINR to value 0"]
impl crate::Resettable for PBDINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
