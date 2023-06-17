#[doc = "Register `PCINER` reader"]
pub struct R(crate::R<PCINER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCINER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCINER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCINER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCINER` writer"]
pub struct W(crate::W<PCINER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCINER_SPEC>;
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
impl From<crate::W<PCINER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCINER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCINEN0` reader - PCINEN0"]
pub type PCINEN0_R = crate::BitReader;
#[doc = "Field `PCINEN0` writer - PCINEN0"]
pub type PCINEN0_W<'a, const O: u8> = crate::BitWriter<'a, PCINER_SPEC, O>;
#[doc = "Field `PCINEN1` reader - PCINEN1"]
pub type PCINEN1_R = crate::BitReader;
#[doc = "Field `PCINEN1` writer - PCINEN1"]
pub type PCINEN1_W<'a, const O: u8> = crate::BitWriter<'a, PCINER_SPEC, O>;
#[doc = "Field `PCINEN2` reader - PCINEN2"]
pub type PCINEN2_R = crate::BitReader;
#[doc = "Field `PCINEN2` writer - PCINEN2"]
pub type PCINEN2_W<'a, const O: u8> = crate::BitWriter<'a, PCINER_SPEC, O>;
#[doc = "Field `PCINEN3` reader - PCINEN3"]
pub type PCINEN3_R = crate::BitReader;
#[doc = "Field `PCINEN3` writer - PCINEN3"]
pub type PCINEN3_W<'a, const O: u8> = crate::BitWriter<'a, PCINER_SPEC, O>;
#[doc = "Field `PCINEN4` reader - PCINEN4"]
pub type PCINEN4_R = crate::BitReader;
#[doc = "Field `PCINEN4` writer - PCINEN4"]
pub type PCINEN4_W<'a, const O: u8> = crate::BitWriter<'a, PCINER_SPEC, O>;
#[doc = "Field `PCINEN5` reader - PCINEN5"]
pub type PCINEN5_R = crate::BitReader;
#[doc = "Field `PCINEN5` writer - PCINEN5"]
pub type PCINEN5_W<'a, const O: u8> = crate::BitWriter<'a, PCINER_SPEC, O>;
#[doc = "Field `PCINEN6` reader - PCINEN6"]
pub type PCINEN6_R = crate::BitReader;
#[doc = "Field `PCINEN6` writer - PCINEN6"]
pub type PCINEN6_W<'a, const O: u8> = crate::BitWriter<'a, PCINER_SPEC, O>;
#[doc = "Field `PCINEN7` reader - PCINEN7"]
pub type PCINEN7_R = crate::BitReader;
#[doc = "Field `PCINEN7` writer - PCINEN7"]
pub type PCINEN7_W<'a, const O: u8> = crate::BitWriter<'a, PCINER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PCINEN0"]
    #[inline(always)]
    pub fn pcinen0(&self) -> PCINEN0_R {
        PCINEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCINEN1"]
    #[inline(always)]
    pub fn pcinen1(&self) -> PCINEN1_R {
        PCINEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCINEN2"]
    #[inline(always)]
    pub fn pcinen2(&self) -> PCINEN2_R {
        PCINEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCINEN3"]
    #[inline(always)]
    pub fn pcinen3(&self) -> PCINEN3_R {
        PCINEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PCINEN4"]
    #[inline(always)]
    pub fn pcinen4(&self) -> PCINEN4_R {
        PCINEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCINEN5"]
    #[inline(always)]
    pub fn pcinen5(&self) -> PCINEN5_R {
        PCINEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCINEN6"]
    #[inline(always)]
    pub fn pcinen6(&self) -> PCINEN6_R {
        PCINEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PCINEN7"]
    #[inline(always)]
    pub fn pcinen7(&self) -> PCINEN7_R {
        PCINEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCINEN0"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen0(&mut self) -> PCINEN0_W<0> {
        PCINEN0_W::new(self)
    }
    #[doc = "Bit 1 - PCINEN1"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen1(&mut self) -> PCINEN1_W<1> {
        PCINEN1_W::new(self)
    }
    #[doc = "Bit 2 - PCINEN2"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen2(&mut self) -> PCINEN2_W<2> {
        PCINEN2_W::new(self)
    }
    #[doc = "Bit 3 - PCINEN3"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen3(&mut self) -> PCINEN3_W<3> {
        PCINEN3_W::new(self)
    }
    #[doc = "Bit 4 - PCINEN4"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen4(&mut self) -> PCINEN4_W<4> {
        PCINEN4_W::new(self)
    }
    #[doc = "Bit 5 - PCINEN5"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen5(&mut self) -> PCINEN5_W<5> {
        PCINEN5_W::new(self)
    }
    #[doc = "Bit 6 - PCINEN6"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen6(&mut self) -> PCINEN6_W<6> {
        PCINEN6_W::new(self)
    }
    #[doc = "Bit 7 - PCINEN7"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen7(&mut self) -> PCINEN7_W<7> {
        PCINEN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCINER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pciner](index.html) module"]
pub struct PCINER_SPEC;
impl crate::RegisterSpec for PCINER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pciner::R](R) reader structure"]
impl crate::Readable for PCINER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pciner::W](W) writer structure"]
impl crate::Writable for PCINER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCINER to value 0"]
impl crate::Resettable for PCINER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
