#[doc = "Register `PCSCER` reader"]
pub struct R(crate::R<PCSCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCSCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCSCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCSCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCSCER` writer"]
pub struct W(crate::W<PCSCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCSCER_SPEC>;
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
impl From<crate::W<PCSCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCSCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCSCE0` reader - PCSCE0"]
pub type PCSCE0_R = crate::BitReader;
#[doc = "Field `PCSCE0` writer - PCSCE0"]
pub type PCSCE0_W<'a, const O: u8> = crate::BitWriter<'a, PCSCER_SPEC, O>;
#[doc = "Field `PCSCE1` reader - PCSCE1"]
pub type PCSCE1_R = crate::BitReader;
#[doc = "Field `PCSCE1` writer - PCSCE1"]
pub type PCSCE1_W<'a, const O: u8> = crate::BitWriter<'a, PCSCER_SPEC, O>;
#[doc = "Field `PCSCE2` reader - PCSCE2"]
pub type PCSCE2_R = crate::BitReader;
#[doc = "Field `PCSCE2` writer - PCSCE2"]
pub type PCSCE2_W<'a, const O: u8> = crate::BitWriter<'a, PCSCER_SPEC, O>;
#[doc = "Field `PCSCE3` reader - PCSCE3"]
pub type PCSCE3_R = crate::BitReader;
#[doc = "Field `PCSCE3` writer - PCSCE3"]
pub type PCSCE3_W<'a, const O: u8> = crate::BitWriter<'a, PCSCER_SPEC, O>;
#[doc = "Field `PCSCE4` reader - PCSCE4"]
pub type PCSCE4_R = crate::BitReader;
#[doc = "Field `PCSCE4` writer - PCSCE4"]
pub type PCSCE4_W<'a, const O: u8> = crate::BitWriter<'a, PCSCER_SPEC, O>;
#[doc = "Field `PCSCE5` reader - PCSCE5"]
pub type PCSCE5_R = crate::BitReader;
#[doc = "Field `PCSCE5` writer - PCSCE5"]
pub type PCSCE5_W<'a, const O: u8> = crate::BitWriter<'a, PCSCER_SPEC, O>;
#[doc = "Field `PCSCE6` reader - PCSCE6"]
pub type PCSCE6_R = crate::BitReader;
#[doc = "Field `PCSCE6` writer - PCSCE6"]
pub type PCSCE6_W<'a, const O: u8> = crate::BitWriter<'a, PCSCER_SPEC, O>;
#[doc = "Field `PCSCE7` reader - PCSCE7"]
pub type PCSCE7_R = crate::BitReader;
#[doc = "Field `PCSCE7` writer - PCSCE7"]
pub type PCSCE7_W<'a, const O: u8> = crate::BitWriter<'a, PCSCER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PCSCE0"]
    #[inline(always)]
    pub fn pcsce0(&self) -> PCSCE0_R {
        PCSCE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCSCE1"]
    #[inline(always)]
    pub fn pcsce1(&self) -> PCSCE1_R {
        PCSCE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCSCE2"]
    #[inline(always)]
    pub fn pcsce2(&self) -> PCSCE2_R {
        PCSCE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCSCE3"]
    #[inline(always)]
    pub fn pcsce3(&self) -> PCSCE3_R {
        PCSCE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PCSCE4"]
    #[inline(always)]
    pub fn pcsce4(&self) -> PCSCE4_R {
        PCSCE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCSCE5"]
    #[inline(always)]
    pub fn pcsce5(&self) -> PCSCE5_R {
        PCSCE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCSCE6"]
    #[inline(always)]
    pub fn pcsce6(&self) -> PCSCE6_R {
        PCSCE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PCSCE7"]
    #[inline(always)]
    pub fn pcsce7(&self) -> PCSCE7_R {
        PCSCE7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCSCE0"]
    #[inline(always)]
    #[must_use]
    pub fn pcsce0(&mut self) -> PCSCE0_W<0> {
        PCSCE0_W::new(self)
    }
    #[doc = "Bit 1 - PCSCE1"]
    #[inline(always)]
    #[must_use]
    pub fn pcsce1(&mut self) -> PCSCE1_W<1> {
        PCSCE1_W::new(self)
    }
    #[doc = "Bit 2 - PCSCE2"]
    #[inline(always)]
    #[must_use]
    pub fn pcsce2(&mut self) -> PCSCE2_W<2> {
        PCSCE2_W::new(self)
    }
    #[doc = "Bit 3 - PCSCE3"]
    #[inline(always)]
    #[must_use]
    pub fn pcsce3(&mut self) -> PCSCE3_W<3> {
        PCSCE3_W::new(self)
    }
    #[doc = "Bit 4 - PCSCE4"]
    #[inline(always)]
    #[must_use]
    pub fn pcsce4(&mut self) -> PCSCE4_W<4> {
        PCSCE4_W::new(self)
    }
    #[doc = "Bit 5 - PCSCE5"]
    #[inline(always)]
    #[must_use]
    pub fn pcsce5(&mut self) -> PCSCE5_W<5> {
        PCSCE5_W::new(self)
    }
    #[doc = "Bit 6 - PCSCE6"]
    #[inline(always)]
    #[must_use]
    pub fn pcsce6(&mut self) -> PCSCE6_W<6> {
        PCSCE6_W::new(self)
    }
    #[doc = "Bit 7 - PCSCE7"]
    #[inline(always)]
    #[must_use]
    pub fn pcsce7(&mut self) -> PCSCE7_W<7> {
        PCSCE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCSCER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcscer](index.html) module"]
pub struct PCSCER_SPEC;
impl crate::RegisterSpec for PCSCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcscer::R](R) reader structure"]
impl crate::Readable for PCSCER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcscer::W](W) writer structure"]
impl crate::Writable for PCSCER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCSCER to value 0"]
impl crate::Resettable for PCSCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
