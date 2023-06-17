#[doc = "Register `PAPUR` reader"]
pub struct R(crate::R<PAPUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAPUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAPUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAPUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAPUR` writer"]
pub struct W(crate::W<PAPUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAPUR_SPEC>;
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
impl From<crate::W<PAPUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAPUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAPU0` reader - PAPU0"]
pub type PAPU0_R = crate::BitReader;
#[doc = "Field `PAPU0` writer - PAPU0"]
pub type PAPU0_W<'a, const O: u8> = crate::BitWriter<'a, PAPUR_SPEC, O>;
#[doc = "Field `PAPU1` reader - PAPU1"]
pub type PAPU1_R = crate::BitReader;
#[doc = "Field `PAPU1` writer - PAPU1"]
pub type PAPU1_W<'a, const O: u8> = crate::BitWriter<'a, PAPUR_SPEC, O>;
#[doc = "Field `PAPU2` reader - PAPU2"]
pub type PAPU2_R = crate::BitReader;
#[doc = "Field `PAPU2` writer - PAPU2"]
pub type PAPU2_W<'a, const O: u8> = crate::BitWriter<'a, PAPUR_SPEC, O>;
#[doc = "Field `PAPU3` reader - PAPU3"]
pub type PAPU3_R = crate::BitReader;
#[doc = "Field `PAPU3` writer - PAPU3"]
pub type PAPU3_W<'a, const O: u8> = crate::BitWriter<'a, PAPUR_SPEC, O>;
#[doc = "Field `PAPU4` reader - PAPU4"]
pub type PAPU4_R = crate::BitReader;
#[doc = "Field `PAPU4` writer - PAPU4"]
pub type PAPU4_W<'a, const O: u8> = crate::BitWriter<'a, PAPUR_SPEC, O>;
#[doc = "Field `PAPU5` reader - PAPU5"]
pub type PAPU5_R = crate::BitReader;
#[doc = "Field `PAPU5` writer - PAPU5"]
pub type PAPU5_W<'a, const O: u8> = crate::BitWriter<'a, PAPUR_SPEC, O>;
#[doc = "Field `PAPU6` reader - PAPU6"]
pub type PAPU6_R = crate::BitReader;
#[doc = "Field `PAPU6` writer - PAPU6"]
pub type PAPU6_W<'a, const O: u8> = crate::BitWriter<'a, PAPUR_SPEC, O>;
#[doc = "Field `PAPU7` reader - PAPU7"]
pub type PAPU7_R = crate::BitReader;
#[doc = "Field `PAPU7` writer - PAPU7"]
pub type PAPU7_W<'a, const O: u8> = crate::BitWriter<'a, PAPUR_SPEC, O>;
#[doc = "Field `PAPU8` reader - PAPU8"]
pub type PAPU8_R = crate::BitReader;
#[doc = "Field `PAPU8` writer - PAPU8"]
pub type PAPU8_W<'a, const O: u8> = crate::BitWriter<'a, PAPUR_SPEC, O>;
#[doc = "Field `PAPU9` reader - PAPU9"]
pub type PAPU9_R = crate::BitReader;
#[doc = "Field `PAPU9` writer - PAPU9"]
pub type PAPU9_W<'a, const O: u8> = crate::BitWriter<'a, PAPUR_SPEC, O>;
#[doc = "Field `PAPU10` reader - PAPU10"]
pub type PAPU10_R = crate::BitReader;
#[doc = "Field `PAPU10` writer - PAPU10"]
pub type PAPU10_W<'a, const O: u8> = crate::BitWriter<'a, PAPUR_SPEC, O>;
#[doc = "Field `PAPU11` reader - PAPU11"]
pub type PAPU11_R = crate::BitReader;
#[doc = "Field `PAPU11` writer - PAPU11"]
pub type PAPU11_W<'a, const O: u8> = crate::BitWriter<'a, PAPUR_SPEC, O>;
#[doc = "Field `PAPU12` reader - PAPU12"]
pub type PAPU12_R = crate::BitReader;
#[doc = "Field `PAPU12` writer - PAPU12"]
pub type PAPU12_W<'a, const O: u8> = crate::BitWriter<'a, PAPUR_SPEC, O>;
#[doc = "Field `PAPU13` reader - PAPU13"]
pub type PAPU13_R = crate::BitReader;
#[doc = "Field `PAPU13` writer - PAPU13"]
pub type PAPU13_W<'a, const O: u8> = crate::BitWriter<'a, PAPUR_SPEC, O>;
#[doc = "Field `PAPU14` reader - PAPU14"]
pub type PAPU14_R = crate::BitReader;
#[doc = "Field `PAPU14` writer - PAPU14"]
pub type PAPU14_W<'a, const O: u8> = crate::BitWriter<'a, PAPUR_SPEC, O>;
#[doc = "Field `PAPU15` reader - PAPU15"]
pub type PAPU15_R = crate::BitReader;
#[doc = "Field `PAPU15` writer - PAPU15"]
pub type PAPU15_W<'a, const O: u8> = crate::BitWriter<'a, PAPUR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PAPU0"]
    #[inline(always)]
    pub fn papu0(&self) -> PAPU0_R {
        PAPU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PAPU1"]
    #[inline(always)]
    pub fn papu1(&self) -> PAPU1_R {
        PAPU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PAPU2"]
    #[inline(always)]
    pub fn papu2(&self) -> PAPU2_R {
        PAPU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PAPU3"]
    #[inline(always)]
    pub fn papu3(&self) -> PAPU3_R {
        PAPU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PAPU4"]
    #[inline(always)]
    pub fn papu4(&self) -> PAPU4_R {
        PAPU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PAPU5"]
    #[inline(always)]
    pub fn papu5(&self) -> PAPU5_R {
        PAPU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PAPU6"]
    #[inline(always)]
    pub fn papu6(&self) -> PAPU6_R {
        PAPU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PAPU7"]
    #[inline(always)]
    pub fn papu7(&self) -> PAPU7_R {
        PAPU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PAPU8"]
    #[inline(always)]
    pub fn papu8(&self) -> PAPU8_R {
        PAPU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PAPU9"]
    #[inline(always)]
    pub fn papu9(&self) -> PAPU9_R {
        PAPU9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PAPU10"]
    #[inline(always)]
    pub fn papu10(&self) -> PAPU10_R {
        PAPU10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAPU11"]
    #[inline(always)]
    pub fn papu11(&self) -> PAPU11_R {
        PAPU11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAPU12"]
    #[inline(always)]
    pub fn papu12(&self) -> PAPU12_R {
        PAPU12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAPU13"]
    #[inline(always)]
    pub fn papu13(&self) -> PAPU13_R {
        PAPU13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAPU14"]
    #[inline(always)]
    pub fn papu14(&self) -> PAPU14_R {
        PAPU14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAPU15"]
    #[inline(always)]
    pub fn papu15(&self) -> PAPU15_R {
        PAPU15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAPU0"]
    #[inline(always)]
    #[must_use]
    pub fn papu0(&mut self) -> PAPU0_W<0> {
        PAPU0_W::new(self)
    }
    #[doc = "Bit 1 - PAPU1"]
    #[inline(always)]
    #[must_use]
    pub fn papu1(&mut self) -> PAPU1_W<1> {
        PAPU1_W::new(self)
    }
    #[doc = "Bit 2 - PAPU2"]
    #[inline(always)]
    #[must_use]
    pub fn papu2(&mut self) -> PAPU2_W<2> {
        PAPU2_W::new(self)
    }
    #[doc = "Bit 3 - PAPU3"]
    #[inline(always)]
    #[must_use]
    pub fn papu3(&mut self) -> PAPU3_W<3> {
        PAPU3_W::new(self)
    }
    #[doc = "Bit 4 - PAPU4"]
    #[inline(always)]
    #[must_use]
    pub fn papu4(&mut self) -> PAPU4_W<4> {
        PAPU4_W::new(self)
    }
    #[doc = "Bit 5 - PAPU5"]
    #[inline(always)]
    #[must_use]
    pub fn papu5(&mut self) -> PAPU5_W<5> {
        PAPU5_W::new(self)
    }
    #[doc = "Bit 6 - PAPU6"]
    #[inline(always)]
    #[must_use]
    pub fn papu6(&mut self) -> PAPU6_W<6> {
        PAPU6_W::new(self)
    }
    #[doc = "Bit 7 - PAPU7"]
    #[inline(always)]
    #[must_use]
    pub fn papu7(&mut self) -> PAPU7_W<7> {
        PAPU7_W::new(self)
    }
    #[doc = "Bit 8 - PAPU8"]
    #[inline(always)]
    #[must_use]
    pub fn papu8(&mut self) -> PAPU8_W<8> {
        PAPU8_W::new(self)
    }
    #[doc = "Bit 9 - PAPU9"]
    #[inline(always)]
    #[must_use]
    pub fn papu9(&mut self) -> PAPU9_W<9> {
        PAPU9_W::new(self)
    }
    #[doc = "Bit 10 - PAPU10"]
    #[inline(always)]
    #[must_use]
    pub fn papu10(&mut self) -> PAPU10_W<10> {
        PAPU10_W::new(self)
    }
    #[doc = "Bit 11 - PAPU11"]
    #[inline(always)]
    #[must_use]
    pub fn papu11(&mut self) -> PAPU11_W<11> {
        PAPU11_W::new(self)
    }
    #[doc = "Bit 12 - PAPU12"]
    #[inline(always)]
    #[must_use]
    pub fn papu12(&mut self) -> PAPU12_W<12> {
        PAPU12_W::new(self)
    }
    #[doc = "Bit 13 - PAPU13"]
    #[inline(always)]
    #[must_use]
    pub fn papu13(&mut self) -> PAPU13_W<13> {
        PAPU13_W::new(self)
    }
    #[doc = "Bit 14 - PAPU14"]
    #[inline(always)]
    #[must_use]
    pub fn papu14(&mut self) -> PAPU14_W<14> {
        PAPU14_W::new(self)
    }
    #[doc = "Bit 15 - PAPU15"]
    #[inline(always)]
    #[must_use]
    pub fn papu15(&mut self) -> PAPU15_W<15> {
        PAPU15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAPUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [papur](index.html) module"]
pub struct PAPUR_SPEC;
impl crate::RegisterSpec for PAPUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [papur::R](R) reader structure"]
impl crate::Readable for PAPUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [papur::W](W) writer structure"]
impl crate::Writable for PAPUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAPUR to value 0"]
impl crate::Resettable for PAPUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
