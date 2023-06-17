#[doc = "Register `PCPDR` reader"]
pub struct R(crate::R<PCPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCPDR` writer"]
pub struct W(crate::W<PCPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCPDR_SPEC>;
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
impl From<crate::W<PCPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCPD0` reader - PCPD0"]
pub type PCPD0_R = crate::BitReader;
#[doc = "Field `PCPD0` writer - PCPD0"]
pub type PCPD0_W<'a, const O: u8> = crate::BitWriter<'a, PCPDR_SPEC, O>;
#[doc = "Field `PCPD1` reader - PCPD1"]
pub type PCPD1_R = crate::BitReader;
#[doc = "Field `PCPD1` writer - PCPD1"]
pub type PCPD1_W<'a, const O: u8> = crate::BitWriter<'a, PCPDR_SPEC, O>;
#[doc = "Field `PCPD2` reader - PCPD2"]
pub type PCPD2_R = crate::BitReader;
#[doc = "Field `PCPD2` writer - PCPD2"]
pub type PCPD2_W<'a, const O: u8> = crate::BitWriter<'a, PCPDR_SPEC, O>;
#[doc = "Field `PCPD3` reader - PCPD3"]
pub type PCPD3_R = crate::BitReader;
#[doc = "Field `PCPD3` writer - PCPD3"]
pub type PCPD3_W<'a, const O: u8> = crate::BitWriter<'a, PCPDR_SPEC, O>;
#[doc = "Field `PCPD4` reader - PCPD4"]
pub type PCPD4_R = crate::BitReader;
#[doc = "Field `PCPD4` writer - PCPD4"]
pub type PCPD4_W<'a, const O: u8> = crate::BitWriter<'a, PCPDR_SPEC, O>;
#[doc = "Field `PCPD5` reader - PCPD5"]
pub type PCPD5_R = crate::BitReader;
#[doc = "Field `PCPD5` writer - PCPD5"]
pub type PCPD5_W<'a, const O: u8> = crate::BitWriter<'a, PCPDR_SPEC, O>;
#[doc = "Field `PCPD6` reader - PCPD6"]
pub type PCPD6_R = crate::BitReader;
#[doc = "Field `PCPD6` writer - PCPD6"]
pub type PCPD6_W<'a, const O: u8> = crate::BitWriter<'a, PCPDR_SPEC, O>;
#[doc = "Field `PCPD7` reader - PCPD7"]
pub type PCPD7_R = crate::BitReader;
#[doc = "Field `PCPD7` writer - PCPD7"]
pub type PCPD7_W<'a, const O: u8> = crate::BitWriter<'a, PCPDR_SPEC, O>;
#[doc = "Field `PCPD8` reader - PCPD8"]
pub type PCPD8_R = crate::BitReader;
#[doc = "Field `PCPD8` writer - PCPD8"]
pub type PCPD8_W<'a, const O: u8> = crate::BitWriter<'a, PCPDR_SPEC, O>;
#[doc = "Field `PCPD9` reader - PCPD9"]
pub type PCPD9_R = crate::BitReader;
#[doc = "Field `PCPD9` writer - PCPD9"]
pub type PCPD9_W<'a, const O: u8> = crate::BitWriter<'a, PCPDR_SPEC, O>;
#[doc = "Field `PCPD10` reader - PCPD10"]
pub type PCPD10_R = crate::BitReader;
#[doc = "Field `PCPD10` writer - PCPD10"]
pub type PCPD10_W<'a, const O: u8> = crate::BitWriter<'a, PCPDR_SPEC, O>;
#[doc = "Field `PCPD11` reader - PCPD11"]
pub type PCPD11_R = crate::BitReader;
#[doc = "Field `PCPD11` writer - PCPD11"]
pub type PCPD11_W<'a, const O: u8> = crate::BitWriter<'a, PCPDR_SPEC, O>;
#[doc = "Field `PCPD12` reader - PCPD12"]
pub type PCPD12_R = crate::BitReader;
#[doc = "Field `PCPD12` writer - PCPD12"]
pub type PCPD12_W<'a, const O: u8> = crate::BitWriter<'a, PCPDR_SPEC, O>;
#[doc = "Field `PCPD13` reader - PCPD13"]
pub type PCPD13_R = crate::BitReader;
#[doc = "Field `PCPD13` writer - PCPD13"]
pub type PCPD13_W<'a, const O: u8> = crate::BitWriter<'a, PCPDR_SPEC, O>;
#[doc = "Field `PCPD14` reader - PCPD14"]
pub type PCPD14_R = crate::BitReader;
#[doc = "Field `PCPD14` writer - PCPD14"]
pub type PCPD14_W<'a, const O: u8> = crate::BitWriter<'a, PCPDR_SPEC, O>;
#[doc = "Field `PCPD15` reader - PCPD15"]
pub type PCPD15_R = crate::BitReader;
#[doc = "Field `PCPD15` writer - PCPD15"]
pub type PCPD15_W<'a, const O: u8> = crate::BitWriter<'a, PCPDR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PCPD0"]
    #[inline(always)]
    pub fn pcpd0(&self) -> PCPD0_R {
        PCPD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCPD1"]
    #[inline(always)]
    pub fn pcpd1(&self) -> PCPD1_R {
        PCPD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCPD2"]
    #[inline(always)]
    pub fn pcpd2(&self) -> PCPD2_R {
        PCPD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCPD3"]
    #[inline(always)]
    pub fn pcpd3(&self) -> PCPD3_R {
        PCPD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PCPD4"]
    #[inline(always)]
    pub fn pcpd4(&self) -> PCPD4_R {
        PCPD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCPD5"]
    #[inline(always)]
    pub fn pcpd5(&self) -> PCPD5_R {
        PCPD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCPD6"]
    #[inline(always)]
    pub fn pcpd6(&self) -> PCPD6_R {
        PCPD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PCPD7"]
    #[inline(always)]
    pub fn pcpd7(&self) -> PCPD7_R {
        PCPD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PCPD8"]
    #[inline(always)]
    pub fn pcpd8(&self) -> PCPD8_R {
        PCPD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PCPD9"]
    #[inline(always)]
    pub fn pcpd9(&self) -> PCPD9_R {
        PCPD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCPD10"]
    #[inline(always)]
    pub fn pcpd10(&self) -> PCPD10_R {
        PCPD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PCPD11"]
    #[inline(always)]
    pub fn pcpd11(&self) -> PCPD11_R {
        PCPD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PCPD12"]
    #[inline(always)]
    pub fn pcpd12(&self) -> PCPD12_R {
        PCPD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PCPD13"]
    #[inline(always)]
    pub fn pcpd13(&self) -> PCPD13_R {
        PCPD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PCPD14"]
    #[inline(always)]
    pub fn pcpd14(&self) -> PCPD14_R {
        PCPD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PCPD15"]
    #[inline(always)]
    pub fn pcpd15(&self) -> PCPD15_R {
        PCPD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCPD0"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd0(&mut self) -> PCPD0_W<0> {
        PCPD0_W::new(self)
    }
    #[doc = "Bit 1 - PCPD1"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd1(&mut self) -> PCPD1_W<1> {
        PCPD1_W::new(self)
    }
    #[doc = "Bit 2 - PCPD2"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd2(&mut self) -> PCPD2_W<2> {
        PCPD2_W::new(self)
    }
    #[doc = "Bit 3 - PCPD3"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd3(&mut self) -> PCPD3_W<3> {
        PCPD3_W::new(self)
    }
    #[doc = "Bit 4 - PCPD4"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd4(&mut self) -> PCPD4_W<4> {
        PCPD4_W::new(self)
    }
    #[doc = "Bit 5 - PCPD5"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd5(&mut self) -> PCPD5_W<5> {
        PCPD5_W::new(self)
    }
    #[doc = "Bit 6 - PCPD6"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd6(&mut self) -> PCPD6_W<6> {
        PCPD6_W::new(self)
    }
    #[doc = "Bit 7 - PCPD7"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd7(&mut self) -> PCPD7_W<7> {
        PCPD7_W::new(self)
    }
    #[doc = "Bit 8 - PCPD8"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd8(&mut self) -> PCPD8_W<8> {
        PCPD8_W::new(self)
    }
    #[doc = "Bit 9 - PCPD9"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd9(&mut self) -> PCPD9_W<9> {
        PCPD9_W::new(self)
    }
    #[doc = "Bit 10 - PCPD10"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd10(&mut self) -> PCPD10_W<10> {
        PCPD10_W::new(self)
    }
    #[doc = "Bit 11 - PCPD11"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd11(&mut self) -> PCPD11_W<11> {
        PCPD11_W::new(self)
    }
    #[doc = "Bit 12 - PCPD12"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd12(&mut self) -> PCPD12_W<12> {
        PCPD12_W::new(self)
    }
    #[doc = "Bit 13 - PCPD13"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd13(&mut self) -> PCPD13_W<13> {
        PCPD13_W::new(self)
    }
    #[doc = "Bit 14 - PCPD14"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd14(&mut self) -> PCPD14_W<14> {
        PCPD14_W::new(self)
    }
    #[doc = "Bit 15 - PCPD15"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd15(&mut self) -> PCPD15_W<15> {
        PCPD15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCPDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcpdr](index.html) module"]
pub struct PCPDR_SPEC;
impl crate::RegisterSpec for PCPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcpdr::R](R) reader structure"]
impl crate::Readable for PCPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcpdr::W](W) writer structure"]
impl crate::Writable for PCPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCPDR to value 0"]
impl crate::Resettable for PCPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
