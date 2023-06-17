#[doc = "Register `USBISR` reader"]
pub struct R(crate::R<USBISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBISR` writer"]
pub struct W(crate::W<USBISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBISR_SPEC>;
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
impl From<crate::W<USBISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFIF` reader - SOFIF"]
pub type SOFIF_R = crate::BitReader;
#[doc = "Field `SOFIF` writer - SOFIF"]
pub type SOFIF_W<'a, const O: u8> = crate::BitWriter<'a, USBISR_SPEC, O>;
#[doc = "Field `URSTIF` reader - URSTIF"]
pub type URSTIF_R = crate::BitReader;
#[doc = "Field `URSTIF` writer - URSTIF"]
pub type URSTIF_W<'a, const O: u8> = crate::BitWriter<'a, USBISR_SPEC, O>;
#[doc = "Field `RSMIF` reader - RSMIF"]
pub type RSMIF_R = crate::BitReader;
#[doc = "Field `RSMIF` writer - RSMIF"]
pub type RSMIF_W<'a, const O: u8> = crate::BitWriter<'a, USBISR_SPEC, O>;
#[doc = "Field `SUSPIF` reader - SUSPIF"]
pub type SUSPIF_R = crate::BitReader;
#[doc = "Field `SUSPIF` writer - SUSPIF"]
pub type SUSPIF_W<'a, const O: u8> = crate::BitWriter<'a, USBISR_SPEC, O>;
#[doc = "Field `ESOFIF` reader - ESOFIF"]
pub type ESOFIF_R = crate::BitReader;
#[doc = "Field `ESOFIF` writer - ESOFIF"]
pub type ESOFIF_W<'a, const O: u8> = crate::BitWriter<'a, USBISR_SPEC, O>;
#[doc = "Field `EPnIF` reader - EPnIF"]
pub type EPN_IF_R = crate::FieldReader;
#[doc = "Field `EPnIF` writer - EPnIF"]
pub type EPN_IF_W<'a, const O: u8> = crate::FieldWriter<'a, USBISR_SPEC, 8, O>;
impl R {
    #[doc = "Bit 1 - SOFIF"]
    #[inline(always)]
    pub fn sofif(&self) -> SOFIF_R {
        SOFIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - URSTIF"]
    #[inline(always)]
    pub fn urstif(&self) -> URSTIF_R {
        URSTIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSMIF"]
    #[inline(always)]
    pub fn rsmif(&self) -> RSMIF_R {
        RSMIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SUSPIF"]
    #[inline(always)]
    pub fn suspif(&self) -> SUSPIF_R {
        SUSPIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ESOFIF"]
    #[inline(always)]
    pub fn esofif(&self) -> ESOFIF_R {
        ESOFIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - EPnIF"]
    #[inline(always)]
    pub fn epn_if(&self) -> EPN_IF_R {
        EPN_IF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - SOFIF"]
    #[inline(always)]
    #[must_use]
    pub fn sofif(&mut self) -> SOFIF_W<1> {
        SOFIF_W::new(self)
    }
    #[doc = "Bit 2 - URSTIF"]
    #[inline(always)]
    #[must_use]
    pub fn urstif(&mut self) -> URSTIF_W<2> {
        URSTIF_W::new(self)
    }
    #[doc = "Bit 3 - RSMIF"]
    #[inline(always)]
    #[must_use]
    pub fn rsmif(&mut self) -> RSMIF_W<3> {
        RSMIF_W::new(self)
    }
    #[doc = "Bit 4 - SUSPIF"]
    #[inline(always)]
    #[must_use]
    pub fn suspif(&mut self) -> SUSPIF_W<4> {
        SUSPIF_W::new(self)
    }
    #[doc = "Bit 5 - ESOFIF"]
    #[inline(always)]
    #[must_use]
    pub fn esofif(&mut self) -> ESOFIF_W<5> {
        ESOFIF_W::new(self)
    }
    #[doc = "Bits 8:15 - EPnIF"]
    #[inline(always)]
    #[must_use]
    pub fn epn_if(&mut self) -> EPN_IF_W<8> {
        EPN_IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USBISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbisr](index.html) module"]
pub struct USBISR_SPEC;
impl crate::RegisterSpec for USBISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbisr::R](R) reader structure"]
impl crate::Readable for USBISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbisr::W](W) writer structure"]
impl crate::Writable for USBISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBISR to value 0"]
impl crate::Resettable for USBISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
