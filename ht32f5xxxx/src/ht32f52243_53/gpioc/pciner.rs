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
#[doc = "Field `PCINEN8` reader - PCINEN8"]
pub type PCINEN8_R = crate::BitReader;
#[doc = "Field `PCINEN8` writer - PCINEN8"]
pub type PCINEN8_W<'a, const O: u8> = crate::BitWriter<'a, PCINER_SPEC, O>;
#[doc = "Field `PCINEN9` reader - PCINEN9"]
pub type PCINEN9_R = crate::BitReader;
#[doc = "Field `PCINEN9` writer - PCINEN9"]
pub type PCINEN9_W<'a, const O: u8> = crate::BitWriter<'a, PCINER_SPEC, O>;
#[doc = "Field `PCINEN10` reader - PCINEN10"]
pub type PCINEN10_R = crate::BitReader;
#[doc = "Field `PCINEN10` writer - PCINEN10"]
pub type PCINEN10_W<'a, const O: u8> = crate::BitWriter<'a, PCINER_SPEC, O>;
#[doc = "Field `PCINEN11` reader - PCINEN11"]
pub type PCINEN11_R = crate::BitReader;
#[doc = "Field `PCINEN11` writer - PCINEN11"]
pub type PCINEN11_W<'a, const O: u8> = crate::BitWriter<'a, PCINER_SPEC, O>;
#[doc = "Field `PCINEN12` reader - PCINEN12"]
pub type PCINEN12_R = crate::BitReader;
#[doc = "Field `PCINEN12` writer - PCINEN12"]
pub type PCINEN12_W<'a, const O: u8> = crate::BitWriter<'a, PCINER_SPEC, O>;
#[doc = "Field `PCINEN13` reader - PCINEN13"]
pub type PCINEN13_R = crate::BitReader;
#[doc = "Field `PCINEN13` writer - PCINEN13"]
pub type PCINEN13_W<'a, const O: u8> = crate::BitWriter<'a, PCINER_SPEC, O>;
#[doc = "Field `PCINEN14` reader - PCINEN14"]
pub type PCINEN14_R = crate::BitReader;
#[doc = "Field `PCINEN14` writer - PCINEN14"]
pub type PCINEN14_W<'a, const O: u8> = crate::BitWriter<'a, PCINER_SPEC, O>;
#[doc = "Field `PCINEN15` reader - PCINEN15"]
pub type PCINEN15_R = crate::BitReader;
#[doc = "Field `PCINEN15` writer - PCINEN15"]
pub type PCINEN15_W<'a, const O: u8> = crate::BitWriter<'a, PCINER_SPEC, O>;
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
    #[doc = "Bit 8 - PCINEN8"]
    #[inline(always)]
    pub fn pcinen8(&self) -> PCINEN8_R {
        PCINEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PCINEN9"]
    #[inline(always)]
    pub fn pcinen9(&self) -> PCINEN9_R {
        PCINEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCINEN10"]
    #[inline(always)]
    pub fn pcinen10(&self) -> PCINEN10_R {
        PCINEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PCINEN11"]
    #[inline(always)]
    pub fn pcinen11(&self) -> PCINEN11_R {
        PCINEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PCINEN12"]
    #[inline(always)]
    pub fn pcinen12(&self) -> PCINEN12_R {
        PCINEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PCINEN13"]
    #[inline(always)]
    pub fn pcinen13(&self) -> PCINEN13_R {
        PCINEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PCINEN14"]
    #[inline(always)]
    pub fn pcinen14(&self) -> PCINEN14_R {
        PCINEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PCINEN15"]
    #[inline(always)]
    pub fn pcinen15(&self) -> PCINEN15_R {
        PCINEN15_R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 8 - PCINEN8"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen8(&mut self) -> PCINEN8_W<8> {
        PCINEN8_W::new(self)
    }
    #[doc = "Bit 9 - PCINEN9"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen9(&mut self) -> PCINEN9_W<9> {
        PCINEN9_W::new(self)
    }
    #[doc = "Bit 10 - PCINEN10"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen10(&mut self) -> PCINEN10_W<10> {
        PCINEN10_W::new(self)
    }
    #[doc = "Bit 11 - PCINEN11"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen11(&mut self) -> PCINEN11_W<11> {
        PCINEN11_W::new(self)
    }
    #[doc = "Bit 12 - PCINEN12"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen12(&mut self) -> PCINEN12_W<12> {
        PCINEN12_W::new(self)
    }
    #[doc = "Bit 13 - PCINEN13"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen13(&mut self) -> PCINEN13_W<13> {
        PCINEN13_W::new(self)
    }
    #[doc = "Bit 14 - PCINEN14"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen14(&mut self) -> PCINEN14_W<14> {
        PCINEN14_W::new(self)
    }
    #[doc = "Bit 15 - PCINEN15"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen15(&mut self) -> PCINEN15_W<15> {
        PCINEN15_W::new(self)
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
