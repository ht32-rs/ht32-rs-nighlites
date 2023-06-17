#[doc = "Register `USART0_MODSR` reader"]
pub struct R(crate::R<USART0_MODSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART0_MODSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART0_MODSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART0_MODSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART0_MODSR` writer"]
pub struct W(crate::W<USART0_MODSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART0_MODSR_SPEC>;
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
impl From<crate::W<USART0_MODSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART0_MODSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCTS` reader - DCTS"]
pub type DCTS_R = crate::BitReader;
#[doc = "Field `DCTS` writer - DCTS"]
pub type DCTS_W<'a, const O: u8> = crate::BitWriter<'a, USART0_MODSR_SPEC, O>;
#[doc = "Field `DDSR` reader - DDSR"]
pub type DDSR_R = crate::BitReader;
#[doc = "Field `DDSR` writer - DDSR"]
pub type DDSR_W<'a, const O: u8> = crate::BitWriter<'a, USART0_MODSR_SPEC, O>;
#[doc = "Field `DRI` reader - DRI"]
pub type DRI_R = crate::BitReader;
#[doc = "Field `DRI` writer - DRI"]
pub type DRI_W<'a, const O: u8> = crate::BitWriter<'a, USART0_MODSR_SPEC, O>;
#[doc = "Field `DDCD` reader - DDCD"]
pub type DDCD_R = crate::BitReader;
#[doc = "Field `DDCD` writer - DDCD"]
pub type DDCD_W<'a, const O: u8> = crate::BitWriter<'a, USART0_MODSR_SPEC, O>;
#[doc = "Field `CTSS` reader - CTSS"]
pub type CTSS_R = crate::BitReader;
#[doc = "Field `CTSS` writer - CTSS"]
pub type CTSS_W<'a, const O: u8> = crate::BitWriter<'a, USART0_MODSR_SPEC, O>;
#[doc = "Field `DSRS` reader - DSRS"]
pub type DSRS_R = crate::BitReader;
#[doc = "Field `DSRS` writer - DSRS"]
pub type DSRS_W<'a, const O: u8> = crate::BitWriter<'a, USART0_MODSR_SPEC, O>;
#[doc = "Field `RIS` reader - RIS"]
pub type RIS_R = crate::BitReader;
#[doc = "Field `RIS` writer - RIS"]
pub type RIS_W<'a, const O: u8> = crate::BitWriter<'a, USART0_MODSR_SPEC, O>;
#[doc = "Field `DCDS` reader - DCDS"]
pub type DCDS_R = crate::BitReader;
#[doc = "Field `DCDS` writer - DCDS"]
pub type DCDS_W<'a, const O: u8> = crate::BitWriter<'a, USART0_MODSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - DCTS"]
    #[inline(always)]
    pub fn dcts(&self) -> DCTS_R {
        DCTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DDSR"]
    #[inline(always)]
    pub fn ddsr(&self) -> DDSR_R {
        DDSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DRI"]
    #[inline(always)]
    pub fn dri(&self) -> DRI_R {
        DRI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DDCD"]
    #[inline(always)]
    pub fn ddcd(&self) -> DDCD_R {
        DDCD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CTSS"]
    #[inline(always)]
    pub fn ctss(&self) -> CTSS_R {
        CTSS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DSRS"]
    #[inline(always)]
    pub fn dsrs(&self) -> DSRS_R {
        DSRS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RIS"]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DCDS"]
    #[inline(always)]
    pub fn dcds(&self) -> DCDS_R {
        DCDS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCTS"]
    #[inline(always)]
    #[must_use]
    pub fn dcts(&mut self) -> DCTS_W<0> {
        DCTS_W::new(self)
    }
    #[doc = "Bit 1 - DDSR"]
    #[inline(always)]
    #[must_use]
    pub fn ddsr(&mut self) -> DDSR_W<1> {
        DDSR_W::new(self)
    }
    #[doc = "Bit 2 - DRI"]
    #[inline(always)]
    #[must_use]
    pub fn dri(&mut self) -> DRI_W<2> {
        DRI_W::new(self)
    }
    #[doc = "Bit 3 - DDCD"]
    #[inline(always)]
    #[must_use]
    pub fn ddcd(&mut self) -> DDCD_W<3> {
        DDCD_W::new(self)
    }
    #[doc = "Bit 4 - CTSS"]
    #[inline(always)]
    #[must_use]
    pub fn ctss(&mut self) -> CTSS_W<4> {
        CTSS_W::new(self)
    }
    #[doc = "Bit 5 - DSRS"]
    #[inline(always)]
    #[must_use]
    pub fn dsrs(&mut self) -> DSRS_W<5> {
        DSRS_W::new(self)
    }
    #[doc = "Bit 6 - RIS"]
    #[inline(always)]
    #[must_use]
    pub fn ris(&mut self) -> RIS_W<6> {
        RIS_W::new(self)
    }
    #[doc = "Bit 7 - DCDS"]
    #[inline(always)]
    #[must_use]
    pub fn dcds(&mut self) -> DCDS_W<7> {
        DCDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART0_MODSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_modsr](index.html) module"]
pub struct USART0_MODSR_SPEC;
impl crate::RegisterSpec for USART0_MODSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart0_modsr::R](R) reader structure"]
impl crate::Readable for USART0_MODSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart0_modsr::W](W) writer structure"]
impl crate::Writable for USART0_MODSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USART0_MODSR to value 0"]
impl crate::Resettable for USART0_MODSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
