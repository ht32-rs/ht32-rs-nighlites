#[doc = "Register `PASCER` reader"]
pub struct R(crate::R<PASCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PASCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PASCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PASCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PASCER` writer"]
pub struct W(crate::W<PASCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PASCER_SPEC>;
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
impl From<crate::W<PASCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PASCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PASCE0` reader - PASCE0"]
pub type PASCE0_R = crate::BitReader;
#[doc = "Field `PASCE0` writer - PASCE0"]
pub type PASCE0_W<'a, const O: u8> = crate::BitWriter<'a, PASCER_SPEC, O>;
#[doc = "Field `PASCE1` reader - PASCE1"]
pub type PASCE1_R = crate::BitReader;
#[doc = "Field `PASCE1` writer - PASCE1"]
pub type PASCE1_W<'a, const O: u8> = crate::BitWriter<'a, PASCER_SPEC, O>;
#[doc = "Field `PASCE2` reader - PASCE2"]
pub type PASCE2_R = crate::BitReader;
#[doc = "Field `PASCE2` writer - PASCE2"]
pub type PASCE2_W<'a, const O: u8> = crate::BitWriter<'a, PASCER_SPEC, O>;
#[doc = "Field `PASCE3` reader - PASCE3"]
pub type PASCE3_R = crate::BitReader;
#[doc = "Field `PASCE3` writer - PASCE3"]
pub type PASCE3_W<'a, const O: u8> = crate::BitWriter<'a, PASCER_SPEC, O>;
#[doc = "Field `PASCE4` reader - PASCE4"]
pub type PASCE4_R = crate::BitReader;
#[doc = "Field `PASCE4` writer - PASCE4"]
pub type PASCE4_W<'a, const O: u8> = crate::BitWriter<'a, PASCER_SPEC, O>;
#[doc = "Field `PASCE5` reader - PASCE5"]
pub type PASCE5_R = crate::BitReader;
#[doc = "Field `PASCE5` writer - PASCE5"]
pub type PASCE5_W<'a, const O: u8> = crate::BitWriter<'a, PASCER_SPEC, O>;
#[doc = "Field `PASCE6` reader - PASCE6"]
pub type PASCE6_R = crate::BitReader;
#[doc = "Field `PASCE6` writer - PASCE6"]
pub type PASCE6_W<'a, const O: u8> = crate::BitWriter<'a, PASCER_SPEC, O>;
#[doc = "Field `PASCE7` reader - PASCE7"]
pub type PASCE7_R = crate::BitReader;
#[doc = "Field `PASCE7` writer - PASCE7"]
pub type PASCE7_W<'a, const O: u8> = crate::BitWriter<'a, PASCER_SPEC, O>;
#[doc = "Field `PASCE8` reader - PASCE8"]
pub type PASCE8_R = crate::BitReader;
#[doc = "Field `PASCE8` writer - PASCE8"]
pub type PASCE8_W<'a, const O: u8> = crate::BitWriter<'a, PASCER_SPEC, O>;
#[doc = "Field `PASCE9` reader - PASCE9"]
pub type PASCE9_R = crate::BitReader;
#[doc = "Field `PASCE9` writer - PASCE9"]
pub type PASCE9_W<'a, const O: u8> = crate::BitWriter<'a, PASCER_SPEC, O>;
#[doc = "Field `PASCE10` reader - PASCE10"]
pub type PASCE10_R = crate::BitReader;
#[doc = "Field `PASCE10` writer - PASCE10"]
pub type PASCE10_W<'a, const O: u8> = crate::BitWriter<'a, PASCER_SPEC, O>;
#[doc = "Field `PASCE11` reader - PASCE11"]
pub type PASCE11_R = crate::BitReader;
#[doc = "Field `PASCE11` writer - PASCE11"]
pub type PASCE11_W<'a, const O: u8> = crate::BitWriter<'a, PASCER_SPEC, O>;
#[doc = "Field `PASCE12` reader - PASCE12"]
pub type PASCE12_R = crate::BitReader;
#[doc = "Field `PASCE12` writer - PASCE12"]
pub type PASCE12_W<'a, const O: u8> = crate::BitWriter<'a, PASCER_SPEC, O>;
#[doc = "Field `PASCE13` reader - PASCE13"]
pub type PASCE13_R = crate::BitReader;
#[doc = "Field `PASCE13` writer - PASCE13"]
pub type PASCE13_W<'a, const O: u8> = crate::BitWriter<'a, PASCER_SPEC, O>;
#[doc = "Field `PASCE14` reader - PASCE14"]
pub type PASCE14_R = crate::BitReader;
#[doc = "Field `PASCE14` writer - PASCE14"]
pub type PASCE14_W<'a, const O: u8> = crate::BitWriter<'a, PASCER_SPEC, O>;
#[doc = "Field `PASCE15` reader - PASCE15"]
pub type PASCE15_R = crate::BitReader;
#[doc = "Field `PASCE15` writer - PASCE15"]
pub type PASCE15_W<'a, const O: u8> = crate::BitWriter<'a, PASCER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PASCE0"]
    #[inline(always)]
    pub fn pasce0(&self) -> PASCE0_R {
        PASCE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PASCE1"]
    #[inline(always)]
    pub fn pasce1(&self) -> PASCE1_R {
        PASCE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PASCE2"]
    #[inline(always)]
    pub fn pasce2(&self) -> PASCE2_R {
        PASCE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PASCE3"]
    #[inline(always)]
    pub fn pasce3(&self) -> PASCE3_R {
        PASCE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PASCE4"]
    #[inline(always)]
    pub fn pasce4(&self) -> PASCE4_R {
        PASCE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PASCE5"]
    #[inline(always)]
    pub fn pasce5(&self) -> PASCE5_R {
        PASCE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PASCE6"]
    #[inline(always)]
    pub fn pasce6(&self) -> PASCE6_R {
        PASCE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PASCE7"]
    #[inline(always)]
    pub fn pasce7(&self) -> PASCE7_R {
        PASCE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PASCE8"]
    #[inline(always)]
    pub fn pasce8(&self) -> PASCE8_R {
        PASCE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PASCE9"]
    #[inline(always)]
    pub fn pasce9(&self) -> PASCE9_R {
        PASCE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PASCE10"]
    #[inline(always)]
    pub fn pasce10(&self) -> PASCE10_R {
        PASCE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PASCE11"]
    #[inline(always)]
    pub fn pasce11(&self) -> PASCE11_R {
        PASCE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PASCE12"]
    #[inline(always)]
    pub fn pasce12(&self) -> PASCE12_R {
        PASCE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PASCE13"]
    #[inline(always)]
    pub fn pasce13(&self) -> PASCE13_R {
        PASCE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PASCE14"]
    #[inline(always)]
    pub fn pasce14(&self) -> PASCE14_R {
        PASCE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PASCE15"]
    #[inline(always)]
    pub fn pasce15(&self) -> PASCE15_R {
        PASCE15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PASCE0"]
    #[inline(always)]
    #[must_use]
    pub fn pasce0(&mut self) -> PASCE0_W<0> {
        PASCE0_W::new(self)
    }
    #[doc = "Bit 1 - PASCE1"]
    #[inline(always)]
    #[must_use]
    pub fn pasce1(&mut self) -> PASCE1_W<1> {
        PASCE1_W::new(self)
    }
    #[doc = "Bit 2 - PASCE2"]
    #[inline(always)]
    #[must_use]
    pub fn pasce2(&mut self) -> PASCE2_W<2> {
        PASCE2_W::new(self)
    }
    #[doc = "Bit 3 - PASCE3"]
    #[inline(always)]
    #[must_use]
    pub fn pasce3(&mut self) -> PASCE3_W<3> {
        PASCE3_W::new(self)
    }
    #[doc = "Bit 4 - PASCE4"]
    #[inline(always)]
    #[must_use]
    pub fn pasce4(&mut self) -> PASCE4_W<4> {
        PASCE4_W::new(self)
    }
    #[doc = "Bit 5 - PASCE5"]
    #[inline(always)]
    #[must_use]
    pub fn pasce5(&mut self) -> PASCE5_W<5> {
        PASCE5_W::new(self)
    }
    #[doc = "Bit 6 - PASCE6"]
    #[inline(always)]
    #[must_use]
    pub fn pasce6(&mut self) -> PASCE6_W<6> {
        PASCE6_W::new(self)
    }
    #[doc = "Bit 7 - PASCE7"]
    #[inline(always)]
    #[must_use]
    pub fn pasce7(&mut self) -> PASCE7_W<7> {
        PASCE7_W::new(self)
    }
    #[doc = "Bit 8 - PASCE8"]
    #[inline(always)]
    #[must_use]
    pub fn pasce8(&mut self) -> PASCE8_W<8> {
        PASCE8_W::new(self)
    }
    #[doc = "Bit 9 - PASCE9"]
    #[inline(always)]
    #[must_use]
    pub fn pasce9(&mut self) -> PASCE9_W<9> {
        PASCE9_W::new(self)
    }
    #[doc = "Bit 10 - PASCE10"]
    #[inline(always)]
    #[must_use]
    pub fn pasce10(&mut self) -> PASCE10_W<10> {
        PASCE10_W::new(self)
    }
    #[doc = "Bit 11 - PASCE11"]
    #[inline(always)]
    #[must_use]
    pub fn pasce11(&mut self) -> PASCE11_W<11> {
        PASCE11_W::new(self)
    }
    #[doc = "Bit 12 - PASCE12"]
    #[inline(always)]
    #[must_use]
    pub fn pasce12(&mut self) -> PASCE12_W<12> {
        PASCE12_W::new(self)
    }
    #[doc = "Bit 13 - PASCE13"]
    #[inline(always)]
    #[must_use]
    pub fn pasce13(&mut self) -> PASCE13_W<13> {
        PASCE13_W::new(self)
    }
    #[doc = "Bit 14 - PASCE14"]
    #[inline(always)]
    #[must_use]
    pub fn pasce14(&mut self) -> PASCE14_W<14> {
        PASCE14_W::new(self)
    }
    #[doc = "Bit 15 - PASCE15"]
    #[inline(always)]
    #[must_use]
    pub fn pasce15(&mut self) -> PASCE15_W<15> {
        PASCE15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PASCER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pascer](index.html) module"]
pub struct PASCER_SPEC;
impl crate::RegisterSpec for PASCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pascer::R](R) reader structure"]
impl crate::Readable for PASCER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pascer::W](W) writer structure"]
impl crate::Writable for PASCER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PASCER to value 0"]
impl crate::Resettable for PASCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
