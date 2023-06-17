#[doc = "Register `PWRCR` reader"]
pub struct R(crate::R<PWRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCR` writer"]
pub struct W(crate::W<PWRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCR_SPEC>;
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
impl From<crate::W<PWRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWCURST` reader - PWCURST"]
pub type PWCURST_R = crate::BitReader;
#[doc = "Field `PWCURST` writer - PWCURST"]
pub type PWCURST_W<'a, const O: u8> = crate::BitWriter<'a, PWRCR_SPEC, O>;
#[doc = "Field `LDOLCM` reader - LDOLCM"]
pub type LDOLCM_R = crate::BitReader;
#[doc = "Field `LDOLCM` writer - LDOLCM"]
pub type LDOLCM_W<'a, const O: u8> = crate::BitWriter<'a, PWRCR_SPEC, O>;
#[doc = "Field `LDOOFF` reader - LDOOFF"]
pub type LDOOFF_R = crate::BitReader;
#[doc = "Field `LDOOFF` writer - LDOOFF"]
pub type LDOOFF_W<'a, const O: u8> = crate::BitWriter<'a, PWRCR_SPEC, O>;
#[doc = "Field `DMOSON` reader - DMOSON"]
pub type DMOSON_R = crate::BitReader;
#[doc = "Field `DMOSON` writer - DMOSON"]
pub type DMOSON_W<'a, const O: u8> = crate::BitWriter<'a, PWRCR_SPEC, O>;
#[doc = "Field `WUP0EN` reader - WUP0EN"]
pub type WUP0EN_R = crate::BitReader;
#[doc = "Field `WUP0EN` writer - WUP0EN"]
pub type WUP0EN_W<'a, const O: u8> = crate::BitWriter<'a, PWRCR_SPEC, O>;
#[doc = "Field `WUP1EN` reader - WUP1EN"]
pub type WUP1EN_R = crate::BitReader;
#[doc = "Field `WUP1EN` writer - WUP1EN"]
pub type WUP1EN_W<'a, const O: u8> = crate::BitWriter<'a, PWRCR_SPEC, O>;
#[doc = "Field `DMOSSTS` reader - DMOSSTS"]
pub type DMOSSTS_R = crate::BitReader;
#[doc = "Field `DMOSSTS` writer - DMOSSTS"]
pub type DMOSSTS_W<'a, const O: u8> = crate::BitWriter<'a, PWRCR_SPEC, O>;
#[doc = "Field `WUP0TYPE` reader - WUP0TYPE"]
pub type WUP0TYPE_R = crate::FieldReader;
#[doc = "Field `WUP0TYPE` writer - WUP0TYPE"]
pub type WUP0TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, PWRCR_SPEC, 2, O>;
#[doc = "Field `WUP1TYPE` reader - WUP1TYPE"]
pub type WUP1TYPE_R = crate::FieldReader;
#[doc = "Field `WUP1TYPE` writer - WUP1TYPE"]
pub type WUP1TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, PWRCR_SPEC, 2, O>;
#[doc = "Field `VREGMS` reader - VREGMS"]
pub type VREGMS_R = crate::FieldReader;
#[doc = "Field `VREGMS` writer - VREGMS"]
pub type VREGMS_W<'a, const O: u8> = crate::FieldWriter<'a, PWRCR_SPEC, 2, O>;
#[doc = "Field `VREGVS` reader - VREGVS"]
pub type VREGVS_R = crate::FieldReader;
#[doc = "Field `VREGVS` writer - VREGVS"]
pub type VREGVS_W<'a, const O: u8> = crate::FieldWriter<'a, PWRCR_SPEC, 2, O>;
#[doc = "Field `VREGOS` reader - VREGOS"]
pub type VREGOS_R = crate::BitReader;
#[doc = "Field `VREGOS` writer - VREGOS"]
pub type VREGOS_W<'a, const O: u8> = crate::BitWriter<'a, PWRCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PWCURST"]
    #[inline(always)]
    pub fn pwcurst(&self) -> PWCURST_R {
        PWCURST_R::new((self.bits & 1) != 0)
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
    #[doc = "Bit 8 - WUP0EN"]
    #[inline(always)]
    pub fn wup0en(&self) -> WUP0EN_R {
        WUP0EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - WUP1EN"]
    #[inline(always)]
    pub fn wup1en(&self) -> WUP1EN_R {
        WUP1EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - DMOSSTS"]
    #[inline(always)]
    pub fn dmossts(&self) -> DMOSSTS_R {
        DMOSSTS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - WUP0TYPE"]
    #[inline(always)]
    pub fn wup0type(&self) -> WUP0TYPE_R {
        WUP0TYPE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - WUP1TYPE"]
    #[inline(always)]
    pub fn wup1type(&self) -> WUP1TYPE_R {
        WUP1TYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 24:25 - VREGMS"]
    #[inline(always)]
    pub fn vregms(&self) -> VREGMS_R {
        VREGMS_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - VREGVS"]
    #[inline(always)]
    pub fn vregvs(&self) -> VREGVS_R {
        VREGVS_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - VREGOS"]
    #[inline(always)]
    pub fn vregos(&self) -> VREGOS_R {
        VREGOS_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWCURST"]
    #[inline(always)]
    #[must_use]
    pub fn pwcurst(&mut self) -> PWCURST_W<0> {
        PWCURST_W::new(self)
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
    #[doc = "Bit 8 - WUP0EN"]
    #[inline(always)]
    #[must_use]
    pub fn wup0en(&mut self) -> WUP0EN_W<8> {
        WUP0EN_W::new(self)
    }
    #[doc = "Bit 10 - WUP1EN"]
    #[inline(always)]
    #[must_use]
    pub fn wup1en(&mut self) -> WUP1EN_W<10> {
        WUP1EN_W::new(self)
    }
    #[doc = "Bit 15 - DMOSSTS"]
    #[inline(always)]
    #[must_use]
    pub fn dmossts(&mut self) -> DMOSSTS_W<15> {
        DMOSSTS_W::new(self)
    }
    #[doc = "Bits 16:17 - WUP0TYPE"]
    #[inline(always)]
    #[must_use]
    pub fn wup0type(&mut self) -> WUP0TYPE_W<16> {
        WUP0TYPE_W::new(self)
    }
    #[doc = "Bits 18:19 - WUP1TYPE"]
    #[inline(always)]
    #[must_use]
    pub fn wup1type(&mut self) -> WUP1TYPE_W<18> {
        WUP1TYPE_W::new(self)
    }
    #[doc = "Bits 24:25 - VREGMS"]
    #[inline(always)]
    #[must_use]
    pub fn vregms(&mut self) -> VREGMS_W<24> {
        VREGMS_W::new(self)
    }
    #[doc = "Bits 26:27 - VREGVS"]
    #[inline(always)]
    #[must_use]
    pub fn vregvs(&mut self) -> VREGVS_W<26> {
        VREGVS_W::new(self)
    }
    #[doc = "Bit 28 - VREGOS"]
    #[inline(always)]
    #[must_use]
    pub fn vregos(&mut self) -> VREGOS_W<28> {
        VREGOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcr](index.html) module"]
pub struct PWRCR_SPEC;
impl crate::RegisterSpec for PWRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrcr::R](R) reader structure"]
impl crate::Readable for PWRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrcr::W](W) writer structure"]
impl crate::Writable for PWRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCR to value 0"]
impl crate::Resettable for PWRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
