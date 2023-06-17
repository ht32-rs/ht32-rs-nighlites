#[doc = "Register `BAKCR` reader"]
pub struct R(crate::R<BAKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAKCR` writer"]
pub struct W(crate::W<BAKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAKCR_SPEC>;
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
impl From<crate::W<BAKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAKRST` reader - BAKRST"]
pub type BAKRST_R = crate::BitReader;
#[doc = "Field `BAKRST` writer - BAKRST"]
pub type BAKRST_W<'a, const O: u8> = crate::BitWriter<'a, BAKCR_SPEC, O>;
#[doc = "Field `LDOLCM` reader - LDOLCM"]
pub type LDOLCM_R = crate::BitReader;
#[doc = "Field `LDOLCM` writer - LDOLCM"]
pub type LDOLCM_W<'a, const O: u8> = crate::BitWriter<'a, BAKCR_SPEC, O>;
#[doc = "Field `LDOOFF` reader - LDOOFF"]
pub type LDOOFF_R = crate::BitReader;
#[doc = "Field `LDOOFF` writer - LDOOFF"]
pub type LDOOFF_W<'a, const O: u8> = crate::BitWriter<'a, BAKCR_SPEC, O>;
#[doc = "Field `ULDOON` reader - ULDOON"]
pub type ULDOON_R = crate::BitReader;
#[doc = "Field `ULDOON` writer - ULDOON"]
pub type ULDOON_W<'a, const O: u8> = crate::BitWriter<'a, BAKCR_SPEC, O>;
#[doc = "Field `WUPEN` reader - WUPEN"]
pub type WUPEN_R = crate::BitReader;
#[doc = "Field `WUPEN` writer - WUPEN"]
pub type WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, BAKCR_SPEC, O>;
#[doc = "Field `ULDOSTS` reader - ULDOSTS"]
pub type ULDOSTS_R = crate::BitReader;
#[doc = "Field `ULDOSTS` writer - ULDOSTS"]
pub type ULDOSTS_W<'a, const O: u8> = crate::BitWriter<'a, BAKCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - BAKRST"]
    #[inline(always)]
    pub fn bakrst(&self) -> BAKRST_R {
        BAKRST_R::new((self.bits & 1) != 0)
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
    #[doc = "Bit 7 - ULDOON"]
    #[inline(always)]
    pub fn uldoon(&self) -> ULDOON_R {
        ULDOON_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WUPEN"]
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - ULDOSTS"]
    #[inline(always)]
    pub fn uldosts(&self) -> ULDOSTS_R {
        ULDOSTS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BAKRST"]
    #[inline(always)]
    #[must_use]
    pub fn bakrst(&mut self) -> BAKRST_W<0> {
        BAKRST_W::new(self)
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
    #[doc = "Bit 7 - ULDOON"]
    #[inline(always)]
    #[must_use]
    pub fn uldoon(&mut self) -> ULDOON_W<7> {
        ULDOON_W::new(self)
    }
    #[doc = "Bit 8 - WUPEN"]
    #[inline(always)]
    #[must_use]
    pub fn wupen(&mut self) -> WUPEN_W<8> {
        WUPEN_W::new(self)
    }
    #[doc = "Bit 15 - ULDOSTS"]
    #[inline(always)]
    #[must_use]
    pub fn uldosts(&mut self) -> ULDOSTS_W<15> {
        ULDOSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BAKCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bakcr](index.html) module"]
pub struct BAKCR_SPEC;
impl crate::RegisterSpec for BAKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bakcr::R](R) reader structure"]
impl crate::Readable for BAKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bakcr::W](W) writer structure"]
impl crate::Writable for BAKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAKCR to value 0"]
impl crate::Resettable for BAKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
