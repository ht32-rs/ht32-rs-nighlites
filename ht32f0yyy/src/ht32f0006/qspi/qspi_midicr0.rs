#[doc = "Register `QSPI_MIDICR0` reader"]
pub struct R(crate::R<QSPI_MIDICR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_MIDICR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_MIDICR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_MIDICR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_MIDICR0` writer"]
pub struct W(crate::W<QSPI_MIDICR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_MIDICR0_SPEC>;
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
impl From<crate::W<QSPI_MIDICR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_MIDICR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATFL` reader - DATFL"]
pub type DATFL_R = crate::FieldReader;
#[doc = "Field `DATFL` writer - DATFL"]
pub type DATFL_W<'a, const O: u8> = crate::FieldWriter<'a, QSPI_MIDICR0_SPEC, 6, O>;
#[doc = "Field `SMDSEL` reader - SMDSEL"]
pub type SMDSEL_R = crate::FieldReader;
#[doc = "Field `SMDSEL` writer - SMDSEL"]
pub type SMDSEL_W<'a, const O: u8> = crate::FieldWriter<'a, QSPI_MIDICR0_SPEC, 2, O>;
#[doc = "Field `DMFL` reader - DMFL"]
pub type DMFL_R = crate::FieldReader;
#[doc = "Field `DMFL` writer - DMFL"]
pub type DMFL_W<'a, const O: u8> = crate::FieldWriter<'a, QSPI_MIDICR0_SPEC, 4, O>;
#[doc = "Field `MDFL` reader - MDFL"]
pub type MDFL_R = crate::FieldReader;
#[doc = "Field `MDFL` writer - MDFL"]
pub type MDFL_W<'a, const O: u8> = crate::FieldWriter<'a, QSPI_MIDICR0_SPEC, 4, O>;
#[doc = "Field `ADFL` reader - ADFL"]
pub type ADFL_R = crate::FieldReader;
#[doc = "Field `ADFL` writer - ADFL"]
pub type ADFL_W<'a, const O: u8> = crate::FieldWriter<'a, QSPI_MIDICR0_SPEC, 6, O>;
#[doc = "Field `QDIOEN` reader - QDIOEN"]
pub type QDIOEN_R = crate::BitReader;
#[doc = "Field `QDIOEN` writer - QDIOEN"]
pub type QDIOEN_W<'a, const O: u8> = crate::BitWriter<'a, QSPI_MIDICR0_SPEC, O>;
#[doc = "Field `MIDICEN` reader - MIDICEN"]
pub type MIDICEN_R = crate::BitReader;
#[doc = "Field `MIDICEN` writer - MIDICEN"]
pub type MIDICEN_W<'a, const O: u8> = crate::BitWriter<'a, QSPI_MIDICR0_SPEC, O>;
#[doc = "Field `CMDFL` reader - CMDFL"]
pub type CMDFL_R = crate::FieldReader;
#[doc = "Field `CMDFL` writer - CMDFL"]
pub type CMDFL_W<'a, const O: u8> = crate::FieldWriter<'a, QSPI_MIDICR0_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:5 - DATFL"]
    #[inline(always)]
    pub fn datfl(&self) -> DATFL_R {
        DATFL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - SMDSEL"]
    #[inline(always)]
    pub fn smdsel(&self) -> SMDSEL_R {
        SMDSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DMFL"]
    #[inline(always)]
    pub fn dmfl(&self) -> DMFL_R {
        DMFL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - MDFL"]
    #[inline(always)]
    pub fn mdfl(&self) -> MDFL_R {
        MDFL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - ADFL"]
    #[inline(always)]
    pub fn adfl(&self) -> ADFL_R {
        ADFL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - QDIOEN"]
    #[inline(always)]
    pub fn qdioen(&self) -> QDIOEN_R {
        QDIOEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MIDICEN"]
    #[inline(always)]
    pub fn midicen(&self) -> MIDICEN_R {
        MIDICEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - CMDFL"]
    #[inline(always)]
    pub fn cmdfl(&self) -> CMDFL_R {
        CMDFL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - DATFL"]
    #[inline(always)]
    #[must_use]
    pub fn datfl(&mut self) -> DATFL_W<0> {
        DATFL_W::new(self)
    }
    #[doc = "Bits 6:7 - SMDSEL"]
    #[inline(always)]
    #[must_use]
    pub fn smdsel(&mut self) -> SMDSEL_W<6> {
        SMDSEL_W::new(self)
    }
    #[doc = "Bits 8:11 - DMFL"]
    #[inline(always)]
    #[must_use]
    pub fn dmfl(&mut self) -> DMFL_W<8> {
        DMFL_W::new(self)
    }
    #[doc = "Bits 12:15 - MDFL"]
    #[inline(always)]
    #[must_use]
    pub fn mdfl(&mut self) -> MDFL_W<12> {
        MDFL_W::new(self)
    }
    #[doc = "Bits 16:21 - ADFL"]
    #[inline(always)]
    #[must_use]
    pub fn adfl(&mut self) -> ADFL_W<16> {
        ADFL_W::new(self)
    }
    #[doc = "Bit 22 - QDIOEN"]
    #[inline(always)]
    #[must_use]
    pub fn qdioen(&mut self) -> QDIOEN_W<22> {
        QDIOEN_W::new(self)
    }
    #[doc = "Bit 23 - MIDICEN"]
    #[inline(always)]
    #[must_use]
    pub fn midicen(&mut self) -> MIDICEN_W<23> {
        MIDICEN_W::new(self)
    }
    #[doc = "Bits 24:27 - CMDFL"]
    #[inline(always)]
    #[must_use]
    pub fn cmdfl(&mut self) -> CMDFL_W<24> {
        CMDFL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_MIDICR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_midicr0](index.html) module"]
pub struct QSPI_MIDICR0_SPEC;
impl crate::RegisterSpec for QSPI_MIDICR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_midicr0::R](R) reader structure"]
impl crate::Readable for QSPI_MIDICR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_midicr0::W](W) writer structure"]
impl crate::Writable for QSPI_MIDICR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QSPI_MIDICR0 to value 0"]
impl crate::Resettable for QSPI_MIDICR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
