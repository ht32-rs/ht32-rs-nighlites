#[doc = "Register `PAPDR` reader"]
pub struct R(crate::R<PAPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAPDR` writer"]
pub struct W(crate::W<PAPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAPDR_SPEC>;
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
impl From<crate::W<PAPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAPD0` reader - PAPD0"]
pub type PAPD0_R = crate::BitReader;
#[doc = "Field `PAPD0` writer - PAPD0"]
pub type PAPD0_W<'a, const O: u8> = crate::BitWriter<'a, PAPDR_SPEC, O>;
#[doc = "Field `PAPD1` reader - PAPD1"]
pub type PAPD1_R = crate::BitReader;
#[doc = "Field `PAPD1` writer - PAPD1"]
pub type PAPD1_W<'a, const O: u8> = crate::BitWriter<'a, PAPDR_SPEC, O>;
#[doc = "Field `PAPD2` reader - PAPD2"]
pub type PAPD2_R = crate::BitReader;
#[doc = "Field `PAPD2` writer - PAPD2"]
pub type PAPD2_W<'a, const O: u8> = crate::BitWriter<'a, PAPDR_SPEC, O>;
#[doc = "Field `PAPD3` reader - PAPD3"]
pub type PAPD3_R = crate::BitReader;
#[doc = "Field `PAPD3` writer - PAPD3"]
pub type PAPD3_W<'a, const O: u8> = crate::BitWriter<'a, PAPDR_SPEC, O>;
#[doc = "Field `PAPD4` reader - PAPD4"]
pub type PAPD4_R = crate::BitReader;
#[doc = "Field `PAPD4` writer - PAPD4"]
pub type PAPD4_W<'a, const O: u8> = crate::BitWriter<'a, PAPDR_SPEC, O>;
#[doc = "Field `PAPD5` reader - PAPD5"]
pub type PAPD5_R = crate::BitReader;
#[doc = "Field `PAPD5` writer - PAPD5"]
pub type PAPD5_W<'a, const O: u8> = crate::BitWriter<'a, PAPDR_SPEC, O>;
#[doc = "Field `PAPD6` reader - PAPD6"]
pub type PAPD6_R = crate::BitReader;
#[doc = "Field `PAPD6` writer - PAPD6"]
pub type PAPD6_W<'a, const O: u8> = crate::BitWriter<'a, PAPDR_SPEC, O>;
#[doc = "Field `PAPD7` reader - PAPD7"]
pub type PAPD7_R = crate::BitReader;
#[doc = "Field `PAPD7` writer - PAPD7"]
pub type PAPD7_W<'a, const O: u8> = crate::BitWriter<'a, PAPDR_SPEC, O>;
#[doc = "Field `PAPD8` reader - PAPD8"]
pub type PAPD8_R = crate::BitReader;
#[doc = "Field `PAPD8` writer - PAPD8"]
pub type PAPD8_W<'a, const O: u8> = crate::BitWriter<'a, PAPDR_SPEC, O>;
#[doc = "Field `PAPD9` reader - PAPD9"]
pub type PAPD9_R = crate::BitReader;
#[doc = "Field `PAPD9` writer - PAPD9"]
pub type PAPD9_W<'a, const O: u8> = crate::BitWriter<'a, PAPDR_SPEC, O>;
#[doc = "Field `PAPD10` reader - PAPD10"]
pub type PAPD10_R = crate::BitReader;
#[doc = "Field `PAPD10` writer - PAPD10"]
pub type PAPD10_W<'a, const O: u8> = crate::BitWriter<'a, PAPDR_SPEC, O>;
#[doc = "Field `PAPD11` reader - PAPD11"]
pub type PAPD11_R = crate::BitReader;
#[doc = "Field `PAPD11` writer - PAPD11"]
pub type PAPD11_W<'a, const O: u8> = crate::BitWriter<'a, PAPDR_SPEC, O>;
#[doc = "Field `PAPD12` reader - PAPD12"]
pub type PAPD12_R = crate::BitReader;
#[doc = "Field `PAPD12` writer - PAPD12"]
pub type PAPD12_W<'a, const O: u8> = crate::BitWriter<'a, PAPDR_SPEC, O>;
#[doc = "Field `PAPD13` reader - PAPD13"]
pub type PAPD13_R = crate::BitReader;
#[doc = "Field `PAPD13` writer - PAPD13"]
pub type PAPD13_W<'a, const O: u8> = crate::BitWriter<'a, PAPDR_SPEC, O>;
#[doc = "Field `PAPD14` reader - PAPD14"]
pub type PAPD14_R = crate::BitReader;
#[doc = "Field `PAPD14` writer - PAPD14"]
pub type PAPD14_W<'a, const O: u8> = crate::BitWriter<'a, PAPDR_SPEC, O>;
#[doc = "Field `PAPD15` reader - PAPD15"]
pub type PAPD15_R = crate::BitReader;
#[doc = "Field `PAPD15` writer - PAPD15"]
pub type PAPD15_W<'a, const O: u8> = crate::BitWriter<'a, PAPDR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PAPD0"]
    #[inline(always)]
    pub fn papd0(&self) -> PAPD0_R {
        PAPD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PAPD1"]
    #[inline(always)]
    pub fn papd1(&self) -> PAPD1_R {
        PAPD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PAPD2"]
    #[inline(always)]
    pub fn papd2(&self) -> PAPD2_R {
        PAPD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PAPD3"]
    #[inline(always)]
    pub fn papd3(&self) -> PAPD3_R {
        PAPD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PAPD4"]
    #[inline(always)]
    pub fn papd4(&self) -> PAPD4_R {
        PAPD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PAPD5"]
    #[inline(always)]
    pub fn papd5(&self) -> PAPD5_R {
        PAPD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PAPD6"]
    #[inline(always)]
    pub fn papd6(&self) -> PAPD6_R {
        PAPD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PAPD7"]
    #[inline(always)]
    pub fn papd7(&self) -> PAPD7_R {
        PAPD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PAPD8"]
    #[inline(always)]
    pub fn papd8(&self) -> PAPD8_R {
        PAPD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PAPD9"]
    #[inline(always)]
    pub fn papd9(&self) -> PAPD9_R {
        PAPD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PAPD10"]
    #[inline(always)]
    pub fn papd10(&self) -> PAPD10_R {
        PAPD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAPD11"]
    #[inline(always)]
    pub fn papd11(&self) -> PAPD11_R {
        PAPD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAPD12"]
    #[inline(always)]
    pub fn papd12(&self) -> PAPD12_R {
        PAPD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAPD13"]
    #[inline(always)]
    pub fn papd13(&self) -> PAPD13_R {
        PAPD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAPD14"]
    #[inline(always)]
    pub fn papd14(&self) -> PAPD14_R {
        PAPD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAPD15"]
    #[inline(always)]
    pub fn papd15(&self) -> PAPD15_R {
        PAPD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAPD0"]
    #[inline(always)]
    #[must_use]
    pub fn papd0(&mut self) -> PAPD0_W<0> {
        PAPD0_W::new(self)
    }
    #[doc = "Bit 1 - PAPD1"]
    #[inline(always)]
    #[must_use]
    pub fn papd1(&mut self) -> PAPD1_W<1> {
        PAPD1_W::new(self)
    }
    #[doc = "Bit 2 - PAPD2"]
    #[inline(always)]
    #[must_use]
    pub fn papd2(&mut self) -> PAPD2_W<2> {
        PAPD2_W::new(self)
    }
    #[doc = "Bit 3 - PAPD3"]
    #[inline(always)]
    #[must_use]
    pub fn papd3(&mut self) -> PAPD3_W<3> {
        PAPD3_W::new(self)
    }
    #[doc = "Bit 4 - PAPD4"]
    #[inline(always)]
    #[must_use]
    pub fn papd4(&mut self) -> PAPD4_W<4> {
        PAPD4_W::new(self)
    }
    #[doc = "Bit 5 - PAPD5"]
    #[inline(always)]
    #[must_use]
    pub fn papd5(&mut self) -> PAPD5_W<5> {
        PAPD5_W::new(self)
    }
    #[doc = "Bit 6 - PAPD6"]
    #[inline(always)]
    #[must_use]
    pub fn papd6(&mut self) -> PAPD6_W<6> {
        PAPD6_W::new(self)
    }
    #[doc = "Bit 7 - PAPD7"]
    #[inline(always)]
    #[must_use]
    pub fn papd7(&mut self) -> PAPD7_W<7> {
        PAPD7_W::new(self)
    }
    #[doc = "Bit 8 - PAPD8"]
    #[inline(always)]
    #[must_use]
    pub fn papd8(&mut self) -> PAPD8_W<8> {
        PAPD8_W::new(self)
    }
    #[doc = "Bit 9 - PAPD9"]
    #[inline(always)]
    #[must_use]
    pub fn papd9(&mut self) -> PAPD9_W<9> {
        PAPD9_W::new(self)
    }
    #[doc = "Bit 10 - PAPD10"]
    #[inline(always)]
    #[must_use]
    pub fn papd10(&mut self) -> PAPD10_W<10> {
        PAPD10_W::new(self)
    }
    #[doc = "Bit 11 - PAPD11"]
    #[inline(always)]
    #[must_use]
    pub fn papd11(&mut self) -> PAPD11_W<11> {
        PAPD11_W::new(self)
    }
    #[doc = "Bit 12 - PAPD12"]
    #[inline(always)]
    #[must_use]
    pub fn papd12(&mut self) -> PAPD12_W<12> {
        PAPD12_W::new(self)
    }
    #[doc = "Bit 13 - PAPD13"]
    #[inline(always)]
    #[must_use]
    pub fn papd13(&mut self) -> PAPD13_W<13> {
        PAPD13_W::new(self)
    }
    #[doc = "Bit 14 - PAPD14"]
    #[inline(always)]
    #[must_use]
    pub fn papd14(&mut self) -> PAPD14_W<14> {
        PAPD14_W::new(self)
    }
    #[doc = "Bit 15 - PAPD15"]
    #[inline(always)]
    #[must_use]
    pub fn papd15(&mut self) -> PAPD15_W<15> {
        PAPD15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAPDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [papdr](index.html) module"]
pub struct PAPDR_SPEC;
impl crate::RegisterSpec for PAPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [papdr::R](R) reader structure"]
impl crate::Readable for PAPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [papdr::W](W) writer structure"]
impl crate::Writable for PAPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAPDR to value 0"]
impl crate::Resettable for PAPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
