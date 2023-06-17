#[doc = "Register `PBODR` reader"]
pub struct R(crate::R<PBODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBODR` writer"]
pub struct W(crate::W<PBODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBODR_SPEC>;
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
impl From<crate::W<PBODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBOD0` reader - PBOD0"]
pub type PBOD0_R = crate::BitReader;
#[doc = "Field `PBOD0` writer - PBOD0"]
pub type PBOD0_W<'a, const O: u8> = crate::BitWriter<'a, PBODR_SPEC, O>;
#[doc = "Field `PBOD1` reader - PBOD1"]
pub type PBOD1_R = crate::BitReader;
#[doc = "Field `PBOD1` writer - PBOD1"]
pub type PBOD1_W<'a, const O: u8> = crate::BitWriter<'a, PBODR_SPEC, O>;
#[doc = "Field `PBOD2` reader - PBOD2"]
pub type PBOD2_R = crate::BitReader;
#[doc = "Field `PBOD2` writer - PBOD2"]
pub type PBOD2_W<'a, const O: u8> = crate::BitWriter<'a, PBODR_SPEC, O>;
#[doc = "Field `PBOD3` reader - PBOD3"]
pub type PBOD3_R = crate::BitReader;
#[doc = "Field `PBOD3` writer - PBOD3"]
pub type PBOD3_W<'a, const O: u8> = crate::BitWriter<'a, PBODR_SPEC, O>;
#[doc = "Field `PBOD4` reader - PBOD4"]
pub type PBOD4_R = crate::BitReader;
#[doc = "Field `PBOD4` writer - PBOD4"]
pub type PBOD4_W<'a, const O: u8> = crate::BitWriter<'a, PBODR_SPEC, O>;
#[doc = "Field `PBOD5` reader - PBOD5"]
pub type PBOD5_R = crate::BitReader;
#[doc = "Field `PBOD5` writer - PBOD5"]
pub type PBOD5_W<'a, const O: u8> = crate::BitWriter<'a, PBODR_SPEC, O>;
#[doc = "Field `PBOD6` reader - PBOD6"]
pub type PBOD6_R = crate::BitReader;
#[doc = "Field `PBOD6` writer - PBOD6"]
pub type PBOD6_W<'a, const O: u8> = crate::BitWriter<'a, PBODR_SPEC, O>;
#[doc = "Field `PBOD7` reader - PBOD7"]
pub type PBOD7_R = crate::BitReader;
#[doc = "Field `PBOD7` writer - PBOD7"]
pub type PBOD7_W<'a, const O: u8> = crate::BitWriter<'a, PBODR_SPEC, O>;
#[doc = "Field `PBOD8` reader - PBOD8"]
pub type PBOD8_R = crate::BitReader;
#[doc = "Field `PBOD8` writer - PBOD8"]
pub type PBOD8_W<'a, const O: u8> = crate::BitWriter<'a, PBODR_SPEC, O>;
#[doc = "Field `PBOD9` reader - PBOD9"]
pub type PBOD9_R = crate::BitReader;
#[doc = "Field `PBOD9` writer - PBOD9"]
pub type PBOD9_W<'a, const O: u8> = crate::BitWriter<'a, PBODR_SPEC, O>;
#[doc = "Field `PBOD10` reader - PBOD10"]
pub type PBOD10_R = crate::BitReader;
#[doc = "Field `PBOD10` writer - PBOD10"]
pub type PBOD10_W<'a, const O: u8> = crate::BitWriter<'a, PBODR_SPEC, O>;
#[doc = "Field `PBOD11` reader - PBOD11"]
pub type PBOD11_R = crate::BitReader;
#[doc = "Field `PBOD11` writer - PBOD11"]
pub type PBOD11_W<'a, const O: u8> = crate::BitWriter<'a, PBODR_SPEC, O>;
#[doc = "Field `PBOD12` reader - PBOD12"]
pub type PBOD12_R = crate::BitReader;
#[doc = "Field `PBOD12` writer - PBOD12"]
pub type PBOD12_W<'a, const O: u8> = crate::BitWriter<'a, PBODR_SPEC, O>;
#[doc = "Field `PBOD13` reader - PBOD13"]
pub type PBOD13_R = crate::BitReader;
#[doc = "Field `PBOD13` writer - PBOD13"]
pub type PBOD13_W<'a, const O: u8> = crate::BitWriter<'a, PBODR_SPEC, O>;
#[doc = "Field `PBOD14` reader - PBOD14"]
pub type PBOD14_R = crate::BitReader;
#[doc = "Field `PBOD14` writer - PBOD14"]
pub type PBOD14_W<'a, const O: u8> = crate::BitWriter<'a, PBODR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PBOD0"]
    #[inline(always)]
    pub fn pbod0(&self) -> PBOD0_R {
        PBOD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PBOD1"]
    #[inline(always)]
    pub fn pbod1(&self) -> PBOD1_R {
        PBOD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBOD2"]
    #[inline(always)]
    pub fn pbod2(&self) -> PBOD2_R {
        PBOD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PBOD3"]
    #[inline(always)]
    pub fn pbod3(&self) -> PBOD3_R {
        PBOD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PBOD4"]
    #[inline(always)]
    pub fn pbod4(&self) -> PBOD4_R {
        PBOD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PBOD5"]
    #[inline(always)]
    pub fn pbod5(&self) -> PBOD5_R {
        PBOD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PBOD6"]
    #[inline(always)]
    pub fn pbod6(&self) -> PBOD6_R {
        PBOD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PBOD7"]
    #[inline(always)]
    pub fn pbod7(&self) -> PBOD7_R {
        PBOD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PBOD8"]
    #[inline(always)]
    pub fn pbod8(&self) -> PBOD8_R {
        PBOD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PBOD9"]
    #[inline(always)]
    pub fn pbod9(&self) -> PBOD9_R {
        PBOD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PBOD10"]
    #[inline(always)]
    pub fn pbod10(&self) -> PBOD10_R {
        PBOD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PBOD11"]
    #[inline(always)]
    pub fn pbod11(&self) -> PBOD11_R {
        PBOD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PBOD12"]
    #[inline(always)]
    pub fn pbod12(&self) -> PBOD12_R {
        PBOD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PBOD13"]
    #[inline(always)]
    pub fn pbod13(&self) -> PBOD13_R {
        PBOD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PBOD14"]
    #[inline(always)]
    pub fn pbod14(&self) -> PBOD14_R {
        PBOD14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PBOD0"]
    #[inline(always)]
    #[must_use]
    pub fn pbod0(&mut self) -> PBOD0_W<0> {
        PBOD0_W::new(self)
    }
    #[doc = "Bit 1 - PBOD1"]
    #[inline(always)]
    #[must_use]
    pub fn pbod1(&mut self) -> PBOD1_W<1> {
        PBOD1_W::new(self)
    }
    #[doc = "Bit 2 - PBOD2"]
    #[inline(always)]
    #[must_use]
    pub fn pbod2(&mut self) -> PBOD2_W<2> {
        PBOD2_W::new(self)
    }
    #[doc = "Bit 3 - PBOD3"]
    #[inline(always)]
    #[must_use]
    pub fn pbod3(&mut self) -> PBOD3_W<3> {
        PBOD3_W::new(self)
    }
    #[doc = "Bit 4 - PBOD4"]
    #[inline(always)]
    #[must_use]
    pub fn pbod4(&mut self) -> PBOD4_W<4> {
        PBOD4_W::new(self)
    }
    #[doc = "Bit 5 - PBOD5"]
    #[inline(always)]
    #[must_use]
    pub fn pbod5(&mut self) -> PBOD5_W<5> {
        PBOD5_W::new(self)
    }
    #[doc = "Bit 6 - PBOD6"]
    #[inline(always)]
    #[must_use]
    pub fn pbod6(&mut self) -> PBOD6_W<6> {
        PBOD6_W::new(self)
    }
    #[doc = "Bit 7 - PBOD7"]
    #[inline(always)]
    #[must_use]
    pub fn pbod7(&mut self) -> PBOD7_W<7> {
        PBOD7_W::new(self)
    }
    #[doc = "Bit 8 - PBOD8"]
    #[inline(always)]
    #[must_use]
    pub fn pbod8(&mut self) -> PBOD8_W<8> {
        PBOD8_W::new(self)
    }
    #[doc = "Bit 9 - PBOD9"]
    #[inline(always)]
    #[must_use]
    pub fn pbod9(&mut self) -> PBOD9_W<9> {
        PBOD9_W::new(self)
    }
    #[doc = "Bit 10 - PBOD10"]
    #[inline(always)]
    #[must_use]
    pub fn pbod10(&mut self) -> PBOD10_W<10> {
        PBOD10_W::new(self)
    }
    #[doc = "Bit 11 - PBOD11"]
    #[inline(always)]
    #[must_use]
    pub fn pbod11(&mut self) -> PBOD11_W<11> {
        PBOD11_W::new(self)
    }
    #[doc = "Bit 12 - PBOD12"]
    #[inline(always)]
    #[must_use]
    pub fn pbod12(&mut self) -> PBOD12_W<12> {
        PBOD12_W::new(self)
    }
    #[doc = "Bit 13 - PBOD13"]
    #[inline(always)]
    #[must_use]
    pub fn pbod13(&mut self) -> PBOD13_W<13> {
        PBOD13_W::new(self)
    }
    #[doc = "Bit 14 - PBOD14"]
    #[inline(always)]
    #[must_use]
    pub fn pbod14(&mut self) -> PBOD14_W<14> {
        PBOD14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbodr](index.html) module"]
pub struct PBODR_SPEC;
impl crate::RegisterSpec for PBODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbodr::R](R) reader structure"]
impl crate::Readable for PBODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbodr::W](W) writer structure"]
impl crate::Writable for PBODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBODR to value 0"]
impl crate::Resettable for PBODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
