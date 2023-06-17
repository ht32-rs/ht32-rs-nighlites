#[doc = "Register `EBIWTR` reader"]
pub struct R(crate::R<EBIWTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBIWTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EBIWTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EBIWTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EBIWTR` writer"]
pub struct W(crate::W<EBIWTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBIWTR_SPEC>;
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
impl From<crate::W<EBIWTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EBIWTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WESETUP` reader - WESETUP"]
pub type WESETUP_R = crate::FieldReader;
#[doc = "Field `WESETUP` writer - WESETUP"]
pub type WESETUP_W<'a, const O: u8> = crate::FieldWriter<'a, EBIWTR_SPEC, 3, O>;
#[doc = "Field `WESTRB` reader - WESTRB"]
pub type WESTRB_R = crate::FieldReader;
#[doc = "Field `WESTRB` writer - WESTRB"]
pub type WESTRB_W<'a, const O: u8> = crate::FieldWriter<'a, EBIWTR_SPEC, 5, O>;
#[doc = "Field `WEHOLD` reader - WEHOLD"]
pub type WEHOLD_R = crate::FieldReader;
#[doc = "Field `WEHOLD` writer - WEHOLD"]
pub type WEHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, EBIWTR_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:2 - WESETUP"]
    #[inline(always)]
    pub fn wesetup(&self) -> WESETUP_R {
        WESETUP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - WESTRB"]
    #[inline(always)]
    pub fn westrb(&self) -> WESTRB_R {
        WESTRB_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - WEHOLD"]
    #[inline(always)]
    pub fn wehold(&self) -> WEHOLD_R {
        WEHOLD_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - WESETUP"]
    #[inline(always)]
    #[must_use]
    pub fn wesetup(&mut self) -> WESETUP_W<0> {
        WESETUP_W::new(self)
    }
    #[doc = "Bits 8:12 - WESTRB"]
    #[inline(always)]
    #[must_use]
    pub fn westrb(&mut self) -> WESTRB_W<8> {
        WESTRB_W::new(self)
    }
    #[doc = "Bits 16:18 - WEHOLD"]
    #[inline(always)]
    #[must_use]
    pub fn wehold(&mut self) -> WEHOLD_W<16> {
        WEHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBIWTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebiwtr](index.html) module"]
pub struct EBIWTR_SPEC;
impl crate::RegisterSpec for EBIWTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ebiwtr::R](R) reader structure"]
impl crate::Readable for EBIWTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ebiwtr::W](W) writer structure"]
impl crate::Writable for EBIWTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EBIWTR to value 0"]
impl crate::Resettable for EBIWTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
