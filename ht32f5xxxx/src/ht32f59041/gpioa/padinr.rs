#[doc = "Register `PADINR` reader"]
pub struct R(crate::R<PADINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADINR` writer"]
pub struct W(crate::W<PADINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADINR_SPEC>;
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
impl From<crate::W<PADINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADIN0` reader - PADIN0"]
pub type PADIN0_R = crate::BitReader;
#[doc = "Field `PADIN0` writer - PADIN0"]
pub type PADIN0_W<'a, const O: u8> = crate::BitWriter<'a, PADINR_SPEC, O>;
#[doc = "Field `PADIN1` reader - PADIN1"]
pub type PADIN1_R = crate::BitReader;
#[doc = "Field `PADIN1` writer - PADIN1"]
pub type PADIN1_W<'a, const O: u8> = crate::BitWriter<'a, PADINR_SPEC, O>;
#[doc = "Field `PADIN2` reader - PADIN2"]
pub type PADIN2_R = crate::BitReader;
#[doc = "Field `PADIN2` writer - PADIN2"]
pub type PADIN2_W<'a, const O: u8> = crate::BitWriter<'a, PADINR_SPEC, O>;
#[doc = "Field `PADIN3` reader - PADIN3"]
pub type PADIN3_R = crate::BitReader;
#[doc = "Field `PADIN3` writer - PADIN3"]
pub type PADIN3_W<'a, const O: u8> = crate::BitWriter<'a, PADINR_SPEC, O>;
#[doc = "Field `PADIN4` reader - PADIN4"]
pub type PADIN4_R = crate::BitReader;
#[doc = "Field `PADIN4` writer - PADIN4"]
pub type PADIN4_W<'a, const O: u8> = crate::BitWriter<'a, PADINR_SPEC, O>;
#[doc = "Field `PADIN5` reader - PADIN5"]
pub type PADIN5_R = crate::BitReader;
#[doc = "Field `PADIN5` writer - PADIN5"]
pub type PADIN5_W<'a, const O: u8> = crate::BitWriter<'a, PADINR_SPEC, O>;
#[doc = "Field `PADIN6` reader - PADIN6"]
pub type PADIN6_R = crate::BitReader;
#[doc = "Field `PADIN6` writer - PADIN6"]
pub type PADIN6_W<'a, const O: u8> = crate::BitWriter<'a, PADINR_SPEC, O>;
#[doc = "Field `PADIN7` reader - PADIN7"]
pub type PADIN7_R = crate::BitReader;
#[doc = "Field `PADIN7` writer - PADIN7"]
pub type PADIN7_W<'a, const O: u8> = crate::BitWriter<'a, PADINR_SPEC, O>;
#[doc = "Field `PADIN8` reader - PADIN8"]
pub type PADIN8_R = crate::BitReader;
#[doc = "Field `PADIN8` writer - PADIN8"]
pub type PADIN8_W<'a, const O: u8> = crate::BitWriter<'a, PADINR_SPEC, O>;
#[doc = "Field `PADIN9` reader - PADIN9"]
pub type PADIN9_R = crate::BitReader;
#[doc = "Field `PADIN9` writer - PADIN9"]
pub type PADIN9_W<'a, const O: u8> = crate::BitWriter<'a, PADINR_SPEC, O>;
#[doc = "Field `PADIN10` reader - PADIN10"]
pub type PADIN10_R = crate::BitReader;
#[doc = "Field `PADIN10` writer - PADIN10"]
pub type PADIN10_W<'a, const O: u8> = crate::BitWriter<'a, PADINR_SPEC, O>;
#[doc = "Field `PADIN11` reader - PADIN11"]
pub type PADIN11_R = crate::BitReader;
#[doc = "Field `PADIN11` writer - PADIN11"]
pub type PADIN11_W<'a, const O: u8> = crate::BitWriter<'a, PADINR_SPEC, O>;
#[doc = "Field `PADIN12` reader - PADIN12"]
pub type PADIN12_R = crate::BitReader;
#[doc = "Field `PADIN12` writer - PADIN12"]
pub type PADIN12_W<'a, const O: u8> = crate::BitWriter<'a, PADINR_SPEC, O>;
#[doc = "Field `PADIN13` reader - PADIN13"]
pub type PADIN13_R = crate::BitReader;
#[doc = "Field `PADIN13` writer - PADIN13"]
pub type PADIN13_W<'a, const O: u8> = crate::BitWriter<'a, PADINR_SPEC, O>;
#[doc = "Field `PADIN14` reader - PADIN14"]
pub type PADIN14_R = crate::BitReader;
#[doc = "Field `PADIN14` writer - PADIN14"]
pub type PADIN14_W<'a, const O: u8> = crate::BitWriter<'a, PADINR_SPEC, O>;
#[doc = "Field `PADIN15` reader - PADIN15"]
pub type PADIN15_R = crate::BitReader;
#[doc = "Field `PADIN15` writer - PADIN15"]
pub type PADIN15_W<'a, const O: u8> = crate::BitWriter<'a, PADINR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PADIN0"]
    #[inline(always)]
    pub fn padin0(&self) -> PADIN0_R {
        PADIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PADIN1"]
    #[inline(always)]
    pub fn padin1(&self) -> PADIN1_R {
        PADIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PADIN2"]
    #[inline(always)]
    pub fn padin2(&self) -> PADIN2_R {
        PADIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PADIN3"]
    #[inline(always)]
    pub fn padin3(&self) -> PADIN3_R {
        PADIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PADIN4"]
    #[inline(always)]
    pub fn padin4(&self) -> PADIN4_R {
        PADIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PADIN5"]
    #[inline(always)]
    pub fn padin5(&self) -> PADIN5_R {
        PADIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PADIN6"]
    #[inline(always)]
    pub fn padin6(&self) -> PADIN6_R {
        PADIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PADIN7"]
    #[inline(always)]
    pub fn padin7(&self) -> PADIN7_R {
        PADIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PADIN8"]
    #[inline(always)]
    pub fn padin8(&self) -> PADIN8_R {
        PADIN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PADIN9"]
    #[inline(always)]
    pub fn padin9(&self) -> PADIN9_R {
        PADIN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PADIN10"]
    #[inline(always)]
    pub fn padin10(&self) -> PADIN10_R {
        PADIN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PADIN11"]
    #[inline(always)]
    pub fn padin11(&self) -> PADIN11_R {
        PADIN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PADIN12"]
    #[inline(always)]
    pub fn padin12(&self) -> PADIN12_R {
        PADIN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PADIN13"]
    #[inline(always)]
    pub fn padin13(&self) -> PADIN13_R {
        PADIN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PADIN14"]
    #[inline(always)]
    pub fn padin14(&self) -> PADIN14_R {
        PADIN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PADIN15"]
    #[inline(always)]
    pub fn padin15(&self) -> PADIN15_R {
        PADIN15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PADIN0"]
    #[inline(always)]
    #[must_use]
    pub fn padin0(&mut self) -> PADIN0_W<0> {
        PADIN0_W::new(self)
    }
    #[doc = "Bit 1 - PADIN1"]
    #[inline(always)]
    #[must_use]
    pub fn padin1(&mut self) -> PADIN1_W<1> {
        PADIN1_W::new(self)
    }
    #[doc = "Bit 2 - PADIN2"]
    #[inline(always)]
    #[must_use]
    pub fn padin2(&mut self) -> PADIN2_W<2> {
        PADIN2_W::new(self)
    }
    #[doc = "Bit 3 - PADIN3"]
    #[inline(always)]
    #[must_use]
    pub fn padin3(&mut self) -> PADIN3_W<3> {
        PADIN3_W::new(self)
    }
    #[doc = "Bit 4 - PADIN4"]
    #[inline(always)]
    #[must_use]
    pub fn padin4(&mut self) -> PADIN4_W<4> {
        PADIN4_W::new(self)
    }
    #[doc = "Bit 5 - PADIN5"]
    #[inline(always)]
    #[must_use]
    pub fn padin5(&mut self) -> PADIN5_W<5> {
        PADIN5_W::new(self)
    }
    #[doc = "Bit 6 - PADIN6"]
    #[inline(always)]
    #[must_use]
    pub fn padin6(&mut self) -> PADIN6_W<6> {
        PADIN6_W::new(self)
    }
    #[doc = "Bit 7 - PADIN7"]
    #[inline(always)]
    #[must_use]
    pub fn padin7(&mut self) -> PADIN7_W<7> {
        PADIN7_W::new(self)
    }
    #[doc = "Bit 8 - PADIN8"]
    #[inline(always)]
    #[must_use]
    pub fn padin8(&mut self) -> PADIN8_W<8> {
        PADIN8_W::new(self)
    }
    #[doc = "Bit 9 - PADIN9"]
    #[inline(always)]
    #[must_use]
    pub fn padin9(&mut self) -> PADIN9_W<9> {
        PADIN9_W::new(self)
    }
    #[doc = "Bit 10 - PADIN10"]
    #[inline(always)]
    #[must_use]
    pub fn padin10(&mut self) -> PADIN10_W<10> {
        PADIN10_W::new(self)
    }
    #[doc = "Bit 11 - PADIN11"]
    #[inline(always)]
    #[must_use]
    pub fn padin11(&mut self) -> PADIN11_W<11> {
        PADIN11_W::new(self)
    }
    #[doc = "Bit 12 - PADIN12"]
    #[inline(always)]
    #[must_use]
    pub fn padin12(&mut self) -> PADIN12_W<12> {
        PADIN12_W::new(self)
    }
    #[doc = "Bit 13 - PADIN13"]
    #[inline(always)]
    #[must_use]
    pub fn padin13(&mut self) -> PADIN13_W<13> {
        PADIN13_W::new(self)
    }
    #[doc = "Bit 14 - PADIN14"]
    #[inline(always)]
    #[must_use]
    pub fn padin14(&mut self) -> PADIN14_W<14> {
        PADIN14_W::new(self)
    }
    #[doc = "Bit 15 - PADIN15"]
    #[inline(always)]
    #[must_use]
    pub fn padin15(&mut self) -> PADIN15_W<15> {
        PADIN15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PADINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padinr](index.html) module"]
pub struct PADINR_SPEC;
impl crate::RegisterSpec for PADINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padinr::R](R) reader structure"]
impl crate::Readable for PADINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padinr::W](W) writer structure"]
impl crate::Writable for PADINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADINR to value 0"]
impl crate::Resettable for PADINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
