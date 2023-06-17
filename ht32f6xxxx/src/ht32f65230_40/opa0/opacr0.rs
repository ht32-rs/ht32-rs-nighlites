#[doc = "Register `OPACR0` reader"]
pub struct R(crate::R<OPACR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPACR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPACR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPACR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPACR0` writer"]
pub struct W(crate::W<OPACR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPACR0_SPEC>;
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
impl From<crate::W<OPACR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPACR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPAnEN` reader - OPAnEN"]
pub type OPAN_EN_R = crate::BitReader;
#[doc = "Field `OPAnEN` writer - OPAnEN"]
pub type OPAN_EN_W<'a, const O: u8> = crate::BitWriter<'a, OPACR0_SPEC, O>;
#[doc = "Field `PROTECT` reader - PROTECT"]
pub type PROTECT_R = crate::FieldReader<u16>;
#[doc = "Field `PROTECT` writer - PROTECT"]
pub type PROTECT_W<'a, const O: u8> = crate::FieldWriter<'a, OPACR0_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - OPAnEN"]
    #[inline(always)]
    pub fn opan_en(&self) -> OPAN_EN_R {
        OPAN_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - PROTECT"]
    #[inline(always)]
    pub fn protect(&self) -> PROTECT_R {
        PROTECT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - OPAnEN"]
    #[inline(always)]
    #[must_use]
    pub fn opan_en(&mut self) -> OPAN_EN_W<0> {
        OPAN_EN_W::new(self)
    }
    #[doc = "Bits 16:31 - PROTECT"]
    #[inline(always)]
    #[must_use]
    pub fn protect(&mut self) -> PROTECT_W<16> {
        PROTECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPACR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr0](index.html) module"]
pub struct OPACR0_SPEC;
impl crate::RegisterSpec for OPACR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opacr0::R](R) reader structure"]
impl crate::Readable for OPACR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opacr0::W](W) writer structure"]
impl crate::Writable for OPACR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPACR0 to value 0"]
impl crate::Resettable for OPACR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
