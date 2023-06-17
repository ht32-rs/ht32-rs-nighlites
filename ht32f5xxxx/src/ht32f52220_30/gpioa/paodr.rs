#[doc = "Register `PAODR` reader"]
pub struct R(crate::R<PAODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAODR` writer"]
pub struct W(crate::W<PAODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAODR_SPEC>;
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
impl From<crate::W<PAODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAOD0` reader - PAOD0"]
pub type PAOD0_R = crate::BitReader;
#[doc = "Field `PAOD0` writer - PAOD0"]
pub type PAOD0_W<'a, const O: u8> = crate::BitWriter<'a, PAODR_SPEC, O>;
#[doc = "Field `PAOD1` reader - PAOD1"]
pub type PAOD1_R = crate::BitReader;
#[doc = "Field `PAOD1` writer - PAOD1"]
pub type PAOD1_W<'a, const O: u8> = crate::BitWriter<'a, PAODR_SPEC, O>;
#[doc = "Field `PAOD2` reader - PAOD2"]
pub type PAOD2_R = crate::BitReader;
#[doc = "Field `PAOD2` writer - PAOD2"]
pub type PAOD2_W<'a, const O: u8> = crate::BitWriter<'a, PAODR_SPEC, O>;
#[doc = "Field `PAOD3` reader - PAOD3"]
pub type PAOD3_R = crate::BitReader;
#[doc = "Field `PAOD3` writer - PAOD3"]
pub type PAOD3_W<'a, const O: u8> = crate::BitWriter<'a, PAODR_SPEC, O>;
#[doc = "Field `PAOD4` reader - PAOD4"]
pub type PAOD4_R = crate::BitReader;
#[doc = "Field `PAOD4` writer - PAOD4"]
pub type PAOD4_W<'a, const O: u8> = crate::BitWriter<'a, PAODR_SPEC, O>;
#[doc = "Field `PAOD5` reader - PAOD5"]
pub type PAOD5_R = crate::BitReader;
#[doc = "Field `PAOD5` writer - PAOD5"]
pub type PAOD5_W<'a, const O: u8> = crate::BitWriter<'a, PAODR_SPEC, O>;
#[doc = "Field `PAOD6` reader - PAOD6"]
pub type PAOD6_R = crate::BitReader;
#[doc = "Field `PAOD6` writer - PAOD6"]
pub type PAOD6_W<'a, const O: u8> = crate::BitWriter<'a, PAODR_SPEC, O>;
#[doc = "Field `PAOD7` reader - PAOD7"]
pub type PAOD7_R = crate::BitReader;
#[doc = "Field `PAOD7` writer - PAOD7"]
pub type PAOD7_W<'a, const O: u8> = crate::BitWriter<'a, PAODR_SPEC, O>;
#[doc = "Field `PAOD8` reader - PAOD8"]
pub type PAOD8_R = crate::BitReader;
#[doc = "Field `PAOD8` writer - PAOD8"]
pub type PAOD8_W<'a, const O: u8> = crate::BitWriter<'a, PAODR_SPEC, O>;
#[doc = "Field `PAOD9` reader - PAOD9"]
pub type PAOD9_R = crate::BitReader;
#[doc = "Field `PAOD9` writer - PAOD9"]
pub type PAOD9_W<'a, const O: u8> = crate::BitWriter<'a, PAODR_SPEC, O>;
#[doc = "Field `PAOD10` reader - PAOD10"]
pub type PAOD10_R = crate::BitReader;
#[doc = "Field `PAOD10` writer - PAOD10"]
pub type PAOD10_W<'a, const O: u8> = crate::BitWriter<'a, PAODR_SPEC, O>;
#[doc = "Field `PAOD11` reader - PAOD11"]
pub type PAOD11_R = crate::BitReader;
#[doc = "Field `PAOD11` writer - PAOD11"]
pub type PAOD11_W<'a, const O: u8> = crate::BitWriter<'a, PAODR_SPEC, O>;
#[doc = "Field `PAOD12` reader - PAOD12"]
pub type PAOD12_R = crate::BitReader;
#[doc = "Field `PAOD12` writer - PAOD12"]
pub type PAOD12_W<'a, const O: u8> = crate::BitWriter<'a, PAODR_SPEC, O>;
#[doc = "Field `PAOD13` reader - PAOD13"]
pub type PAOD13_R = crate::BitReader;
#[doc = "Field `PAOD13` writer - PAOD13"]
pub type PAOD13_W<'a, const O: u8> = crate::BitWriter<'a, PAODR_SPEC, O>;
#[doc = "Field `PAOD14` reader - PAOD14"]
pub type PAOD14_R = crate::BitReader;
#[doc = "Field `PAOD14` writer - PAOD14"]
pub type PAOD14_W<'a, const O: u8> = crate::BitWriter<'a, PAODR_SPEC, O>;
#[doc = "Field `PAOD15` reader - PAOD15"]
pub type PAOD15_R = crate::BitReader;
#[doc = "Field `PAOD15` writer - PAOD15"]
pub type PAOD15_W<'a, const O: u8> = crate::BitWriter<'a, PAODR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PAOD0"]
    #[inline(always)]
    pub fn paod0(&self) -> PAOD0_R {
        PAOD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PAOD1"]
    #[inline(always)]
    pub fn paod1(&self) -> PAOD1_R {
        PAOD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PAOD2"]
    #[inline(always)]
    pub fn paod2(&self) -> PAOD2_R {
        PAOD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PAOD3"]
    #[inline(always)]
    pub fn paod3(&self) -> PAOD3_R {
        PAOD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PAOD4"]
    #[inline(always)]
    pub fn paod4(&self) -> PAOD4_R {
        PAOD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PAOD5"]
    #[inline(always)]
    pub fn paod5(&self) -> PAOD5_R {
        PAOD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PAOD6"]
    #[inline(always)]
    pub fn paod6(&self) -> PAOD6_R {
        PAOD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PAOD7"]
    #[inline(always)]
    pub fn paod7(&self) -> PAOD7_R {
        PAOD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PAOD8"]
    #[inline(always)]
    pub fn paod8(&self) -> PAOD8_R {
        PAOD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PAOD9"]
    #[inline(always)]
    pub fn paod9(&self) -> PAOD9_R {
        PAOD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PAOD10"]
    #[inline(always)]
    pub fn paod10(&self) -> PAOD10_R {
        PAOD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAOD11"]
    #[inline(always)]
    pub fn paod11(&self) -> PAOD11_R {
        PAOD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAOD12"]
    #[inline(always)]
    pub fn paod12(&self) -> PAOD12_R {
        PAOD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAOD13"]
    #[inline(always)]
    pub fn paod13(&self) -> PAOD13_R {
        PAOD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAOD14"]
    #[inline(always)]
    pub fn paod14(&self) -> PAOD14_R {
        PAOD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAOD15"]
    #[inline(always)]
    pub fn paod15(&self) -> PAOD15_R {
        PAOD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAOD0"]
    #[inline(always)]
    #[must_use]
    pub fn paod0(&mut self) -> PAOD0_W<0> {
        PAOD0_W::new(self)
    }
    #[doc = "Bit 1 - PAOD1"]
    #[inline(always)]
    #[must_use]
    pub fn paod1(&mut self) -> PAOD1_W<1> {
        PAOD1_W::new(self)
    }
    #[doc = "Bit 2 - PAOD2"]
    #[inline(always)]
    #[must_use]
    pub fn paod2(&mut self) -> PAOD2_W<2> {
        PAOD2_W::new(self)
    }
    #[doc = "Bit 3 - PAOD3"]
    #[inline(always)]
    #[must_use]
    pub fn paod3(&mut self) -> PAOD3_W<3> {
        PAOD3_W::new(self)
    }
    #[doc = "Bit 4 - PAOD4"]
    #[inline(always)]
    #[must_use]
    pub fn paod4(&mut self) -> PAOD4_W<4> {
        PAOD4_W::new(self)
    }
    #[doc = "Bit 5 - PAOD5"]
    #[inline(always)]
    #[must_use]
    pub fn paod5(&mut self) -> PAOD5_W<5> {
        PAOD5_W::new(self)
    }
    #[doc = "Bit 6 - PAOD6"]
    #[inline(always)]
    #[must_use]
    pub fn paod6(&mut self) -> PAOD6_W<6> {
        PAOD6_W::new(self)
    }
    #[doc = "Bit 7 - PAOD7"]
    #[inline(always)]
    #[must_use]
    pub fn paod7(&mut self) -> PAOD7_W<7> {
        PAOD7_W::new(self)
    }
    #[doc = "Bit 8 - PAOD8"]
    #[inline(always)]
    #[must_use]
    pub fn paod8(&mut self) -> PAOD8_W<8> {
        PAOD8_W::new(self)
    }
    #[doc = "Bit 9 - PAOD9"]
    #[inline(always)]
    #[must_use]
    pub fn paod9(&mut self) -> PAOD9_W<9> {
        PAOD9_W::new(self)
    }
    #[doc = "Bit 10 - PAOD10"]
    #[inline(always)]
    #[must_use]
    pub fn paod10(&mut self) -> PAOD10_W<10> {
        PAOD10_W::new(self)
    }
    #[doc = "Bit 11 - PAOD11"]
    #[inline(always)]
    #[must_use]
    pub fn paod11(&mut self) -> PAOD11_W<11> {
        PAOD11_W::new(self)
    }
    #[doc = "Bit 12 - PAOD12"]
    #[inline(always)]
    #[must_use]
    pub fn paod12(&mut self) -> PAOD12_W<12> {
        PAOD12_W::new(self)
    }
    #[doc = "Bit 13 - PAOD13"]
    #[inline(always)]
    #[must_use]
    pub fn paod13(&mut self) -> PAOD13_W<13> {
        PAOD13_W::new(self)
    }
    #[doc = "Bit 14 - PAOD14"]
    #[inline(always)]
    #[must_use]
    pub fn paod14(&mut self) -> PAOD14_W<14> {
        PAOD14_W::new(self)
    }
    #[doc = "Bit 15 - PAOD15"]
    #[inline(always)]
    #[must_use]
    pub fn paod15(&mut self) -> PAOD15_W<15> {
        PAOD15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paodr](index.html) module"]
pub struct PAODR_SPEC;
impl crate::RegisterSpec for PAODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [paodr::R](R) reader structure"]
impl crate::Readable for PAODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [paodr::W](W) writer structure"]
impl crate::Writable for PAODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAODR to value 0"]
impl crate::Resettable for PAODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
