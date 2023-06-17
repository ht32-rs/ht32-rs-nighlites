#[doc = "Register `CSIF_IMGWH` reader"]
pub struct R(crate::R<CSIF_IMGWH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIF_IMGWH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIF_IMGWH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIF_IMGWH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSIF_IMGWH` writer"]
pub struct W(crate::W<CSIF_IMGWH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIF_IMGWH_SPEC>;
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
impl From<crate::W<CSIF_IMGWH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIF_IMGWH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMG_WID` reader - IMG_WID"]
pub type IMG_WID_R = crate::FieldReader<u16>;
#[doc = "Field `IMG_WID` writer - IMG_WID"]
pub type IMG_WID_W<'a, const O: u8> = crate::FieldWriter<'a, CSIF_IMGWH_SPEC, 11, O, u16>;
#[doc = "Field `IMG_HGH` reader - IMG_HGH"]
pub type IMG_HGH_R = crate::FieldReader<u16>;
#[doc = "Field `IMG_HGH` writer - IMG_HGH"]
pub type IMG_HGH_W<'a, const O: u8> = crate::FieldWriter<'a, CSIF_IMGWH_SPEC, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - IMG_WID"]
    #[inline(always)]
    pub fn img_wid(&self) -> IMG_WID_R {
        IMG_WID_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - IMG_HGH"]
    #[inline(always)]
    pub fn img_hgh(&self) -> IMG_HGH_R {
        IMG_HGH_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - IMG_WID"]
    #[inline(always)]
    #[must_use]
    pub fn img_wid(&mut self) -> IMG_WID_W<0> {
        IMG_WID_W::new(self)
    }
    #[doc = "Bits 16:26 - IMG_HGH"]
    #[inline(always)]
    #[must_use]
    pub fn img_hgh(&mut self) -> IMG_HGH_W<16> {
        IMG_HGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIF_IMGWH\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_imgwh](index.html) module"]
pub struct CSIF_IMGWH_SPEC;
impl crate::RegisterSpec for CSIF_IMGWH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csif_imgwh::R](R) reader structure"]
impl crate::Readable for CSIF_IMGWH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csif_imgwh::W](W) writer structure"]
impl crate::Writable for CSIF_IMGWH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSIF_IMGWH to value 0"]
impl crate::Resettable for CSIF_IMGWH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
