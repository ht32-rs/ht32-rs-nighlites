#[doc = "Register `USB_FCR` reader"]
pub struct R(crate::R<USB_FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_FCR` writer"]
pub struct W(crate::W<USB_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_FCR_SPEC>;
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
impl From<crate::W<USB_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRNUM` reader - FRNUM"]
pub type FRNUM_R = crate::FieldReader<u16>;
#[doc = "Field `FRNUM` writer - FRNUM"]
pub type FRNUM_W<'a, const O: u8> = crate::FieldWriter<'a, USB_FCR_SPEC, 11, O, u16>;
#[doc = "Field `SOFLCK` reader - SOFLCK"]
pub type SOFLCK_R = crate::BitReader;
#[doc = "Field `SOFLCK` writer - SOFLCK"]
pub type SOFLCK_W<'a, const O: u8> = crate::BitWriter<'a, USB_FCR_SPEC, O>;
#[doc = "Field `LSOF` reader - LSOF"]
pub type LSOF_R = crate::FieldReader;
#[doc = "Field `LSOF` writer - LSOF"]
pub type LSOF_W<'a, const O: u8> = crate::FieldWriter<'a, USB_FCR_SPEC, 2, O>;
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
#[doc = "Lost Start-of-Frame number and the USB frame count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_fcr](index.html) module"]
pub struct USB_FCR_SPEC;
impl crate::RegisterSpec for USB_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_fcr::R](R) reader structure"]
impl crate::Readable for USB_FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_fcr::W](W) writer structure"]
impl crate::Writable for USB_FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_FCR to value 0"]
impl crate::Resettable for USB_FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
