#[doc = "Register `PDMA_ISCR1` reader"]
pub struct R(crate::R<PDMA_ISCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_ISCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_ISCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_ISCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_ISCR1` writer"]
pub struct W(crate::W<PDMA_ISCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_ISCR1_SPEC>;
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
impl From<crate::W<PDMA_ISCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_ISCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEICLR6` reader - GEICLR6"]
pub type GEICLR6_R = crate::BitReader;
#[doc = "Field `GEICLR6` writer - GEICLR6"]
pub type GEICLR6_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `BEICLR6` reader - BEICLR6"]
pub type BEICLR6_R = crate::BitReader;
#[doc = "Field `BEICLR6` writer - BEICLR6"]
pub type BEICLR6_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `HTICLR6` reader - HTICLR6"]
pub type HTICLR6_R = crate::BitReader;
#[doc = "Field `HTICLR6` writer - HTICLR6"]
pub type HTICLR6_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `TCICLR6` reader - TCICLR6"]
pub type TCICLR6_R = crate::BitReader;
#[doc = "Field `TCICLR6` writer - TCICLR6"]
pub type TCICLR6_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `TEICLR6` reader - TEICLR6"]
pub type TEICLR6_R = crate::BitReader;
#[doc = "Field `TEICLR6` writer - TEICLR6"]
pub type TEICLR6_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `GEICLR7` reader - GEICLR7"]
pub type GEICLR7_R = crate::BitReader;
#[doc = "Field `GEICLR7` writer - GEICLR7"]
pub type GEICLR7_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `BEICLR7` reader - BEICLR7"]
pub type BEICLR7_R = crate::BitReader;
#[doc = "Field `BEICLR7` writer - BEICLR7"]
pub type BEICLR7_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `HTICLR7` reader - HTICLR7"]
pub type HTICLR7_R = crate::BitReader;
#[doc = "Field `HTICLR7` writer - HTICLR7"]
pub type HTICLR7_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `TCICLR7` reader - TCICLR7"]
pub type TCICLR7_R = crate::BitReader;
#[doc = "Field `TCICLR7` writer - TCICLR7"]
pub type TCICLR7_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `TEICLR7` reader - TEICLR7"]
pub type TEICLR7_R = crate::BitReader;
#[doc = "Field `TEICLR7` writer - TEICLR7"]
pub type TEICLR7_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `GEICLR8` reader - GEICLR8"]
pub type GEICLR8_R = crate::BitReader;
#[doc = "Field `GEICLR8` writer - GEICLR8"]
pub type GEICLR8_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `BEICLR8` reader - BEICLR8"]
pub type BEICLR8_R = crate::BitReader;
#[doc = "Field `BEICLR8` writer - BEICLR8"]
pub type BEICLR8_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `HTICLR8` reader - HTICLR8"]
pub type HTICLR8_R = crate::BitReader;
#[doc = "Field `HTICLR8` writer - HTICLR8"]
pub type HTICLR8_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `TCICLR8` reader - TCICLR8"]
pub type TCICLR8_R = crate::BitReader;
#[doc = "Field `TCICLR8` writer - TCICLR8"]
pub type TCICLR8_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `TEICLR8` reader - TEICLR8"]
pub type TEICLR8_R = crate::BitReader;
#[doc = "Field `TEICLR8` writer - TEICLR8"]
pub type TEICLR8_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `GEICLR9` reader - GEICLR9"]
pub type GEICLR9_R = crate::BitReader;
#[doc = "Field `GEICLR9` writer - GEICLR9"]
pub type GEICLR9_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `BEICLR9` reader - BEICLR9"]
pub type BEICLR9_R = crate::BitReader;
#[doc = "Field `BEICLR9` writer - BEICLR9"]
pub type BEICLR9_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `HTICLR9` reader - HTICLR9"]
pub type HTICLR9_R = crate::BitReader;
#[doc = "Field `HTICLR9` writer - HTICLR9"]
pub type HTICLR9_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `TCICLR9` reader - TCICLR9"]
pub type TCICLR9_R = crate::BitReader;
#[doc = "Field `TCICLR9` writer - TCICLR9"]
pub type TCICLR9_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `TEICLR9` reader - TEICLR9"]
pub type TEICLR9_R = crate::BitReader;
#[doc = "Field `TEICLR9` writer - TEICLR9"]
pub type TEICLR9_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `GEICLR10` reader - GEICLR10"]
pub type GEICLR10_R = crate::BitReader;
#[doc = "Field `GEICLR10` writer - GEICLR10"]
pub type GEICLR10_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `BEICLR10` reader - BEICLR10"]
pub type BEICLR10_R = crate::BitReader;
#[doc = "Field `BEICLR10` writer - BEICLR10"]
pub type BEICLR10_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `HTICLR10` reader - HTICLR10"]
pub type HTICLR10_R = crate::BitReader;
#[doc = "Field `HTICLR10` writer - HTICLR10"]
pub type HTICLR10_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `TCICLR10` reader - TCICLR10"]
pub type TCICLR10_R = crate::BitReader;
#[doc = "Field `TCICLR10` writer - TCICLR10"]
pub type TCICLR10_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `TEICLR10` reader - TEICLR10"]
pub type TEICLR10_R = crate::BitReader;
#[doc = "Field `TEICLR10` writer - TEICLR10"]
pub type TEICLR10_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `GEICLR11` reader - GEICLR11"]
pub type GEICLR11_R = crate::BitReader;
#[doc = "Field `GEICLR11` writer - GEICLR11"]
pub type GEICLR11_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `BEICLR11` reader - BEICLR11"]
pub type BEICLR11_R = crate::BitReader;
#[doc = "Field `BEICLR11` writer - BEICLR11"]
pub type BEICLR11_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `HTICLR11` reader - HTICLR11"]
pub type HTICLR11_R = crate::BitReader;
#[doc = "Field `HTICLR11` writer - HTICLR11"]
pub type HTICLR11_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `TCICLR11` reader - TCICLR11"]
pub type TCICLR11_R = crate::BitReader;
#[doc = "Field `TCICLR11` writer - TCICLR11"]
pub type TCICLR11_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
#[doc = "Field `TEICLR11` reader - TEICLR11"]
pub type TEICLR11_R = crate::BitReader;
#[doc = "Field `TEICLR11` writer - TEICLR11"]
pub type TEICLR11_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISCR1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - GEICLR6"]
    #[inline(always)]
    pub fn geiclr6(&self) -> GEICLR6_R {
        GEICLR6_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BEICLR6"]
    #[inline(always)]
    pub fn beiclr6(&self) -> BEICLR6_R {
        BEICLR6_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HTICLR6"]
    #[inline(always)]
    pub fn hticlr6(&self) -> HTICLR6_R {
        HTICLR6_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TCICLR6"]
    #[inline(always)]
    pub fn tciclr6(&self) -> TCICLR6_R {
        TCICLR6_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TEICLR6"]
    #[inline(always)]
    pub fn teiclr6(&self) -> TEICLR6_R {
        TEICLR6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GEICLR7"]
    #[inline(always)]
    pub fn geiclr7(&self) -> GEICLR7_R {
        GEICLR7_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BEICLR7"]
    #[inline(always)]
    pub fn beiclr7(&self) -> BEICLR7_R {
        BEICLR7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HTICLR7"]
    #[inline(always)]
    pub fn hticlr7(&self) -> HTICLR7_R {
        HTICLR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TCICLR7"]
    #[inline(always)]
    pub fn tciclr7(&self) -> TCICLR7_R {
        TCICLR7_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TEICLR7"]
    #[inline(always)]
    pub fn teiclr7(&self) -> TEICLR7_R {
        TEICLR7_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GEICLR8"]
    #[inline(always)]
    pub fn geiclr8(&self) -> GEICLR8_R {
        GEICLR8_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BEICLR8"]
    #[inline(always)]
    pub fn beiclr8(&self) -> BEICLR8_R {
        BEICLR8_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HTICLR8"]
    #[inline(always)]
    pub fn hticlr8(&self) -> HTICLR8_R {
        HTICLR8_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TCICLR8"]
    #[inline(always)]
    pub fn tciclr8(&self) -> TCICLR8_R {
        TCICLR8_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TEICLR8"]
    #[inline(always)]
    pub fn teiclr8(&self) -> TEICLR8_R {
        TEICLR8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GEICLR9"]
    #[inline(always)]
    pub fn geiclr9(&self) -> GEICLR9_R {
        GEICLR9_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - BEICLR9"]
    #[inline(always)]
    pub fn beiclr9(&self) -> BEICLR9_R {
        BEICLR9_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HTICLR9"]
    #[inline(always)]
    pub fn hticlr9(&self) -> HTICLR9_R {
        HTICLR9_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TCICLR9"]
    #[inline(always)]
    pub fn tciclr9(&self) -> TCICLR9_R {
        TCICLR9_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TEICLR9"]
    #[inline(always)]
    pub fn teiclr9(&self) -> TEICLR9_R {
        TEICLR9_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GEICLR10"]
    #[inline(always)]
    pub fn geiclr10(&self) -> GEICLR10_R {
        GEICLR10_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BEICLR10"]
    #[inline(always)]
    pub fn beiclr10(&self) -> BEICLR10_R {
        BEICLR10_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - HTICLR10"]
    #[inline(always)]
    pub fn hticlr10(&self) -> HTICLR10_R {
        HTICLR10_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TCICLR10"]
    #[inline(always)]
    pub fn tciclr10(&self) -> TCICLR10_R {
        TCICLR10_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TEICLR10"]
    #[inline(always)]
    pub fn teiclr10(&self) -> TEICLR10_R {
        TEICLR10_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GEICLR11"]
    #[inline(always)]
    pub fn geiclr11(&self) -> GEICLR11_R {
        GEICLR11_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - BEICLR11"]
    #[inline(always)]
    pub fn beiclr11(&self) -> BEICLR11_R {
        BEICLR11_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - HTICLR11"]
    #[inline(always)]
    pub fn hticlr11(&self) -> HTICLR11_R {
        HTICLR11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TCICLR11"]
    #[inline(always)]
    pub fn tciclr11(&self) -> TCICLR11_R {
        TCICLR11_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TEICLR11"]
    #[inline(always)]
    pub fn teiclr11(&self) -> TEICLR11_R {
        TEICLR11_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GEICLR6"]
    #[inline(always)]
    #[must_use]
    pub fn geiclr6(&mut self) -> GEICLR6_W<0> {
        GEICLR6_W::new(self)
    }
    #[doc = "Bit 1 - BEICLR6"]
    #[inline(always)]
    #[must_use]
    pub fn beiclr6(&mut self) -> BEICLR6_W<1> {
        BEICLR6_W::new(self)
    }
    #[doc = "Bit 2 - HTICLR6"]
    #[inline(always)]
    #[must_use]
    pub fn hticlr6(&mut self) -> HTICLR6_W<2> {
        HTICLR6_W::new(self)
    }
    #[doc = "Bit 3 - TCICLR6"]
    #[inline(always)]
    #[must_use]
    pub fn tciclr6(&mut self) -> TCICLR6_W<3> {
        TCICLR6_W::new(self)
    }
    #[doc = "Bit 4 - TEICLR6"]
    #[inline(always)]
    #[must_use]
    pub fn teiclr6(&mut self) -> TEICLR6_W<4> {
        TEICLR6_W::new(self)
    }
    #[doc = "Bit 5 - GEICLR7"]
    #[inline(always)]
    #[must_use]
    pub fn geiclr7(&mut self) -> GEICLR7_W<5> {
        GEICLR7_W::new(self)
    }
    #[doc = "Bit 6 - BEICLR7"]
    #[inline(always)]
    #[must_use]
    pub fn beiclr7(&mut self) -> BEICLR7_W<6> {
        BEICLR7_W::new(self)
    }
    #[doc = "Bit 7 - HTICLR7"]
    #[inline(always)]
    #[must_use]
    pub fn hticlr7(&mut self) -> HTICLR7_W<7> {
        HTICLR7_W::new(self)
    }
    #[doc = "Bit 8 - TCICLR7"]
    #[inline(always)]
    #[must_use]
    pub fn tciclr7(&mut self) -> TCICLR7_W<8> {
        TCICLR7_W::new(self)
    }
    #[doc = "Bit 9 - TEICLR7"]
    #[inline(always)]
    #[must_use]
    pub fn teiclr7(&mut self) -> TEICLR7_W<9> {
        TEICLR7_W::new(self)
    }
    #[doc = "Bit 10 - GEICLR8"]
    #[inline(always)]
    #[must_use]
    pub fn geiclr8(&mut self) -> GEICLR8_W<10> {
        GEICLR8_W::new(self)
    }
    #[doc = "Bit 11 - BEICLR8"]
    #[inline(always)]
    #[must_use]
    pub fn beiclr8(&mut self) -> BEICLR8_W<11> {
        BEICLR8_W::new(self)
    }
    #[doc = "Bit 12 - HTICLR8"]
    #[inline(always)]
    #[must_use]
    pub fn hticlr8(&mut self) -> HTICLR8_W<12> {
        HTICLR8_W::new(self)
    }
    #[doc = "Bit 13 - TCICLR8"]
    #[inline(always)]
    #[must_use]
    pub fn tciclr8(&mut self) -> TCICLR8_W<13> {
        TCICLR8_W::new(self)
    }
    #[doc = "Bit 14 - TEICLR8"]
    #[inline(always)]
    #[must_use]
    pub fn teiclr8(&mut self) -> TEICLR8_W<14> {
        TEICLR8_W::new(self)
    }
    #[doc = "Bit 15 - GEICLR9"]
    #[inline(always)]
    #[must_use]
    pub fn geiclr9(&mut self) -> GEICLR9_W<15> {
        GEICLR9_W::new(self)
    }
    #[doc = "Bit 16 - BEICLR9"]
    #[inline(always)]
    #[must_use]
    pub fn beiclr9(&mut self) -> BEICLR9_W<16> {
        BEICLR9_W::new(self)
    }
    #[doc = "Bit 17 - HTICLR9"]
    #[inline(always)]
    #[must_use]
    pub fn hticlr9(&mut self) -> HTICLR9_W<17> {
        HTICLR9_W::new(self)
    }
    #[doc = "Bit 18 - TCICLR9"]
    #[inline(always)]
    #[must_use]
    pub fn tciclr9(&mut self) -> TCICLR9_W<18> {
        TCICLR9_W::new(self)
    }
    #[doc = "Bit 19 - TEICLR9"]
    #[inline(always)]
    #[must_use]
    pub fn teiclr9(&mut self) -> TEICLR9_W<19> {
        TEICLR9_W::new(self)
    }
    #[doc = "Bit 20 - GEICLR10"]
    #[inline(always)]
    #[must_use]
    pub fn geiclr10(&mut self) -> GEICLR10_W<20> {
        GEICLR10_W::new(self)
    }
    #[doc = "Bit 21 - BEICLR10"]
    #[inline(always)]
    #[must_use]
    pub fn beiclr10(&mut self) -> BEICLR10_W<21> {
        BEICLR10_W::new(self)
    }
    #[doc = "Bit 22 - HTICLR10"]
    #[inline(always)]
    #[must_use]
    pub fn hticlr10(&mut self) -> HTICLR10_W<22> {
        HTICLR10_W::new(self)
    }
    #[doc = "Bit 23 - TCICLR10"]
    #[inline(always)]
    #[must_use]
    pub fn tciclr10(&mut self) -> TCICLR10_W<23> {
        TCICLR10_W::new(self)
    }
    #[doc = "Bit 24 - TEICLR10"]
    #[inline(always)]
    #[must_use]
    pub fn teiclr10(&mut self) -> TEICLR10_W<24> {
        TEICLR10_W::new(self)
    }
    #[doc = "Bit 25 - GEICLR11"]
    #[inline(always)]
    #[must_use]
    pub fn geiclr11(&mut self) -> GEICLR11_W<25> {
        GEICLR11_W::new(self)
    }
    #[doc = "Bit 26 - BEICLR11"]
    #[inline(always)]
    #[must_use]
    pub fn beiclr11(&mut self) -> BEICLR11_W<26> {
        BEICLR11_W::new(self)
    }
    #[doc = "Bit 27 - HTICLR11"]
    #[inline(always)]
    #[must_use]
    pub fn hticlr11(&mut self) -> HTICLR11_W<27> {
        HTICLR11_W::new(self)
    }
    #[doc = "Bit 28 - TCICLR11"]
    #[inline(always)]
    #[must_use]
    pub fn tciclr11(&mut self) -> TCICLR11_W<28> {
        TCICLR11_W::new(self)
    }
    #[doc = "Bit 29 - TEICLR11"]
    #[inline(always)]
    #[must_use]
    pub fn teiclr11(&mut self) -> TEICLR11_W<29> {
        TEICLR11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA_ISCR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_iscr1](index.html) module"]
pub struct PDMA_ISCR1_SPEC;
impl crate::RegisterSpec for PDMA_ISCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_iscr1::R](R) reader structure"]
impl crate::Readable for PDMA_ISCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_iscr1::W](W) writer structure"]
impl crate::Writable for PDMA_ISCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDMA_ISCR1 to value 0"]
impl crate::Resettable for PDMA_ISCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
