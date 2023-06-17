#[doc = "Register `PDODR` reader"]
pub struct R(crate::R<PDODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDODR` writer"]
pub struct W(crate::W<PDODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDODR_SPEC>;
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
impl From<crate::W<PDODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDOD0` reader - PDOD0"]
pub type PDOD0_R = crate::BitReader;
#[doc = "Field `PDOD0` writer - PDOD0"]
pub type PDOD0_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD1` reader - PDOD1"]
pub type PDOD1_R = crate::BitReader;
#[doc = "Field `PDOD1` writer - PDOD1"]
pub type PDOD1_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD2` reader - PDOD2"]
pub type PDOD2_R = crate::BitReader;
#[doc = "Field `PDOD2` writer - PDOD2"]
pub type PDOD2_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD3` reader - PDOD3"]
pub type PDOD3_R = crate::BitReader;
#[doc = "Field `PDOD3` writer - PDOD3"]
pub type PDOD3_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD4` reader - PDOD4"]
pub type PDOD4_R = crate::BitReader;
#[doc = "Field `PDOD4` writer - PDOD4"]
pub type PDOD4_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD5` reader - PDOD5"]
pub type PDOD5_R = crate::BitReader;
#[doc = "Field `PDOD5` writer - PDOD5"]
pub type PDOD5_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD6` reader - PDOD6"]
pub type PDOD6_R = crate::BitReader;
#[doc = "Field `PDOD6` writer - PDOD6"]
pub type PDOD6_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD7` reader - PDOD7"]
pub type PDOD7_R = crate::BitReader;
#[doc = "Field `PDOD7` writer - PDOD7"]
pub type PDOD7_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD8` reader - PDOD8"]
pub type PDOD8_R = crate::BitReader;
#[doc = "Field `PDOD8` writer - PDOD8"]
pub type PDOD8_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD9` reader - PDOD9"]
pub type PDOD9_R = crate::BitReader;
#[doc = "Field `PDOD9` writer - PDOD9"]
pub type PDOD9_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD10` reader - PDOD10"]
pub type PDOD10_R = crate::BitReader;
#[doc = "Field `PDOD10` writer - PDOD10"]
pub type PDOD10_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD11` reader - PDOD11"]
pub type PDOD11_R = crate::BitReader;
#[doc = "Field `PDOD11` writer - PDOD11"]
pub type PDOD11_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD12` reader - PDOD12"]
pub type PDOD12_R = crate::BitReader;
#[doc = "Field `PDOD12` writer - PDOD12"]
pub type PDOD12_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD13` reader - PDOD13"]
pub type PDOD13_R = crate::BitReader;
#[doc = "Field `PDOD13` writer - PDOD13"]
pub type PDOD13_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD14` reader - PDOD14"]
pub type PDOD14_R = crate::BitReader;
#[doc = "Field `PDOD14` writer - PDOD14"]
pub type PDOD14_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD15` reader - PDOD15"]
pub type PDOD15_R = crate::BitReader;
#[doc = "Field `PDOD15` writer - PDOD15"]
pub type PDOD15_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDOD0"]
    #[inline(always)]
    pub fn pdod0(&self) -> PDOD0_R {
        PDOD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDOD1"]
    #[inline(always)]
    pub fn pdod1(&self) -> PDOD1_R {
        PDOD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDOD2"]
    #[inline(always)]
    pub fn pdod2(&self) -> PDOD2_R {
        PDOD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDOD3"]
    #[inline(always)]
    pub fn pdod3(&self) -> PDOD3_R {
        PDOD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PDOD4"]
    #[inline(always)]
    pub fn pdod4(&self) -> PDOD4_R {
        PDOD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PDOD5"]
    #[inline(always)]
    pub fn pdod5(&self) -> PDOD5_R {
        PDOD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDOD6"]
    #[inline(always)]
    pub fn pdod6(&self) -> PDOD6_R {
        PDOD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDOD7"]
    #[inline(always)]
    pub fn pdod7(&self) -> PDOD7_R {
        PDOD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PDOD8"]
    #[inline(always)]
    pub fn pdod8(&self) -> PDOD8_R {
        PDOD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PDOD9"]
    #[inline(always)]
    pub fn pdod9(&self) -> PDOD9_R {
        PDOD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PDOD10"]
    #[inline(always)]
    pub fn pdod10(&self) -> PDOD10_R {
        PDOD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PDOD11"]
    #[inline(always)]
    pub fn pdod11(&self) -> PDOD11_R {
        PDOD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PDOD12"]
    #[inline(always)]
    pub fn pdod12(&self) -> PDOD12_R {
        PDOD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PDOD13"]
    #[inline(always)]
    pub fn pdod13(&self) -> PDOD13_R {
        PDOD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PDOD14"]
    #[inline(always)]
    pub fn pdod14(&self) -> PDOD14_R {
        PDOD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PDOD15"]
    #[inline(always)]
    pub fn pdod15(&self) -> PDOD15_R {
        PDOD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDOD0"]
    #[inline(always)]
    #[must_use]
    pub fn pdod0(&mut self) -> PDOD0_W<0> {
        PDOD0_W::new(self)
    }
    #[doc = "Bit 1 - PDOD1"]
    #[inline(always)]
    #[must_use]
    pub fn pdod1(&mut self) -> PDOD1_W<1> {
        PDOD1_W::new(self)
    }
    #[doc = "Bit 2 - PDOD2"]
    #[inline(always)]
    #[must_use]
    pub fn pdod2(&mut self) -> PDOD2_W<2> {
        PDOD2_W::new(self)
    }
    #[doc = "Bit 3 - PDOD3"]
    #[inline(always)]
    #[must_use]
    pub fn pdod3(&mut self) -> PDOD3_W<3> {
        PDOD3_W::new(self)
    }
    #[doc = "Bit 4 - PDOD4"]
    #[inline(always)]
    #[must_use]
    pub fn pdod4(&mut self) -> PDOD4_W<4> {
        PDOD4_W::new(self)
    }
    #[doc = "Bit 5 - PDOD5"]
    #[inline(always)]
    #[must_use]
    pub fn pdod5(&mut self) -> PDOD5_W<5> {
        PDOD5_W::new(self)
    }
    #[doc = "Bit 6 - PDOD6"]
    #[inline(always)]
    #[must_use]
    pub fn pdod6(&mut self) -> PDOD6_W<6> {
        PDOD6_W::new(self)
    }
    #[doc = "Bit 7 - PDOD7"]
    #[inline(always)]
    #[must_use]
    pub fn pdod7(&mut self) -> PDOD7_W<7> {
        PDOD7_W::new(self)
    }
    #[doc = "Bit 8 - PDOD8"]
    #[inline(always)]
    #[must_use]
    pub fn pdod8(&mut self) -> PDOD8_W<8> {
        PDOD8_W::new(self)
    }
    #[doc = "Bit 9 - PDOD9"]
    #[inline(always)]
    #[must_use]
    pub fn pdod9(&mut self) -> PDOD9_W<9> {
        PDOD9_W::new(self)
    }
    #[doc = "Bit 10 - PDOD10"]
    #[inline(always)]
    #[must_use]
    pub fn pdod10(&mut self) -> PDOD10_W<10> {
        PDOD10_W::new(self)
    }
    #[doc = "Bit 11 - PDOD11"]
    #[inline(always)]
    #[must_use]
    pub fn pdod11(&mut self) -> PDOD11_W<11> {
        PDOD11_W::new(self)
    }
    #[doc = "Bit 12 - PDOD12"]
    #[inline(always)]
    #[must_use]
    pub fn pdod12(&mut self) -> PDOD12_W<12> {
        PDOD12_W::new(self)
    }
    #[doc = "Bit 13 - PDOD13"]
    #[inline(always)]
    #[must_use]
    pub fn pdod13(&mut self) -> PDOD13_W<13> {
        PDOD13_W::new(self)
    }
    #[doc = "Bit 14 - PDOD14"]
    #[inline(always)]
    #[must_use]
    pub fn pdod14(&mut self) -> PDOD14_W<14> {
        PDOD14_W::new(self)
    }
    #[doc = "Bit 15 - PDOD15"]
    #[inline(always)]
    #[must_use]
    pub fn pdod15(&mut self) -> PDOD15_W<15> {
        PDOD15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdodr](index.html) module"]
pub struct PDODR_SPEC;
impl crate::RegisterSpec for PDODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdodr::R](R) reader structure"]
impl crate::Readable for PDODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdodr::W](W) writer structure"]
impl crate::Writable for PDODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDODR to value 0"]
impl crate::Resettable for PDODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
