#[doc = "Register `CKCU_AHBCCR` reader"]
pub struct R(crate::R<CKCU_AHBCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_AHBCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_AHBCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_AHBCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_AHBCCR` writer"]
pub struct W(crate::W<CKCU_AHBCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_AHBCCR_SPEC>;
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
impl From<crate::W<CKCU_AHBCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_AHBCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMCEN` reader - FMCEN"]
pub type FMCEN_R = crate::BitReader;
#[doc = "Field `FMCEN` writer - FMCEN"]
pub type FMCEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_AHBCCR_SPEC, O>;
#[doc = "Field `SRAMEN` reader - SRAMEN"]
pub type SRAMEN_R = crate::BitReader;
#[doc = "Field `SRAMEN` writer - SRAMEN"]
pub type SRAMEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_AHBCCR_SPEC, O>;
#[doc = "Field `PDMAEN` reader - PDMAEN"]
pub type PDMAEN_R = crate::BitReader;
#[doc = "Field `PDMAEN` writer - PDMAEN"]
pub type PDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_AHBCCR_SPEC, O>;
#[doc = "Field `BMEN` reader - BMEN"]
pub type BMEN_R = crate::BitReader;
#[doc = "Field `BMEN` writer - BMEN"]
pub type BMEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_AHBCCR_SPEC, O>;
#[doc = "Field `APB0EN` reader - APB0EN"]
pub type APB0EN_R = crate::BitReader;
#[doc = "Field `APB0EN` writer - APB0EN"]
pub type APB0EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_AHBCCR_SPEC, O>;
#[doc = "Field `APB1EN` reader - APB1EN"]
pub type APB1EN_R = crate::BitReader;
#[doc = "Field `APB1EN` writer - APB1EN"]
pub type APB1EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_AHBCCR_SPEC, O>;
#[doc = "Field `CSIFEN` reader - CSIFEN"]
pub type CSIFEN_R = crate::BitReader;
#[doc = "Field `CSIFEN` writer - CSIFEN"]
pub type CSIFEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_AHBCCR_SPEC, O>;
#[doc = "Field `CSIFMEN` reader - CSIFMEN"]
pub type CSIFMEN_R = crate::BitReader;
#[doc = "Field `CSIFMEN` writer - CSIFMEN"]
pub type CSIFMEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_AHBCCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - FMCEN"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SRAMEN"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - PDMAEN"]
    #[inline(always)]
    pub fn pdmaen(&self) -> PDMAEN_R {
        PDMAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BMEN"]
    #[inline(always)]
    pub fn bmen(&self) -> BMEN_R {
        BMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - APB0EN"]
    #[inline(always)]
    pub fn apb0en(&self) -> APB0EN_R {
        APB0EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - APB1EN"]
    #[inline(always)]
    pub fn apb1en(&self) -> APB1EN_R {
        APB1EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CSIFEN"]
    #[inline(always)]
    pub fn csifen(&self) -> CSIFEN_R {
        CSIFEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CSIFMEN"]
    #[inline(always)]
    pub fn csifmen(&self) -> CSIFMEN_R {
        CSIFMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FMCEN"]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<0> {
        FMCEN_W::new(self)
    }
    #[doc = "Bit 2 - SRAMEN"]
    #[inline(always)]
    #[must_use]
    pub fn sramen(&mut self) -> SRAMEN_W<2> {
        SRAMEN_W::new(self)
    }
    #[doc = "Bit 4 - PDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn pdmaen(&mut self) -> PDMAEN_W<4> {
        PDMAEN_W::new(self)
    }
    #[doc = "Bit 5 - BMEN"]
    #[inline(always)]
    #[must_use]
    pub fn bmen(&mut self) -> BMEN_W<5> {
        BMEN_W::new(self)
    }
    #[doc = "Bit 6 - APB0EN"]
    #[inline(always)]
    #[must_use]
    pub fn apb0en(&mut self) -> APB0EN_W<6> {
        APB0EN_W::new(self)
    }
    #[doc = "Bit 7 - APB1EN"]
    #[inline(always)]
    #[must_use]
    pub fn apb1en(&mut self) -> APB1EN_W<7> {
        APB1EN_W::new(self)
    }
    #[doc = "Bit 8 - CSIFEN"]
    #[inline(always)]
    #[must_use]
    pub fn csifen(&mut self) -> CSIFEN_W<8> {
        CSIFEN_W::new(self)
    }
    #[doc = "Bit 9 - CSIFMEN"]
    #[inline(always)]
    #[must_use]
    pub fn csifmen(&mut self) -> CSIFMEN_W<9> {
        CSIFMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_AHBCCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_ahbccr](index.html) module"]
pub struct CKCU_AHBCCR_SPEC;
impl crate::RegisterSpec for CKCU_AHBCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_ahbccr::R](R) reader structure"]
impl crate::Readable for CKCU_AHBCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_ahbccr::W](W) writer structure"]
impl crate::Writable for CKCU_AHBCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_AHBCCR to value 0"]
impl crate::Resettable for CKCU_AHBCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
