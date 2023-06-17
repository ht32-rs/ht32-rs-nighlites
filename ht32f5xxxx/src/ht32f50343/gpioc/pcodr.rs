#[doc = "Register `PCODR` reader"]
pub struct R(crate::R<PCODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCODR` writer"]
pub struct W(crate::W<PCODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCODR_SPEC>;
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
impl From<crate::W<PCODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCOD0` reader - PCOD0"]
pub type PCOD0_R = crate::BitReader;
#[doc = "Field `PCOD0` writer - PCOD0"]
pub type PCOD0_W<'a, const O: u8> = crate::BitWriter<'a, PCODR_SPEC, O>;
#[doc = "Field `PCOD1` reader - PCOD1"]
pub type PCOD1_R = crate::BitReader;
#[doc = "Field `PCOD1` writer - PCOD1"]
pub type PCOD1_W<'a, const O: u8> = crate::BitWriter<'a, PCODR_SPEC, O>;
#[doc = "Field `PCOD2` reader - PCOD2"]
pub type PCOD2_R = crate::BitReader;
#[doc = "Field `PCOD2` writer - PCOD2"]
pub type PCOD2_W<'a, const O: u8> = crate::BitWriter<'a, PCODR_SPEC, O>;
#[doc = "Field `PCOD3` reader - PCOD3"]
pub type PCOD3_R = crate::BitReader;
#[doc = "Field `PCOD3` writer - PCOD3"]
pub type PCOD3_W<'a, const O: u8> = crate::BitWriter<'a, PCODR_SPEC, O>;
#[doc = "Field `PCOD4` reader - PCOD4"]
pub type PCOD4_R = crate::BitReader;
#[doc = "Field `PCOD4` writer - PCOD4"]
pub type PCOD4_W<'a, const O: u8> = crate::BitWriter<'a, PCODR_SPEC, O>;
#[doc = "Field `PCOD5` reader - PCOD5"]
pub type PCOD5_R = crate::BitReader;
#[doc = "Field `PCOD5` writer - PCOD5"]
pub type PCOD5_W<'a, const O: u8> = crate::BitWriter<'a, PCODR_SPEC, O>;
#[doc = "Field `PCOD6` reader - PCOD6"]
pub type PCOD6_R = crate::BitReader;
#[doc = "Field `PCOD6` writer - PCOD6"]
pub type PCOD6_W<'a, const O: u8> = crate::BitWriter<'a, PCODR_SPEC, O>;
#[doc = "Field `PCOD7` reader - PCOD7"]
pub type PCOD7_R = crate::BitReader;
#[doc = "Field `PCOD7` writer - PCOD7"]
pub type PCOD7_W<'a, const O: u8> = crate::BitWriter<'a, PCODR_SPEC, O>;
#[doc = "Field `PCOD8` reader - PCOD8"]
pub type PCOD8_R = crate::BitReader;
#[doc = "Field `PCOD8` writer - PCOD8"]
pub type PCOD8_W<'a, const O: u8> = crate::BitWriter<'a, PCODR_SPEC, O>;
#[doc = "Field `PCOD9` reader - PCOD9"]
pub type PCOD9_R = crate::BitReader;
#[doc = "Field `PCOD9` writer - PCOD9"]
pub type PCOD9_W<'a, const O: u8> = crate::BitWriter<'a, PCODR_SPEC, O>;
#[doc = "Field `PCOD10` reader - PCOD10"]
pub type PCOD10_R = crate::BitReader;
#[doc = "Field `PCOD10` writer - PCOD10"]
pub type PCOD10_W<'a, const O: u8> = crate::BitWriter<'a, PCODR_SPEC, O>;
#[doc = "Field `PCOD11` reader - PCOD11"]
pub type PCOD11_R = crate::BitReader;
#[doc = "Field `PCOD11` writer - PCOD11"]
pub type PCOD11_W<'a, const O: u8> = crate::BitWriter<'a, PCODR_SPEC, O>;
#[doc = "Field `PCOD12` reader - PCOD12"]
pub type PCOD12_R = crate::BitReader;
#[doc = "Field `PCOD12` writer - PCOD12"]
pub type PCOD12_W<'a, const O: u8> = crate::BitWriter<'a, PCODR_SPEC, O>;
#[doc = "Field `PCOD13` reader - PCOD13"]
pub type PCOD13_R = crate::BitReader;
#[doc = "Field `PCOD13` writer - PCOD13"]
pub type PCOD13_W<'a, const O: u8> = crate::BitWriter<'a, PCODR_SPEC, O>;
#[doc = "Field `PCOD14` reader - PCOD14"]
pub type PCOD14_R = crate::BitReader;
#[doc = "Field `PCOD14` writer - PCOD14"]
pub type PCOD14_W<'a, const O: u8> = crate::BitWriter<'a, PCODR_SPEC, O>;
#[doc = "Field `PCOD15` reader - PCOD15"]
pub type PCOD15_R = crate::BitReader;
#[doc = "Field `PCOD15` writer - PCOD15"]
pub type PCOD15_W<'a, const O: u8> = crate::BitWriter<'a, PCODR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PCOD0"]
    #[inline(always)]
    pub fn pcod0(&self) -> PCOD0_R {
        PCOD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCOD1"]
    #[inline(always)]
    pub fn pcod1(&self) -> PCOD1_R {
        PCOD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCOD2"]
    #[inline(always)]
    pub fn pcod2(&self) -> PCOD2_R {
        PCOD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCOD3"]
    #[inline(always)]
    pub fn pcod3(&self) -> PCOD3_R {
        PCOD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PCOD4"]
    #[inline(always)]
    pub fn pcod4(&self) -> PCOD4_R {
        PCOD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCOD5"]
    #[inline(always)]
    pub fn pcod5(&self) -> PCOD5_R {
        PCOD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCOD6"]
    #[inline(always)]
    pub fn pcod6(&self) -> PCOD6_R {
        PCOD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PCOD7"]
    #[inline(always)]
    pub fn pcod7(&self) -> PCOD7_R {
        PCOD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PCOD8"]
    #[inline(always)]
    pub fn pcod8(&self) -> PCOD8_R {
        PCOD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PCOD9"]
    #[inline(always)]
    pub fn pcod9(&self) -> PCOD9_R {
        PCOD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCOD10"]
    #[inline(always)]
    pub fn pcod10(&self) -> PCOD10_R {
        PCOD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PCOD11"]
    #[inline(always)]
    pub fn pcod11(&self) -> PCOD11_R {
        PCOD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PCOD12"]
    #[inline(always)]
    pub fn pcod12(&self) -> PCOD12_R {
        PCOD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PCOD13"]
    #[inline(always)]
    pub fn pcod13(&self) -> PCOD13_R {
        PCOD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PCOD14"]
    #[inline(always)]
    pub fn pcod14(&self) -> PCOD14_R {
        PCOD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PCOD15"]
    #[inline(always)]
    pub fn pcod15(&self) -> PCOD15_R {
        PCOD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCOD0"]
    #[inline(always)]
    #[must_use]
    pub fn pcod0(&mut self) -> PCOD0_W<0> {
        PCOD0_W::new(self)
    }
    #[doc = "Bit 1 - PCOD1"]
    #[inline(always)]
    #[must_use]
    pub fn pcod1(&mut self) -> PCOD1_W<1> {
        PCOD1_W::new(self)
    }
    #[doc = "Bit 2 - PCOD2"]
    #[inline(always)]
    #[must_use]
    pub fn pcod2(&mut self) -> PCOD2_W<2> {
        PCOD2_W::new(self)
    }
    #[doc = "Bit 3 - PCOD3"]
    #[inline(always)]
    #[must_use]
    pub fn pcod3(&mut self) -> PCOD3_W<3> {
        PCOD3_W::new(self)
    }
    #[doc = "Bit 4 - PCOD4"]
    #[inline(always)]
    #[must_use]
    pub fn pcod4(&mut self) -> PCOD4_W<4> {
        PCOD4_W::new(self)
    }
    #[doc = "Bit 5 - PCOD5"]
    #[inline(always)]
    #[must_use]
    pub fn pcod5(&mut self) -> PCOD5_W<5> {
        PCOD5_W::new(self)
    }
    #[doc = "Bit 6 - PCOD6"]
    #[inline(always)]
    #[must_use]
    pub fn pcod6(&mut self) -> PCOD6_W<6> {
        PCOD6_W::new(self)
    }
    #[doc = "Bit 7 - PCOD7"]
    #[inline(always)]
    #[must_use]
    pub fn pcod7(&mut self) -> PCOD7_W<7> {
        PCOD7_W::new(self)
    }
    #[doc = "Bit 8 - PCOD8"]
    #[inline(always)]
    #[must_use]
    pub fn pcod8(&mut self) -> PCOD8_W<8> {
        PCOD8_W::new(self)
    }
    #[doc = "Bit 9 - PCOD9"]
    #[inline(always)]
    #[must_use]
    pub fn pcod9(&mut self) -> PCOD9_W<9> {
        PCOD9_W::new(self)
    }
    #[doc = "Bit 10 - PCOD10"]
    #[inline(always)]
    #[must_use]
    pub fn pcod10(&mut self) -> PCOD10_W<10> {
        PCOD10_W::new(self)
    }
    #[doc = "Bit 11 - PCOD11"]
    #[inline(always)]
    #[must_use]
    pub fn pcod11(&mut self) -> PCOD11_W<11> {
        PCOD11_W::new(self)
    }
    #[doc = "Bit 12 - PCOD12"]
    #[inline(always)]
    #[must_use]
    pub fn pcod12(&mut self) -> PCOD12_W<12> {
        PCOD12_W::new(self)
    }
    #[doc = "Bit 13 - PCOD13"]
    #[inline(always)]
    #[must_use]
    pub fn pcod13(&mut self) -> PCOD13_W<13> {
        PCOD13_W::new(self)
    }
    #[doc = "Bit 14 - PCOD14"]
    #[inline(always)]
    #[must_use]
    pub fn pcod14(&mut self) -> PCOD14_W<14> {
        PCOD14_W::new(self)
    }
    #[doc = "Bit 15 - PCOD15"]
    #[inline(always)]
    #[must_use]
    pub fn pcod15(&mut self) -> PCOD15_W<15> {
        PCOD15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcodr](index.html) module"]
pub struct PCODR_SPEC;
impl crate::RegisterSpec for PCODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcodr::R](R) reader structure"]
impl crate::Readable for PCODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcodr::W](W) writer structure"]
impl crate::Writable for PCODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCODR to value 0"]
impl crate::Resettable for PCODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
