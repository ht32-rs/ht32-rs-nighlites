#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPBUFA` reader - EPBUFA"]
pub type EPBUFA_R = crate::FieldReader<u16>;
#[doc = "Field `EPBUFA` writer - EPBUFA"]
pub type EPBUFA_W<'a, const O: u8> = crate::FieldWriter<'a, CFGR_SPEC, 10, O, u16>;
#[doc = "Field `EPLEN` reader - EPLEN"]
pub type EPLEN_R = crate::FieldReader;
#[doc = "Field `EPLEN` writer - EPLEN"]
pub type EPLEN_W<'a, const O: u8> = crate::FieldWriter<'a, CFGR_SPEC, 7, O>;
#[doc = "Field `EPADR` reader - EPADR"]
pub type EPADR_R = crate::FieldReader;
#[doc = "Field `EPADR` writer - EPADR"]
pub type EPADR_W<'a, const O: u8> = crate::FieldWriter<'a, CFGR_SPEC, 4, O>;
#[doc = "Field `EPDIR` reader - EPDIR"]
pub type EPDIR_R = crate::BitReader;
#[doc = "Field `EPDIR` writer - EPDIR"]
pub type EPDIR_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O>;
#[doc = "Field `EPTYPE` reader - EPTYPE"]
pub type EPTYPE_R = crate::BitReader;
#[doc = "Field `EPTYPE` writer - EPTYPE"]
pub type EPTYPE_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O>;
#[doc = "Field `EPEN` reader - EPEN"]
pub type EPEN_R = crate::BitReader;
#[doc = "Field `EPEN` writer - EPEN"]
pub type EPEN_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O>;
impl R {
    #[doc = "Bits 0:9 - EPBUFA"]
    #[inline(always)]
    pub fn epbufa(&self) -> EPBUFA_R {
        EPBUFA_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:16 - EPLEN"]
    #[inline(always)]
    pub fn eplen(&self) -> EPLEN_R {
        EPLEN_R::new(((self.bits >> 10) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - EPADR"]
    #[inline(always)]
    pub fn epadr(&self) -> EPADR_R {
        EPADR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - EPDIR"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - EPTYPE"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - EPEN"]
    #[inline(always)]
    pub fn epen(&self) -> EPEN_R {
        EPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - EPBUFA"]
    #[inline(always)]
    #[must_use]
    pub fn epbufa(&mut self) -> EPBUFA_W<0> {
        EPBUFA_W::new(self)
    }
    #[doc = "Bits 10:16 - EPLEN"]
    #[inline(always)]
    #[must_use]
    pub fn eplen(&mut self) -> EPLEN_W<10> {
        EPLEN_W::new(self)
    }
    #[doc = "Bits 24:27 - EPADR"]
    #[inline(always)]
    #[must_use]
    pub fn epadr(&mut self) -> EPADR_W<24> {
        EPADR_W::new(self)
    }
    #[doc = "Bit 28 - EPDIR"]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EPDIR_W<28> {
        EPDIR_W::new(self)
    }
    #[doc = "Bit 29 - EPTYPE"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<29> {
        EPTYPE_W::new(self)
    }
    #[doc = "Bit 31 - EPEN"]
    #[inline(always)]
    #[must_use]
    pub fn epen(&mut self) -> EPEN_W<31> {
        EPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
