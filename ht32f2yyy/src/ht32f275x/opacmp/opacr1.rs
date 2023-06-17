#[doc = "Register `OPACR1` reader"]
pub struct R(crate::R<OPACR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPACR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPACR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPACR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPACR1` writer"]
pub struct W(crate::W<OPACR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPACR1_SPEC>;
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
impl From<crate::W<OPACR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPACR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPA1EN` reader - OPA1EN"]
pub type OPA1EN_R = crate::BitReader;
#[doc = "Field `OPA1EN` writer - OPA1EN"]
pub type OPA1EN_W<'a, const O: u8> = crate::BitWriter<'a, OPACR1_SPEC, O>;
#[doc = "Field `OPC1MS` reader - OPC1MS"]
pub type OPC1MS_R = crate::BitReader;
#[doc = "Field `OPC1MS` writer - OPC1MS"]
pub type OPC1MS_W<'a, const O: u8> = crate::BitWriter<'a, OPACR1_SPEC, O>;
#[doc = "Field `A1OFM` reader - A1OFM"]
pub type A1OFM_R = crate::BitReader;
#[doc = "Field `A1OFM` writer - A1OFM"]
pub type A1OFM_W<'a, const O: u8> = crate::BitWriter<'a, OPACR1_SPEC, O>;
#[doc = "Field `A1RS` reader - A1RS"]
pub type A1RS_R = crate::BitReader;
#[doc = "Field `A1RS` writer - A1RS"]
pub type A1RS_W<'a, const O: u8> = crate::BitWriter<'a, OPACR1_SPEC, O>;
#[doc = "Field `CMP1S` reader - CMP1S"]
pub type CMP1S_R = crate::BitReader;
#[doc = "Field `CMP1S` writer - CMP1S"]
pub type CMP1S_W<'a, const O: u8> = crate::BitWriter<'a, OPACR1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - OPA1EN"]
    #[inline(always)]
    pub fn opa1en(&self) -> OPA1EN_R {
        OPA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OPC1MS"]
    #[inline(always)]
    pub fn opc1ms(&self) -> OPC1MS_R {
        OPC1MS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A1OFM"]
    #[inline(always)]
    pub fn a1ofm(&self) -> A1OFM_R {
        A1OFM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A1RS"]
    #[inline(always)]
    pub fn a1rs(&self) -> A1RS_R {
        A1RS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - CMP1S"]
    #[inline(always)]
    pub fn cmp1s(&self) -> CMP1S_R {
        CMP1S_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPA1EN"]
    #[inline(always)]
    #[must_use]
    pub fn opa1en(&mut self) -> OPA1EN_W<0> {
        OPA1EN_W::new(self)
    }
    #[doc = "Bit 1 - OPC1MS"]
    #[inline(always)]
    #[must_use]
    pub fn opc1ms(&mut self) -> OPC1MS_W<1> {
        OPC1MS_W::new(self)
    }
    #[doc = "Bit 2 - A1OFM"]
    #[inline(always)]
    #[must_use]
    pub fn a1ofm(&mut self) -> A1OFM_W<2> {
        A1OFM_W::new(self)
    }
    #[doc = "Bit 3 - A1RS"]
    #[inline(always)]
    #[must_use]
    pub fn a1rs(&mut self) -> A1RS_W<3> {
        A1RS_W::new(self)
    }
    #[doc = "Bit 8 - CMP1S"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1s(&mut self) -> CMP1S_W<8> {
        CMP1S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPACR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr1](index.html) module"]
pub struct OPACR1_SPEC;
impl crate::RegisterSpec for OPACR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opacr1::R](R) reader structure"]
impl crate::Readable for OPACR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opacr1::W](W) writer structure"]
impl crate::Writable for OPACR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPACR1 to value 0"]
impl crate::Resettable for OPACR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
