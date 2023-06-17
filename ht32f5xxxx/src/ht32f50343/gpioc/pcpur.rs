#[doc = "Register `PCPUR` reader"]
pub struct R(crate::R<PCPUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCPUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCPUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCPUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCPUR` writer"]
pub struct W(crate::W<PCPUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCPUR_SPEC>;
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
impl From<crate::W<PCPUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCPUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCPU0` reader - PCPU0"]
pub type PCPU0_R = crate::BitReader;
#[doc = "Field `PCPU0` writer - PCPU0"]
pub type PCPU0_W<'a, const O: u8> = crate::BitWriter<'a, PCPUR_SPEC, O>;
#[doc = "Field `PCPU1` reader - PCPU1"]
pub type PCPU1_R = crate::BitReader;
#[doc = "Field `PCPU1` writer - PCPU1"]
pub type PCPU1_W<'a, const O: u8> = crate::BitWriter<'a, PCPUR_SPEC, O>;
#[doc = "Field `PCPU2` reader - PCPU2"]
pub type PCPU2_R = crate::BitReader;
#[doc = "Field `PCPU2` writer - PCPU2"]
pub type PCPU2_W<'a, const O: u8> = crate::BitWriter<'a, PCPUR_SPEC, O>;
#[doc = "Field `PCPU3` reader - PCPU3"]
pub type PCPU3_R = crate::BitReader;
#[doc = "Field `PCPU3` writer - PCPU3"]
pub type PCPU3_W<'a, const O: u8> = crate::BitWriter<'a, PCPUR_SPEC, O>;
#[doc = "Field `PCPU4` reader - PCPU4"]
pub type PCPU4_R = crate::BitReader;
#[doc = "Field `PCPU4` writer - PCPU4"]
pub type PCPU4_W<'a, const O: u8> = crate::BitWriter<'a, PCPUR_SPEC, O>;
#[doc = "Field `PCPU5` reader - PCPU5"]
pub type PCPU5_R = crate::BitReader;
#[doc = "Field `PCPU5` writer - PCPU5"]
pub type PCPU5_W<'a, const O: u8> = crate::BitWriter<'a, PCPUR_SPEC, O>;
#[doc = "Field `PCPU6` reader - PCPU6"]
pub type PCPU6_R = crate::BitReader;
#[doc = "Field `PCPU6` writer - PCPU6"]
pub type PCPU6_W<'a, const O: u8> = crate::BitWriter<'a, PCPUR_SPEC, O>;
#[doc = "Field `PCPU7` reader - PCPU7"]
pub type PCPU7_R = crate::BitReader;
#[doc = "Field `PCPU7` writer - PCPU7"]
pub type PCPU7_W<'a, const O: u8> = crate::BitWriter<'a, PCPUR_SPEC, O>;
#[doc = "Field `PCPU8` reader - PCPU8"]
pub type PCPU8_R = crate::BitReader;
#[doc = "Field `PCPU8` writer - PCPU8"]
pub type PCPU8_W<'a, const O: u8> = crate::BitWriter<'a, PCPUR_SPEC, O>;
#[doc = "Field `PCPU9` reader - PCPU9"]
pub type PCPU9_R = crate::BitReader;
#[doc = "Field `PCPU9` writer - PCPU9"]
pub type PCPU9_W<'a, const O: u8> = crate::BitWriter<'a, PCPUR_SPEC, O>;
#[doc = "Field `PCPU10` reader - PCPU10"]
pub type PCPU10_R = crate::BitReader;
#[doc = "Field `PCPU10` writer - PCPU10"]
pub type PCPU10_W<'a, const O: u8> = crate::BitWriter<'a, PCPUR_SPEC, O>;
#[doc = "Field `PCPU11` reader - PCPU11"]
pub type PCPU11_R = crate::BitReader;
#[doc = "Field `PCPU11` writer - PCPU11"]
pub type PCPU11_W<'a, const O: u8> = crate::BitWriter<'a, PCPUR_SPEC, O>;
#[doc = "Field `PCPU12` reader - PCPU12"]
pub type PCPU12_R = crate::BitReader;
#[doc = "Field `PCPU12` writer - PCPU12"]
pub type PCPU12_W<'a, const O: u8> = crate::BitWriter<'a, PCPUR_SPEC, O>;
#[doc = "Field `PCPU13` reader - PCPU13"]
pub type PCPU13_R = crate::BitReader;
#[doc = "Field `PCPU13` writer - PCPU13"]
pub type PCPU13_W<'a, const O: u8> = crate::BitWriter<'a, PCPUR_SPEC, O>;
#[doc = "Field `PCPU14` reader - PCPU14"]
pub type PCPU14_R = crate::BitReader;
#[doc = "Field `PCPU14` writer - PCPU14"]
pub type PCPU14_W<'a, const O: u8> = crate::BitWriter<'a, PCPUR_SPEC, O>;
#[doc = "Field `PCPU15` reader - PCPU15"]
pub type PCPU15_R = crate::BitReader;
#[doc = "Field `PCPU15` writer - PCPU15"]
pub type PCPU15_W<'a, const O: u8> = crate::BitWriter<'a, PCPUR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PCPU0"]
    #[inline(always)]
    pub fn pcpu0(&self) -> PCPU0_R {
        PCPU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCPU1"]
    #[inline(always)]
    pub fn pcpu1(&self) -> PCPU1_R {
        PCPU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCPU2"]
    #[inline(always)]
    pub fn pcpu2(&self) -> PCPU2_R {
        PCPU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCPU3"]
    #[inline(always)]
    pub fn pcpu3(&self) -> PCPU3_R {
        PCPU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PCPU4"]
    #[inline(always)]
    pub fn pcpu4(&self) -> PCPU4_R {
        PCPU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCPU5"]
    #[inline(always)]
    pub fn pcpu5(&self) -> PCPU5_R {
        PCPU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCPU6"]
    #[inline(always)]
    pub fn pcpu6(&self) -> PCPU6_R {
        PCPU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PCPU7"]
    #[inline(always)]
    pub fn pcpu7(&self) -> PCPU7_R {
        PCPU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PCPU8"]
    #[inline(always)]
    pub fn pcpu8(&self) -> PCPU8_R {
        PCPU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PCPU9"]
    #[inline(always)]
    pub fn pcpu9(&self) -> PCPU9_R {
        PCPU9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCPU10"]
    #[inline(always)]
    pub fn pcpu10(&self) -> PCPU10_R {
        PCPU10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PCPU11"]
    #[inline(always)]
    pub fn pcpu11(&self) -> PCPU11_R {
        PCPU11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PCPU12"]
    #[inline(always)]
    pub fn pcpu12(&self) -> PCPU12_R {
        PCPU12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PCPU13"]
    #[inline(always)]
    pub fn pcpu13(&self) -> PCPU13_R {
        PCPU13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PCPU14"]
    #[inline(always)]
    pub fn pcpu14(&self) -> PCPU14_R {
        PCPU14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PCPU15"]
    #[inline(always)]
    pub fn pcpu15(&self) -> PCPU15_R {
        PCPU15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCPU0"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu0(&mut self) -> PCPU0_W<0> {
        PCPU0_W::new(self)
    }
    #[doc = "Bit 1 - PCPU1"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu1(&mut self) -> PCPU1_W<1> {
        PCPU1_W::new(self)
    }
    #[doc = "Bit 2 - PCPU2"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu2(&mut self) -> PCPU2_W<2> {
        PCPU2_W::new(self)
    }
    #[doc = "Bit 3 - PCPU3"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu3(&mut self) -> PCPU3_W<3> {
        PCPU3_W::new(self)
    }
    #[doc = "Bit 4 - PCPU4"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu4(&mut self) -> PCPU4_W<4> {
        PCPU4_W::new(self)
    }
    #[doc = "Bit 5 - PCPU5"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu5(&mut self) -> PCPU5_W<5> {
        PCPU5_W::new(self)
    }
    #[doc = "Bit 6 - PCPU6"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu6(&mut self) -> PCPU6_W<6> {
        PCPU6_W::new(self)
    }
    #[doc = "Bit 7 - PCPU7"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu7(&mut self) -> PCPU7_W<7> {
        PCPU7_W::new(self)
    }
    #[doc = "Bit 8 - PCPU8"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu8(&mut self) -> PCPU8_W<8> {
        PCPU8_W::new(self)
    }
    #[doc = "Bit 9 - PCPU9"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu9(&mut self) -> PCPU9_W<9> {
        PCPU9_W::new(self)
    }
    #[doc = "Bit 10 - PCPU10"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu10(&mut self) -> PCPU10_W<10> {
        PCPU10_W::new(self)
    }
    #[doc = "Bit 11 - PCPU11"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu11(&mut self) -> PCPU11_W<11> {
        PCPU11_W::new(self)
    }
    #[doc = "Bit 12 - PCPU12"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu12(&mut self) -> PCPU12_W<12> {
        PCPU12_W::new(self)
    }
    #[doc = "Bit 13 - PCPU13"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu13(&mut self) -> PCPU13_W<13> {
        PCPU13_W::new(self)
    }
    #[doc = "Bit 14 - PCPU14"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu14(&mut self) -> PCPU14_W<14> {
        PCPU14_W::new(self)
    }
    #[doc = "Bit 15 - PCPU15"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu15(&mut self) -> PCPU15_W<15> {
        PCPU15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCPUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcpur](index.html) module"]
pub struct PCPUR_SPEC;
impl crate::RegisterSpec for PCPUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcpur::R](R) reader structure"]
impl crate::Readable for PCPUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcpur::W](W) writer structure"]
impl crate::Writable for PCPUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCPUR to value 0"]
impl crate::Resettable for PCPUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
