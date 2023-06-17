#[doc = "Register `FMC_CFCR` reader"]
pub struct R(crate::R<FMC_CFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_CFCR` writer"]
pub struct W(crate::W<FMC_CFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CFCR_SPEC>;
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
impl From<crate::W<FMC_CFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAIT` reader - WAIT"]
pub type WAIT_R = crate::FieldReader;
#[doc = "Field `WAIT` writer - WAIT"]
pub type WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, FMC_CFCR_SPEC, 3, O>;
#[doc = "Field `PFBE` reader - PFBE"]
pub type PFBE_R = crate::BitReader;
#[doc = "Field `PFBE` writer - PFBE"]
pub type PFBE_W<'a, const O: u8> = crate::BitWriter<'a, FMC_CFCR_SPEC, O>;
#[doc = "Field `DCDB` reader - DCDB"]
pub type DCDB_R = crate::BitReader;
#[doc = "Field `DCDB` writer - DCDB"]
pub type DCDB_W<'a, const O: u8> = crate::BitWriter<'a, FMC_CFCR_SPEC, O>;
#[doc = "Field `CE` reader - CE"]
pub type CE_R = crate::BitReader;
#[doc = "Field `CE` writer - CE"]
pub type CE_W<'a, const O: u8> = crate::BitWriter<'a, FMC_CFCR_SPEC, O>;
#[doc = "Field `FHLAEN` reader - FHLAEN"]
pub type FHLAEN_R = crate::BitReader;
#[doc = "Field `FHLAEN` writer - FHLAEN"]
pub type FHLAEN_W<'a, const O: u8> = crate::BitWriter<'a, FMC_CFCR_SPEC, O>;
#[doc = "Field `FZWPSEN` reader - FZWPSEN"]
pub type FZWPSEN_R = crate::BitReader;
#[doc = "Field `FZWPSEN` writer - FZWPSEN"]
pub type FZWPSEN_W<'a, const O: u8> = crate::BitWriter<'a, FMC_CFCR_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - WAIT"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - PFBE"]
    #[inline(always)]
    pub fn pfbe(&self) -> PFBE_R {
        PFBE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - DCDB"]
    #[inline(always)]
    pub fn dcdb(&self) -> DCDB_R {
        DCDB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CE"]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - FHLAEN"]
    #[inline(always)]
    pub fn fhlaen(&self) -> FHLAEN_R {
        FHLAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FZWPSEN"]
    #[inline(always)]
    pub fn fzwpsen(&self) -> FZWPSEN_R {
        FZWPSEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - WAIT"]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<0> {
        WAIT_W::new(self)
    }
    #[doc = "Bit 4 - PFBE"]
    #[inline(always)]
    #[must_use]
    pub fn pfbe(&mut self) -> PFBE_W<4> {
        PFBE_W::new(self)
    }
    #[doc = "Bit 7 - DCDB"]
    #[inline(always)]
    #[must_use]
    pub fn dcdb(&mut self) -> DCDB_W<7> {
        DCDB_W::new(self)
    }
    #[doc = "Bit 12 - CE"]
    #[inline(always)]
    #[must_use]
    pub fn ce(&mut self) -> CE_W<12> {
        CE_W::new(self)
    }
    #[doc = "Bit 15 - FHLAEN"]
    #[inline(always)]
    #[must_use]
    pub fn fhlaen(&mut self) -> FHLAEN_W<15> {
        FHLAEN_W::new(self)
    }
    #[doc = "Bit 16 - FZWPSEN"]
    #[inline(always)]
    #[must_use]
    pub fn fzwpsen(&mut self) -> FZWPSEN_W<16> {
        FZWPSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC_CFCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_cfcr](index.html) module"]
pub struct FMC_CFCR_SPEC;
impl crate::RegisterSpec for FMC_CFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_cfcr::R](R) reader structure"]
impl crate::Readable for FMC_CFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_cfcr::W](W) writer structure"]
impl crate::Writable for FMC_CFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_CFCR to value 0"]
impl crate::Resettable for FMC_CFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
