#[doc = "Register `PEINER` reader"]
pub struct R(crate::R<PEINER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEINER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEINER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEINER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEINER` writer"]
pub struct W(crate::W<PEINER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEINER_SPEC>;
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
impl From<crate::W<PEINER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEINER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEINEN0` reader - PEINEN0"]
pub type PEINEN0_R = crate::BitReader;
#[doc = "Field `PEINEN0` writer - PEINEN0"]
pub type PEINEN0_W<'a, const O: u8> = crate::BitWriter<'a, PEINER_SPEC, O>;
#[doc = "Field `PEINEN1` reader - PEINEN1"]
pub type PEINEN1_R = crate::BitReader;
#[doc = "Field `PEINEN1` writer - PEINEN1"]
pub type PEINEN1_W<'a, const O: u8> = crate::BitWriter<'a, PEINER_SPEC, O>;
#[doc = "Field `PEINEN2` reader - PEINEN2"]
pub type PEINEN2_R = crate::BitReader;
#[doc = "Field `PEINEN2` writer - PEINEN2"]
pub type PEINEN2_W<'a, const O: u8> = crate::BitWriter<'a, PEINER_SPEC, O>;
#[doc = "Field `PEINEN3` reader - PEINEN3"]
pub type PEINEN3_R = crate::BitReader;
#[doc = "Field `PEINEN3` writer - PEINEN3"]
pub type PEINEN3_W<'a, const O: u8> = crate::BitWriter<'a, PEINER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PEINEN0"]
    #[inline(always)]
    pub fn peinen0(&self) -> PEINEN0_R {
        PEINEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PEINEN1"]
    #[inline(always)]
    pub fn peinen1(&self) -> PEINEN1_R {
        PEINEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PEINEN2"]
    #[inline(always)]
    pub fn peinen2(&self) -> PEINEN2_R {
        PEINEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PEINEN3"]
    #[inline(always)]
    pub fn peinen3(&self) -> PEINEN3_R {
        PEINEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PEINEN0"]
    #[inline(always)]
    #[must_use]
    pub fn peinen0(&mut self) -> PEINEN0_W<0> {
        PEINEN0_W::new(self)
    }
    #[doc = "Bit 1 - PEINEN1"]
    #[inline(always)]
    #[must_use]
    pub fn peinen1(&mut self) -> PEINEN1_W<1> {
        PEINEN1_W::new(self)
    }
    #[doc = "Bit 2 - PEINEN2"]
    #[inline(always)]
    #[must_use]
    pub fn peinen2(&mut self) -> PEINEN2_W<2> {
        PEINEN2_W::new(self)
    }
    #[doc = "Bit 3 - PEINEN3"]
    #[inline(always)]
    #[must_use]
    pub fn peinen3(&mut self) -> PEINEN3_W<3> {
        PEINEN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PEINER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peiner](index.html) module"]
pub struct PEINER_SPEC;
impl crate::RegisterSpec for PEINER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peiner::R](R) reader structure"]
impl crate::Readable for PEINER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peiner::W](W) writer structure"]
impl crate::Writable for PEINER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PEINER to value 0"]
impl crate::Resettable for PEINER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
