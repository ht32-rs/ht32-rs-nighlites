#[doc = "Register `PDMA_ISR1` reader"]
pub struct R(crate::R<PDMA_ISR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_ISR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_ISR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_ISR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_ISR1` writer"]
pub struct W(crate::W<PDMA_ISR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_ISR1_SPEC>;
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
impl From<crate::W<PDMA_ISR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_ISR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEISTA6` reader - GEISTA6"]
pub type GEISTA6_R = crate::BitReader;
#[doc = "Field `GEISTA6` writer - GEISTA6"]
pub type GEISTA6_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `BEISTA6` reader - BEISTA6"]
pub type BEISTA6_R = crate::BitReader;
#[doc = "Field `BEISTA6` writer - BEISTA6"]
pub type BEISTA6_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `HTISTA6` reader - HTISTA6"]
pub type HTISTA6_R = crate::BitReader;
#[doc = "Field `HTISTA6` writer - HTISTA6"]
pub type HTISTA6_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `TCISTA6` reader - TCISTA6"]
pub type TCISTA6_R = crate::BitReader;
#[doc = "Field `TCISTA6` writer - TCISTA6"]
pub type TCISTA6_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `TEISTA6` reader - TEISTA6"]
pub type TEISTA6_R = crate::BitReader;
#[doc = "Field `TEISTA6` writer - TEISTA6"]
pub type TEISTA6_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `GEISTA7` reader - GEISTA7"]
pub type GEISTA7_R = crate::BitReader;
#[doc = "Field `GEISTA7` writer - GEISTA7"]
pub type GEISTA7_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `BEISTA7` reader - BEISTA7"]
pub type BEISTA7_R = crate::BitReader;
#[doc = "Field `BEISTA7` writer - BEISTA7"]
pub type BEISTA7_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `HTISTA7` reader - HTISTA7"]
pub type HTISTA7_R = crate::BitReader;
#[doc = "Field `HTISTA7` writer - HTISTA7"]
pub type HTISTA7_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `TCISTA7` reader - TCISTA7"]
pub type TCISTA7_R = crate::BitReader;
#[doc = "Field `TCISTA7` writer - TCISTA7"]
pub type TCISTA7_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `TEISTA7` reader - TEISTA7"]
pub type TEISTA7_R = crate::BitReader;
#[doc = "Field `TEISTA7` writer - TEISTA7"]
pub type TEISTA7_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `GEISTA8` reader - GEISTA8"]
pub type GEISTA8_R = crate::BitReader;
#[doc = "Field `GEISTA8` writer - GEISTA8"]
pub type GEISTA8_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `BEISTA8` reader - BEISTA8"]
pub type BEISTA8_R = crate::BitReader;
#[doc = "Field `BEISTA8` writer - BEISTA8"]
pub type BEISTA8_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `HTISTA8` reader - HTISTA8"]
pub type HTISTA8_R = crate::BitReader;
#[doc = "Field `HTISTA8` writer - HTISTA8"]
pub type HTISTA8_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `TCISTA8` reader - TCISTA8"]
pub type TCISTA8_R = crate::BitReader;
#[doc = "Field `TCISTA8` writer - TCISTA8"]
pub type TCISTA8_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `TEISTA8` reader - TEISTA8"]
pub type TEISTA8_R = crate::BitReader;
#[doc = "Field `TEISTA8` writer - TEISTA8"]
pub type TEISTA8_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `GEISTA9` reader - GEISTA9"]
pub type GEISTA9_R = crate::BitReader;
#[doc = "Field `GEISTA9` writer - GEISTA9"]
pub type GEISTA9_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `BEISTA9` reader - BEISTA9"]
pub type BEISTA9_R = crate::BitReader;
#[doc = "Field `BEISTA9` writer - BEISTA9"]
pub type BEISTA9_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `HTISTA9` reader - HTISTA9"]
pub type HTISTA9_R = crate::BitReader;
#[doc = "Field `HTISTA9` writer - HTISTA9"]
pub type HTISTA9_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `TCISTA9` reader - TCISTA9"]
pub type TCISTA9_R = crate::BitReader;
#[doc = "Field `TCISTA9` writer - TCISTA9"]
pub type TCISTA9_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `TEISTA9` reader - TEISTA9"]
pub type TEISTA9_R = crate::BitReader;
#[doc = "Field `TEISTA9` writer - TEISTA9"]
pub type TEISTA9_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `GEISTA10` reader - GEISTA10"]
pub type GEISTA10_R = crate::BitReader;
#[doc = "Field `GEISTA10` writer - GEISTA10"]
pub type GEISTA10_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `BEISTA10` reader - BEISTA10"]
pub type BEISTA10_R = crate::BitReader;
#[doc = "Field `BEISTA10` writer - BEISTA10"]
pub type BEISTA10_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `HTISTA10` reader - HTISTA10"]
pub type HTISTA10_R = crate::BitReader;
#[doc = "Field `HTISTA10` writer - HTISTA10"]
pub type HTISTA10_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `TCISTA10` reader - TCISTA10"]
pub type TCISTA10_R = crate::BitReader;
#[doc = "Field `TCISTA10` writer - TCISTA10"]
pub type TCISTA10_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `TEISTA10` reader - TEISTA10"]
pub type TEISTA10_R = crate::BitReader;
#[doc = "Field `TEISTA10` writer - TEISTA10"]
pub type TEISTA10_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `GEISTA11` reader - GEISTA11"]
pub type GEISTA11_R = crate::BitReader;
#[doc = "Field `GEISTA11` writer - GEISTA11"]
pub type GEISTA11_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `BEISTA11` reader - BEISTA11"]
pub type BEISTA11_R = crate::BitReader;
#[doc = "Field `BEISTA11` writer - BEISTA11"]
pub type BEISTA11_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `HTISTA11` reader - HTISTA11"]
pub type HTISTA11_R = crate::BitReader;
#[doc = "Field `HTISTA11` writer - HTISTA11"]
pub type HTISTA11_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `TCISTA11` reader - TCISTA11"]
pub type TCISTA11_R = crate::BitReader;
#[doc = "Field `TCISTA11` writer - TCISTA11"]
pub type TCISTA11_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
#[doc = "Field `TEISTA11` reader - TEISTA11"]
pub type TEISTA11_R = crate::BitReader;
#[doc = "Field `TEISTA11` writer - TEISTA11"]
pub type TEISTA11_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_ISR1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - GEISTA6"]
    #[inline(always)]
    pub fn geista6(&self) -> GEISTA6_R {
        GEISTA6_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BEISTA6"]
    #[inline(always)]
    pub fn beista6(&self) -> BEISTA6_R {
        BEISTA6_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HTISTA6"]
    #[inline(always)]
    pub fn htista6(&self) -> HTISTA6_R {
        HTISTA6_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TCISTA6"]
    #[inline(always)]
    pub fn tcista6(&self) -> TCISTA6_R {
        TCISTA6_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TEISTA6"]
    #[inline(always)]
    pub fn teista6(&self) -> TEISTA6_R {
        TEISTA6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GEISTA7"]
    #[inline(always)]
    pub fn geista7(&self) -> GEISTA7_R {
        GEISTA7_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BEISTA7"]
    #[inline(always)]
    pub fn beista7(&self) -> BEISTA7_R {
        BEISTA7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HTISTA7"]
    #[inline(always)]
    pub fn htista7(&self) -> HTISTA7_R {
        HTISTA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TCISTA7"]
    #[inline(always)]
    pub fn tcista7(&self) -> TCISTA7_R {
        TCISTA7_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TEISTA7"]
    #[inline(always)]
    pub fn teista7(&self) -> TEISTA7_R {
        TEISTA7_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GEISTA8"]
    #[inline(always)]
    pub fn geista8(&self) -> GEISTA8_R {
        GEISTA8_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BEISTA8"]
    #[inline(always)]
    pub fn beista8(&self) -> BEISTA8_R {
        BEISTA8_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HTISTA8"]
    #[inline(always)]
    pub fn htista8(&self) -> HTISTA8_R {
        HTISTA8_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TCISTA8"]
    #[inline(always)]
    pub fn tcista8(&self) -> TCISTA8_R {
        TCISTA8_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TEISTA8"]
    #[inline(always)]
    pub fn teista8(&self) -> TEISTA8_R {
        TEISTA8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GEISTA9"]
    #[inline(always)]
    pub fn geista9(&self) -> GEISTA9_R {
        GEISTA9_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - BEISTA9"]
    #[inline(always)]
    pub fn beista9(&self) -> BEISTA9_R {
        BEISTA9_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HTISTA9"]
    #[inline(always)]
    pub fn htista9(&self) -> HTISTA9_R {
        HTISTA9_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TCISTA9"]
    #[inline(always)]
    pub fn tcista9(&self) -> TCISTA9_R {
        TCISTA9_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TEISTA9"]
    #[inline(always)]
    pub fn teista9(&self) -> TEISTA9_R {
        TEISTA9_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GEISTA10"]
    #[inline(always)]
    pub fn geista10(&self) -> GEISTA10_R {
        GEISTA10_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BEISTA10"]
    #[inline(always)]
    pub fn beista10(&self) -> BEISTA10_R {
        BEISTA10_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - HTISTA10"]
    #[inline(always)]
    pub fn htista10(&self) -> HTISTA10_R {
        HTISTA10_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TCISTA10"]
    #[inline(always)]
    pub fn tcista10(&self) -> TCISTA10_R {
        TCISTA10_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TEISTA10"]
    #[inline(always)]
    pub fn teista10(&self) -> TEISTA10_R {
        TEISTA10_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GEISTA11"]
    #[inline(always)]
    pub fn geista11(&self) -> GEISTA11_R {
        GEISTA11_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - BEISTA11"]
    #[inline(always)]
    pub fn beista11(&self) -> BEISTA11_R {
        BEISTA11_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - HTISTA11"]
    #[inline(always)]
    pub fn htista11(&self) -> HTISTA11_R {
        HTISTA11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TCISTA11"]
    #[inline(always)]
    pub fn tcista11(&self) -> TCISTA11_R {
        TCISTA11_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TEISTA11"]
    #[inline(always)]
    pub fn teista11(&self) -> TEISTA11_R {
        TEISTA11_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GEISTA6"]
    #[inline(always)]
    #[must_use]
    pub fn geista6(&mut self) -> GEISTA6_W<0> {
        GEISTA6_W::new(self)
    }
    #[doc = "Bit 1 - BEISTA6"]
    #[inline(always)]
    #[must_use]
    pub fn beista6(&mut self) -> BEISTA6_W<1> {
        BEISTA6_W::new(self)
    }
    #[doc = "Bit 2 - HTISTA6"]
    #[inline(always)]
    #[must_use]
    pub fn htista6(&mut self) -> HTISTA6_W<2> {
        HTISTA6_W::new(self)
    }
    #[doc = "Bit 3 - TCISTA6"]
    #[inline(always)]
    #[must_use]
    pub fn tcista6(&mut self) -> TCISTA6_W<3> {
        TCISTA6_W::new(self)
    }
    #[doc = "Bit 4 - TEISTA6"]
    #[inline(always)]
    #[must_use]
    pub fn teista6(&mut self) -> TEISTA6_W<4> {
        TEISTA6_W::new(self)
    }
    #[doc = "Bit 5 - GEISTA7"]
    #[inline(always)]
    #[must_use]
    pub fn geista7(&mut self) -> GEISTA7_W<5> {
        GEISTA7_W::new(self)
    }
    #[doc = "Bit 6 - BEISTA7"]
    #[inline(always)]
    #[must_use]
    pub fn beista7(&mut self) -> BEISTA7_W<6> {
        BEISTA7_W::new(self)
    }
    #[doc = "Bit 7 - HTISTA7"]
    #[inline(always)]
    #[must_use]
    pub fn htista7(&mut self) -> HTISTA7_W<7> {
        HTISTA7_W::new(self)
    }
    #[doc = "Bit 8 - TCISTA7"]
    #[inline(always)]
    #[must_use]
    pub fn tcista7(&mut self) -> TCISTA7_W<8> {
        TCISTA7_W::new(self)
    }
    #[doc = "Bit 9 - TEISTA7"]
    #[inline(always)]
    #[must_use]
    pub fn teista7(&mut self) -> TEISTA7_W<9> {
        TEISTA7_W::new(self)
    }
    #[doc = "Bit 10 - GEISTA8"]
    #[inline(always)]
    #[must_use]
    pub fn geista8(&mut self) -> GEISTA8_W<10> {
        GEISTA8_W::new(self)
    }
    #[doc = "Bit 11 - BEISTA8"]
    #[inline(always)]
    #[must_use]
    pub fn beista8(&mut self) -> BEISTA8_W<11> {
        BEISTA8_W::new(self)
    }
    #[doc = "Bit 12 - HTISTA8"]
    #[inline(always)]
    #[must_use]
    pub fn htista8(&mut self) -> HTISTA8_W<12> {
        HTISTA8_W::new(self)
    }
    #[doc = "Bit 13 - TCISTA8"]
    #[inline(always)]
    #[must_use]
    pub fn tcista8(&mut self) -> TCISTA8_W<13> {
        TCISTA8_W::new(self)
    }
    #[doc = "Bit 14 - TEISTA8"]
    #[inline(always)]
    #[must_use]
    pub fn teista8(&mut self) -> TEISTA8_W<14> {
        TEISTA8_W::new(self)
    }
    #[doc = "Bit 15 - GEISTA9"]
    #[inline(always)]
    #[must_use]
    pub fn geista9(&mut self) -> GEISTA9_W<15> {
        GEISTA9_W::new(self)
    }
    #[doc = "Bit 16 - BEISTA9"]
    #[inline(always)]
    #[must_use]
    pub fn beista9(&mut self) -> BEISTA9_W<16> {
        BEISTA9_W::new(self)
    }
    #[doc = "Bit 17 - HTISTA9"]
    #[inline(always)]
    #[must_use]
    pub fn htista9(&mut self) -> HTISTA9_W<17> {
        HTISTA9_W::new(self)
    }
    #[doc = "Bit 18 - TCISTA9"]
    #[inline(always)]
    #[must_use]
    pub fn tcista9(&mut self) -> TCISTA9_W<18> {
        TCISTA9_W::new(self)
    }
    #[doc = "Bit 19 - TEISTA9"]
    #[inline(always)]
    #[must_use]
    pub fn teista9(&mut self) -> TEISTA9_W<19> {
        TEISTA9_W::new(self)
    }
    #[doc = "Bit 20 - GEISTA10"]
    #[inline(always)]
    #[must_use]
    pub fn geista10(&mut self) -> GEISTA10_W<20> {
        GEISTA10_W::new(self)
    }
    #[doc = "Bit 21 - BEISTA10"]
    #[inline(always)]
    #[must_use]
    pub fn beista10(&mut self) -> BEISTA10_W<21> {
        BEISTA10_W::new(self)
    }
    #[doc = "Bit 22 - HTISTA10"]
    #[inline(always)]
    #[must_use]
    pub fn htista10(&mut self) -> HTISTA10_W<22> {
        HTISTA10_W::new(self)
    }
    #[doc = "Bit 23 - TCISTA10"]
    #[inline(always)]
    #[must_use]
    pub fn tcista10(&mut self) -> TCISTA10_W<23> {
        TCISTA10_W::new(self)
    }
    #[doc = "Bit 24 - TEISTA10"]
    #[inline(always)]
    #[must_use]
    pub fn teista10(&mut self) -> TEISTA10_W<24> {
        TEISTA10_W::new(self)
    }
    #[doc = "Bit 25 - GEISTA11"]
    #[inline(always)]
    #[must_use]
    pub fn geista11(&mut self) -> GEISTA11_W<25> {
        GEISTA11_W::new(self)
    }
    #[doc = "Bit 26 - BEISTA11"]
    #[inline(always)]
    #[must_use]
    pub fn beista11(&mut self) -> BEISTA11_W<26> {
        BEISTA11_W::new(self)
    }
    #[doc = "Bit 27 - HTISTA11"]
    #[inline(always)]
    #[must_use]
    pub fn htista11(&mut self) -> HTISTA11_W<27> {
        HTISTA11_W::new(self)
    }
    #[doc = "Bit 28 - TCISTA11"]
    #[inline(always)]
    #[must_use]
    pub fn tcista11(&mut self) -> TCISTA11_W<28> {
        TCISTA11_W::new(self)
    }
    #[doc = "Bit 29 - TEISTA11"]
    #[inline(always)]
    #[must_use]
    pub fn teista11(&mut self) -> TEISTA11_W<29> {
        TEISTA11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA_ISR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_isr1](index.html) module"]
pub struct PDMA_ISR1_SPEC;
impl crate::RegisterSpec for PDMA_ISR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_isr1::R](R) reader structure"]
impl crate::Readable for PDMA_ISR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_isr1::W](W) writer structure"]
impl crate::Writable for PDMA_ISR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDMA_ISR1 to value 0"]
impl crate::Resettable for PDMA_ISR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
