#[doc = "Register `PDPDR` reader"]
pub struct R(crate::R<PDPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDPDR` writer"]
pub struct W(crate::W<PDPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDPDR_SPEC>;
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
impl From<crate::W<PDPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDPD0` reader - PDPD0"]
pub type PDPD0_R = crate::BitReader;
#[doc = "Field `PDPD0` writer - PDPD0"]
pub type PDPD0_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD1` reader - PDPD1"]
pub type PDPD1_R = crate::BitReader;
#[doc = "Field `PDPD1` writer - PDPD1"]
pub type PDPD1_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD2` reader - PDPD2"]
pub type PDPD2_R = crate::BitReader;
#[doc = "Field `PDPD2` writer - PDPD2"]
pub type PDPD2_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD3` reader - PDPD3"]
pub type PDPD3_R = crate::BitReader;
#[doc = "Field `PDPD3` writer - PDPD3"]
pub type PDPD3_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD4` reader - PDPD4"]
pub type PDPD4_R = crate::BitReader;
#[doc = "Field `PDPD4` writer - PDPD4"]
pub type PDPD4_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD5` reader - PDPD5"]
pub type PDPD5_R = crate::BitReader;
#[doc = "Field `PDPD5` writer - PDPD5"]
pub type PDPD5_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD6` reader - PDPD6"]
pub type PDPD6_R = crate::BitReader;
#[doc = "Field `PDPD6` writer - PDPD6"]
pub type PDPD6_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD7` reader - PDPD7"]
pub type PDPD7_R = crate::BitReader;
#[doc = "Field `PDPD7` writer - PDPD7"]
pub type PDPD7_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD8` reader - PDPD8"]
pub type PDPD8_R = crate::BitReader;
#[doc = "Field `PDPD8` writer - PDPD8"]
pub type PDPD8_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD9` reader - PDPD9"]
pub type PDPD9_R = crate::BitReader;
#[doc = "Field `PDPD9` writer - PDPD9"]
pub type PDPD9_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD10` reader - PDPD10"]
pub type PDPD10_R = crate::BitReader;
#[doc = "Field `PDPD10` writer - PDPD10"]
pub type PDPD10_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD11` reader - PDPD11"]
pub type PDPD11_R = crate::BitReader;
#[doc = "Field `PDPD11` writer - PDPD11"]
pub type PDPD11_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD12` reader - PDPD12"]
pub type PDPD12_R = crate::BitReader;
#[doc = "Field `PDPD12` writer - PDPD12"]
pub type PDPD12_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD13` reader - PDPD13"]
pub type PDPD13_R = crate::BitReader;
#[doc = "Field `PDPD13` writer - PDPD13"]
pub type PDPD13_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD14` reader - PDPD14"]
pub type PDPD14_R = crate::BitReader;
#[doc = "Field `PDPD14` writer - PDPD14"]
pub type PDPD14_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
#[doc = "Field `PDPD15` reader - PDPD15"]
pub type PDPD15_R = crate::BitReader;
#[doc = "Field `PDPD15` writer - PDPD15"]
pub type PDPD15_W<'a, const O: u8> = crate::BitWriter<'a, PDPDR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDPD0"]
    #[inline(always)]
    pub fn pdpd0(&self) -> PDPD0_R {
        PDPD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDPD1"]
    #[inline(always)]
    pub fn pdpd1(&self) -> PDPD1_R {
        PDPD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDPD2"]
    #[inline(always)]
    pub fn pdpd2(&self) -> PDPD2_R {
        PDPD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDPD3"]
    #[inline(always)]
    pub fn pdpd3(&self) -> PDPD3_R {
        PDPD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PDPD4"]
    #[inline(always)]
    pub fn pdpd4(&self) -> PDPD4_R {
        PDPD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PDPD5"]
    #[inline(always)]
    pub fn pdpd5(&self) -> PDPD5_R {
        PDPD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDPD6"]
    #[inline(always)]
    pub fn pdpd6(&self) -> PDPD6_R {
        PDPD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDPD7"]
    #[inline(always)]
    pub fn pdpd7(&self) -> PDPD7_R {
        PDPD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PDPD8"]
    #[inline(always)]
    pub fn pdpd8(&self) -> PDPD8_R {
        PDPD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PDPD9"]
    #[inline(always)]
    pub fn pdpd9(&self) -> PDPD9_R {
        PDPD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PDPD10"]
    #[inline(always)]
    pub fn pdpd10(&self) -> PDPD10_R {
        PDPD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PDPD11"]
    #[inline(always)]
    pub fn pdpd11(&self) -> PDPD11_R {
        PDPD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PDPD12"]
    #[inline(always)]
    pub fn pdpd12(&self) -> PDPD12_R {
        PDPD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PDPD13"]
    #[inline(always)]
    pub fn pdpd13(&self) -> PDPD13_R {
        PDPD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PDPD14"]
    #[inline(always)]
    pub fn pdpd14(&self) -> PDPD14_R {
        PDPD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PDPD15"]
    #[inline(always)]
    pub fn pdpd15(&self) -> PDPD15_R {
        PDPD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDPD0"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd0(&mut self) -> PDPD0_W<0> {
        PDPD0_W::new(self)
    }
    #[doc = "Bit 1 - PDPD1"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd1(&mut self) -> PDPD1_W<1> {
        PDPD1_W::new(self)
    }
    #[doc = "Bit 2 - PDPD2"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd2(&mut self) -> PDPD2_W<2> {
        PDPD2_W::new(self)
    }
    #[doc = "Bit 3 - PDPD3"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd3(&mut self) -> PDPD3_W<3> {
        PDPD3_W::new(self)
    }
    #[doc = "Bit 4 - PDPD4"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd4(&mut self) -> PDPD4_W<4> {
        PDPD4_W::new(self)
    }
    #[doc = "Bit 5 - PDPD5"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd5(&mut self) -> PDPD5_W<5> {
        PDPD5_W::new(self)
    }
    #[doc = "Bit 6 - PDPD6"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd6(&mut self) -> PDPD6_W<6> {
        PDPD6_W::new(self)
    }
    #[doc = "Bit 7 - PDPD7"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd7(&mut self) -> PDPD7_W<7> {
        PDPD7_W::new(self)
    }
    #[doc = "Bit 8 - PDPD8"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd8(&mut self) -> PDPD8_W<8> {
        PDPD8_W::new(self)
    }
    #[doc = "Bit 9 - PDPD9"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd9(&mut self) -> PDPD9_W<9> {
        PDPD9_W::new(self)
    }
    #[doc = "Bit 10 - PDPD10"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd10(&mut self) -> PDPD10_W<10> {
        PDPD10_W::new(self)
    }
    #[doc = "Bit 11 - PDPD11"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd11(&mut self) -> PDPD11_W<11> {
        PDPD11_W::new(self)
    }
    #[doc = "Bit 12 - PDPD12"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd12(&mut self) -> PDPD12_W<12> {
        PDPD12_W::new(self)
    }
    #[doc = "Bit 13 - PDPD13"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd13(&mut self) -> PDPD13_W<13> {
        PDPD13_W::new(self)
    }
    #[doc = "Bit 14 - PDPD14"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd14(&mut self) -> PDPD14_W<14> {
        PDPD14_W::new(self)
    }
    #[doc = "Bit 15 - PDPD15"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd15(&mut self) -> PDPD15_W<15> {
        PDPD15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDPDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdpdr](index.html) module"]
pub struct PDPDR_SPEC;
impl crate::RegisterSpec for PDPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdpdr::R](R) reader structure"]
impl crate::Readable for PDPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdpdr::W](W) writer structure"]
impl crate::Writable for PDPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDPDR to value 0"]
impl crate::Resettable for PDPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
