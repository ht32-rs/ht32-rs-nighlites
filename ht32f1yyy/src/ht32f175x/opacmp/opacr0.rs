#[doc = "Register `OPACR0` reader"]
pub struct R(crate::R<OPACR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPACR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPACR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPACR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPACR0` writer"]
pub struct W(crate::W<OPACR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPACR0_SPEC>;
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
impl From<crate::W<OPACR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPACR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPA0EN` reader - OPA0EN"]
pub type OPA0EN_R = crate::BitReader;
#[doc = "Field `OPA0EN` writer - OPA0EN"]
pub type OPA0EN_W<'a, const O: u8> = crate::BitWriter<'a, OPACR0_SPEC, O>;
#[doc = "Field `OPC0MS` reader - OPC0MS"]
pub type OPC0MS_R = crate::BitReader;
#[doc = "Field `OPC0MS` writer - OPC0MS"]
pub type OPC0MS_W<'a, const O: u8> = crate::BitWriter<'a, OPACR0_SPEC, O>;
#[doc = "Field `A0OFM` reader - A0OFM"]
pub type A0OFM_R = crate::BitReader;
#[doc = "Field `A0OFM` writer - A0OFM"]
pub type A0OFM_W<'a, const O: u8> = crate::BitWriter<'a, OPACR0_SPEC, O>;
#[doc = "Field `A0RS` reader - A0RS"]
pub type A0RS_R = crate::BitReader;
#[doc = "Field `A0RS` writer - A0RS"]
pub type A0RS_W<'a, const O: u8> = crate::BitWriter<'a, OPACR0_SPEC, O>;
#[doc = "Field `CMP0S` reader - CMP0S"]
pub type CMP0S_R = crate::BitReader;
#[doc = "Field `CMP0S` writer - CMP0S"]
pub type CMP0S_W<'a, const O: u8> = crate::BitWriter<'a, OPACR0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - OPA0EN"]
    #[inline(always)]
    pub fn opa0en(&self) -> OPA0EN_R {
        OPA0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OPC0MS"]
    #[inline(always)]
    pub fn opc0ms(&self) -> OPC0MS_R {
        OPC0MS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A0OFM"]
    #[inline(always)]
    pub fn a0ofm(&self) -> A0OFM_R {
        A0OFM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A0RS"]
    #[inline(always)]
    pub fn a0rs(&self) -> A0RS_R {
        A0RS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - CMP0S"]
    #[inline(always)]
    pub fn cmp0s(&self) -> CMP0S_R {
        CMP0S_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPA0EN"]
    #[inline(always)]
    #[must_use]
    pub fn opa0en(&mut self) -> OPA0EN_W<0> {
        OPA0EN_W::new(self)
    }
    #[doc = "Bit 1 - OPC0MS"]
    #[inline(always)]
    #[must_use]
    pub fn opc0ms(&mut self) -> OPC0MS_W<1> {
        OPC0MS_W::new(self)
    }
    #[doc = "Bit 2 - A0OFM"]
    #[inline(always)]
    #[must_use]
    pub fn a0ofm(&mut self) -> A0OFM_W<2> {
        A0OFM_W::new(self)
    }
    #[doc = "Bit 3 - A0RS"]
    #[inline(always)]
    #[must_use]
    pub fn a0rs(&mut self) -> A0RS_W<3> {
        A0RS_W::new(self)
    }
    #[doc = "Bit 8 - CMP0S"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0s(&mut self) -> CMP0S_W<8> {
        CMP0S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPACR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr0](index.html) module"]
pub struct OPACR0_SPEC;
impl crate::RegisterSpec for OPACR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opacr0::R](R) reader structure"]
impl crate::Readable for OPACR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opacr0::W](W) writer structure"]
impl crate::Writable for OPACR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPACR0 to value 0"]
impl crate::Resettable for OPACR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
