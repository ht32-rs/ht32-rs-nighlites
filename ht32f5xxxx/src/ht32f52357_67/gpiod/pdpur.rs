#[doc = "Register `PDPUR` reader"]
pub struct R(crate::R<PDPUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDPUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDPUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDPUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDPUR` writer"]
pub struct W(crate::W<PDPUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDPUR_SPEC>;
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
impl From<crate::W<PDPUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDPUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDPU0` reader - PDPU0"]
pub type PDPU0_R = crate::BitReader;
#[doc = "Field `PDPU0` writer - PDPU0"]
pub type PDPU0_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU1` reader - PDPU1"]
pub type PDPU1_R = crate::BitReader;
#[doc = "Field `PDPU1` writer - PDPU1"]
pub type PDPU1_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU2` reader - PDPU2"]
pub type PDPU2_R = crate::BitReader;
#[doc = "Field `PDPU2` writer - PDPU2"]
pub type PDPU2_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU3` reader - PDPU3"]
pub type PDPU3_R = crate::BitReader;
#[doc = "Field `PDPU3` writer - PDPU3"]
pub type PDPU3_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU4` reader - PDPU4"]
pub type PDPU4_R = crate::BitReader;
#[doc = "Field `PDPU4` writer - PDPU4"]
pub type PDPU4_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU5` reader - PDPU5"]
pub type PDPU5_R = crate::BitReader;
#[doc = "Field `PDPU5` writer - PDPU5"]
pub type PDPU5_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU6` reader - PDPU6"]
pub type PDPU6_R = crate::BitReader;
#[doc = "Field `PDPU6` writer - PDPU6"]
pub type PDPU6_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU7` reader - PDPU7"]
pub type PDPU7_R = crate::BitReader;
#[doc = "Field `PDPU7` writer - PDPU7"]
pub type PDPU7_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU8` reader - PDPU8"]
pub type PDPU8_R = crate::BitReader;
#[doc = "Field `PDPU8` writer - PDPU8"]
pub type PDPU8_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU9` reader - PDPU9"]
pub type PDPU9_R = crate::BitReader;
#[doc = "Field `PDPU9` writer - PDPU9"]
pub type PDPU9_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU10` reader - PDPU10"]
pub type PDPU10_R = crate::BitReader;
#[doc = "Field `PDPU10` writer - PDPU10"]
pub type PDPU10_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU11` reader - PDPU11"]
pub type PDPU11_R = crate::BitReader;
#[doc = "Field `PDPU11` writer - PDPU11"]
pub type PDPU11_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU12` reader - PDPU12"]
pub type PDPU12_R = crate::BitReader;
#[doc = "Field `PDPU12` writer - PDPU12"]
pub type PDPU12_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU13` reader - PDPU13"]
pub type PDPU13_R = crate::BitReader;
#[doc = "Field `PDPU13` writer - PDPU13"]
pub type PDPU13_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU14` reader - PDPU14"]
pub type PDPU14_R = crate::BitReader;
#[doc = "Field `PDPU14` writer - PDPU14"]
pub type PDPU14_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU15` reader - PDPU15"]
pub type PDPU15_R = crate::BitReader;
#[doc = "Field `PDPU15` writer - PDPU15"]
pub type PDPU15_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDPU0"]
    #[inline(always)]
    pub fn pdpu0(&self) -> PDPU0_R {
        PDPU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDPU1"]
    #[inline(always)]
    pub fn pdpu1(&self) -> PDPU1_R {
        PDPU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDPU2"]
    #[inline(always)]
    pub fn pdpu2(&self) -> PDPU2_R {
        PDPU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDPU3"]
    #[inline(always)]
    pub fn pdpu3(&self) -> PDPU3_R {
        PDPU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PDPU4"]
    #[inline(always)]
    pub fn pdpu4(&self) -> PDPU4_R {
        PDPU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PDPU5"]
    #[inline(always)]
    pub fn pdpu5(&self) -> PDPU5_R {
        PDPU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDPU6"]
    #[inline(always)]
    pub fn pdpu6(&self) -> PDPU6_R {
        PDPU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDPU7"]
    #[inline(always)]
    pub fn pdpu7(&self) -> PDPU7_R {
        PDPU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PDPU8"]
    #[inline(always)]
    pub fn pdpu8(&self) -> PDPU8_R {
        PDPU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PDPU9"]
    #[inline(always)]
    pub fn pdpu9(&self) -> PDPU9_R {
        PDPU9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PDPU10"]
    #[inline(always)]
    pub fn pdpu10(&self) -> PDPU10_R {
        PDPU10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PDPU11"]
    #[inline(always)]
    pub fn pdpu11(&self) -> PDPU11_R {
        PDPU11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PDPU12"]
    #[inline(always)]
    pub fn pdpu12(&self) -> PDPU12_R {
        PDPU12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PDPU13"]
    #[inline(always)]
    pub fn pdpu13(&self) -> PDPU13_R {
        PDPU13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PDPU14"]
    #[inline(always)]
    pub fn pdpu14(&self) -> PDPU14_R {
        PDPU14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PDPU15"]
    #[inline(always)]
    pub fn pdpu15(&self) -> PDPU15_R {
        PDPU15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDPU0"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu0(&mut self) -> PDPU0_W<0> {
        PDPU0_W::new(self)
    }
    #[doc = "Bit 1 - PDPU1"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu1(&mut self) -> PDPU1_W<1> {
        PDPU1_W::new(self)
    }
    #[doc = "Bit 2 - PDPU2"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu2(&mut self) -> PDPU2_W<2> {
        PDPU2_W::new(self)
    }
    #[doc = "Bit 3 - PDPU3"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu3(&mut self) -> PDPU3_W<3> {
        PDPU3_W::new(self)
    }
    #[doc = "Bit 4 - PDPU4"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu4(&mut self) -> PDPU4_W<4> {
        PDPU4_W::new(self)
    }
    #[doc = "Bit 5 - PDPU5"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu5(&mut self) -> PDPU5_W<5> {
        PDPU5_W::new(self)
    }
    #[doc = "Bit 6 - PDPU6"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu6(&mut self) -> PDPU6_W<6> {
        PDPU6_W::new(self)
    }
    #[doc = "Bit 7 - PDPU7"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu7(&mut self) -> PDPU7_W<7> {
        PDPU7_W::new(self)
    }
    #[doc = "Bit 8 - PDPU8"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu8(&mut self) -> PDPU8_W<8> {
        PDPU8_W::new(self)
    }
    #[doc = "Bit 9 - PDPU9"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu9(&mut self) -> PDPU9_W<9> {
        PDPU9_W::new(self)
    }
    #[doc = "Bit 10 - PDPU10"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu10(&mut self) -> PDPU10_W<10> {
        PDPU10_W::new(self)
    }
    #[doc = "Bit 11 - PDPU11"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu11(&mut self) -> PDPU11_W<11> {
        PDPU11_W::new(self)
    }
    #[doc = "Bit 12 - PDPU12"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu12(&mut self) -> PDPU12_W<12> {
        PDPU12_W::new(self)
    }
    #[doc = "Bit 13 - PDPU13"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu13(&mut self) -> PDPU13_W<13> {
        PDPU13_W::new(self)
    }
    #[doc = "Bit 14 - PDPU14"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu14(&mut self) -> PDPU14_W<14> {
        PDPU14_W::new(self)
    }
    #[doc = "Bit 15 - PDPU15"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu15(&mut self) -> PDPU15_W<15> {
        PDPU15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDPUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdpur](index.html) module"]
pub struct PDPUR_SPEC;
impl crate::RegisterSpec for PDPUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdpur::R](R) reader structure"]
impl crate::Readable for PDPUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdpur::W](W) writer structure"]
impl crate::Writable for PDPUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDPUR to value 0"]
impl crate::Resettable for PDPUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
