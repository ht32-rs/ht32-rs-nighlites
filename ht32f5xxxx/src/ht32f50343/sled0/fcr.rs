#[doc = "Register `FCR` reader"]
pub struct R(crate::R<FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRST` reader - FRST"]
pub type FRST_R = crate::BitReader;
#[doc = "Field `FRST` writer - FRST"]
pub type FRST_W<'a, const O: u8> = crate::BitWriter<'a, FCR_SPEC, O>;
#[doc = "Field `FTLS` reader - FTLS"]
pub type FTLS_R = crate::FieldReader;
#[doc = "Field `FTLS` writer - FTLS"]
pub type FTLS_W<'a, const O: u8> = crate::FieldWriter<'a, FCR_SPEC, 2, O>;
#[doc = "Field `FCNT` reader - FCNT"]
pub type FCNT_R = crate::FieldReader;
#[doc = "Field `FCNT` writer - FCNT"]
pub type FCNT_W<'a, const O: u8> = crate::FieldWriter<'a, FCR_SPEC, 3, O>;
impl R {
    #[doc = "Bit 0 - FRST"]
    #[inline(always)]
    pub fn frst(&self) -> FRST_R {
        FRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 6:7 - FTLS"]
    #[inline(always)]
    pub fn ftls(&self) -> FTLS_R {
        FTLS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 24:26 - FCNT"]
    #[inline(always)]
    pub fn fcnt(&self) -> FCNT_R {
        FCNT_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FRST"]
    #[inline(always)]
    #[must_use]
    pub fn frst(&mut self) -> FRST_W<0> {
        FRST_W::new(self)
    }
    #[doc = "Bits 6:7 - FTLS"]
    #[inline(always)]
    #[must_use]
    pub fn ftls(&mut self) -> FTLS_W<6> {
        FTLS_W::new(self)
    }
    #[doc = "Bits 24:26 - FCNT"]
    #[inline(always)]
    #[must_use]
    pub fn fcnt(&mut self) -> FCNT_W<24> {
        FCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcr::R](R) reader structure"]
impl crate::Readable for FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
