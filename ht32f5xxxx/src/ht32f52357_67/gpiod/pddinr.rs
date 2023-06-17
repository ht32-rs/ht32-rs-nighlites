#[doc = "Register `PDDINR` reader"]
pub struct R(crate::R<PDDINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDDINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDDINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDDINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDDINR` writer"]
pub struct W(crate::W<PDDINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDDINR_SPEC>;
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
impl From<crate::W<PDDINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDDINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDDIN0` reader - PDDIN0"]
pub type PDDIN0_R = crate::BitReader;
#[doc = "Field `PDDIN0` writer - PDDIN0"]
pub type PDDIN0_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN1` reader - PDDIN1"]
pub type PDDIN1_R = crate::BitReader;
#[doc = "Field `PDDIN1` writer - PDDIN1"]
pub type PDDIN1_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN2` reader - PDDIN2"]
pub type PDDIN2_R = crate::BitReader;
#[doc = "Field `PDDIN2` writer - PDDIN2"]
pub type PDDIN2_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN3` reader - PDDIN3"]
pub type PDDIN3_R = crate::BitReader;
#[doc = "Field `PDDIN3` writer - PDDIN3"]
pub type PDDIN3_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN4` reader - PDDIN4"]
pub type PDDIN4_R = crate::BitReader;
#[doc = "Field `PDDIN4` writer - PDDIN4"]
pub type PDDIN4_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN5` reader - PDDIN5"]
pub type PDDIN5_R = crate::BitReader;
#[doc = "Field `PDDIN5` writer - PDDIN5"]
pub type PDDIN5_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN6` reader - PDDIN6"]
pub type PDDIN6_R = crate::BitReader;
#[doc = "Field `PDDIN6` writer - PDDIN6"]
pub type PDDIN6_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN7` reader - PDDIN7"]
pub type PDDIN7_R = crate::BitReader;
#[doc = "Field `PDDIN7` writer - PDDIN7"]
pub type PDDIN7_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN8` reader - PDDIN8"]
pub type PDDIN8_R = crate::BitReader;
#[doc = "Field `PDDIN8` writer - PDDIN8"]
pub type PDDIN8_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN9` reader - PDDIN9"]
pub type PDDIN9_R = crate::BitReader;
#[doc = "Field `PDDIN9` writer - PDDIN9"]
pub type PDDIN9_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN10` reader - PDDIN10"]
pub type PDDIN10_R = crate::BitReader;
#[doc = "Field `PDDIN10` writer - PDDIN10"]
pub type PDDIN10_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN11` reader - PDDIN11"]
pub type PDDIN11_R = crate::BitReader;
#[doc = "Field `PDDIN11` writer - PDDIN11"]
pub type PDDIN11_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN12` reader - PDDIN12"]
pub type PDDIN12_R = crate::BitReader;
#[doc = "Field `PDDIN12` writer - PDDIN12"]
pub type PDDIN12_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN13` reader - PDDIN13"]
pub type PDDIN13_R = crate::BitReader;
#[doc = "Field `PDDIN13` writer - PDDIN13"]
pub type PDDIN13_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN14` reader - PDDIN14"]
pub type PDDIN14_R = crate::BitReader;
#[doc = "Field `PDDIN14` writer - PDDIN14"]
pub type PDDIN14_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
#[doc = "Field `PDDIN15` reader - PDDIN15"]
pub type PDDIN15_R = crate::BitReader;
#[doc = "Field `PDDIN15` writer - PDDIN15"]
pub type PDDIN15_W<'a, const O: u8> = crate::BitWriter<'a, PDDINR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDDIN0"]
    #[inline(always)]
    pub fn pddin0(&self) -> PDDIN0_R {
        PDDIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDDIN1"]
    #[inline(always)]
    pub fn pddin1(&self) -> PDDIN1_R {
        PDDIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDDIN2"]
    #[inline(always)]
    pub fn pddin2(&self) -> PDDIN2_R {
        PDDIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDDIN3"]
    #[inline(always)]
    pub fn pddin3(&self) -> PDDIN3_R {
        PDDIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PDDIN4"]
    #[inline(always)]
    pub fn pddin4(&self) -> PDDIN4_R {
        PDDIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PDDIN5"]
    #[inline(always)]
    pub fn pddin5(&self) -> PDDIN5_R {
        PDDIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDDIN6"]
    #[inline(always)]
    pub fn pddin6(&self) -> PDDIN6_R {
        PDDIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDDIN7"]
    #[inline(always)]
    pub fn pddin7(&self) -> PDDIN7_R {
        PDDIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PDDIN8"]
    #[inline(always)]
    pub fn pddin8(&self) -> PDDIN8_R {
        PDDIN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PDDIN9"]
    #[inline(always)]
    pub fn pddin9(&self) -> PDDIN9_R {
        PDDIN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PDDIN10"]
    #[inline(always)]
    pub fn pddin10(&self) -> PDDIN10_R {
        PDDIN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PDDIN11"]
    #[inline(always)]
    pub fn pddin11(&self) -> PDDIN11_R {
        PDDIN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PDDIN12"]
    #[inline(always)]
    pub fn pddin12(&self) -> PDDIN12_R {
        PDDIN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PDDIN13"]
    #[inline(always)]
    pub fn pddin13(&self) -> PDDIN13_R {
        PDDIN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PDDIN14"]
    #[inline(always)]
    pub fn pddin14(&self) -> PDDIN14_R {
        PDDIN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PDDIN15"]
    #[inline(always)]
    pub fn pddin15(&self) -> PDDIN15_R {
        PDDIN15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDDIN0"]
    #[inline(always)]
    #[must_use]
    pub fn pddin0(&mut self) -> PDDIN0_W<0> {
        PDDIN0_W::new(self)
    }
    #[doc = "Bit 1 - PDDIN1"]
    #[inline(always)]
    #[must_use]
    pub fn pddin1(&mut self) -> PDDIN1_W<1> {
        PDDIN1_W::new(self)
    }
    #[doc = "Bit 2 - PDDIN2"]
    #[inline(always)]
    #[must_use]
    pub fn pddin2(&mut self) -> PDDIN2_W<2> {
        PDDIN2_W::new(self)
    }
    #[doc = "Bit 3 - PDDIN3"]
    #[inline(always)]
    #[must_use]
    pub fn pddin3(&mut self) -> PDDIN3_W<3> {
        PDDIN3_W::new(self)
    }
    #[doc = "Bit 4 - PDDIN4"]
    #[inline(always)]
    #[must_use]
    pub fn pddin4(&mut self) -> PDDIN4_W<4> {
        PDDIN4_W::new(self)
    }
    #[doc = "Bit 5 - PDDIN5"]
    #[inline(always)]
    #[must_use]
    pub fn pddin5(&mut self) -> PDDIN5_W<5> {
        PDDIN5_W::new(self)
    }
    #[doc = "Bit 6 - PDDIN6"]
    #[inline(always)]
    #[must_use]
    pub fn pddin6(&mut self) -> PDDIN6_W<6> {
        PDDIN6_W::new(self)
    }
    #[doc = "Bit 7 - PDDIN7"]
    #[inline(always)]
    #[must_use]
    pub fn pddin7(&mut self) -> PDDIN7_W<7> {
        PDDIN7_W::new(self)
    }
    #[doc = "Bit 8 - PDDIN8"]
    #[inline(always)]
    #[must_use]
    pub fn pddin8(&mut self) -> PDDIN8_W<8> {
        PDDIN8_W::new(self)
    }
    #[doc = "Bit 9 - PDDIN9"]
    #[inline(always)]
    #[must_use]
    pub fn pddin9(&mut self) -> PDDIN9_W<9> {
        PDDIN9_W::new(self)
    }
    #[doc = "Bit 10 - PDDIN10"]
    #[inline(always)]
    #[must_use]
    pub fn pddin10(&mut self) -> PDDIN10_W<10> {
        PDDIN10_W::new(self)
    }
    #[doc = "Bit 11 - PDDIN11"]
    #[inline(always)]
    #[must_use]
    pub fn pddin11(&mut self) -> PDDIN11_W<11> {
        PDDIN11_W::new(self)
    }
    #[doc = "Bit 12 - PDDIN12"]
    #[inline(always)]
    #[must_use]
    pub fn pddin12(&mut self) -> PDDIN12_W<12> {
        PDDIN12_W::new(self)
    }
    #[doc = "Bit 13 - PDDIN13"]
    #[inline(always)]
    #[must_use]
    pub fn pddin13(&mut self) -> PDDIN13_W<13> {
        PDDIN13_W::new(self)
    }
    #[doc = "Bit 14 - PDDIN14"]
    #[inline(always)]
    #[must_use]
    pub fn pddin14(&mut self) -> PDDIN14_W<14> {
        PDDIN14_W::new(self)
    }
    #[doc = "Bit 15 - PDDIN15"]
    #[inline(always)]
    #[must_use]
    pub fn pddin15(&mut self) -> PDDIN15_W<15> {
        PDDIN15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDDINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddinr](index.html) module"]
pub struct PDDINR_SPEC;
impl crate::RegisterSpec for PDDINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pddinr::R](R) reader structure"]
impl crate::Readable for PDDINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pddinr::W](W) writer structure"]
impl crate::Writable for PDDINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDDINR to value 0"]
impl crate::Resettable for PDDINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
