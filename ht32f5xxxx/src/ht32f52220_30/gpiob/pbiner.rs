#[doc = "Register `PBINER` reader"]
pub struct R(crate::R<PBINER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBINER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBINER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBINER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBINER` writer"]
pub struct W(crate::W<PBINER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBINER_SPEC>;
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
impl From<crate::W<PBINER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBINER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBINEN0` reader - PBINEN0"]
pub type PBINEN0_R = crate::BitReader;
#[doc = "Field `PBINEN0` writer - PBINEN0"]
pub type PBINEN0_W<'a, const O: u8> = crate::BitWriter<'a, PBINER_SPEC, O>;
#[doc = "Field `PBINEN1` reader - PBINEN1"]
pub type PBINEN1_R = crate::BitReader;
#[doc = "Field `PBINEN1` writer - PBINEN1"]
pub type PBINEN1_W<'a, const O: u8> = crate::BitWriter<'a, PBINER_SPEC, O>;
#[doc = "Field `PBINEN2` reader - PBINEN2"]
pub type PBINEN2_R = crate::BitReader;
#[doc = "Field `PBINEN2` writer - PBINEN2"]
pub type PBINEN2_W<'a, const O: u8> = crate::BitWriter<'a, PBINER_SPEC, O>;
#[doc = "Field `PBINEN3` reader - PBINEN3"]
pub type PBINEN3_R = crate::BitReader;
#[doc = "Field `PBINEN3` writer - PBINEN3"]
pub type PBINEN3_W<'a, const O: u8> = crate::BitWriter<'a, PBINER_SPEC, O>;
#[doc = "Field `PBINEN4` reader - PBINEN4"]
pub type PBINEN4_R = crate::BitReader;
#[doc = "Field `PBINEN4` writer - PBINEN4"]
pub type PBINEN4_W<'a, const O: u8> = crate::BitWriter<'a, PBINER_SPEC, O>;
#[doc = "Field `PBINEN5` reader - PBINEN5"]
pub type PBINEN5_R = crate::BitReader;
#[doc = "Field `PBINEN5` writer - PBINEN5"]
pub type PBINEN5_W<'a, const O: u8> = crate::BitWriter<'a, PBINER_SPEC, O>;
#[doc = "Field `PBINEN6` reader - PBINEN6"]
pub type PBINEN6_R = crate::BitReader;
#[doc = "Field `PBINEN6` writer - PBINEN6"]
pub type PBINEN6_W<'a, const O: u8> = crate::BitWriter<'a, PBINER_SPEC, O>;
#[doc = "Field `PBINEN7` reader - PBINEN7"]
pub type PBINEN7_R = crate::BitReader;
#[doc = "Field `PBINEN7` writer - PBINEN7"]
pub type PBINEN7_W<'a, const O: u8> = crate::BitWriter<'a, PBINER_SPEC, O>;
#[doc = "Field `PBINEN8` reader - PBINEN8"]
pub type PBINEN8_R = crate::BitReader;
#[doc = "Field `PBINEN8` writer - PBINEN8"]
pub type PBINEN8_W<'a, const O: u8> = crate::BitWriter<'a, PBINER_SPEC, O>;
#[doc = "Field `PBINEN9` reader - PBINEN9"]
pub type PBINEN9_R = crate::BitReader;
#[doc = "Field `PBINEN9` writer - PBINEN9"]
pub type PBINEN9_W<'a, const O: u8> = crate::BitWriter<'a, PBINER_SPEC, O>;
#[doc = "Field `PBINEN10` reader - PBINEN10"]
pub type PBINEN10_R = crate::BitReader;
#[doc = "Field `PBINEN10` writer - PBINEN10"]
pub type PBINEN10_W<'a, const O: u8> = crate::BitWriter<'a, PBINER_SPEC, O>;
#[doc = "Field `PBINEN11` reader - PBINEN11"]
pub type PBINEN11_R = crate::BitReader;
#[doc = "Field `PBINEN11` writer - PBINEN11"]
pub type PBINEN11_W<'a, const O: u8> = crate::BitWriter<'a, PBINER_SPEC, O>;
#[doc = "Field `PBINEN12` reader - PBINEN12"]
pub type PBINEN12_R = crate::BitReader;
#[doc = "Field `PBINEN12` writer - PBINEN12"]
pub type PBINEN12_W<'a, const O: u8> = crate::BitWriter<'a, PBINER_SPEC, O>;
#[doc = "Field `PBINEN13` reader - PBINEN13"]
pub type PBINEN13_R = crate::BitReader;
#[doc = "Field `PBINEN13` writer - PBINEN13"]
pub type PBINEN13_W<'a, const O: u8> = crate::BitWriter<'a, PBINER_SPEC, O>;
#[doc = "Field `PBINEN14` reader - PBINEN14"]
pub type PBINEN14_R = crate::BitReader;
#[doc = "Field `PBINEN14` writer - PBINEN14"]
pub type PBINEN14_W<'a, const O: u8> = crate::BitWriter<'a, PBINER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PBINEN0"]
    #[inline(always)]
    pub fn pbinen0(&self) -> PBINEN0_R {
        PBINEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PBINEN1"]
    #[inline(always)]
    pub fn pbinen1(&self) -> PBINEN1_R {
        PBINEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBINEN2"]
    #[inline(always)]
    pub fn pbinen2(&self) -> PBINEN2_R {
        PBINEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PBINEN3"]
    #[inline(always)]
    pub fn pbinen3(&self) -> PBINEN3_R {
        PBINEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PBINEN4"]
    #[inline(always)]
    pub fn pbinen4(&self) -> PBINEN4_R {
        PBINEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PBINEN5"]
    #[inline(always)]
    pub fn pbinen5(&self) -> PBINEN5_R {
        PBINEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PBINEN6"]
    #[inline(always)]
    pub fn pbinen6(&self) -> PBINEN6_R {
        PBINEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PBINEN7"]
    #[inline(always)]
    pub fn pbinen7(&self) -> PBINEN7_R {
        PBINEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PBINEN8"]
    #[inline(always)]
    pub fn pbinen8(&self) -> PBINEN8_R {
        PBINEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PBINEN9"]
    #[inline(always)]
    pub fn pbinen9(&self) -> PBINEN9_R {
        PBINEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PBINEN10"]
    #[inline(always)]
    pub fn pbinen10(&self) -> PBINEN10_R {
        PBINEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PBINEN11"]
    #[inline(always)]
    pub fn pbinen11(&self) -> PBINEN11_R {
        PBINEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PBINEN12"]
    #[inline(always)]
    pub fn pbinen12(&self) -> PBINEN12_R {
        PBINEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PBINEN13"]
    #[inline(always)]
    pub fn pbinen13(&self) -> PBINEN13_R {
        PBINEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PBINEN14"]
    #[inline(always)]
    pub fn pbinen14(&self) -> PBINEN14_R {
        PBINEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PBINEN0"]
    #[inline(always)]
    #[must_use]
    pub fn pbinen0(&mut self) -> PBINEN0_W<0> {
        PBINEN0_W::new(self)
    }
    #[doc = "Bit 1 - PBINEN1"]
    #[inline(always)]
    #[must_use]
    pub fn pbinen1(&mut self) -> PBINEN1_W<1> {
        PBINEN1_W::new(self)
    }
    #[doc = "Bit 2 - PBINEN2"]
    #[inline(always)]
    #[must_use]
    pub fn pbinen2(&mut self) -> PBINEN2_W<2> {
        PBINEN2_W::new(self)
    }
    #[doc = "Bit 3 - PBINEN3"]
    #[inline(always)]
    #[must_use]
    pub fn pbinen3(&mut self) -> PBINEN3_W<3> {
        PBINEN3_W::new(self)
    }
    #[doc = "Bit 4 - PBINEN4"]
    #[inline(always)]
    #[must_use]
    pub fn pbinen4(&mut self) -> PBINEN4_W<4> {
        PBINEN4_W::new(self)
    }
    #[doc = "Bit 5 - PBINEN5"]
    #[inline(always)]
    #[must_use]
    pub fn pbinen5(&mut self) -> PBINEN5_W<5> {
        PBINEN5_W::new(self)
    }
    #[doc = "Bit 6 - PBINEN6"]
    #[inline(always)]
    #[must_use]
    pub fn pbinen6(&mut self) -> PBINEN6_W<6> {
        PBINEN6_W::new(self)
    }
    #[doc = "Bit 7 - PBINEN7"]
    #[inline(always)]
    #[must_use]
    pub fn pbinen7(&mut self) -> PBINEN7_W<7> {
        PBINEN7_W::new(self)
    }
    #[doc = "Bit 8 - PBINEN8"]
    #[inline(always)]
    #[must_use]
    pub fn pbinen8(&mut self) -> PBINEN8_W<8> {
        PBINEN8_W::new(self)
    }
    #[doc = "Bit 9 - PBINEN9"]
    #[inline(always)]
    #[must_use]
    pub fn pbinen9(&mut self) -> PBINEN9_W<9> {
        PBINEN9_W::new(self)
    }
    #[doc = "Bit 10 - PBINEN10"]
    #[inline(always)]
    #[must_use]
    pub fn pbinen10(&mut self) -> PBINEN10_W<10> {
        PBINEN10_W::new(self)
    }
    #[doc = "Bit 11 - PBINEN11"]
    #[inline(always)]
    #[must_use]
    pub fn pbinen11(&mut self) -> PBINEN11_W<11> {
        PBINEN11_W::new(self)
    }
    #[doc = "Bit 12 - PBINEN12"]
    #[inline(always)]
    #[must_use]
    pub fn pbinen12(&mut self) -> PBINEN12_W<12> {
        PBINEN12_W::new(self)
    }
    #[doc = "Bit 13 - PBINEN13"]
    #[inline(always)]
    #[must_use]
    pub fn pbinen13(&mut self) -> PBINEN13_W<13> {
        PBINEN13_W::new(self)
    }
    #[doc = "Bit 14 - PBINEN14"]
    #[inline(always)]
    #[must_use]
    pub fn pbinen14(&mut self) -> PBINEN14_W<14> {
        PBINEN14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBINER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbiner](index.html) module"]
pub struct PBINER_SPEC;
impl crate::RegisterSpec for PBINER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbiner::R](R) reader structure"]
impl crate::Readable for PBINER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbiner::W](W) writer structure"]
impl crate::Writable for PBINER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBINER to value 0"]
impl crate::Resettable for PBINER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
