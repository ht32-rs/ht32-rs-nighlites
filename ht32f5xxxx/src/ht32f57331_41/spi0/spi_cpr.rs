#[doc = "Register `SPI_CPR` reader"]
pub struct R(crate::R<SPI_CPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CPR` writer"]
pub struct W(crate::W<SPI_CPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CPR_SPEC>;
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
impl From<crate::W<SPI_CPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CP` reader - CP"]
pub type CP_R = crate::FieldReader<u16>;
#[doc = "Field `CP` writer - CP"]
pub type CP_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_CPR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CP"]
    #[inline(always)]
    pub fn cp(&self) -> CP_R {
        CP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CP"]
    #[inline(always)]
    #[must_use]
    pub fn cp(&mut self) -> CP_W<0> {
        CP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_CPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cpr](index.html) module"]
pub struct SPI_CPR_SPEC;
impl crate::RegisterSpec for SPI_CPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_cpr::R](R) reader structure"]
impl crate::Readable for SPI_CPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_cpr::W](W) writer structure"]
impl crate::Writable for SPI_CPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_CPR to value 0"]
impl crate::Resettable for SPI_CPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
