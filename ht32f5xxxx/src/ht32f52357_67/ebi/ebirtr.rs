#[doc = "Register `EBIRTR` reader"]
pub struct R(crate::R<EBIRTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBIRTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EBIRTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EBIRTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EBIRTR` writer"]
pub struct W(crate::W<EBIRTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBIRTR_SPEC>;
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
impl From<crate::W<EBIRTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EBIRTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDSETUP` reader - RDSETUP"]
pub type RDSETUP_R = crate::FieldReader;
#[doc = "Field `RDSETUP` writer - RDSETUP"]
pub type RDSETUP_W<'a, const O: u8> = crate::FieldWriter<'a, EBIRTR_SPEC, 3, O>;
#[doc = "Field `RDSTRB` reader - RDSTRB"]
pub type RDSTRB_R = crate::FieldReader;
#[doc = "Field `RDSTRB` writer - RDSTRB"]
pub type RDSTRB_W<'a, const O: u8> = crate::FieldWriter<'a, EBIRTR_SPEC, 5, O>;
#[doc = "Field `RDHOLD` reader - RDHOLD"]
pub type RDHOLD_R = crate::FieldReader;
#[doc = "Field `RDHOLD` writer - RDHOLD"]
pub type RDHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, EBIRTR_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:2 - RDSETUP"]
    #[inline(always)]
    pub fn rdsetup(&self) -> RDSETUP_R {
        RDSETUP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - RDSTRB"]
    #[inline(always)]
    pub fn rdstrb(&self) -> RDSTRB_R {
        RDSTRB_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - RDHOLD"]
    #[inline(always)]
    pub fn rdhold(&self) -> RDHOLD_R {
        RDHOLD_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - RDSETUP"]
    #[inline(always)]
    #[must_use]
    pub fn rdsetup(&mut self) -> RDSETUP_W<0> {
        RDSETUP_W::new(self)
    }
    #[doc = "Bits 8:12 - RDSTRB"]
    #[inline(always)]
    #[must_use]
    pub fn rdstrb(&mut self) -> RDSTRB_W<8> {
        RDSTRB_W::new(self)
    }
    #[doc = "Bits 16:18 - RDHOLD"]
    #[inline(always)]
    #[must_use]
    pub fn rdhold(&mut self) -> RDHOLD_W<16> {
        RDHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBIRTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebirtr](index.html) module"]
pub struct EBIRTR_SPEC;
impl crate::RegisterSpec for EBIRTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ebirtr::R](R) reader structure"]
impl crate::Readable for EBIRTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ebirtr::W](W) writer structure"]
impl crate::Writable for EBIRTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EBIRTR to value 0"]
impl crate::Resettable for EBIRTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
