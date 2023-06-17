#[doc = "Register `PDDIRCR` reader"]
pub struct R(crate::R<PDDIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDDIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDDIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDDIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDDIRCR` writer"]
pub struct W(crate::W<PDDIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDDIRCR_SPEC>;
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
impl From<crate::W<PDDIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDDIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDDIR0` reader - PDDIR0"]
pub type PDDIR0_R = crate::BitReader;
#[doc = "Field `PDDIR0` writer - PDDIR0"]
pub type PDDIR0_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR1` reader - PDDIR1"]
pub type PDDIR1_R = crate::BitReader;
#[doc = "Field `PDDIR1` writer - PDDIR1"]
pub type PDDIR1_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR2` reader - PDDIR2"]
pub type PDDIR2_R = crate::BitReader;
#[doc = "Field `PDDIR2` writer - PDDIR2"]
pub type PDDIR2_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
#[doc = "Field `PDDIR3` reader - PDDIR3"]
pub type PDDIR3_R = crate::BitReader;
#[doc = "Field `PDDIR3` writer - PDDIR3"]
pub type PDDIR3_W<'a, const O: u8> = crate::BitWriter<'a, PDDIRCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDDIR0"]
    #[inline(always)]
    pub fn pddir0(&self) -> PDDIR0_R {
        PDDIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDDIR1"]
    #[inline(always)]
    pub fn pddir1(&self) -> PDDIR1_R {
        PDDIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDDIR2"]
    #[inline(always)]
    pub fn pddir2(&self) -> PDDIR2_R {
        PDDIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDDIR3"]
    #[inline(always)]
    pub fn pddir3(&self) -> PDDIR3_R {
        PDDIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDDIR0"]
    #[inline(always)]
    #[must_use]
    pub fn pddir0(&mut self) -> PDDIR0_W<0> {
        PDDIR0_W::new(self)
    }
    #[doc = "Bit 1 - PDDIR1"]
    #[inline(always)]
    #[must_use]
    pub fn pddir1(&mut self) -> PDDIR1_W<1> {
        PDDIR1_W::new(self)
    }
    #[doc = "Bit 2 - PDDIR2"]
    #[inline(always)]
    #[must_use]
    pub fn pddir2(&mut self) -> PDDIR2_W<2> {
        PDDIR2_W::new(self)
    }
    #[doc = "Bit 3 - PDDIR3"]
    #[inline(always)]
    #[must_use]
    pub fn pddir3(&mut self) -> PDDIR3_W<3> {
        PDDIR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDDIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddircr](index.html) module"]
pub struct PDDIRCR_SPEC;
impl crate::RegisterSpec for PDDIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pddircr::R](R) reader structure"]
impl crate::Readable for PDDIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pddircr::W](W) writer structure"]
impl crate::Writable for PDDIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDDIRCR to value 0"]
impl crate::Resettable for PDDIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
