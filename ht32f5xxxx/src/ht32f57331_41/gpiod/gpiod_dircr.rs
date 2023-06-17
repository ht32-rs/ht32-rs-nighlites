#[doc = "Register `GPIOD_DIRCR` reader"]
pub struct R(crate::R<GPIOD_DIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOD_DIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOD_DIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOD_DIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOD_DIRCR` writer"]
pub struct W(crate::W<GPIOD_DIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOD_DIRCR_SPEC>;
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
impl From<crate::W<GPIOD_DIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOD_DIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR0` reader - DIR0"]
pub type DIR0_R = crate::BitReader;
#[doc = "Field `DIR0` writer - DIR0"]
pub type DIR0_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_DIRCR_SPEC, O>;
#[doc = "Field `DIR1` reader - DIR1"]
pub type DIR1_R = crate::BitReader;
#[doc = "Field `DIR1` writer - DIR1"]
pub type DIR1_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_DIRCR_SPEC, O>;
#[doc = "Field `DIR2` reader - DIR2"]
pub type DIR2_R = crate::BitReader;
#[doc = "Field `DIR2` writer - DIR2"]
pub type DIR2_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_DIRCR_SPEC, O>;
#[doc = "Field `DIR3` reader - DIR3"]
pub type DIR3_R = crate::BitReader;
#[doc = "Field `DIR3` writer - DIR3"]
pub type DIR3_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_DIRCR_SPEC, O>;
#[doc = "Field `DIR4` reader - DIR4"]
pub type DIR4_R = crate::BitReader;
#[doc = "Field `DIR4` writer - DIR4"]
pub type DIR4_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_DIRCR_SPEC, O>;
#[doc = "Field `DIR5` reader - DIR5"]
pub type DIR5_R = crate::BitReader;
#[doc = "Field `DIR5` writer - DIR5"]
pub type DIR5_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_DIRCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - DIR0"]
    #[inline(always)]
    pub fn dir0(&self) -> DIR0_R {
        DIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIR1"]
    #[inline(always)]
    pub fn dir1(&self) -> DIR1_R {
        DIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIR2"]
    #[inline(always)]
    pub fn dir2(&self) -> DIR2_R {
        DIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIR3"]
    #[inline(always)]
    pub fn dir3(&self) -> DIR3_R {
        DIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIR4"]
    #[inline(always)]
    pub fn dir4(&self) -> DIR4_R {
        DIR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DIR5"]
    #[inline(always)]
    pub fn dir5(&self) -> DIR5_R {
        DIR5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIR0"]
    #[inline(always)]
    #[must_use]
    pub fn dir0(&mut self) -> DIR0_W<0> {
        DIR0_W::new(self)
    }
    #[doc = "Bit 1 - DIR1"]
    #[inline(always)]
    #[must_use]
    pub fn dir1(&mut self) -> DIR1_W<1> {
        DIR1_W::new(self)
    }
    #[doc = "Bit 2 - DIR2"]
    #[inline(always)]
    #[must_use]
    pub fn dir2(&mut self) -> DIR2_W<2> {
        DIR2_W::new(self)
    }
    #[doc = "Bit 3 - DIR3"]
    #[inline(always)]
    #[must_use]
    pub fn dir3(&mut self) -> DIR3_W<3> {
        DIR3_W::new(self)
    }
    #[doc = "Bit 4 - DIR4"]
    #[inline(always)]
    #[must_use]
    pub fn dir4(&mut self) -> DIR4_W<4> {
        DIR4_W::new(self)
    }
    #[doc = "Bit 5 - DIR5"]
    #[inline(always)]
    #[must_use]
    pub fn dir5(&mut self) -> DIR5_W<5> {
        DIR5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOD_DIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_dircr](index.html) module"]
pub struct GPIOD_DIRCR_SPEC;
impl crate::RegisterSpec for GPIOD_DIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiod_dircr::R](R) reader structure"]
impl crate::Readable for GPIOD_DIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiod_dircr::W](W) writer structure"]
impl crate::Writable for GPIOD_DIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOD_DIRCR to value 0"]
impl crate::Resettable for GPIOD_DIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
