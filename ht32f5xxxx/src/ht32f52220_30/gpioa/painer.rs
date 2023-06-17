#[doc = "Register `PAINER` reader"]
pub struct R(crate::R<PAINER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAINER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAINER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAINER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAINER` writer"]
pub struct W(crate::W<PAINER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAINER_SPEC>;
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
impl From<crate::W<PAINER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAINER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAINEN0` reader - PAINEN0"]
pub type PAINEN0_R = crate::BitReader;
#[doc = "Field `PAINEN0` writer - PAINEN0"]
pub type PAINEN0_W<'a, const O: u8> = crate::BitWriter<'a, PAINER_SPEC, O>;
#[doc = "Field `PAINEN1` reader - PAINEN1"]
pub type PAINEN1_R = crate::BitReader;
#[doc = "Field `PAINEN1` writer - PAINEN1"]
pub type PAINEN1_W<'a, const O: u8> = crate::BitWriter<'a, PAINER_SPEC, O>;
#[doc = "Field `PAINEN2` reader - PAINEN2"]
pub type PAINEN2_R = crate::BitReader;
#[doc = "Field `PAINEN2` writer - PAINEN2"]
pub type PAINEN2_W<'a, const O: u8> = crate::BitWriter<'a, PAINER_SPEC, O>;
#[doc = "Field `PAINEN3` reader - PAINEN3"]
pub type PAINEN3_R = crate::BitReader;
#[doc = "Field `PAINEN3` writer - PAINEN3"]
pub type PAINEN3_W<'a, const O: u8> = crate::BitWriter<'a, PAINER_SPEC, O>;
#[doc = "Field `PAINEN4` reader - PAINEN4"]
pub type PAINEN4_R = crate::BitReader;
#[doc = "Field `PAINEN4` writer - PAINEN4"]
pub type PAINEN4_W<'a, const O: u8> = crate::BitWriter<'a, PAINER_SPEC, O>;
#[doc = "Field `PAINEN5` reader - PAINEN5"]
pub type PAINEN5_R = crate::BitReader;
#[doc = "Field `PAINEN5` writer - PAINEN5"]
pub type PAINEN5_W<'a, const O: u8> = crate::BitWriter<'a, PAINER_SPEC, O>;
#[doc = "Field `PAINEN6` reader - PAINEN6"]
pub type PAINEN6_R = crate::BitReader;
#[doc = "Field `PAINEN6` writer - PAINEN6"]
pub type PAINEN6_W<'a, const O: u8> = crate::BitWriter<'a, PAINER_SPEC, O>;
#[doc = "Field `PAINEN7` reader - PAINEN7"]
pub type PAINEN7_R = crate::BitReader;
#[doc = "Field `PAINEN7` writer - PAINEN7"]
pub type PAINEN7_W<'a, const O: u8> = crate::BitWriter<'a, PAINER_SPEC, O>;
#[doc = "Field `PAINEN8` reader - PAINEN8"]
pub type PAINEN8_R = crate::BitReader;
#[doc = "Field `PAINEN8` writer - PAINEN8"]
pub type PAINEN8_W<'a, const O: u8> = crate::BitWriter<'a, PAINER_SPEC, O>;
#[doc = "Field `PAINEN9` reader - PAINEN9"]
pub type PAINEN9_R = crate::BitReader;
#[doc = "Field `PAINEN9` writer - PAINEN9"]
pub type PAINEN9_W<'a, const O: u8> = crate::BitWriter<'a, PAINER_SPEC, O>;
#[doc = "Field `PAINEN10` reader - PAINEN10"]
pub type PAINEN10_R = crate::BitReader;
#[doc = "Field `PAINEN10` writer - PAINEN10"]
pub type PAINEN10_W<'a, const O: u8> = crate::BitWriter<'a, PAINER_SPEC, O>;
#[doc = "Field `PAINEN11` reader - PAINEN11"]
pub type PAINEN11_R = crate::BitReader;
#[doc = "Field `PAINEN11` writer - PAINEN11"]
pub type PAINEN11_W<'a, const O: u8> = crate::BitWriter<'a, PAINER_SPEC, O>;
#[doc = "Field `PAINEN12` reader - PAINEN12"]
pub type PAINEN12_R = crate::BitReader;
#[doc = "Field `PAINEN12` writer - PAINEN12"]
pub type PAINEN12_W<'a, const O: u8> = crate::BitWriter<'a, PAINER_SPEC, O>;
#[doc = "Field `PAINEN13` reader - PAINEN13"]
pub type PAINEN13_R = crate::BitReader;
#[doc = "Field `PAINEN13` writer - PAINEN13"]
pub type PAINEN13_W<'a, const O: u8> = crate::BitWriter<'a, PAINER_SPEC, O>;
#[doc = "Field `PAINEN14` reader - PAINEN14"]
pub type PAINEN14_R = crate::BitReader;
#[doc = "Field `PAINEN14` writer - PAINEN14"]
pub type PAINEN14_W<'a, const O: u8> = crate::BitWriter<'a, PAINER_SPEC, O>;
#[doc = "Field `PAINEN15` reader - PAINEN15"]
pub type PAINEN15_R = crate::BitReader;
#[doc = "Field `PAINEN15` writer - PAINEN15"]
pub type PAINEN15_W<'a, const O: u8> = crate::BitWriter<'a, PAINER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PAINEN0"]
    #[inline(always)]
    pub fn painen0(&self) -> PAINEN0_R {
        PAINEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PAINEN1"]
    #[inline(always)]
    pub fn painen1(&self) -> PAINEN1_R {
        PAINEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PAINEN2"]
    #[inline(always)]
    pub fn painen2(&self) -> PAINEN2_R {
        PAINEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PAINEN3"]
    #[inline(always)]
    pub fn painen3(&self) -> PAINEN3_R {
        PAINEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PAINEN4"]
    #[inline(always)]
    pub fn painen4(&self) -> PAINEN4_R {
        PAINEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PAINEN5"]
    #[inline(always)]
    pub fn painen5(&self) -> PAINEN5_R {
        PAINEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PAINEN6"]
    #[inline(always)]
    pub fn painen6(&self) -> PAINEN6_R {
        PAINEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PAINEN7"]
    #[inline(always)]
    pub fn painen7(&self) -> PAINEN7_R {
        PAINEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PAINEN8"]
    #[inline(always)]
    pub fn painen8(&self) -> PAINEN8_R {
        PAINEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PAINEN9"]
    #[inline(always)]
    pub fn painen9(&self) -> PAINEN9_R {
        PAINEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PAINEN10"]
    #[inline(always)]
    pub fn painen10(&self) -> PAINEN10_R {
        PAINEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAINEN11"]
    #[inline(always)]
    pub fn painen11(&self) -> PAINEN11_R {
        PAINEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAINEN12"]
    #[inline(always)]
    pub fn painen12(&self) -> PAINEN12_R {
        PAINEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAINEN13"]
    #[inline(always)]
    pub fn painen13(&self) -> PAINEN13_R {
        PAINEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAINEN14"]
    #[inline(always)]
    pub fn painen14(&self) -> PAINEN14_R {
        PAINEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAINEN15"]
    #[inline(always)]
    pub fn painen15(&self) -> PAINEN15_R {
        PAINEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAINEN0"]
    #[inline(always)]
    #[must_use]
    pub fn painen0(&mut self) -> PAINEN0_W<0> {
        PAINEN0_W::new(self)
    }
    #[doc = "Bit 1 - PAINEN1"]
    #[inline(always)]
    #[must_use]
    pub fn painen1(&mut self) -> PAINEN1_W<1> {
        PAINEN1_W::new(self)
    }
    #[doc = "Bit 2 - PAINEN2"]
    #[inline(always)]
    #[must_use]
    pub fn painen2(&mut self) -> PAINEN2_W<2> {
        PAINEN2_W::new(self)
    }
    #[doc = "Bit 3 - PAINEN3"]
    #[inline(always)]
    #[must_use]
    pub fn painen3(&mut self) -> PAINEN3_W<3> {
        PAINEN3_W::new(self)
    }
    #[doc = "Bit 4 - PAINEN4"]
    #[inline(always)]
    #[must_use]
    pub fn painen4(&mut self) -> PAINEN4_W<4> {
        PAINEN4_W::new(self)
    }
    #[doc = "Bit 5 - PAINEN5"]
    #[inline(always)]
    #[must_use]
    pub fn painen5(&mut self) -> PAINEN5_W<5> {
        PAINEN5_W::new(self)
    }
    #[doc = "Bit 6 - PAINEN6"]
    #[inline(always)]
    #[must_use]
    pub fn painen6(&mut self) -> PAINEN6_W<6> {
        PAINEN6_W::new(self)
    }
    #[doc = "Bit 7 - PAINEN7"]
    #[inline(always)]
    #[must_use]
    pub fn painen7(&mut self) -> PAINEN7_W<7> {
        PAINEN7_W::new(self)
    }
    #[doc = "Bit 8 - PAINEN8"]
    #[inline(always)]
    #[must_use]
    pub fn painen8(&mut self) -> PAINEN8_W<8> {
        PAINEN8_W::new(self)
    }
    #[doc = "Bit 9 - PAINEN9"]
    #[inline(always)]
    #[must_use]
    pub fn painen9(&mut self) -> PAINEN9_W<9> {
        PAINEN9_W::new(self)
    }
    #[doc = "Bit 10 - PAINEN10"]
    #[inline(always)]
    #[must_use]
    pub fn painen10(&mut self) -> PAINEN10_W<10> {
        PAINEN10_W::new(self)
    }
    #[doc = "Bit 11 - PAINEN11"]
    #[inline(always)]
    #[must_use]
    pub fn painen11(&mut self) -> PAINEN11_W<11> {
        PAINEN11_W::new(self)
    }
    #[doc = "Bit 12 - PAINEN12"]
    #[inline(always)]
    #[must_use]
    pub fn painen12(&mut self) -> PAINEN12_W<12> {
        PAINEN12_W::new(self)
    }
    #[doc = "Bit 13 - PAINEN13"]
    #[inline(always)]
    #[must_use]
    pub fn painen13(&mut self) -> PAINEN13_W<13> {
        PAINEN13_W::new(self)
    }
    #[doc = "Bit 14 - PAINEN14"]
    #[inline(always)]
    #[must_use]
    pub fn painen14(&mut self) -> PAINEN14_W<14> {
        PAINEN14_W::new(self)
    }
    #[doc = "Bit 15 - PAINEN15"]
    #[inline(always)]
    #[must_use]
    pub fn painen15(&mut self) -> PAINEN15_W<15> {
        PAINEN15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAINER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [painer](index.html) module"]
pub struct PAINER_SPEC;
impl crate::RegisterSpec for PAINER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [painer::R](R) reader structure"]
impl crate::Readable for PAINER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [painer::W](W) writer structure"]
impl crate::Writable for PAINER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAINER to value 0"]
impl crate::Resettable for PAINER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
