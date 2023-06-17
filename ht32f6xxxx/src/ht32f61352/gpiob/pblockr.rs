#[doc = "Register `PBLOCKR` reader"]
pub struct R(crate::R<PBLOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBLOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBLOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBLOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBLOCKR` writer"]
pub struct W(crate::W<PBLOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBLOCKR_SPEC>;
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
impl From<crate::W<PBLOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBLOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBLOCK` reader - PBLOCK"]
pub type PBLOCK_R = crate::FieldReader<u16>;
#[doc = "Field `PBLOCK` writer - PBLOCK"]
pub type PBLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, PBLOCKR_SPEC, 16, O, u16>;
#[doc = "Field `PBLKEY` reader - PBLKEY"]
pub type PBLKEY_R = crate::FieldReader<u16>;
#[doc = "Field `PBLKEY` writer - PBLKEY"]
pub type PBLKEY_W<'a, const O: u8> = crate::FieldWriter<'a, PBLOCKR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PBLOCK"]
    #[inline(always)]
    pub fn pblock(&self) -> PBLOCK_R {
        PBLOCK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PBLKEY"]
    #[inline(always)]
    pub fn pblkey(&self) -> PBLKEY_R {
        PBLKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PBLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn pblock(&mut self) -> PBLOCK_W<0> {
        PBLOCK_W::new(self)
    }
    #[doc = "Bits 16:31 - PBLKEY"]
    #[inline(always)]
    #[must_use]
    pub fn pblkey(&mut self) -> PBLKEY_W<16> {
        PBLKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBLOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pblockr](index.html) module"]
pub struct PBLOCKR_SPEC;
impl crate::RegisterSpec for PBLOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pblockr::R](R) reader structure"]
impl crate::Readable for PBLOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pblockr::W](W) writer structure"]
impl crate::Writable for PBLOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBLOCKR to value 0"]
impl crate::Resettable for PBLOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
