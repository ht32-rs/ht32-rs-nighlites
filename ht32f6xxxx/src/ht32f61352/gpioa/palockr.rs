#[doc = "Register `PALOCKR` reader"]
pub struct R(crate::R<PALOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PALOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PALOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PALOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PALOCKR` writer"]
pub struct W(crate::W<PALOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PALOCKR_SPEC>;
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
impl From<crate::W<PALOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PALOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PALOCK` reader - PALOCK"]
pub type PALOCK_R = crate::FieldReader<u16>;
#[doc = "Field `PALOCK` writer - PALOCK"]
pub type PALOCK_W<'a, const O: u8> = crate::FieldWriter<'a, PALOCKR_SPEC, 16, O, u16>;
#[doc = "Field `PALKEY` reader - PALKEY"]
pub type PALKEY_R = crate::FieldReader<u16>;
#[doc = "Field `PALKEY` writer - PALKEY"]
pub type PALKEY_W<'a, const O: u8> = crate::FieldWriter<'a, PALOCKR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PALOCK"]
    #[inline(always)]
    pub fn palock(&self) -> PALOCK_R {
        PALOCK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PALKEY"]
    #[inline(always)]
    pub fn palkey(&self) -> PALKEY_R {
        PALKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PALOCK"]
    #[inline(always)]
    #[must_use]
    pub fn palock(&mut self) -> PALOCK_W<0> {
        PALOCK_W::new(self)
    }
    #[doc = "Bits 16:31 - PALKEY"]
    #[inline(always)]
    #[must_use]
    pub fn palkey(&mut self) -> PALKEY_W<16> {
        PALKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PALOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [palockr](index.html) module"]
pub struct PALOCKR_SPEC;
impl crate::RegisterSpec for PALOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [palockr::R](R) reader structure"]
impl crate::Readable for PALOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [palockr::W](W) writer structure"]
impl crate::Writable for PALOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PALOCKR to value 0"]
impl crate::Resettable for PALOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
