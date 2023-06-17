#[doc = "Register `PDINER` reader"]
pub struct R(crate::R<PDINER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDINER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDINER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDINER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDINER` writer"]
pub struct W(crate::W<PDINER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDINER_SPEC>;
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
impl From<crate::W<PDINER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDINER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDINEN0` reader - PDINEN0"]
pub type PDINEN0_R = crate::BitReader;
#[doc = "Field `PDINEN0` writer - PDINEN0"]
pub type PDINEN0_W<'a, const O: u8> = crate::BitWriter<'a, PDINER_SPEC, O>;
#[doc = "Field `PDINEN1` reader - PDINEN1"]
pub type PDINEN1_R = crate::BitReader;
#[doc = "Field `PDINEN1` writer - PDINEN1"]
pub type PDINEN1_W<'a, const O: u8> = crate::BitWriter<'a, PDINER_SPEC, O>;
#[doc = "Field `PDINEN2` reader - PDINEN2"]
pub type PDINEN2_R = crate::BitReader;
#[doc = "Field `PDINEN2` writer - PDINEN2"]
pub type PDINEN2_W<'a, const O: u8> = crate::BitWriter<'a, PDINER_SPEC, O>;
#[doc = "Field `PDINEN3` reader - PDINEN3"]
pub type PDINEN3_R = crate::BitReader;
#[doc = "Field `PDINEN3` writer - PDINEN3"]
pub type PDINEN3_W<'a, const O: u8> = crate::BitWriter<'a, PDINER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDINEN0"]
    #[inline(always)]
    pub fn pdinen0(&self) -> PDINEN0_R {
        PDINEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDINEN1"]
    #[inline(always)]
    pub fn pdinen1(&self) -> PDINEN1_R {
        PDINEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDINEN2"]
    #[inline(always)]
    pub fn pdinen2(&self) -> PDINEN2_R {
        PDINEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDINEN3"]
    #[inline(always)]
    pub fn pdinen3(&self) -> PDINEN3_R {
        PDINEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDINEN0"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen0(&mut self) -> PDINEN0_W<0> {
        PDINEN0_W::new(self)
    }
    #[doc = "Bit 1 - PDINEN1"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen1(&mut self) -> PDINEN1_W<1> {
        PDINEN1_W::new(self)
    }
    #[doc = "Bit 2 - PDINEN2"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen2(&mut self) -> PDINEN2_W<2> {
        PDINEN2_W::new(self)
    }
    #[doc = "Bit 3 - PDINEN3"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen3(&mut self) -> PDINEN3_W<3> {
        PDINEN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDINER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdiner](index.html) module"]
pub struct PDINER_SPEC;
impl crate::RegisterSpec for PDINER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdiner::R](R) reader structure"]
impl crate::Readable for PDINER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdiner::W](W) writer structure"]
impl crate::Writable for PDINER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDINER to value 0"]
impl crate::Resettable for PDINER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
