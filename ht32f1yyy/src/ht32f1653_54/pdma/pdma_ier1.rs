#[doc = "Register `PDMA_IER1` reader"]
pub struct R(crate::R<PDMA_IER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_IER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_IER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_IER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_IER1` writer"]
pub struct W(crate::W<PDMA_IER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_IER1_SPEC>;
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
impl From<crate::W<PDMA_IER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_IER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEIE6` reader - GEIE6"]
pub type GEIE6_R = crate::BitReader;
#[doc = "Field `GEIE6` writer - GEIE6"]
pub type GEIE6_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `BEIE6` reader - BEIE6"]
pub type BEIE6_R = crate::BitReader;
#[doc = "Field `BEIE6` writer - BEIE6"]
pub type BEIE6_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `HTIE6` reader - HTIE6"]
pub type HTIE6_R = crate::BitReader;
#[doc = "Field `HTIE6` writer - HTIE6"]
pub type HTIE6_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `TCIE6` reader - TCIE6"]
pub type TCIE6_R = crate::BitReader;
#[doc = "Field `TCIE6` writer - TCIE6"]
pub type TCIE6_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `TEIE6` reader - TEIE6"]
pub type TEIE6_R = crate::BitReader;
#[doc = "Field `TEIE6` writer - TEIE6"]
pub type TEIE6_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `GEIE7` reader - GEIE7"]
pub type GEIE7_R = crate::BitReader;
#[doc = "Field `GEIE7` writer - GEIE7"]
pub type GEIE7_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `BEIE7` reader - BEIE7"]
pub type BEIE7_R = crate::BitReader;
#[doc = "Field `BEIE7` writer - BEIE7"]
pub type BEIE7_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `HTIE7` reader - HTIE7"]
pub type HTIE7_R = crate::BitReader;
#[doc = "Field `HTIE7` writer - HTIE7"]
pub type HTIE7_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `TCIE7` reader - TCIE7"]
pub type TCIE7_R = crate::BitReader;
#[doc = "Field `TCIE7` writer - TCIE7"]
pub type TCIE7_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `TEIE7` reader - TEIE7"]
pub type TEIE7_R = crate::BitReader;
#[doc = "Field `TEIE7` writer - TEIE7"]
pub type TEIE7_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `GEIE8` reader - GEIE8"]
pub type GEIE8_R = crate::BitReader;
#[doc = "Field `GEIE8` writer - GEIE8"]
pub type GEIE8_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `BEIE8` reader - BEIE8"]
pub type BEIE8_R = crate::BitReader;
#[doc = "Field `BEIE8` writer - BEIE8"]
pub type BEIE8_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `HTIE8` reader - HTIE8"]
pub type HTIE8_R = crate::BitReader;
#[doc = "Field `HTIE8` writer - HTIE8"]
pub type HTIE8_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `TCIE8` reader - TCIE8"]
pub type TCIE8_R = crate::BitReader;
#[doc = "Field `TCIE8` writer - TCIE8"]
pub type TCIE8_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `TEIE8` reader - TEIE8"]
pub type TEIE8_R = crate::BitReader;
#[doc = "Field `TEIE8` writer - TEIE8"]
pub type TEIE8_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `GEIE9` reader - GEIE9"]
pub type GEIE9_R = crate::BitReader;
#[doc = "Field `GEIE9` writer - GEIE9"]
pub type GEIE9_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `BEIE9` reader - BEIE9"]
pub type BEIE9_R = crate::BitReader;
#[doc = "Field `BEIE9` writer - BEIE9"]
pub type BEIE9_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `HTIE9` reader - HTIE9"]
pub type HTIE9_R = crate::BitReader;
#[doc = "Field `HTIE9` writer - HTIE9"]
pub type HTIE9_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `TCIE9` reader - TCIE9"]
pub type TCIE9_R = crate::BitReader;
#[doc = "Field `TCIE9` writer - TCIE9"]
pub type TCIE9_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `TEIE9` reader - TEIE9"]
pub type TEIE9_R = crate::BitReader;
#[doc = "Field `TEIE9` writer - TEIE9"]
pub type TEIE9_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `GEIE10` reader - GEIE10"]
pub type GEIE10_R = crate::BitReader;
#[doc = "Field `GEIE10` writer - GEIE10"]
pub type GEIE10_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `BEIE10` reader - BEIE10"]
pub type BEIE10_R = crate::BitReader;
#[doc = "Field `BEIE10` writer - BEIE10"]
pub type BEIE10_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `HTIE10` reader - HTIE10"]
pub type HTIE10_R = crate::BitReader;
#[doc = "Field `HTIE10` writer - HTIE10"]
pub type HTIE10_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `TCIE10` reader - TCIE10"]
pub type TCIE10_R = crate::BitReader;
#[doc = "Field `TCIE10` writer - TCIE10"]
pub type TCIE10_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `TEIE10` reader - TEIE10"]
pub type TEIE10_R = crate::BitReader;
#[doc = "Field `TEIE10` writer - TEIE10"]
pub type TEIE10_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `GEIE11` reader - GEIE11"]
pub type GEIE11_R = crate::BitReader;
#[doc = "Field `GEIE11` writer - GEIE11"]
pub type GEIE11_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `BEIE11` reader - BEIE11"]
pub type BEIE11_R = crate::BitReader;
#[doc = "Field `BEIE11` writer - BEIE11"]
pub type BEIE11_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `HTIE11` reader - HTIE11"]
pub type HTIE11_R = crate::BitReader;
#[doc = "Field `HTIE11` writer - HTIE11"]
pub type HTIE11_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `TCIE11` reader - TCIE11"]
pub type TCIE11_R = crate::BitReader;
#[doc = "Field `TCIE11` writer - TCIE11"]
pub type TCIE11_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
#[doc = "Field `TEIE11` reader - TEIE11"]
pub type TEIE11_R = crate::BitReader;
#[doc = "Field `TEIE11` writer - TEIE11"]
pub type TEIE11_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_IER1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - GEIE6"]
    #[inline(always)]
    pub fn geie6(&self) -> GEIE6_R {
        GEIE6_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BEIE6"]
    #[inline(always)]
    pub fn beie6(&self) -> BEIE6_R {
        BEIE6_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HTIE6"]
    #[inline(always)]
    pub fn htie6(&self) -> HTIE6_R {
        HTIE6_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TCIE6"]
    #[inline(always)]
    pub fn tcie6(&self) -> TCIE6_R {
        TCIE6_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TEIE6"]
    #[inline(always)]
    pub fn teie6(&self) -> TEIE6_R {
        TEIE6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GEIE7"]
    #[inline(always)]
    pub fn geie7(&self) -> GEIE7_R {
        GEIE7_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BEIE7"]
    #[inline(always)]
    pub fn beie7(&self) -> BEIE7_R {
        BEIE7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HTIE7"]
    #[inline(always)]
    pub fn htie7(&self) -> HTIE7_R {
        HTIE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TCIE7"]
    #[inline(always)]
    pub fn tcie7(&self) -> TCIE7_R {
        TCIE7_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TEIE7"]
    #[inline(always)]
    pub fn teie7(&self) -> TEIE7_R {
        TEIE7_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GEIE8"]
    #[inline(always)]
    pub fn geie8(&self) -> GEIE8_R {
        GEIE8_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BEIE8"]
    #[inline(always)]
    pub fn beie8(&self) -> BEIE8_R {
        BEIE8_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HTIE8"]
    #[inline(always)]
    pub fn htie8(&self) -> HTIE8_R {
        HTIE8_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TCIE8"]
    #[inline(always)]
    pub fn tcie8(&self) -> TCIE8_R {
        TCIE8_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TEIE8"]
    #[inline(always)]
    pub fn teie8(&self) -> TEIE8_R {
        TEIE8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GEIE9"]
    #[inline(always)]
    pub fn geie9(&self) -> GEIE9_R {
        GEIE9_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - BEIE9"]
    #[inline(always)]
    pub fn beie9(&self) -> BEIE9_R {
        BEIE9_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HTIE9"]
    #[inline(always)]
    pub fn htie9(&self) -> HTIE9_R {
        HTIE9_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TCIE9"]
    #[inline(always)]
    pub fn tcie9(&self) -> TCIE9_R {
        TCIE9_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TEIE9"]
    #[inline(always)]
    pub fn teie9(&self) -> TEIE9_R {
        TEIE9_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GEIE10"]
    #[inline(always)]
    pub fn geie10(&self) -> GEIE10_R {
        GEIE10_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BEIE10"]
    #[inline(always)]
    pub fn beie10(&self) -> BEIE10_R {
        BEIE10_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - HTIE10"]
    #[inline(always)]
    pub fn htie10(&self) -> HTIE10_R {
        HTIE10_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TCIE10"]
    #[inline(always)]
    pub fn tcie10(&self) -> TCIE10_R {
        TCIE10_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TEIE10"]
    #[inline(always)]
    pub fn teie10(&self) -> TEIE10_R {
        TEIE10_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GEIE11"]
    #[inline(always)]
    pub fn geie11(&self) -> GEIE11_R {
        GEIE11_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - BEIE11"]
    #[inline(always)]
    pub fn beie11(&self) -> BEIE11_R {
        BEIE11_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - HTIE11"]
    #[inline(always)]
    pub fn htie11(&self) -> HTIE11_R {
        HTIE11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TCIE11"]
    #[inline(always)]
    pub fn tcie11(&self) -> TCIE11_R {
        TCIE11_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TEIE11"]
    #[inline(always)]
    pub fn teie11(&self) -> TEIE11_R {
        TEIE11_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GEIE6"]
    #[inline(always)]
    #[must_use]
    pub fn geie6(&mut self) -> GEIE6_W<0> {
        GEIE6_W::new(self)
    }
    #[doc = "Bit 1 - BEIE6"]
    #[inline(always)]
    #[must_use]
    pub fn beie6(&mut self) -> BEIE6_W<1> {
        BEIE6_W::new(self)
    }
    #[doc = "Bit 2 - HTIE6"]
    #[inline(always)]
    #[must_use]
    pub fn htie6(&mut self) -> HTIE6_W<2> {
        HTIE6_W::new(self)
    }
    #[doc = "Bit 3 - TCIE6"]
    #[inline(always)]
    #[must_use]
    pub fn tcie6(&mut self) -> TCIE6_W<3> {
        TCIE6_W::new(self)
    }
    #[doc = "Bit 4 - TEIE6"]
    #[inline(always)]
    #[must_use]
    pub fn teie6(&mut self) -> TEIE6_W<4> {
        TEIE6_W::new(self)
    }
    #[doc = "Bit 5 - GEIE7"]
    #[inline(always)]
    #[must_use]
    pub fn geie7(&mut self) -> GEIE7_W<5> {
        GEIE7_W::new(self)
    }
    #[doc = "Bit 6 - BEIE7"]
    #[inline(always)]
    #[must_use]
    pub fn beie7(&mut self) -> BEIE7_W<6> {
        BEIE7_W::new(self)
    }
    #[doc = "Bit 7 - HTIE7"]
    #[inline(always)]
    #[must_use]
    pub fn htie7(&mut self) -> HTIE7_W<7> {
        HTIE7_W::new(self)
    }
    #[doc = "Bit 8 - TCIE7"]
    #[inline(always)]
    #[must_use]
    pub fn tcie7(&mut self) -> TCIE7_W<8> {
        TCIE7_W::new(self)
    }
    #[doc = "Bit 9 - TEIE7"]
    #[inline(always)]
    #[must_use]
    pub fn teie7(&mut self) -> TEIE7_W<9> {
        TEIE7_W::new(self)
    }
    #[doc = "Bit 10 - GEIE8"]
    #[inline(always)]
    #[must_use]
    pub fn geie8(&mut self) -> GEIE8_W<10> {
        GEIE8_W::new(self)
    }
    #[doc = "Bit 11 - BEIE8"]
    #[inline(always)]
    #[must_use]
    pub fn beie8(&mut self) -> BEIE8_W<11> {
        BEIE8_W::new(self)
    }
    #[doc = "Bit 12 - HTIE8"]
    #[inline(always)]
    #[must_use]
    pub fn htie8(&mut self) -> HTIE8_W<12> {
        HTIE8_W::new(self)
    }
    #[doc = "Bit 13 - TCIE8"]
    #[inline(always)]
    #[must_use]
    pub fn tcie8(&mut self) -> TCIE8_W<13> {
        TCIE8_W::new(self)
    }
    #[doc = "Bit 14 - TEIE8"]
    #[inline(always)]
    #[must_use]
    pub fn teie8(&mut self) -> TEIE8_W<14> {
        TEIE8_W::new(self)
    }
    #[doc = "Bit 15 - GEIE9"]
    #[inline(always)]
    #[must_use]
    pub fn geie9(&mut self) -> GEIE9_W<15> {
        GEIE9_W::new(self)
    }
    #[doc = "Bit 16 - BEIE9"]
    #[inline(always)]
    #[must_use]
    pub fn beie9(&mut self) -> BEIE9_W<16> {
        BEIE9_W::new(self)
    }
    #[doc = "Bit 17 - HTIE9"]
    #[inline(always)]
    #[must_use]
    pub fn htie9(&mut self) -> HTIE9_W<17> {
        HTIE9_W::new(self)
    }
    #[doc = "Bit 18 - TCIE9"]
    #[inline(always)]
    #[must_use]
    pub fn tcie9(&mut self) -> TCIE9_W<18> {
        TCIE9_W::new(self)
    }
    #[doc = "Bit 19 - TEIE9"]
    #[inline(always)]
    #[must_use]
    pub fn teie9(&mut self) -> TEIE9_W<19> {
        TEIE9_W::new(self)
    }
    #[doc = "Bit 20 - GEIE10"]
    #[inline(always)]
    #[must_use]
    pub fn geie10(&mut self) -> GEIE10_W<20> {
        GEIE10_W::new(self)
    }
    #[doc = "Bit 21 - BEIE10"]
    #[inline(always)]
    #[must_use]
    pub fn beie10(&mut self) -> BEIE10_W<21> {
        BEIE10_W::new(self)
    }
    #[doc = "Bit 22 - HTIE10"]
    #[inline(always)]
    #[must_use]
    pub fn htie10(&mut self) -> HTIE10_W<22> {
        HTIE10_W::new(self)
    }
    #[doc = "Bit 23 - TCIE10"]
    #[inline(always)]
    #[must_use]
    pub fn tcie10(&mut self) -> TCIE10_W<23> {
        TCIE10_W::new(self)
    }
    #[doc = "Bit 24 - TEIE10"]
    #[inline(always)]
    #[must_use]
    pub fn teie10(&mut self) -> TEIE10_W<24> {
        TEIE10_W::new(self)
    }
    #[doc = "Bit 25 - GEIE11"]
    #[inline(always)]
    #[must_use]
    pub fn geie11(&mut self) -> GEIE11_W<25> {
        GEIE11_W::new(self)
    }
    #[doc = "Bit 26 - BEIE11"]
    #[inline(always)]
    #[must_use]
    pub fn beie11(&mut self) -> BEIE11_W<26> {
        BEIE11_W::new(self)
    }
    #[doc = "Bit 27 - HTIE11"]
    #[inline(always)]
    #[must_use]
    pub fn htie11(&mut self) -> HTIE11_W<27> {
        HTIE11_W::new(self)
    }
    #[doc = "Bit 28 - TCIE11"]
    #[inline(always)]
    #[must_use]
    pub fn tcie11(&mut self) -> TCIE11_W<28> {
        TCIE11_W::new(self)
    }
    #[doc = "Bit 29 - TEIE11"]
    #[inline(always)]
    #[must_use]
    pub fn teie11(&mut self) -> TEIE11_W<29> {
        TEIE11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA_IER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ier1](index.html) module"]
pub struct PDMA_IER1_SPEC;
impl crate::RegisterSpec for PDMA_IER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_ier1::R](R) reader structure"]
impl crate::Readable for PDMA_IER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_ier1::W](W) writer structure"]
impl crate::Writable for PDMA_IER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDMA_IER1 to value 0"]
impl crate::Resettable for PDMA_IER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
