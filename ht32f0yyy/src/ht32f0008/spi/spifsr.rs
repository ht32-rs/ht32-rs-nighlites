#[doc = "Register `SPIFSR` reader"]
pub struct R(crate::R<SPIFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIFSR` writer"]
pub struct W(crate::W<SPIFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIFSR_SPEC>;
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
impl From<crate::W<SPIFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFS` reader - TXFS"]
pub type TXFS_R = crate::FieldReader;
#[doc = "Field `TXFS` writer - TXFS"]
pub type TXFS_W<'a, const O: u8> = crate::FieldWriter<'a, SPIFSR_SPEC, 4, O>;
#[doc = "Field `RXFS` reader - RXFS"]
pub type RXFS_R = crate::FieldReader;
#[doc = "Field `RXFS` writer - RXFS"]
pub type RXFS_W<'a, const O: u8> = crate::FieldWriter<'a, SPIFSR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - TXFS"]
    #[inline(always)]
    pub fn txfs(&self) -> TXFS_R {
        TXFS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RXFS"]
    #[inline(always)]
    pub fn rxfs(&self) -> RXFS_R {
        RXFS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TXFS"]
    #[inline(always)]
    #[must_use]
    pub fn txfs(&mut self) -> TXFS_W<0> {
        TXFS_W::new(self)
    }
    #[doc = "Bits 4:7 - RXFS"]
    #[inline(always)]
    #[must_use]
    pub fn rxfs(&mut self) -> RXFS_W<4> {
        RXFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIFSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spifsr](index.html) module"]
pub struct SPIFSR_SPEC;
impl crate::RegisterSpec for SPIFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spifsr::R](R) reader structure"]
impl crate::Readable for SPIFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spifsr::W](W) writer structure"]
impl crate::Writable for SPIFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPIFSR to value 0"]
impl crate::Resettable for SPIFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
