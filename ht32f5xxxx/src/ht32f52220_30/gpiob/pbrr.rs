#[doc = "Register `PBRR` reader"]
pub struct R(crate::R<PBRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBRR` writer"]
pub struct W(crate::W<PBRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBRR_SPEC>;
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
impl From<crate::W<PBRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBRST0` reader - PBRST0"]
pub type PBRST0_R = crate::BitReader;
#[doc = "Field `PBRST0` writer - PBRST0"]
pub type PBRST0_W<'a, const O: u8> = crate::BitWriter<'a, PBRR_SPEC, O>;
#[doc = "Field `PBRST1` reader - PBRST1"]
pub type PBRST1_R = crate::BitReader;
#[doc = "Field `PBRST1` writer - PBRST1"]
pub type PBRST1_W<'a, const O: u8> = crate::BitWriter<'a, PBRR_SPEC, O>;
#[doc = "Field `PBRST2` reader - PBRST2"]
pub type PBRST2_R = crate::BitReader;
#[doc = "Field `PBRST2` writer - PBRST2"]
pub type PBRST2_W<'a, const O: u8> = crate::BitWriter<'a, PBRR_SPEC, O>;
#[doc = "Field `PBRST3` reader - PBRST3"]
pub type PBRST3_R = crate::BitReader;
#[doc = "Field `PBRST3` writer - PBRST3"]
pub type PBRST3_W<'a, const O: u8> = crate::BitWriter<'a, PBRR_SPEC, O>;
#[doc = "Field `PBRST4` reader - PBRST4"]
pub type PBRST4_R = crate::BitReader;
#[doc = "Field `PBRST4` writer - PBRST4"]
pub type PBRST4_W<'a, const O: u8> = crate::BitWriter<'a, PBRR_SPEC, O>;
#[doc = "Field `PBRST5` reader - PBRST5"]
pub type PBRST5_R = crate::BitReader;
#[doc = "Field `PBRST5` writer - PBRST5"]
pub type PBRST5_W<'a, const O: u8> = crate::BitWriter<'a, PBRR_SPEC, O>;
#[doc = "Field `PBRST6` reader - PBRST6"]
pub type PBRST6_R = crate::BitReader;
#[doc = "Field `PBRST6` writer - PBRST6"]
pub type PBRST6_W<'a, const O: u8> = crate::BitWriter<'a, PBRR_SPEC, O>;
#[doc = "Field `PBRST7` reader - PBRST7"]
pub type PBRST7_R = crate::BitReader;
#[doc = "Field `PBRST7` writer - PBRST7"]
pub type PBRST7_W<'a, const O: u8> = crate::BitWriter<'a, PBRR_SPEC, O>;
#[doc = "Field `PBRST8` reader - PBRST8"]
pub type PBRST8_R = crate::BitReader;
#[doc = "Field `PBRST8` writer - PBRST8"]
pub type PBRST8_W<'a, const O: u8> = crate::BitWriter<'a, PBRR_SPEC, O>;
#[doc = "Field `PBRST9` reader - PBRST9"]
pub type PBRST9_R = crate::BitReader;
#[doc = "Field `PBRST9` writer - PBRST9"]
pub type PBRST9_W<'a, const O: u8> = crate::BitWriter<'a, PBRR_SPEC, O>;
#[doc = "Field `PBRST10` reader - PBRST10"]
pub type PBRST10_R = crate::BitReader;
#[doc = "Field `PBRST10` writer - PBRST10"]
pub type PBRST10_W<'a, const O: u8> = crate::BitWriter<'a, PBRR_SPEC, O>;
#[doc = "Field `PBRST11` reader - PBRST11"]
pub type PBRST11_R = crate::BitReader;
#[doc = "Field `PBRST11` writer - PBRST11"]
pub type PBRST11_W<'a, const O: u8> = crate::BitWriter<'a, PBRR_SPEC, O>;
#[doc = "Field `PBRST12` reader - PBRST12"]
pub type PBRST12_R = crate::BitReader;
#[doc = "Field `PBRST12` writer - PBRST12"]
pub type PBRST12_W<'a, const O: u8> = crate::BitWriter<'a, PBRR_SPEC, O>;
#[doc = "Field `PBRST13` reader - PBRST13"]
pub type PBRST13_R = crate::BitReader;
#[doc = "Field `PBRST13` writer - PBRST13"]
pub type PBRST13_W<'a, const O: u8> = crate::BitWriter<'a, PBRR_SPEC, O>;
#[doc = "Field `PBRST14` reader - PBRST14"]
pub type PBRST14_R = crate::BitReader;
#[doc = "Field `PBRST14` writer - PBRST14"]
pub type PBRST14_W<'a, const O: u8> = crate::BitWriter<'a, PBRR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PBRST0"]
    #[inline(always)]
    pub fn pbrst0(&self) -> PBRST0_R {
        PBRST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PBRST1"]
    #[inline(always)]
    pub fn pbrst1(&self) -> PBRST1_R {
        PBRST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBRST2"]
    #[inline(always)]
    pub fn pbrst2(&self) -> PBRST2_R {
        PBRST2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PBRST3"]
    #[inline(always)]
    pub fn pbrst3(&self) -> PBRST3_R {
        PBRST3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PBRST4"]
    #[inline(always)]
    pub fn pbrst4(&self) -> PBRST4_R {
        PBRST4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PBRST5"]
    #[inline(always)]
    pub fn pbrst5(&self) -> PBRST5_R {
        PBRST5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PBRST6"]
    #[inline(always)]
    pub fn pbrst6(&self) -> PBRST6_R {
        PBRST6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PBRST7"]
    #[inline(always)]
    pub fn pbrst7(&self) -> PBRST7_R {
        PBRST7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PBRST8"]
    #[inline(always)]
    pub fn pbrst8(&self) -> PBRST8_R {
        PBRST8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PBRST9"]
    #[inline(always)]
    pub fn pbrst9(&self) -> PBRST9_R {
        PBRST9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PBRST10"]
    #[inline(always)]
    pub fn pbrst10(&self) -> PBRST10_R {
        PBRST10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PBRST11"]
    #[inline(always)]
    pub fn pbrst11(&self) -> PBRST11_R {
        PBRST11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PBRST12"]
    #[inline(always)]
    pub fn pbrst12(&self) -> PBRST12_R {
        PBRST12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PBRST13"]
    #[inline(always)]
    pub fn pbrst13(&self) -> PBRST13_R {
        PBRST13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PBRST14"]
    #[inline(always)]
    pub fn pbrst14(&self) -> PBRST14_R {
        PBRST14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PBRST0"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst0(&mut self) -> PBRST0_W<0> {
        PBRST0_W::new(self)
    }
    #[doc = "Bit 1 - PBRST1"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst1(&mut self) -> PBRST1_W<1> {
        PBRST1_W::new(self)
    }
    #[doc = "Bit 2 - PBRST2"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst2(&mut self) -> PBRST2_W<2> {
        PBRST2_W::new(self)
    }
    #[doc = "Bit 3 - PBRST3"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst3(&mut self) -> PBRST3_W<3> {
        PBRST3_W::new(self)
    }
    #[doc = "Bit 4 - PBRST4"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst4(&mut self) -> PBRST4_W<4> {
        PBRST4_W::new(self)
    }
    #[doc = "Bit 5 - PBRST5"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst5(&mut self) -> PBRST5_W<5> {
        PBRST5_W::new(self)
    }
    #[doc = "Bit 6 - PBRST6"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst6(&mut self) -> PBRST6_W<6> {
        PBRST6_W::new(self)
    }
    #[doc = "Bit 7 - PBRST7"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst7(&mut self) -> PBRST7_W<7> {
        PBRST7_W::new(self)
    }
    #[doc = "Bit 8 - PBRST8"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst8(&mut self) -> PBRST8_W<8> {
        PBRST8_W::new(self)
    }
    #[doc = "Bit 9 - PBRST9"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst9(&mut self) -> PBRST9_W<9> {
        PBRST9_W::new(self)
    }
    #[doc = "Bit 10 - PBRST10"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst10(&mut self) -> PBRST10_W<10> {
        PBRST10_W::new(self)
    }
    #[doc = "Bit 11 - PBRST11"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst11(&mut self) -> PBRST11_W<11> {
        PBRST11_W::new(self)
    }
    #[doc = "Bit 12 - PBRST12"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst12(&mut self) -> PBRST12_W<12> {
        PBRST12_W::new(self)
    }
    #[doc = "Bit 13 - PBRST13"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst13(&mut self) -> PBRST13_W<13> {
        PBRST13_W::new(self)
    }
    #[doc = "Bit 14 - PBRST14"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst14(&mut self) -> PBRST14_W<14> {
        PBRST14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbrr](index.html) module"]
pub struct PBRR_SPEC;
impl crate::RegisterSpec for PBRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbrr::R](R) reader structure"]
impl crate::Readable for PBRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbrr::W](W) writer structure"]
impl crate::Writable for PBRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBRR to value 0"]
impl crate::Resettable for PBRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
