#[doc = "Register `CSIF_CR` reader"]
pub struct R(crate::R<CSIF_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIF_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIF_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIF_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSIF_CR` writer"]
pub struct W(crate::W<CSIF_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIF_CR_SPEC>;
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
impl From<crate::W<CSIF_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIF_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VSYNCTYP` reader - VSYNCTYP"]
pub type VSYNCTYP_R = crate::BitReader;
#[doc = "Field `VSYNCTYP` writer - VSYNCTYP"]
pub type VSYNCTYP_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_CR_SPEC, O>;
#[doc = "Field `HSYNCTYP` reader - HSYNCTYP"]
pub type HSYNCTYP_R = crate::BitReader;
#[doc = "Field `HSYNCTYP` writer - HSYNCTYP"]
pub type HSYNCTYP_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_CR_SPEC, O>;
#[doc = "Field `CLKEDGE` reader - CLKEDGE"]
pub type CLKEDGE_R = crate::BitReader;
#[doc = "Field `CLKEDGE` writer - CLKEDGE"]
pub type CLKEDGE_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_CR_SPEC, O>;
#[doc = "Field `IMG_FMT` reader - IMG_FMT"]
pub type IMG_FMT_R = crate::BitReader;
#[doc = "Field `IMG_FMT` writer - IMG_FMT"]
pub type IMG_FMT_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_CR_SPEC, O>;
#[doc = "Field `VSYNCPOL` reader - VSYNCPOL"]
pub type VSYNCPOL_R = crate::BitReader;
#[doc = "Field `VSYNCPOL` writer - VSYNCPOL"]
pub type VSYNCPOL_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_CR_SPEC, O>;
#[doc = "Field `HSYNCPOL` reader - HSYNCPOL"]
pub type HSYNCPOL_R = crate::BitReader;
#[doc = "Field `HSYNCPOL` writer - HSYNCPOL"]
pub type HSYNCPOL_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_CR_SPEC, O>;
#[doc = "Field `IMG_SLD` reader - IMG_SLD"]
pub type IMG_SLD_R = crate::FieldReader;
#[doc = "Field `IMG_SLD` writer - IMG_SLD"]
pub type IMG_SLD_W<'a, const O: u8> = crate::FieldWriter<'a, CSIF_CR_SPEC, 8, O>;
#[doc = "Field `IMG_SFD` reader - IMG_SFD"]
pub type IMG_SFD_R = crate::FieldReader;
#[doc = "Field `IMG_SFD` writer - IMG_SFD"]
pub type IMG_SFD_W<'a, const O: u8> = crate::FieldWriter<'a, CSIF_CR_SPEC, 8, O>;
impl R {
    #[doc = "Bit 1 - VSYNCTYP"]
    #[inline(always)]
    pub fn vsynctyp(&self) -> VSYNCTYP_R {
        VSYNCTYP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSYNCTYP"]
    #[inline(always)]
    pub fn hsynctyp(&self) -> HSYNCTYP_R {
        HSYNCTYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CLKEDGE"]
    #[inline(always)]
    pub fn clkedge(&self) -> CLKEDGE_R {
        CLKEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IMG_FMT"]
    #[inline(always)]
    pub fn img_fmt(&self) -> IMG_FMT_R {
        IMG_FMT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - VSYNCPOL"]
    #[inline(always)]
    pub fn vsyncpol(&self) -> VSYNCPOL_R {
        VSYNCPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HSYNCPOL"]
    #[inline(always)]
    pub fn hsyncpol(&self) -> HSYNCPOL_R {
        HSYNCPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - IMG_SLD"]
    #[inline(always)]
    pub fn img_sld(&self) -> IMG_SLD_R {
        IMG_SLD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - IMG_SFD"]
    #[inline(always)]
    pub fn img_sfd(&self) -> IMG_SFD_R {
        IMG_SFD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - VSYNCTYP"]
    #[inline(always)]
    #[must_use]
    pub fn vsynctyp(&mut self) -> VSYNCTYP_W<1> {
        VSYNCTYP_W::new(self)
    }
    #[doc = "Bit 2 - HSYNCTYP"]
    #[inline(always)]
    #[must_use]
    pub fn hsynctyp(&mut self) -> HSYNCTYP_W<2> {
        HSYNCTYP_W::new(self)
    }
    #[doc = "Bit 3 - CLKEDGE"]
    #[inline(always)]
    #[must_use]
    pub fn clkedge(&mut self) -> CLKEDGE_W<3> {
        CLKEDGE_W::new(self)
    }
    #[doc = "Bit 4 - IMG_FMT"]
    #[inline(always)]
    #[must_use]
    pub fn img_fmt(&mut self) -> IMG_FMT_W<4> {
        IMG_FMT_W::new(self)
    }
    #[doc = "Bit 6 - VSYNCPOL"]
    #[inline(always)]
    #[must_use]
    pub fn vsyncpol(&mut self) -> VSYNCPOL_W<6> {
        VSYNCPOL_W::new(self)
    }
    #[doc = "Bit 7 - HSYNCPOL"]
    #[inline(always)]
    #[must_use]
    pub fn hsyncpol(&mut self) -> HSYNCPOL_W<7> {
        HSYNCPOL_W::new(self)
    }
    #[doc = "Bits 8:15 - IMG_SLD"]
    #[inline(always)]
    #[must_use]
    pub fn img_sld(&mut self) -> IMG_SLD_W<8> {
        IMG_SLD_W::new(self)
    }
    #[doc = "Bits 16:23 - IMG_SFD"]
    #[inline(always)]
    #[must_use]
    pub fn img_sfd(&mut self) -> IMG_SFD_W<16> {
        IMG_SFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIF_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_cr](index.html) module"]
pub struct CSIF_CR_SPEC;
impl crate::RegisterSpec for CSIF_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csif_cr::R](R) reader structure"]
impl crate::Readable for CSIF_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csif_cr::W](W) writer structure"]
impl crate::Writable for CSIF_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSIF_CR to value 0"]
impl crate::Resettable for CSIF_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
