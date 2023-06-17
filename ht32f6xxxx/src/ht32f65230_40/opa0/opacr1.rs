#[doc = "Register `OPACR1` reader"]
pub struct R(crate::R<OPACR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPACR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPACR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPACR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPACR1` writer"]
pub struct W(crate::W<OPACR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPACR1_SPEC>;
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
impl From<crate::W<OPACR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPACR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPAnEN` reader - OPAnEN"]
pub type OPAN_EN_R = crate::BitReader;
#[doc = "Field `OPAnEN` writer - OPAnEN"]
pub type OPAN_EN_W<'a, const O: u8> = crate::BitWriter<'a, OPACR1_SPEC, O>;
#[doc = "Field `PROTECT` reader - PROTECT"]
pub type PROTECT_R = crate::FieldReader<u16>;
#[doc = "Field `PROTECT` writer - PROTECT"]
pub type PROTECT_W<'a, const O: u8> = crate::FieldWriter<'a, OPACR1_SPEC, 16, O, u16>;
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
#[doc = "OPACR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr1](index.html) module"]
pub struct OPACR1_SPEC;
impl crate::RegisterSpec for OPACR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opacr1::R](R) reader structure"]
impl crate::Readable for OPACR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opacr1::W](W) writer structure"]
impl crate::Writable for OPACR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPACR1 to value 0"]
impl crate::Resettable for OPACR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
