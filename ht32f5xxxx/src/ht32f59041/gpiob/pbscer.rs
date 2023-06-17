#[doc = "Register `PBSCER` reader"]
pub struct R(crate::R<PBSCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBSCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBSCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBSCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBSCER` writer"]
pub struct W(crate::W<PBSCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBSCER_SPEC>;
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
impl From<crate::W<PBSCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBSCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBSCE0` reader - PBSCE0"]
pub type PBSCE0_R = crate::BitReader;
#[doc = "Field `PBSCE0` writer - PBSCE0"]
pub type PBSCE0_W<'a, const O: u8> = crate::BitWriter<'a, PBSCER_SPEC, O>;
#[doc = "Field `PBSCE1` reader - PBSCE1"]
pub type PBSCE1_R = crate::BitReader;
#[doc = "Field `PBSCE1` writer - PBSCE1"]
pub type PBSCE1_W<'a, const O: u8> = crate::BitWriter<'a, PBSCER_SPEC, O>;
#[doc = "Field `PBSCE2` reader - PBSCE2"]
pub type PBSCE2_R = crate::BitReader;
#[doc = "Field `PBSCE2` writer - PBSCE2"]
pub type PBSCE2_W<'a, const O: u8> = crate::BitWriter<'a, PBSCER_SPEC, O>;
#[doc = "Field `PBSCE3` reader - PBSCE3"]
pub type PBSCE3_R = crate::BitReader;
#[doc = "Field `PBSCE3` writer - PBSCE3"]
pub type PBSCE3_W<'a, const O: u8> = crate::BitWriter<'a, PBSCER_SPEC, O>;
#[doc = "Field `PBSCE4` reader - PBSCE4"]
pub type PBSCE4_R = crate::BitReader;
#[doc = "Field `PBSCE4` writer - PBSCE4"]
pub type PBSCE4_W<'a, const O: u8> = crate::BitWriter<'a, PBSCER_SPEC, O>;
#[doc = "Field `PBSCE5` reader - PBSCE5"]
pub type PBSCE5_R = crate::BitReader;
#[doc = "Field `PBSCE5` writer - PBSCE5"]
pub type PBSCE5_W<'a, const O: u8> = crate::BitWriter<'a, PBSCER_SPEC, O>;
#[doc = "Field `PBSCE6` reader - PBSCE6"]
pub type PBSCE6_R = crate::BitReader;
#[doc = "Field `PBSCE6` writer - PBSCE6"]
pub type PBSCE6_W<'a, const O: u8> = crate::BitWriter<'a, PBSCER_SPEC, O>;
#[doc = "Field `PBSCE7` reader - PBSCE7"]
pub type PBSCE7_R = crate::BitReader;
#[doc = "Field `PBSCE7` writer - PBSCE7"]
pub type PBSCE7_W<'a, const O: u8> = crate::BitWriter<'a, PBSCER_SPEC, O>;
#[doc = "Field `PBSCE8` reader - PBSCE8"]
pub type PBSCE8_R = crate::BitReader;
#[doc = "Field `PBSCE8` writer - PBSCE8"]
pub type PBSCE8_W<'a, const O: u8> = crate::BitWriter<'a, PBSCER_SPEC, O>;
#[doc = "Field `PBSCE9` reader - PBSCE9"]
pub type PBSCE9_R = crate::BitReader;
#[doc = "Field `PBSCE9` writer - PBSCE9"]
pub type PBSCE9_W<'a, const O: u8> = crate::BitWriter<'a, PBSCER_SPEC, O>;
#[doc = "Field `PBSCE10` reader - PBSCE10"]
pub type PBSCE10_R = crate::BitReader;
#[doc = "Field `PBSCE10` writer - PBSCE10"]
pub type PBSCE10_W<'a, const O: u8> = crate::BitWriter<'a, PBSCER_SPEC, O>;
#[doc = "Field `PBSCE11` reader - PBSCE11"]
pub type PBSCE11_R = crate::BitReader;
#[doc = "Field `PBSCE11` writer - PBSCE11"]
pub type PBSCE11_W<'a, const O: u8> = crate::BitWriter<'a, PBSCER_SPEC, O>;
#[doc = "Field `PBSCE12` reader - PBSCE12"]
pub type PBSCE12_R = crate::BitReader;
#[doc = "Field `PBSCE12` writer - PBSCE12"]
pub type PBSCE12_W<'a, const O: u8> = crate::BitWriter<'a, PBSCER_SPEC, O>;
#[doc = "Field `PBSCE13` reader - PBSCE13"]
pub type PBSCE13_R = crate::BitReader;
#[doc = "Field `PBSCE13` writer - PBSCE13"]
pub type PBSCE13_W<'a, const O: u8> = crate::BitWriter<'a, PBSCER_SPEC, O>;
#[doc = "Field `PBSCE14` reader - PBSCE14"]
pub type PBSCE14_R = crate::BitReader;
#[doc = "Field `PBSCE14` writer - PBSCE14"]
pub type PBSCE14_W<'a, const O: u8> = crate::BitWriter<'a, PBSCER_SPEC, O>;
#[doc = "Field `PBSCE15` reader - PBSCE15"]
pub type PBSCE15_R = crate::BitReader;
#[doc = "Field `PBSCE15` writer - PBSCE15"]
pub type PBSCE15_W<'a, const O: u8> = crate::BitWriter<'a, PBSCER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PBSCE0"]
    #[inline(always)]
    pub fn pbsce0(&self) -> PBSCE0_R {
        PBSCE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PBSCE1"]
    #[inline(always)]
    pub fn pbsce1(&self) -> PBSCE1_R {
        PBSCE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBSCE2"]
    #[inline(always)]
    pub fn pbsce2(&self) -> PBSCE2_R {
        PBSCE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PBSCE3"]
    #[inline(always)]
    pub fn pbsce3(&self) -> PBSCE3_R {
        PBSCE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PBSCE4"]
    #[inline(always)]
    pub fn pbsce4(&self) -> PBSCE4_R {
        PBSCE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PBSCE5"]
    #[inline(always)]
    pub fn pbsce5(&self) -> PBSCE5_R {
        PBSCE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PBSCE6"]
    #[inline(always)]
    pub fn pbsce6(&self) -> PBSCE6_R {
        PBSCE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PBSCE7"]
    #[inline(always)]
    pub fn pbsce7(&self) -> PBSCE7_R {
        PBSCE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PBSCE8"]
    #[inline(always)]
    pub fn pbsce8(&self) -> PBSCE8_R {
        PBSCE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PBSCE9"]
    #[inline(always)]
    pub fn pbsce9(&self) -> PBSCE9_R {
        PBSCE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PBSCE10"]
    #[inline(always)]
    pub fn pbsce10(&self) -> PBSCE10_R {
        PBSCE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PBSCE11"]
    #[inline(always)]
    pub fn pbsce11(&self) -> PBSCE11_R {
        PBSCE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PBSCE12"]
    #[inline(always)]
    pub fn pbsce12(&self) -> PBSCE12_R {
        PBSCE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PBSCE13"]
    #[inline(always)]
    pub fn pbsce13(&self) -> PBSCE13_R {
        PBSCE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PBSCE14"]
    #[inline(always)]
    pub fn pbsce14(&self) -> PBSCE14_R {
        PBSCE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PBSCE15"]
    #[inline(always)]
    pub fn pbsce15(&self) -> PBSCE15_R {
        PBSCE15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PBSCE0"]
    #[inline(always)]
    #[must_use]
    pub fn pbsce0(&mut self) -> PBSCE0_W<0> {
        PBSCE0_W::new(self)
    }
    #[doc = "Bit 1 - PBSCE1"]
    #[inline(always)]
    #[must_use]
    pub fn pbsce1(&mut self) -> PBSCE1_W<1> {
        PBSCE1_W::new(self)
    }
    #[doc = "Bit 2 - PBSCE2"]
    #[inline(always)]
    #[must_use]
    pub fn pbsce2(&mut self) -> PBSCE2_W<2> {
        PBSCE2_W::new(self)
    }
    #[doc = "Bit 3 - PBSCE3"]
    #[inline(always)]
    #[must_use]
    pub fn pbsce3(&mut self) -> PBSCE3_W<3> {
        PBSCE3_W::new(self)
    }
    #[doc = "Bit 4 - PBSCE4"]
    #[inline(always)]
    #[must_use]
    pub fn pbsce4(&mut self) -> PBSCE4_W<4> {
        PBSCE4_W::new(self)
    }
    #[doc = "Bit 5 - PBSCE5"]
    #[inline(always)]
    #[must_use]
    pub fn pbsce5(&mut self) -> PBSCE5_W<5> {
        PBSCE5_W::new(self)
    }
    #[doc = "Bit 6 - PBSCE6"]
    #[inline(always)]
    #[must_use]
    pub fn pbsce6(&mut self) -> PBSCE6_W<6> {
        PBSCE6_W::new(self)
    }
    #[doc = "Bit 7 - PBSCE7"]
    #[inline(always)]
    #[must_use]
    pub fn pbsce7(&mut self) -> PBSCE7_W<7> {
        PBSCE7_W::new(self)
    }
    #[doc = "Bit 8 - PBSCE8"]
    #[inline(always)]
    #[must_use]
    pub fn pbsce8(&mut self) -> PBSCE8_W<8> {
        PBSCE8_W::new(self)
    }
    #[doc = "Bit 9 - PBSCE9"]
    #[inline(always)]
    #[must_use]
    pub fn pbsce9(&mut self) -> PBSCE9_W<9> {
        PBSCE9_W::new(self)
    }
    #[doc = "Bit 10 - PBSCE10"]
    #[inline(always)]
    #[must_use]
    pub fn pbsce10(&mut self) -> PBSCE10_W<10> {
        PBSCE10_W::new(self)
    }
    #[doc = "Bit 11 - PBSCE11"]
    #[inline(always)]
    #[must_use]
    pub fn pbsce11(&mut self) -> PBSCE11_W<11> {
        PBSCE11_W::new(self)
    }
    #[doc = "Bit 12 - PBSCE12"]
    #[inline(always)]
    #[must_use]
    pub fn pbsce12(&mut self) -> PBSCE12_W<12> {
        PBSCE12_W::new(self)
    }
    #[doc = "Bit 13 - PBSCE13"]
    #[inline(always)]
    #[must_use]
    pub fn pbsce13(&mut self) -> PBSCE13_W<13> {
        PBSCE13_W::new(self)
    }
    #[doc = "Bit 14 - PBSCE14"]
    #[inline(always)]
    #[must_use]
    pub fn pbsce14(&mut self) -> PBSCE14_W<14> {
        PBSCE14_W::new(self)
    }
    #[doc = "Bit 15 - PBSCE15"]
    #[inline(always)]
    #[must_use]
    pub fn pbsce15(&mut self) -> PBSCE15_W<15> {
        PBSCE15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBSCER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbscer](index.html) module"]
pub struct PBSCER_SPEC;
impl crate::RegisterSpec for PBSCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbscer::R](R) reader structure"]
impl crate::Readable for PBSCER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbscer::W](W) writer structure"]
impl crate::Writable for PBSCER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBSCER to value 0"]
impl crate::Resettable for PBSCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
