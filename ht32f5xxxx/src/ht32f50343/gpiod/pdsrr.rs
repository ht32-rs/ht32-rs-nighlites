#[doc = "Register `PDSRR` reader"]
pub struct R(crate::R<PDSRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDSRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDSRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDSRR` writer"]
pub struct W(crate::W<PDSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDSRR_SPEC>;
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
impl From<crate::W<PDSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDSRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDSET0` reader - PDSET0"]
pub type PDSET0_R = crate::BitReader;
#[doc = "Field `PDSET0` writer - PDSET0"]
pub type PDSET0_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET1` reader - PDSET1"]
pub type PDSET1_R = crate::BitReader;
#[doc = "Field `PDSET1` writer - PDSET1"]
pub type PDSET1_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET2` reader - PDSET2"]
pub type PDSET2_R = crate::BitReader;
#[doc = "Field `PDSET2` writer - PDSET2"]
pub type PDSET2_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET3` reader - PDSET3"]
pub type PDSET3_R = crate::BitReader;
#[doc = "Field `PDSET3` writer - PDSET3"]
pub type PDSET3_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET4` reader - PDSET4"]
pub type PDSET4_R = crate::BitReader;
#[doc = "Field `PDSET4` writer - PDSET4"]
pub type PDSET4_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDSET5` reader - PDSET5"]
pub type PDSET5_R = crate::BitReader;
#[doc = "Field `PDSET5` writer - PDSET5"]
pub type PDSET5_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST0` reader - PDRST0"]
pub type PDRST0_R = crate::BitReader;
#[doc = "Field `PDRST0` writer - PDRST0"]
pub type PDRST0_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST1` reader - PDRST1"]
pub type PDRST1_R = crate::BitReader;
#[doc = "Field `PDRST1` writer - PDRST1"]
pub type PDRST1_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST2` reader - PDRST2"]
pub type PDRST2_R = crate::BitReader;
#[doc = "Field `PDRST2` writer - PDRST2"]
pub type PDRST2_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST3` reader - PDRST3"]
pub type PDRST3_R = crate::BitReader;
#[doc = "Field `PDRST3` writer - PDRST3"]
pub type PDRST3_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST4` reader - PDRST4"]
pub type PDRST4_R = crate::BitReader;
#[doc = "Field `PDRST4` writer - PDRST4"]
pub type PDRST4_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
#[doc = "Field `PDRST5` reader - PDRST5"]
pub type PDRST5_R = crate::BitReader;
#[doc = "Field `PDRST5` writer - PDRST5"]
pub type PDRST5_W<'a, const O: u8> = crate::BitWriter<'a, PDSRR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDSET0"]
    #[inline(always)]
    pub fn pdset0(&self) -> PDSET0_R {
        PDSET0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDSET1"]
    #[inline(always)]
    pub fn pdset1(&self) -> PDSET1_R {
        PDSET1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDSET2"]
    #[inline(always)]
    pub fn pdset2(&self) -> PDSET2_R {
        PDSET2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDSET3"]
    #[inline(always)]
    pub fn pdset3(&self) -> PDSET3_R {
        PDSET3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PDSET4"]
    #[inline(always)]
    pub fn pdset4(&self) -> PDSET4_R {
        PDSET4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PDSET5"]
    #[inline(always)]
    pub fn pdset5(&self) -> PDSET5_R {
        PDSET5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - PDRST0"]
    #[inline(always)]
    pub fn pdrst0(&self) -> PDRST0_R {
        PDRST0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PDRST1"]
    #[inline(always)]
    pub fn pdrst1(&self) -> PDRST1_R {
        PDRST1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PDRST2"]
    #[inline(always)]
    pub fn pdrst2(&self) -> PDRST2_R {
        PDRST2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PDRST3"]
    #[inline(always)]
    pub fn pdrst3(&self) -> PDRST3_R {
        PDRST3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PDRST4"]
    #[inline(always)]
    pub fn pdrst4(&self) -> PDRST4_R {
        PDRST4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PDRST5"]
    #[inline(always)]
    pub fn pdrst5(&self) -> PDRST5_R {
        PDRST5_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDSET0"]
    #[inline(always)]
    #[must_use]
    pub fn pdset0(&mut self) -> PDSET0_W<0> {
        PDSET0_W::new(self)
    }
    #[doc = "Bit 1 - PDSET1"]
    #[inline(always)]
    #[must_use]
    pub fn pdset1(&mut self) -> PDSET1_W<1> {
        PDSET1_W::new(self)
    }
    #[doc = "Bit 2 - PDSET2"]
    #[inline(always)]
    #[must_use]
    pub fn pdset2(&mut self) -> PDSET2_W<2> {
        PDSET2_W::new(self)
    }
    #[doc = "Bit 3 - PDSET3"]
    #[inline(always)]
    #[must_use]
    pub fn pdset3(&mut self) -> PDSET3_W<3> {
        PDSET3_W::new(self)
    }
    #[doc = "Bit 4 - PDSET4"]
    #[inline(always)]
    #[must_use]
    pub fn pdset4(&mut self) -> PDSET4_W<4> {
        PDSET4_W::new(self)
    }
    #[doc = "Bit 5 - PDSET5"]
    #[inline(always)]
    #[must_use]
    pub fn pdset5(&mut self) -> PDSET5_W<5> {
        PDSET5_W::new(self)
    }
    #[doc = "Bit 16 - PDRST0"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst0(&mut self) -> PDRST0_W<16> {
        PDRST0_W::new(self)
    }
    #[doc = "Bit 17 - PDRST1"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst1(&mut self) -> PDRST1_W<17> {
        PDRST1_W::new(self)
    }
    #[doc = "Bit 18 - PDRST2"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst2(&mut self) -> PDRST2_W<18> {
        PDRST2_W::new(self)
    }
    #[doc = "Bit 19 - PDRST3"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst3(&mut self) -> PDRST3_W<19> {
        PDRST3_W::new(self)
    }
    #[doc = "Bit 20 - PDRST4"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst4(&mut self) -> PDRST4_W<20> {
        PDRST4_W::new(self)
    }
    #[doc = "Bit 21 - PDRST5"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst5(&mut self) -> PDRST5_W<21> {
        PDRST5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDSRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsrr](index.html) module"]
pub struct PDSRR_SPEC;
impl crate::RegisterSpec for PDSRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdsrr::R](R) reader structure"]
impl crate::Readable for PDSRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdsrr::W](W) writer structure"]
impl crate::Writable for PDSRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDSRR to value 0"]
impl crate::Resettable for PDSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
