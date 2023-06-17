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
#[doc = "Field `PDINEN4` reader - PDINEN4"]
pub type PDINEN4_R = crate::BitReader;
#[doc = "Field `PDINEN4` writer - PDINEN4"]
pub type PDINEN4_W<'a, const O: u8> = crate::BitWriter<'a, PDINER_SPEC, O>;
#[doc = "Field `PDINEN5` reader - PDINEN5"]
pub type PDINEN5_R = crate::BitReader;
#[doc = "Field `PDINEN5` writer - PDINEN5"]
pub type PDINEN5_W<'a, const O: u8> = crate::BitWriter<'a, PDINER_SPEC, O>;
#[doc = "Field `PDINEN6` reader - PDINEN6"]
pub type PDINEN6_R = crate::BitReader;
#[doc = "Field `PDINEN6` writer - PDINEN6"]
pub type PDINEN6_W<'a, const O: u8> = crate::BitWriter<'a, PDINER_SPEC, O>;
#[doc = "Field `PDINEN7` reader - PDINEN7"]
pub type PDINEN7_R = crate::BitReader;
#[doc = "Field `PDINEN7` writer - PDINEN7"]
pub type PDINEN7_W<'a, const O: u8> = crate::BitWriter<'a, PDINER_SPEC, O>;
#[doc = "Field `PDINEN8` reader - PDINEN8"]
pub type PDINEN8_R = crate::BitReader;
#[doc = "Field `PDINEN8` writer - PDINEN8"]
pub type PDINEN8_W<'a, const O: u8> = crate::BitWriter<'a, PDINER_SPEC, O>;
#[doc = "Field `PDINEN9` reader - PDINEN9"]
pub type PDINEN9_R = crate::BitReader;
#[doc = "Field `PDINEN9` writer - PDINEN9"]
pub type PDINEN9_W<'a, const O: u8> = crate::BitWriter<'a, PDINER_SPEC, O>;
#[doc = "Field `PDINEN10` reader - PDINEN10"]
pub type PDINEN10_R = crate::BitReader;
#[doc = "Field `PDINEN10` writer - PDINEN10"]
pub type PDINEN10_W<'a, const O: u8> = crate::BitWriter<'a, PDINER_SPEC, O>;
#[doc = "Field `PDINEN11` reader - PDINEN11"]
pub type PDINEN11_R = crate::BitReader;
#[doc = "Field `PDINEN11` writer - PDINEN11"]
pub type PDINEN11_W<'a, const O: u8> = crate::BitWriter<'a, PDINER_SPEC, O>;
#[doc = "Field `PDINEN12` reader - PDINEN12"]
pub type PDINEN12_R = crate::BitReader;
#[doc = "Field `PDINEN12` writer - PDINEN12"]
pub type PDINEN12_W<'a, const O: u8> = crate::BitWriter<'a, PDINER_SPEC, O>;
#[doc = "Field `PDINEN13` reader - PDINEN13"]
pub type PDINEN13_R = crate::BitReader;
#[doc = "Field `PDINEN13` writer - PDINEN13"]
pub type PDINEN13_W<'a, const O: u8> = crate::BitWriter<'a, PDINER_SPEC, O>;
#[doc = "Field `PDINEN14` reader - PDINEN14"]
pub type PDINEN14_R = crate::BitReader;
#[doc = "Field `PDINEN14` writer - PDINEN14"]
pub type PDINEN14_W<'a, const O: u8> = crate::BitWriter<'a, PDINER_SPEC, O>;
#[doc = "Field `PDINEN15` reader - PDINEN15"]
pub type PDINEN15_R = crate::BitReader;
#[doc = "Field `PDINEN15` writer - PDINEN15"]
pub type PDINEN15_W<'a, const O: u8> = crate::BitWriter<'a, PDINER_SPEC, O>;
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
    #[doc = "Bit 4 - PDINEN4"]
    #[inline(always)]
    pub fn pdinen4(&self) -> PDINEN4_R {
        PDINEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PDINEN5"]
    #[inline(always)]
    pub fn pdinen5(&self) -> PDINEN5_R {
        PDINEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDINEN6"]
    #[inline(always)]
    pub fn pdinen6(&self) -> PDINEN6_R {
        PDINEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDINEN7"]
    #[inline(always)]
    pub fn pdinen7(&self) -> PDINEN7_R {
        PDINEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PDINEN8"]
    #[inline(always)]
    pub fn pdinen8(&self) -> PDINEN8_R {
        PDINEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PDINEN9"]
    #[inline(always)]
    pub fn pdinen9(&self) -> PDINEN9_R {
        PDINEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PDINEN10"]
    #[inline(always)]
    pub fn pdinen10(&self) -> PDINEN10_R {
        PDINEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PDINEN11"]
    #[inline(always)]
    pub fn pdinen11(&self) -> PDINEN11_R {
        PDINEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PDINEN12"]
    #[inline(always)]
    pub fn pdinen12(&self) -> PDINEN12_R {
        PDINEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PDINEN13"]
    #[inline(always)]
    pub fn pdinen13(&self) -> PDINEN13_R {
        PDINEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PDINEN14"]
    #[inline(always)]
    pub fn pdinen14(&self) -> PDINEN14_R {
        PDINEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PDINEN15"]
    #[inline(always)]
    pub fn pdinen15(&self) -> PDINEN15_R {
        PDINEN15_R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 4 - PDINEN4"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen4(&mut self) -> PDINEN4_W<4> {
        PDINEN4_W::new(self)
    }
    #[doc = "Bit 5 - PDINEN5"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen5(&mut self) -> PDINEN5_W<5> {
        PDINEN5_W::new(self)
    }
    #[doc = "Bit 6 - PDINEN6"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen6(&mut self) -> PDINEN6_W<6> {
        PDINEN6_W::new(self)
    }
    #[doc = "Bit 7 - PDINEN7"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen7(&mut self) -> PDINEN7_W<7> {
        PDINEN7_W::new(self)
    }
    #[doc = "Bit 8 - PDINEN8"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen8(&mut self) -> PDINEN8_W<8> {
        PDINEN8_W::new(self)
    }
    #[doc = "Bit 9 - PDINEN9"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen9(&mut self) -> PDINEN9_W<9> {
        PDINEN9_W::new(self)
    }
    #[doc = "Bit 10 - PDINEN10"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen10(&mut self) -> PDINEN10_W<10> {
        PDINEN10_W::new(self)
    }
    #[doc = "Bit 11 - PDINEN11"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen11(&mut self) -> PDINEN11_W<11> {
        PDINEN11_W::new(self)
    }
    #[doc = "Bit 12 - PDINEN12"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen12(&mut self) -> PDINEN12_W<12> {
        PDINEN12_W::new(self)
    }
    #[doc = "Bit 13 - PDINEN13"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen13(&mut self) -> PDINEN13_W<13> {
        PDINEN13_W::new(self)
    }
    #[doc = "Bit 14 - PDINEN14"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen14(&mut self) -> PDINEN14_W<14> {
        PDINEN14_W::new(self)
    }
    #[doc = "Bit 15 - PDINEN15"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen15(&mut self) -> PDINEN15_W<15> {
        PDINEN15_W::new(self)
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
