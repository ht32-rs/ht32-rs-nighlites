#[doc = "Register `USBFCR` reader"]
pub struct R(crate::R<USBFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBFCR` writer"]
pub struct W(crate::W<USBFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBFCR_SPEC>;
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
impl From<crate::W<USBFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRNUM` reader - FRNUM"]
pub type FRNUM_R = crate::FieldReader<u16>;
#[doc = "Field `FRNUM` writer - FRNUM"]
pub type FRNUM_W<'a, const O: u8> = crate::FieldWriter<'a, USBFCR_SPEC, 11, O, u16>;
#[doc = "Field `SOFLCK` reader - SOFLCK"]
pub type SOFLCK_R = crate::BitReader;
#[doc = "Field `SOFLCK` writer - SOFLCK"]
pub type SOFLCK_W<'a, const O: u8> = crate::BitWriter<'a, USBFCR_SPEC, O>;
#[doc = "Field `LSOF` reader - LSOF"]
pub type LSOF_R = crate::FieldReader;
#[doc = "Field `LSOF` writer - LSOF"]
pub type LSOF_W<'a, const O: u8> = crate::FieldWriter<'a, USBFCR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:10 - FRNUM"]
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - SOFLCK"]
    #[inline(always)]
    pub fn soflck(&self) -> SOFLCK_R {
        SOFLCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - LSOF"]
    #[inline(always)]
    pub fn lsof(&self) -> LSOF_R {
        LSOF_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - FRNUM"]
    #[inline(always)]
    #[must_use]
    pub fn frnum(&mut self) -> FRNUM_W<0> {
        FRNUM_W::new(self)
    }
    #[doc = "Bit 16 - SOFLCK"]
    #[inline(always)]
    #[must_use]
    pub fn soflck(&mut self) -> SOFLCK_W<16> {
        SOFLCK_W::new(self)
    }
    #[doc = "Bits 17:18 - LSOF"]
    #[inline(always)]
    #[must_use]
    pub fn lsof(&mut self) -> LSOF_W<17> {
        LSOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USBFCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbfcr](index.html) module"]
pub struct USBFCR_SPEC;
impl crate::RegisterSpec for USBFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbfcr::R](R) reader structure"]
impl crate::Readable for USBFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbfcr::W](W) writer structure"]
impl crate::Writable for USBFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBFCR to value 0"]
impl crate::Resettable for USBFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
