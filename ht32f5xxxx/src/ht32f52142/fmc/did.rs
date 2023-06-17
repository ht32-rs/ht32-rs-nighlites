#[doc = "Register `DID` reader"]
pub struct R(crate::R<DID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DID` writer"]
pub struct W(crate::W<DID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DID_SPEC>;
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
impl From<crate::W<DID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ChipID` reader - ChipID"]
pub type CHIP_ID_R = crate::FieldReader<u16>;
#[doc = "Field `ChipID` writer - ChipID"]
pub type CHIP_ID_W<'a, const O: u8> = crate::FieldWriter<'a, DID_SPEC, 16, O, u16>;
#[doc = "Field `MFID` reader - MFID"]
pub type MFID_R = crate::FieldReader<u16>;
#[doc = "Field `MFID` writer - MFID"]
pub type MFID_W<'a, const O: u8> = crate::FieldWriter<'a, DID_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - ChipID"]
    #[inline(always)]
    pub fn chip_id(&self) -> CHIP_ID_R {
        CHIP_ID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MFID"]
    #[inline(always)]
    pub fn mfid(&self) -> MFID_R {
        MFID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ChipID"]
    #[inline(always)]
    #[must_use]
    pub fn chip_id(&mut self) -> CHIP_ID_W<0> {
        CHIP_ID_W::new(self)
    }
    #[doc = "Bits 16:31 - MFID"]
    #[inline(always)]
    #[must_use]
    pub fn mfid(&mut self) -> MFID_W<16> {
        MFID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [did](index.html) module"]
pub struct DID_SPEC;
impl crate::RegisterSpec for DID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [did::R](R) reader structure"]
impl crate::Readable for DID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [did::W](W) writer structure"]
impl crate::Writable for DID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DID to value 0"]
impl crate::Resettable for DID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
