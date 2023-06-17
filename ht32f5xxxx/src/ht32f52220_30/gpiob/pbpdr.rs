#[doc = "Register `PBPDR` reader"]
pub struct R(crate::R<PBPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBPDR` writer"]
pub struct W(crate::W<PBPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBPDR_SPEC>;
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
impl From<crate::W<PBPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBPD0` reader - PBPD0"]
pub type PBPD0_R = crate::BitReader;
#[doc = "Field `PBPD0` writer - PBPD0"]
pub type PBPD0_W<'a, const O: u8> = crate::BitWriter<'a, PBPDR_SPEC, O>;
#[doc = "Field `PBPD1` reader - PBPD1"]
pub type PBPD1_R = crate::BitReader;
#[doc = "Field `PBPD1` writer - PBPD1"]
pub type PBPD1_W<'a, const O: u8> = crate::BitWriter<'a, PBPDR_SPEC, O>;
#[doc = "Field `PBPD2` reader - PBPD2"]
pub type PBPD2_R = crate::BitReader;
#[doc = "Field `PBPD2` writer - PBPD2"]
pub type PBPD2_W<'a, const O: u8> = crate::BitWriter<'a, PBPDR_SPEC, O>;
#[doc = "Field `PBPD3` reader - PBPD3"]
pub type PBPD3_R = crate::BitReader;
#[doc = "Field `PBPD3` writer - PBPD3"]
pub type PBPD3_W<'a, const O: u8> = crate::BitWriter<'a, PBPDR_SPEC, O>;
#[doc = "Field `PBPD4` reader - PBPD4"]
pub type PBPD4_R = crate::BitReader;
#[doc = "Field `PBPD4` writer - PBPD4"]
pub type PBPD4_W<'a, const O: u8> = crate::BitWriter<'a, PBPDR_SPEC, O>;
#[doc = "Field `PBPD5` reader - PBPD5"]
pub type PBPD5_R = crate::BitReader;
#[doc = "Field `PBPD5` writer - PBPD5"]
pub type PBPD5_W<'a, const O: u8> = crate::BitWriter<'a, PBPDR_SPEC, O>;
#[doc = "Field `PBPD6` reader - PBPD6"]
pub type PBPD6_R = crate::BitReader;
#[doc = "Field `PBPD6` writer - PBPD6"]
pub type PBPD6_W<'a, const O: u8> = crate::BitWriter<'a, PBPDR_SPEC, O>;
#[doc = "Field `PBPD7` reader - PBPD7"]
pub type PBPD7_R = crate::BitReader;
#[doc = "Field `PBPD7` writer - PBPD7"]
pub type PBPD7_W<'a, const O: u8> = crate::BitWriter<'a, PBPDR_SPEC, O>;
#[doc = "Field `PBPD8` reader - PBPD8"]
pub type PBPD8_R = crate::BitReader;
#[doc = "Field `PBPD8` writer - PBPD8"]
pub type PBPD8_W<'a, const O: u8> = crate::BitWriter<'a, PBPDR_SPEC, O>;
#[doc = "Field `PBPD9` reader - PBPD9"]
pub type PBPD9_R = crate::BitReader;
#[doc = "Field `PBPD9` writer - PBPD9"]
pub type PBPD9_W<'a, const O: u8> = crate::BitWriter<'a, PBPDR_SPEC, O>;
#[doc = "Field `PBPD10` reader - PBPD10"]
pub type PBPD10_R = crate::BitReader;
#[doc = "Field `PBPD10` writer - PBPD10"]
pub type PBPD10_W<'a, const O: u8> = crate::BitWriter<'a, PBPDR_SPEC, O>;
#[doc = "Field `PBPD11` reader - PBPD11"]
pub type PBPD11_R = crate::BitReader;
#[doc = "Field `PBPD11` writer - PBPD11"]
pub type PBPD11_W<'a, const O: u8> = crate::BitWriter<'a, PBPDR_SPEC, O>;
#[doc = "Field `PBPD12` reader - PBPD12"]
pub type PBPD12_R = crate::BitReader;
#[doc = "Field `PBPD12` writer - PBPD12"]
pub type PBPD12_W<'a, const O: u8> = crate::BitWriter<'a, PBPDR_SPEC, O>;
#[doc = "Field `PBPD13` reader - PBPD13"]
pub type PBPD13_R = crate::BitReader;
#[doc = "Field `PBPD13` writer - PBPD13"]
pub type PBPD13_W<'a, const O: u8> = crate::BitWriter<'a, PBPDR_SPEC, O>;
#[doc = "Field `PBPD14` reader - PBPD14"]
pub type PBPD14_R = crate::BitReader;
#[doc = "Field `PBPD14` writer - PBPD14"]
pub type PBPD14_W<'a, const O: u8> = crate::BitWriter<'a, PBPDR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PBPD0"]
    #[inline(always)]
    pub fn pbpd0(&self) -> PBPD0_R {
        PBPD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PBPD1"]
    #[inline(always)]
    pub fn pbpd1(&self) -> PBPD1_R {
        PBPD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBPD2"]
    #[inline(always)]
    pub fn pbpd2(&self) -> PBPD2_R {
        PBPD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PBPD3"]
    #[inline(always)]
    pub fn pbpd3(&self) -> PBPD3_R {
        PBPD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PBPD4"]
    #[inline(always)]
    pub fn pbpd4(&self) -> PBPD4_R {
        PBPD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PBPD5"]
    #[inline(always)]
    pub fn pbpd5(&self) -> PBPD5_R {
        PBPD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PBPD6"]
    #[inline(always)]
    pub fn pbpd6(&self) -> PBPD6_R {
        PBPD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PBPD7"]
    #[inline(always)]
    pub fn pbpd7(&self) -> PBPD7_R {
        PBPD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PBPD8"]
    #[inline(always)]
    pub fn pbpd8(&self) -> PBPD8_R {
        PBPD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PBPD9"]
    #[inline(always)]
    pub fn pbpd9(&self) -> PBPD9_R {
        PBPD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PBPD10"]
    #[inline(always)]
    pub fn pbpd10(&self) -> PBPD10_R {
        PBPD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PBPD11"]
    #[inline(always)]
    pub fn pbpd11(&self) -> PBPD11_R {
        PBPD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PBPD12"]
    #[inline(always)]
    pub fn pbpd12(&self) -> PBPD12_R {
        PBPD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PBPD13"]
    #[inline(always)]
    pub fn pbpd13(&self) -> PBPD13_R {
        PBPD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PBPD14"]
    #[inline(always)]
    pub fn pbpd14(&self) -> PBPD14_R {
        PBPD14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PBPD0"]
    #[inline(always)]
    #[must_use]
    pub fn pbpd0(&mut self) -> PBPD0_W<0> {
        PBPD0_W::new(self)
    }
    #[doc = "Bit 1 - PBPD1"]
    #[inline(always)]
    #[must_use]
    pub fn pbpd1(&mut self) -> PBPD1_W<1> {
        PBPD1_W::new(self)
    }
    #[doc = "Bit 2 - PBPD2"]
    #[inline(always)]
    #[must_use]
    pub fn pbpd2(&mut self) -> PBPD2_W<2> {
        PBPD2_W::new(self)
    }
    #[doc = "Bit 3 - PBPD3"]
    #[inline(always)]
    #[must_use]
    pub fn pbpd3(&mut self) -> PBPD3_W<3> {
        PBPD3_W::new(self)
    }
    #[doc = "Bit 4 - PBPD4"]
    #[inline(always)]
    #[must_use]
    pub fn pbpd4(&mut self) -> PBPD4_W<4> {
        PBPD4_W::new(self)
    }
    #[doc = "Bit 5 - PBPD5"]
    #[inline(always)]
    #[must_use]
    pub fn pbpd5(&mut self) -> PBPD5_W<5> {
        PBPD5_W::new(self)
    }
    #[doc = "Bit 6 - PBPD6"]
    #[inline(always)]
    #[must_use]
    pub fn pbpd6(&mut self) -> PBPD6_W<6> {
        PBPD6_W::new(self)
    }
    #[doc = "Bit 7 - PBPD7"]
    #[inline(always)]
    #[must_use]
    pub fn pbpd7(&mut self) -> PBPD7_W<7> {
        PBPD7_W::new(self)
    }
    #[doc = "Bit 8 - PBPD8"]
    #[inline(always)]
    #[must_use]
    pub fn pbpd8(&mut self) -> PBPD8_W<8> {
        PBPD8_W::new(self)
    }
    #[doc = "Bit 9 - PBPD9"]
    #[inline(always)]
    #[must_use]
    pub fn pbpd9(&mut self) -> PBPD9_W<9> {
        PBPD9_W::new(self)
    }
    #[doc = "Bit 10 - PBPD10"]
    #[inline(always)]
    #[must_use]
    pub fn pbpd10(&mut self) -> PBPD10_W<10> {
        PBPD10_W::new(self)
    }
    #[doc = "Bit 11 - PBPD11"]
    #[inline(always)]
    #[must_use]
    pub fn pbpd11(&mut self) -> PBPD11_W<11> {
        PBPD11_W::new(self)
    }
    #[doc = "Bit 12 - PBPD12"]
    #[inline(always)]
    #[must_use]
    pub fn pbpd12(&mut self) -> PBPD12_W<12> {
        PBPD12_W::new(self)
    }
    #[doc = "Bit 13 - PBPD13"]
    #[inline(always)]
    #[must_use]
    pub fn pbpd13(&mut self) -> PBPD13_W<13> {
        PBPD13_W::new(self)
    }
    #[doc = "Bit 14 - PBPD14"]
    #[inline(always)]
    #[must_use]
    pub fn pbpd14(&mut self) -> PBPD14_W<14> {
        PBPD14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBPDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbpdr](index.html) module"]
pub struct PBPDR_SPEC;
impl crate::RegisterSpec for PBPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbpdr::R](R) reader structure"]
impl crate::Readable for PBPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbpdr::W](W) writer structure"]
impl crate::Writable for PBPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBPDR to value 0"]
impl crate::Resettable for PBPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
