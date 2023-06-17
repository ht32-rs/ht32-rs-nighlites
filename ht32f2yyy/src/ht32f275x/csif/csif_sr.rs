#[doc = "Register `CSIF_SR` reader"]
pub struct R(crate::R<CSIF_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIF_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIF_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIF_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSIF_SR` writer"]
pub struct W(crate::W<CSIF_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIF_SR_SPEC>;
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
impl From<crate::W<CSIF_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIF_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOF_FLG` reader - SOF_FLG"]
pub type SOF_FLG_R = crate::BitReader;
#[doc = "Field `SOF_FLG` writer - SOF_FLG"]
pub type SOF_FLG_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_SR_SPEC, O>;
#[doc = "Field `EOF_FLG` reader - EOF_FLG"]
pub type EOF_FLG_R = crate::BitReader;
#[doc = "Field `EOF_FLG` writer - EOF_FLG"]
pub type EOF_FLG_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_SR_SPEC, O>;
#[doc = "Field `CAP_STA` reader - CAP_STA"]
pub type CAP_STA_R = crate::BitReader;
#[doc = "Field `CAP_STA` writer - CAP_STA"]
pub type CAP_STA_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_SR_SPEC, O>;
#[doc = "Field `CAP_STS` reader - CAP_STS"]
pub type CAP_STS_R = crate::BitReader;
#[doc = "Field `CAP_STS` writer - CAP_STS"]
pub type CAP_STS_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_SR_SPEC, O>;
#[doc = "Field `BAD_FRM` reader - BAD_FRM"]
pub type BAD_FRM_R = crate::BitReader;
#[doc = "Field `BAD_FRM` writer - BAD_FRM"]
pub type BAD_FRM_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_SR_SPEC, O>;
#[doc = "Field `FIFO_OVR` reader - FIFO_OVR"]
pub type FIFO_OVR_R = crate::BitReader;
#[doc = "Field `FIFO_OVR` writer - FIFO_OVR"]
pub type FIFO_OVR_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_SR_SPEC, O>;
#[doc = "Field `FIFO_EMP` reader - FIFO_EMP"]
pub type FIFO_EMP_R = crate::BitReader;
#[doc = "Field `FIFO_EMP` writer - FIFO_EMP"]
pub type FIFO_EMP_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_SR_SPEC, O>;
#[doc = "Field `FIFO_FUL` reader - FIFO_FUL"]
pub type FIFO_FUL_R = crate::BitReader;
#[doc = "Field `FIFO_FUL` writer - FIFO_FUL"]
pub type FIFO_FUL_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_SR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - SOF_FLG"]
    #[inline(always)]
    pub fn sof_flg(&self) -> SOF_FLG_R {
        SOF_FLG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EOF_FLG"]
    #[inline(always)]
    pub fn eof_flg(&self) -> EOF_FLG_R {
        EOF_FLG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CAP_STA"]
    #[inline(always)]
    pub fn cap_sta(&self) -> CAP_STA_R {
        CAP_STA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CAP_STS"]
    #[inline(always)]
    pub fn cap_sts(&self) -> CAP_STS_R {
        CAP_STS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BAD_FRM"]
    #[inline(always)]
    pub fn bad_frm(&self) -> BAD_FRM_R {
        BAD_FRM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO_OVR"]
    #[inline(always)]
    pub fn fifo_ovr(&self) -> FIFO_OVR_R {
        FIFO_OVR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FIFO_EMP"]
    #[inline(always)]
    pub fn fifo_emp(&self) -> FIFO_EMP_R {
        FIFO_EMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FIFO_FUL"]
    #[inline(always)]
    pub fn fifo_ful(&self) -> FIFO_FUL_R {
        FIFO_FUL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SOF_FLG"]
    #[inline(always)]
    #[must_use]
    pub fn sof_flg(&mut self) -> SOF_FLG_W<0> {
        SOF_FLG_W::new(self)
    }
    #[doc = "Bit 1 - EOF_FLG"]
    #[inline(always)]
    #[must_use]
    pub fn eof_flg(&mut self) -> EOF_FLG_W<1> {
        EOF_FLG_W::new(self)
    }
    #[doc = "Bit 2 - CAP_STA"]
    #[inline(always)]
    #[must_use]
    pub fn cap_sta(&mut self) -> CAP_STA_W<2> {
        CAP_STA_W::new(self)
    }
    #[doc = "Bit 3 - CAP_STS"]
    #[inline(always)]
    #[must_use]
    pub fn cap_sts(&mut self) -> CAP_STS_W<3> {
        CAP_STS_W::new(self)
    }
    #[doc = "Bit 4 - BAD_FRM"]
    #[inline(always)]
    #[must_use]
    pub fn bad_frm(&mut self) -> BAD_FRM_W<4> {
        BAD_FRM_W::new(self)
    }
    #[doc = "Bit 8 - FIFO_OVR"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_ovr(&mut self) -> FIFO_OVR_W<8> {
        FIFO_OVR_W::new(self)
    }
    #[doc = "Bit 9 - FIFO_EMP"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_emp(&mut self) -> FIFO_EMP_W<9> {
        FIFO_EMP_W::new(self)
    }
    #[doc = "Bit 10 - FIFO_FUL"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_ful(&mut self) -> FIFO_FUL_W<10> {
        FIFO_FUL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIF_SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_sr](index.html) module"]
pub struct CSIF_SR_SPEC;
impl crate::RegisterSpec for CSIF_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csif_sr::R](R) reader structure"]
impl crate::Readable for CSIF_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csif_sr::W](W) writer structure"]
impl crate::Writable for CSIF_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSIF_SR to value 0"]
impl crate::Resettable for CSIF_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
