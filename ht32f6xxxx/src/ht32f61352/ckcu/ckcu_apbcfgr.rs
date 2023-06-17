#[doc = "Register `CKCU_APBCFGR` reader"]
pub struct R(crate::R<CKCU_APBCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_APBCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_APBCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_APBCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_APBCFGR` writer"]
pub struct W(crate::W<CKCU_APBCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_APBCFGR_SPEC>;
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
impl From<crate::W<CKCU_APBCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_APBCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCDIV` reader - ADCDIV"]
pub type ADCDIV_R = crate::FieldReader;
#[doc = "Field `ADCDIV` writer - ADCDIV"]
pub type ADCDIV_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBCFGR_SPEC, 3, O>;
#[doc = "Field `MIDIDIV` reader - MIDIDIV"]
pub type MIDIDIV_R = crate::FieldReader;
#[doc = "Field `MIDIDIV` writer - MIDIDIV"]
pub type MIDIDIV_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBCFGR_SPEC, 3, O>;
impl R {
    #[doc = "Bits 16:18 - ADCDIV"]
    #[inline(always)]
    pub fn adcdiv(&self) -> ADCDIV_R {
        ADCDIV_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - MIDIDIV"]
    #[inline(always)]
    pub fn mididiv(&self) -> MIDIDIV_R {
        MIDIDIV_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - ADCDIV"]
    #[inline(always)]
    #[must_use]
    pub fn adcdiv(&mut self) -> ADCDIV_W<16> {
        ADCDIV_W::new(self)
    }
    #[doc = "Bits 24:26 - MIDIDIV"]
    #[inline(always)]
    #[must_use]
    pub fn mididiv(&mut self) -> MIDIDIV_W<24> {
        MIDIDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_APBCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_apbcfgr](index.html) module"]
pub struct CKCU_APBCFGR_SPEC;
impl crate::RegisterSpec for CKCU_APBCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_apbcfgr::R](R) reader structure"]
impl crate::Readable for CKCU_APBCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_apbcfgr::W](W) writer structure"]
impl crate::Writable for CKCU_APBCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_APBCFGR to value 0"]
impl crate::Resettable for CKCU_APBCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
