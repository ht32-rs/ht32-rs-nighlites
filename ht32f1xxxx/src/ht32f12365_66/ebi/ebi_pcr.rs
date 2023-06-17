#[doc = "Register `EBI_PCR` reader"]
pub struct R(crate::R<EBI_PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBI_PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EBI_PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EBI_PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EBI_PCR` writer"]
pub struct W(crate::W<EBI_PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBI_PCR_SPEC>;
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
impl From<crate::W<EBI_PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EBI_PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAGELEN` reader - PAGELEN"]
pub type PAGELEN_R = crate::FieldReader;
#[doc = "Field `PAGELEN` writer - PAGELEN"]
pub type PAGELEN_W<'a, const O: u8> = crate::FieldWriter<'a, EBI_PCR_SPEC, 2, O>;
#[doc = "Field `INCHIT` reader - INCHIT"]
pub type INCHIT_R = crate::BitReader;
#[doc = "Field `INCHIT` writer - INCHIT"]
pub type INCHIT_W<'a, const O: u8> = crate::BitWriter<'a, EBI_PCR_SPEC, O>;
#[doc = "Field `RDPG` reader - RDPG"]
pub type RDPG_R = crate::FieldReader;
#[doc = "Field `RDPG` writer - RDPG"]
pub type RDPG_W<'a, const O: u8> = crate::FieldWriter<'a, EBI_PCR_SPEC, 4, O>;
#[doc = "Field `PAGEOPEN` reader - PAGEOPEN"]
pub type PAGEOPEN_R = crate::FieldReader;
#[doc = "Field `PAGEOPEN` writer - PAGEOPEN"]
pub type PAGEOPEN_W<'a, const O: u8> = crate::FieldWriter<'a, EBI_PCR_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:1 - PAGELEN"]
    #[inline(always)]
    pub fn pagelen(&self) -> PAGELEN_R {
        PAGELEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - INCHIT"]
    #[inline(always)]
    pub fn inchit(&self) -> INCHIT_R {
        INCHIT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - RDPG"]
    #[inline(always)]
    pub fn rdpg(&self) -> RDPG_R {
        RDPG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - PAGEOPEN"]
    #[inline(always)]
    pub fn pageopen(&self) -> PAGEOPEN_R {
        PAGEOPEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PAGELEN"]
    #[inline(always)]
    #[must_use]
    pub fn pagelen(&mut self) -> PAGELEN_W<0> {
        PAGELEN_W::new(self)
    }
    #[doc = "Bit 4 - INCHIT"]
    #[inline(always)]
    #[must_use]
    pub fn inchit(&mut self) -> INCHIT_W<4> {
        INCHIT_W::new(self)
    }
    #[doc = "Bits 8:11 - RDPG"]
    #[inline(always)]
    #[must_use]
    pub fn rdpg(&mut self) -> RDPG_W<8> {
        RDPG_W::new(self)
    }
    #[doc = "Bits 16:23 - PAGEOPEN"]
    #[inline(always)]
    #[must_use]
    pub fn pageopen(&mut self) -> PAGEOPEN_W<16> {
        PAGEOPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBI_PCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_pcr](index.html) module"]
pub struct EBI_PCR_SPEC;
impl crate::RegisterSpec for EBI_PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ebi_pcr::R](R) reader structure"]
impl crate::Readable for EBI_PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ebi_pcr::W](W) writer structure"]
impl crate::Writable for EBI_PCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EBI_PCR to value 0"]
impl crate::Resettable for EBI_PCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
