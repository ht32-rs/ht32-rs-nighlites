#[doc = "Register `PBPUR` reader"]
pub struct R(crate::R<PBPUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBPUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBPUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBPUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBPUR` writer"]
pub struct W(crate::W<PBPUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBPUR_SPEC>;
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
impl From<crate::W<PBPUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBPUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBPU0` reader - PBPU0"]
pub type PBPU0_R = crate::BitReader;
#[doc = "Field `PBPU0` writer - PBPU0"]
pub type PBPU0_W<'a, const O: u8> = crate::BitWriter<'a, PBPUR_SPEC, O>;
#[doc = "Field `PBPU1` reader - PBPU1"]
pub type PBPU1_R = crate::BitReader;
#[doc = "Field `PBPU1` writer - PBPU1"]
pub type PBPU1_W<'a, const O: u8> = crate::BitWriter<'a, PBPUR_SPEC, O>;
#[doc = "Field `PBPU2` reader - PBPU2"]
pub type PBPU2_R = crate::BitReader;
#[doc = "Field `PBPU2` writer - PBPU2"]
pub type PBPU2_W<'a, const O: u8> = crate::BitWriter<'a, PBPUR_SPEC, O>;
#[doc = "Field `PBPU3` reader - PBPU3"]
pub type PBPU3_R = crate::BitReader;
#[doc = "Field `PBPU3` writer - PBPU3"]
pub type PBPU3_W<'a, const O: u8> = crate::BitWriter<'a, PBPUR_SPEC, O>;
#[doc = "Field `PBPU4` reader - PBPU4"]
pub type PBPU4_R = crate::BitReader;
#[doc = "Field `PBPU4` writer - PBPU4"]
pub type PBPU4_W<'a, const O: u8> = crate::BitWriter<'a, PBPUR_SPEC, O>;
#[doc = "Field `PBPU5` reader - PBPU5"]
pub type PBPU5_R = crate::BitReader;
#[doc = "Field `PBPU5` writer - PBPU5"]
pub type PBPU5_W<'a, const O: u8> = crate::BitWriter<'a, PBPUR_SPEC, O>;
#[doc = "Field `PBPU6` reader - PBPU6"]
pub type PBPU6_R = crate::BitReader;
#[doc = "Field `PBPU6` writer - PBPU6"]
pub type PBPU6_W<'a, const O: u8> = crate::BitWriter<'a, PBPUR_SPEC, O>;
#[doc = "Field `PBPU7` reader - PBPU7"]
pub type PBPU7_R = crate::BitReader;
#[doc = "Field `PBPU7` writer - PBPU7"]
pub type PBPU7_W<'a, const O: u8> = crate::BitWriter<'a, PBPUR_SPEC, O>;
#[doc = "Field `PBPU8` reader - PBPU8"]
pub type PBPU8_R = crate::BitReader;
#[doc = "Field `PBPU8` writer - PBPU8"]
pub type PBPU8_W<'a, const O: u8> = crate::BitWriter<'a, PBPUR_SPEC, O>;
#[doc = "Field `PBPU9` reader - PBPU9"]
pub type PBPU9_R = crate::BitReader;
#[doc = "Field `PBPU9` writer - PBPU9"]
pub type PBPU9_W<'a, const O: u8> = crate::BitWriter<'a, PBPUR_SPEC, O>;
#[doc = "Field `PBPU10` reader - PBPU10"]
pub type PBPU10_R = crate::BitReader;
#[doc = "Field `PBPU10` writer - PBPU10"]
pub type PBPU10_W<'a, const O: u8> = crate::BitWriter<'a, PBPUR_SPEC, O>;
#[doc = "Field `PBPU11` reader - PBPU11"]
pub type PBPU11_R = crate::BitReader;
#[doc = "Field `PBPU11` writer - PBPU11"]
pub type PBPU11_W<'a, const O: u8> = crate::BitWriter<'a, PBPUR_SPEC, O>;
#[doc = "Field `PBPU12` reader - PBPU12"]
pub type PBPU12_R = crate::BitReader;
#[doc = "Field `PBPU12` writer - PBPU12"]
pub type PBPU12_W<'a, const O: u8> = crate::BitWriter<'a, PBPUR_SPEC, O>;
#[doc = "Field `PBPU13` reader - PBPU13"]
pub type PBPU13_R = crate::BitReader;
#[doc = "Field `PBPU13` writer - PBPU13"]
pub type PBPU13_W<'a, const O: u8> = crate::BitWriter<'a, PBPUR_SPEC, O>;
#[doc = "Field `PBPU14` reader - PBPU14"]
pub type PBPU14_R = crate::BitReader;
#[doc = "Field `PBPU14` writer - PBPU14"]
pub type PBPU14_W<'a, const O: u8> = crate::BitWriter<'a, PBPUR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PBPU0"]
    #[inline(always)]
    pub fn pbpu0(&self) -> PBPU0_R {
        PBPU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PBPU1"]
    #[inline(always)]
    pub fn pbpu1(&self) -> PBPU1_R {
        PBPU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBPU2"]
    #[inline(always)]
    pub fn pbpu2(&self) -> PBPU2_R {
        PBPU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PBPU3"]
    #[inline(always)]
    pub fn pbpu3(&self) -> PBPU3_R {
        PBPU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PBPU4"]
    #[inline(always)]
    pub fn pbpu4(&self) -> PBPU4_R {
        PBPU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PBPU5"]
    #[inline(always)]
    pub fn pbpu5(&self) -> PBPU5_R {
        PBPU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PBPU6"]
    #[inline(always)]
    pub fn pbpu6(&self) -> PBPU6_R {
        PBPU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PBPU7"]
    #[inline(always)]
    pub fn pbpu7(&self) -> PBPU7_R {
        PBPU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PBPU8"]
    #[inline(always)]
    pub fn pbpu8(&self) -> PBPU8_R {
        PBPU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PBPU9"]
    #[inline(always)]
    pub fn pbpu9(&self) -> PBPU9_R {
        PBPU9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PBPU10"]
    #[inline(always)]
    pub fn pbpu10(&self) -> PBPU10_R {
        PBPU10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PBPU11"]
    #[inline(always)]
    pub fn pbpu11(&self) -> PBPU11_R {
        PBPU11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PBPU12"]
    #[inline(always)]
    pub fn pbpu12(&self) -> PBPU12_R {
        PBPU12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PBPU13"]
    #[inline(always)]
    pub fn pbpu13(&self) -> PBPU13_R {
        PBPU13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PBPU14"]
    #[inline(always)]
    pub fn pbpu14(&self) -> PBPU14_R {
        PBPU14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PBPU0"]
    #[inline(always)]
    #[must_use]
    pub fn pbpu0(&mut self) -> PBPU0_W<0> {
        PBPU0_W::new(self)
    }
    #[doc = "Bit 1 - PBPU1"]
    #[inline(always)]
    #[must_use]
    pub fn pbpu1(&mut self) -> PBPU1_W<1> {
        PBPU1_W::new(self)
    }
    #[doc = "Bit 2 - PBPU2"]
    #[inline(always)]
    #[must_use]
    pub fn pbpu2(&mut self) -> PBPU2_W<2> {
        PBPU2_W::new(self)
    }
    #[doc = "Bit 3 - PBPU3"]
    #[inline(always)]
    #[must_use]
    pub fn pbpu3(&mut self) -> PBPU3_W<3> {
        PBPU3_W::new(self)
    }
    #[doc = "Bit 4 - PBPU4"]
    #[inline(always)]
    #[must_use]
    pub fn pbpu4(&mut self) -> PBPU4_W<4> {
        PBPU4_W::new(self)
    }
    #[doc = "Bit 5 - PBPU5"]
    #[inline(always)]
    #[must_use]
    pub fn pbpu5(&mut self) -> PBPU5_W<5> {
        PBPU5_W::new(self)
    }
    #[doc = "Bit 6 - PBPU6"]
    #[inline(always)]
    #[must_use]
    pub fn pbpu6(&mut self) -> PBPU6_W<6> {
        PBPU6_W::new(self)
    }
    #[doc = "Bit 7 - PBPU7"]
    #[inline(always)]
    #[must_use]
    pub fn pbpu7(&mut self) -> PBPU7_W<7> {
        PBPU7_W::new(self)
    }
    #[doc = "Bit 8 - PBPU8"]
    #[inline(always)]
    #[must_use]
    pub fn pbpu8(&mut self) -> PBPU8_W<8> {
        PBPU8_W::new(self)
    }
    #[doc = "Bit 9 - PBPU9"]
    #[inline(always)]
    #[must_use]
    pub fn pbpu9(&mut self) -> PBPU9_W<9> {
        PBPU9_W::new(self)
    }
    #[doc = "Bit 10 - PBPU10"]
    #[inline(always)]
    #[must_use]
    pub fn pbpu10(&mut self) -> PBPU10_W<10> {
        PBPU10_W::new(self)
    }
    #[doc = "Bit 11 - PBPU11"]
    #[inline(always)]
    #[must_use]
    pub fn pbpu11(&mut self) -> PBPU11_W<11> {
        PBPU11_W::new(self)
    }
    #[doc = "Bit 12 - PBPU12"]
    #[inline(always)]
    #[must_use]
    pub fn pbpu12(&mut self) -> PBPU12_W<12> {
        PBPU12_W::new(self)
    }
    #[doc = "Bit 13 - PBPU13"]
    #[inline(always)]
    #[must_use]
    pub fn pbpu13(&mut self) -> PBPU13_W<13> {
        PBPU13_W::new(self)
    }
    #[doc = "Bit 14 - PBPU14"]
    #[inline(always)]
    #[must_use]
    pub fn pbpu14(&mut self) -> PBPU14_W<14> {
        PBPU14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBPUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbpur](index.html) module"]
pub struct PBPUR_SPEC;
impl crate::RegisterSpec for PBPUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbpur::R](R) reader structure"]
impl crate::Readable for PBPUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbpur::W](W) writer structure"]
impl crate::Writable for PBPUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBPUR to value 0"]
impl crate::Resettable for PBPUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
