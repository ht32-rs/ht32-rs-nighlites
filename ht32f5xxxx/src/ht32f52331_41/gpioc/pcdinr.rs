#[doc = "Register `PCDINR` reader"]
pub struct R(crate::R<PCDINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCDINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCDINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCDINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCDINR` writer"]
pub struct W(crate::W<PCDINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCDINR_SPEC>;
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
impl From<crate::W<PCDINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCDINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCDIN0` reader - PCDIN0"]
pub type PCDIN0_R = crate::BitReader;
#[doc = "Field `PCDIN0` writer - PCDIN0"]
pub type PCDIN0_W<'a, const O: u8> = crate::BitWriter<'a, PCDINR_SPEC, O>;
#[doc = "Field `PCDIN1` reader - PCDIN1"]
pub type PCDIN1_R = crate::BitReader;
#[doc = "Field `PCDIN1` writer - PCDIN1"]
pub type PCDIN1_W<'a, const O: u8> = crate::BitWriter<'a, PCDINR_SPEC, O>;
#[doc = "Field `PCDIN2` reader - PCDIN2"]
pub type PCDIN2_R = crate::BitReader;
#[doc = "Field `PCDIN2` writer - PCDIN2"]
pub type PCDIN2_W<'a, const O: u8> = crate::BitWriter<'a, PCDINR_SPEC, O>;
#[doc = "Field `PCDIN3` reader - PCDIN3"]
pub type PCDIN3_R = crate::BitReader;
#[doc = "Field `PCDIN3` writer - PCDIN3"]
pub type PCDIN3_W<'a, const O: u8> = crate::BitWriter<'a, PCDINR_SPEC, O>;
#[doc = "Field `PCDIN4` reader - PCDIN4"]
pub type PCDIN4_R = crate::BitReader;
#[doc = "Field `PCDIN4` writer - PCDIN4"]
pub type PCDIN4_W<'a, const O: u8> = crate::BitWriter<'a, PCDINR_SPEC, O>;
#[doc = "Field `PCDIN5` reader - PCDIN5"]
pub type PCDIN5_R = crate::BitReader;
#[doc = "Field `PCDIN5` writer - PCDIN5"]
pub type PCDIN5_W<'a, const O: u8> = crate::BitWriter<'a, PCDINR_SPEC, O>;
#[doc = "Field `PCDIN6` reader - PCDIN6"]
pub type PCDIN6_R = crate::BitReader;
#[doc = "Field `PCDIN6` writer - PCDIN6"]
pub type PCDIN6_W<'a, const O: u8> = crate::BitWriter<'a, PCDINR_SPEC, O>;
#[doc = "Field `PCDIN7` reader - PCDIN7"]
pub type PCDIN7_R = crate::BitReader;
#[doc = "Field `PCDIN7` writer - PCDIN7"]
pub type PCDIN7_W<'a, const O: u8> = crate::BitWriter<'a, PCDINR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PCDIN0"]
    #[inline(always)]
    pub fn pcdin0(&self) -> PCDIN0_R {
        PCDIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCDIN1"]
    #[inline(always)]
    pub fn pcdin1(&self) -> PCDIN1_R {
        PCDIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCDIN2"]
    #[inline(always)]
    pub fn pcdin2(&self) -> PCDIN2_R {
        PCDIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCDIN3"]
    #[inline(always)]
    pub fn pcdin3(&self) -> PCDIN3_R {
        PCDIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PCDIN4"]
    #[inline(always)]
    pub fn pcdin4(&self) -> PCDIN4_R {
        PCDIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCDIN5"]
    #[inline(always)]
    pub fn pcdin5(&self) -> PCDIN5_R {
        PCDIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCDIN6"]
    #[inline(always)]
    pub fn pcdin6(&self) -> PCDIN6_R {
        PCDIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PCDIN7"]
    #[inline(always)]
    pub fn pcdin7(&self) -> PCDIN7_R {
        PCDIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCDIN0"]
    #[inline(always)]
    #[must_use]
    pub fn pcdin0(&mut self) -> PCDIN0_W<0> {
        PCDIN0_W::new(self)
    }
    #[doc = "Bit 1 - PCDIN1"]
    #[inline(always)]
    #[must_use]
    pub fn pcdin1(&mut self) -> PCDIN1_W<1> {
        PCDIN1_W::new(self)
    }
    #[doc = "Bit 2 - PCDIN2"]
    #[inline(always)]
    #[must_use]
    pub fn pcdin2(&mut self) -> PCDIN2_W<2> {
        PCDIN2_W::new(self)
    }
    #[doc = "Bit 3 - PCDIN3"]
    #[inline(always)]
    #[must_use]
    pub fn pcdin3(&mut self) -> PCDIN3_W<3> {
        PCDIN3_W::new(self)
    }
    #[doc = "Bit 4 - PCDIN4"]
    #[inline(always)]
    #[must_use]
    pub fn pcdin4(&mut self) -> PCDIN4_W<4> {
        PCDIN4_W::new(self)
    }
    #[doc = "Bit 5 - PCDIN5"]
    #[inline(always)]
    #[must_use]
    pub fn pcdin5(&mut self) -> PCDIN5_W<5> {
        PCDIN5_W::new(self)
    }
    #[doc = "Bit 6 - PCDIN6"]
    #[inline(always)]
    #[must_use]
    pub fn pcdin6(&mut self) -> PCDIN6_W<6> {
        PCDIN6_W::new(self)
    }
    #[doc = "Bit 7 - PCDIN7"]
    #[inline(always)]
    #[must_use]
    pub fn pcdin7(&mut self) -> PCDIN7_W<7> {
        PCDIN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCDINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdinr](index.html) module"]
pub struct PCDINR_SPEC;
impl crate::RegisterSpec for PCDINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcdinr::R](R) reader structure"]
impl crate::Readable for PCDINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcdinr::W](W) writer structure"]
impl crate::Writable for PCDINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCDINR to value 0"]
impl crate::Resettable for PCDINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
