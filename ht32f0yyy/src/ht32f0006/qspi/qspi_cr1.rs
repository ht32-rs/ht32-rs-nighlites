#[doc = "Register `QSPI_CR1` reader"]
pub struct R(crate::R<QSPI_CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_CR1` writer"]
pub struct W(crate::W<QSPI_CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_CR1_SPEC>;
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
impl From<crate::W<QSPI_CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFL` reader - DFL"]
pub type DFL_R = crate::FieldReader;
#[doc = "Field `DFL` writer - DFL"]
pub type DFL_W<'a, const O: u8> = crate::FieldWriter<'a, QSPI_CR1_SPEC, 4, O>;
#[doc = "Field `FORMAT` reader - FORMAT"]
pub type FORMAT_R = crate::FieldReader;
#[doc = "Field `FORMAT` writer - FORMAT"]
pub type FORMAT_W<'a, const O: u8> = crate::FieldWriter<'a, QSPI_CR1_SPEC, 3, O>;
#[doc = "Field `SELAP` reader - SELAP"]
pub type SELAP_R = crate::BitReader;
#[doc = "Field `SELAP` writer - SELAP"]
pub type SELAP_W<'a, const O: u8> = crate::BitWriter<'a, QSPI_CR1_SPEC, O>;
#[doc = "Field `FIRSTBIT` reader - FIRSTBIT"]
pub type FIRSTBIT_R = crate::BitReader;
#[doc = "Field `FIRSTBIT` writer - FIRSTBIT"]
pub type FIRSTBIT_W<'a, const O: u8> = crate::BitWriter<'a, QSPI_CR1_SPEC, O>;
#[doc = "Field `SELM` reader - SELM"]
pub type SELM_R = crate::BitReader;
#[doc = "Field `SELM` writer - SELM"]
pub type SELM_W<'a, const O: u8> = crate::BitWriter<'a, QSPI_CR1_SPEC, O>;
#[doc = "Field `MODE` reader - MODE"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - MODE"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, QSPI_CR1_SPEC, O>;
impl R {
    #[doc = "Bits 0:3 - DFL"]
    #[inline(always)]
    pub fn dfl(&self) -> DFL_R {
        DFL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - FORMAT"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - SELAP"]
    #[inline(always)]
    pub fn selap(&self) -> SELAP_R {
        SELAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FIRSTBIT"]
    #[inline(always)]
    pub fn firstbit(&self) -> FIRSTBIT_R {
        FIRSTBIT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SELM"]
    #[inline(always)]
    pub fn selm(&self) -> SELM_R {
        SELM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - DFL"]
    #[inline(always)]
    #[must_use]
    pub fn dfl(&mut self) -> DFL_W<0> {
        DFL_W::new(self)
    }
    #[doc = "Bits 8:10 - FORMAT"]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FORMAT_W<8> {
        FORMAT_W::new(self)
    }
    #[doc = "Bit 11 - SELAP"]
    #[inline(always)]
    #[must_use]
    pub fn selap(&mut self) -> SELAP_W<11> {
        SELAP_W::new(self)
    }
    #[doc = "Bit 12 - FIRSTBIT"]
    #[inline(always)]
    #[must_use]
    pub fn firstbit(&mut self) -> FIRSTBIT_W<12> {
        FIRSTBIT_W::new(self)
    }
    #[doc = "Bit 13 - SELM"]
    #[inline(always)]
    #[must_use]
    pub fn selm(&mut self) -> SELM_W<13> {
        SELM_W::new(self)
    }
    #[doc = "Bit 14 - MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<14> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_CR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_cr1](index.html) module"]
pub struct QSPI_CR1_SPEC;
impl crate::RegisterSpec for QSPI_CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_cr1::R](R) reader structure"]
impl crate::Readable for QSPI_CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_cr1::W](W) writer structure"]
impl crate::Writable for QSPI_CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QSPI_CR1 to value 0"]
impl crate::Resettable for QSPI_CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
