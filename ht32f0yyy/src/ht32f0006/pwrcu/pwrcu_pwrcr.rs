#[doc = "Register `PWRCU_PWRCR` reader"]
pub struct R(crate::R<PWRCU_PWRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCU_PWRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCU_PWRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCU_PWRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCU_PWRCR` writer"]
pub struct W(crate::W<PWRCU_PWRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCU_PWRCR_SPEC>;
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
impl From<crate::W<PWRCU_PWRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCU_PWRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRST` reader - PWRST"]
pub type PWRST_R = crate::BitReader;
#[doc = "Field `PWRST` writer - PWRST"]
pub type PWRST_W<'a, const O: u8> = crate::BitWriter<'a, PWRCU_PWRCR_SPEC, O>;
#[doc = "Field `LDOLCM` reader - LDOLCM"]
pub type LDOLCM_R = crate::BitReader;
#[doc = "Field `LDOLCM` writer - LDOLCM"]
pub type LDOLCM_W<'a, const O: u8> = crate::BitWriter<'a, PWRCU_PWRCR_SPEC, O>;
#[doc = "Field `LDOOFF` reader - LDOOFF"]
pub type LDOOFF_R = crate::BitReader;
#[doc = "Field `LDOOFF` writer - LDOOFF"]
pub type LDOOFF_W<'a, const O: u8> = crate::BitWriter<'a, PWRCU_PWRCR_SPEC, O>;
#[doc = "Field `DMOSON` reader - DMOSON"]
pub type DMOSON_R = crate::BitReader;
#[doc = "Field `DMOSON` writer - DMOSON"]
pub type DMOSON_W<'a, const O: u8> = crate::BitWriter<'a, PWRCU_PWRCR_SPEC, O>;
#[doc = "Field `WUPEN` reader - WUPEN"]
pub type WUPEN_R = crate::BitReader;
#[doc = "Field `WUPEN` writer - WUPEN"]
pub type WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, PWRCU_PWRCR_SPEC, O>;
#[doc = "Field `WUPIEN` reader - WUPIEN"]
pub type WUPIEN_R = crate::BitReader;
#[doc = "Field `WUPIEN` writer - WUPIEN"]
pub type WUPIEN_W<'a, const O: u8> = crate::BitWriter<'a, PWRCU_PWRCR_SPEC, O>;
#[doc = "Field `V15RDYSC` reader - V15RDYSC"]
pub type V15RDYSC_R = crate::BitReader;
#[doc = "Field `V15RDYSC` writer - V15RDYSC"]
pub type V15RDYSC_W<'a, const O: u8> = crate::BitWriter<'a, PWRCU_PWRCR_SPEC, O>;
#[doc = "Field `DMOSSTS` reader - DMOSSTS"]
pub type DMOSSTS_R = crate::BitReader;
#[doc = "Field `DMOSSTS` writer - DMOSSTS"]
pub type DMOSSTS_W<'a, const O: u8> = crate::BitWriter<'a, PWRCU_PWRCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PWRST"]
    #[inline(always)]
    pub fn pwrst(&self) -> PWRST_R {
        PWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - LDOLCM"]
    #[inline(always)]
    pub fn ldolcm(&self) -> LDOLCM_R {
        LDOLCM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LDOOFF"]
    #[inline(always)]
    pub fn ldooff(&self) -> LDOOFF_R {
        LDOOFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - DMOSON"]
    #[inline(always)]
    pub fn dmoson(&self) -> DMOSON_R {
        DMOSON_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WUPEN"]
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WUPIEN"]
    #[inline(always)]
    pub fn wupien(&self) -> WUPIEN_R {
        WUPIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - V15RDYSC"]
    #[inline(always)]
    pub fn v15rdysc(&self) -> V15RDYSC_R {
        V15RDYSC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - DMOSSTS"]
    #[inline(always)]
    pub fn dmossts(&self) -> DMOSSTS_R {
        DMOSSTS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWRST"]
    #[inline(always)]
    #[must_use]
    pub fn pwrst(&mut self) -> PWRST_W<0> {
        PWRST_W::new(self)
    }
    #[doc = "Bit 2 - LDOLCM"]
    #[inline(always)]
    #[must_use]
    pub fn ldolcm(&mut self) -> LDOLCM_W<2> {
        LDOLCM_W::new(self)
    }
    #[doc = "Bit 3 - LDOOFF"]
    #[inline(always)]
    #[must_use]
    pub fn ldooff(&mut self) -> LDOOFF_W<3> {
        LDOOFF_W::new(self)
    }
    #[doc = "Bit 7 - DMOSON"]
    #[inline(always)]
    #[must_use]
    pub fn dmoson(&mut self) -> DMOSON_W<7> {
        DMOSON_W::new(self)
    }
    #[doc = "Bit 8 - WUPEN"]
    #[inline(always)]
    #[must_use]
    pub fn wupen(&mut self) -> WUPEN_W<8> {
        WUPEN_W::new(self)
    }
    #[doc = "Bit 9 - WUPIEN"]
    #[inline(always)]
    #[must_use]
    pub fn wupien(&mut self) -> WUPIEN_W<9> {
        WUPIEN_W::new(self)
    }
    #[doc = "Bit 12 - V15RDYSC"]
    #[inline(always)]
    #[must_use]
    pub fn v15rdysc(&mut self) -> V15RDYSC_W<12> {
        V15RDYSC_W::new(self)
    }
    #[doc = "Bit 15 - DMOSSTS"]
    #[inline(always)]
    #[must_use]
    pub fn dmossts(&mut self) -> DMOSSTS_W<15> {
        DMOSSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWRCU_PWRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_pwrcr](index.html) module"]
pub struct PWRCU_PWRCR_SPEC;
impl crate::RegisterSpec for PWRCU_PWRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrcu_pwrcr::R](R) reader structure"]
impl crate::Readable for PWRCU_PWRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrcu_pwrcr::W](W) writer structure"]
impl crate::Writable for PWRCU_PWRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCU_PWRCR to value 0"]
impl crate::Resettable for PWRCU_PWRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
