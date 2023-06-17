#[doc = "Register `GPIOE_DIRCR` reader"]
pub struct R(crate::R<GPIOE_DIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOE_DIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOE_DIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOE_DIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOE_DIRCR` writer"]
pub struct W(crate::W<GPIOE_DIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOE_DIRCR_SPEC>;
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
impl From<crate::W<GPIOE_DIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOE_DIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR0` reader - DIR0"]
pub type DIR0_R = crate::BitReader;
#[doc = "Field `DIR0` writer - DIR0"]
pub type DIR0_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_DIRCR_SPEC, O>;
#[doc = "Field `DIR1` reader - DIR1"]
pub type DIR1_R = crate::BitReader;
#[doc = "Field `DIR1` writer - DIR1"]
pub type DIR1_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_DIRCR_SPEC, O>;
#[doc = "Field `DIR2` reader - DIR2"]
pub type DIR2_R = crate::BitReader;
#[doc = "Field `DIR2` writer - DIR2"]
pub type DIR2_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_DIRCR_SPEC, O>;
#[doc = "Field `DIR3` reader - DIR3"]
pub type DIR3_R = crate::BitReader;
#[doc = "Field `DIR3` writer - DIR3"]
pub type DIR3_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_DIRCR_SPEC, O>;
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOE_DIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_dircr](index.html) module"]
pub struct GPIOE_DIRCR_SPEC;
impl crate::RegisterSpec for GPIOE_DIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioe_dircr::R](R) reader structure"]
impl crate::Readable for GPIOE_DIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioe_dircr::W](W) writer structure"]
impl crate::Writable for GPIOE_DIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOE_DIRCR to value 0"]
impl crate::Resettable for GPIOE_DIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
