#[doc = "Register `PCSRR` reader"]
pub struct R(crate::R<PCSRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCSRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCSRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCSRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCSRR` writer"]
pub struct W(crate::W<PCSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCSRR_SPEC>;
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
impl From<crate::W<PCSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCSRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCSET0` reader - PCSET0"]
pub type PCSET0_R = crate::BitReader;
#[doc = "Field `PCSET0` writer - PCSET0"]
pub type PCSET0_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET1` reader - PCSET1"]
pub type PCSET1_R = crate::BitReader;
#[doc = "Field `PCSET1` writer - PCSET1"]
pub type PCSET1_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET2` reader - PCSET2"]
pub type PCSET2_R = crate::BitReader;
#[doc = "Field `PCSET2` writer - PCSET2"]
pub type PCSET2_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET3` reader - PCSET3"]
pub type PCSET3_R = crate::BitReader;
#[doc = "Field `PCSET3` writer - PCSET3"]
pub type PCSET3_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET4` reader - PCSET4"]
pub type PCSET4_R = crate::BitReader;
#[doc = "Field `PCSET4` writer - PCSET4"]
pub type PCSET4_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET5` reader - PCSET5"]
pub type PCSET5_R = crate::BitReader;
#[doc = "Field `PCSET5` writer - PCSET5"]
pub type PCSET5_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET6` reader - PCSET6"]
pub type PCSET6_R = crate::BitReader;
#[doc = "Field `PCSET6` writer - PCSET6"]
pub type PCSET6_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCSET7` reader - PCSET7"]
pub type PCSET7_R = crate::BitReader;
#[doc = "Field `PCSET7` writer - PCSET7"]
pub type PCSET7_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST0` reader - PCRST0"]
pub type PCRST0_R = crate::BitReader;
#[doc = "Field `PCRST0` writer - PCRST0"]
pub type PCRST0_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST1` reader - PCRST1"]
pub type PCRST1_R = crate::BitReader;
#[doc = "Field `PCRST1` writer - PCRST1"]
pub type PCRST1_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST2` reader - PCRST2"]
pub type PCRST2_R = crate::BitReader;
#[doc = "Field `PCRST2` writer - PCRST2"]
pub type PCRST2_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST3` reader - PCRST3"]
pub type PCRST3_R = crate::BitReader;
#[doc = "Field `PCRST3` writer - PCRST3"]
pub type PCRST3_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST4` reader - PCRST4"]
pub type PCRST4_R = crate::BitReader;
#[doc = "Field `PCRST4` writer - PCRST4"]
pub type PCRST4_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
#[doc = "Field `PCRST5` reader - PCRST5"]
pub type PCRST5_R = crate::BitReader;
#[doc = "Field `PCRST5` writer - PCRST5"]
pub type PCRST5_W<'a, const O: u8> = crate::BitWriter<'a, PCSRR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PCSET0"]
    #[inline(always)]
    pub fn pcset0(&self) -> PCSET0_R {
        PCSET0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCSET1"]
    #[inline(always)]
    pub fn pcset1(&self) -> PCSET1_R {
        PCSET1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCSET2"]
    #[inline(always)]
    pub fn pcset2(&self) -> PCSET2_R {
        PCSET2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCSET3"]
    #[inline(always)]
    pub fn pcset3(&self) -> PCSET3_R {
        PCSET3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PCSET4"]
    #[inline(always)]
    pub fn pcset4(&self) -> PCSET4_R {
        PCSET4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCSET5"]
    #[inline(always)]
    pub fn pcset5(&self) -> PCSET5_R {
        PCSET5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCSET6"]
    #[inline(always)]
    pub fn pcset6(&self) -> PCSET6_R {
        PCSET6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PCSET7"]
    #[inline(always)]
    pub fn pcset7(&self) -> PCSET7_R {
        PCSET7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - PCRST0"]
    #[inline(always)]
    pub fn pcrst0(&self) -> PCRST0_R {
        PCRST0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PCRST1"]
    #[inline(always)]
    pub fn pcrst1(&self) -> PCRST1_R {
        PCRST1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PCRST2"]
    #[inline(always)]
    pub fn pcrst2(&self) -> PCRST2_R {
        PCRST2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PCRST3"]
    #[inline(always)]
    pub fn pcrst3(&self) -> PCRST3_R {
        PCRST3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PCRST4"]
    #[inline(always)]
    pub fn pcrst4(&self) -> PCRST4_R {
        PCRST4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PCRST5"]
    #[inline(always)]
    pub fn pcrst5(&self) -> PCRST5_R {
        PCRST5_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCSET0"]
    #[inline(always)]
    #[must_use]
    pub fn pcset0(&mut self) -> PCSET0_W<0> {
        PCSET0_W::new(self)
    }
    #[doc = "Bit 1 - PCSET1"]
    #[inline(always)]
    #[must_use]
    pub fn pcset1(&mut self) -> PCSET1_W<1> {
        PCSET1_W::new(self)
    }
    #[doc = "Bit 2 - PCSET2"]
    #[inline(always)]
    #[must_use]
    pub fn pcset2(&mut self) -> PCSET2_W<2> {
        PCSET2_W::new(self)
    }
    #[doc = "Bit 3 - PCSET3"]
    #[inline(always)]
    #[must_use]
    pub fn pcset3(&mut self) -> PCSET3_W<3> {
        PCSET3_W::new(self)
    }
    #[doc = "Bit 4 - PCSET4"]
    #[inline(always)]
    #[must_use]
    pub fn pcset4(&mut self) -> PCSET4_W<4> {
        PCSET4_W::new(self)
    }
    #[doc = "Bit 5 - PCSET5"]
    #[inline(always)]
    #[must_use]
    pub fn pcset5(&mut self) -> PCSET5_W<5> {
        PCSET5_W::new(self)
    }
    #[doc = "Bit 6 - PCSET6"]
    #[inline(always)]
    #[must_use]
    pub fn pcset6(&mut self) -> PCSET6_W<6> {
        PCSET6_W::new(self)
    }
    #[doc = "Bit 7 - PCSET7"]
    #[inline(always)]
    #[must_use]
    pub fn pcset7(&mut self) -> PCSET7_W<7> {
        PCSET7_W::new(self)
    }
    #[doc = "Bit 16 - PCRST0"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst0(&mut self) -> PCRST0_W<16> {
        PCRST0_W::new(self)
    }
    #[doc = "Bit 17 - PCRST1"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst1(&mut self) -> PCRST1_W<17> {
        PCRST1_W::new(self)
    }
    #[doc = "Bit 18 - PCRST2"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst2(&mut self) -> PCRST2_W<18> {
        PCRST2_W::new(self)
    }
    #[doc = "Bit 19 - PCRST3"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst3(&mut self) -> PCRST3_W<19> {
        PCRST3_W::new(self)
    }
    #[doc = "Bit 20 - PCRST4"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst4(&mut self) -> PCRST4_W<20> {
        PCRST4_W::new(self)
    }
    #[doc = "Bit 21 - PCRST5"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst5(&mut self) -> PCRST5_W<21> {
        PCRST5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCSRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsrr](index.html) module"]
pub struct PCSRR_SPEC;
impl crate::RegisterSpec for PCSRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcsrr::R](R) reader structure"]
impl crate::Readable for PCSRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcsrr::W](W) writer structure"]
impl crate::Writable for PCSRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCSRR to value 0"]
impl crate::Resettable for PCSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
