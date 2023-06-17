#[doc = "Register `BFTMSR` reader"]
pub struct R(crate::R<BFTMSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFTMSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFTMSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFTMSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFTMSR` writer"]
pub struct W(crate::W<BFTMSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFTMSR_SPEC>;
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
impl From<crate::W<BFTMSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFTMSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIF` reader - MIF"]
pub type MIF_R = crate::BitReader;
#[doc = "Field `MIF` writer - MIF"]
pub type MIF_W<'a, const O: u8> = crate::BitWriter<'a, BFTMSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - MIF"]
    #[inline(always)]
    pub fn mif(&self) -> MIF_R {
        MIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MIF"]
    #[inline(always)]
    #[must_use]
    pub fn mif(&mut self) -> MIF_W<0> {
        MIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BFTMSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bftmsr](index.html) module"]
pub struct BFTMSR_SPEC;
impl crate::RegisterSpec for BFTMSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bftmsr::R](R) reader structure"]
impl crate::Readable for BFTMSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bftmsr::W](W) writer structure"]
impl crate::Writable for BFTMSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BFTMSR to value 0"]
impl crate::Resettable for BFTMSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
