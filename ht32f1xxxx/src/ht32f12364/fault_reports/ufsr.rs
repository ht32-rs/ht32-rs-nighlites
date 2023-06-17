#[doc = "Register `UFSR` reader"]
pub struct R(crate::R<UFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UFSR` writer"]
pub struct W(crate::W<UFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UFSR_SPEC>;
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
impl From<crate::W<UFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNDEFINSTR` reader - UNDEFINSTR"]
pub type UNDEFINSTR_R = crate::BitReader;
#[doc = "Field `UNDEFINSTR` writer - UNDEFINSTR"]
pub type UNDEFINSTR_W<'a, const O: u8> = crate::BitWriter<'a, UFSR_SPEC, O>;
#[doc = "Field `INVSTATE` reader - INVSTATE"]
pub type INVSTATE_R = crate::BitReader;
#[doc = "Field `INVSTATE` writer - INVSTATE"]
pub type INVSTATE_W<'a, const O: u8> = crate::BitWriter<'a, UFSR_SPEC, O>;
#[doc = "Field `INVPC` reader - INVPC"]
pub type INVPC_R = crate::BitReader;
#[doc = "Field `INVPC` writer - INVPC"]
pub type INVPC_W<'a, const O: u8> = crate::BitWriter<'a, UFSR_SPEC, O>;
#[doc = "Field `NOCP` reader - NOCP"]
pub type NOCP_R = crate::BitReader;
#[doc = "Field `NOCP` writer - NOCP"]
pub type NOCP_W<'a, const O: u8> = crate::BitWriter<'a, UFSR_SPEC, O>;
#[doc = "Field `UNALIGNED` reader - UNALIGNED"]
pub type UNALIGNED_R = crate::BitReader;
#[doc = "Field `UNALIGNED` writer - UNALIGNED"]
pub type UNALIGNED_W<'a, const O: u8> = crate::BitWriter<'a, UFSR_SPEC, O>;
#[doc = "Field `DIVBYZERO` reader - DIVBYZERO"]
pub type DIVBYZERO_R = crate::BitReader;
#[doc = "Field `DIVBYZERO` writer - DIVBYZERO"]
pub type DIVBYZERO_W<'a, const O: u8> = crate::BitWriter<'a, UFSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - UNDEFINSTR"]
    #[inline(always)]
    pub fn undefinstr(&self) -> UNDEFINSTR_R {
        UNDEFINSTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - INVSTATE"]
    #[inline(always)]
    pub fn invstate(&self) -> INVSTATE_R {
        INVSTATE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - INVPC"]
    #[inline(always)]
    pub fn invpc(&self) -> INVPC_R {
        INVPC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NOCP"]
    #[inline(always)]
    pub fn nocp(&self) -> NOCP_R {
        NOCP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - UNALIGNED"]
    #[inline(always)]
    pub fn unaligned(&self) -> UNALIGNED_R {
        UNALIGNED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DIVBYZERO"]
    #[inline(always)]
    pub fn divbyzero(&self) -> DIVBYZERO_R {
        DIVBYZERO_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UNDEFINSTR"]
    #[inline(always)]
    #[must_use]
    pub fn undefinstr(&mut self) -> UNDEFINSTR_W<0> {
        UNDEFINSTR_W::new(self)
    }
    #[doc = "Bit 1 - INVSTATE"]
    #[inline(always)]
    #[must_use]
    pub fn invstate(&mut self) -> INVSTATE_W<1> {
        INVSTATE_W::new(self)
    }
    #[doc = "Bit 2 - INVPC"]
    #[inline(always)]
    #[must_use]
    pub fn invpc(&mut self) -> INVPC_W<2> {
        INVPC_W::new(self)
    }
    #[doc = "Bit 3 - NOCP"]
    #[inline(always)]
    #[must_use]
    pub fn nocp(&mut self) -> NOCP_W<3> {
        NOCP_W::new(self)
    }
    #[doc = "Bit 8 - UNALIGNED"]
    #[inline(always)]
    #[must_use]
    pub fn unaligned(&mut self) -> UNALIGNED_W<8> {
        UNALIGNED_W::new(self)
    }
    #[doc = "Bit 9 - DIVBYZERO"]
    #[inline(always)]
    #[must_use]
    pub fn divbyzero(&mut self) -> DIVBYZERO_W<9> {
        DIVBYZERO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UFSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ufsr](index.html) module"]
pub struct UFSR_SPEC;
impl crate::RegisterSpec for UFSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ufsr::R](R) reader structure"]
impl crate::Readable for UFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ufsr::W](W) writer structure"]
impl crate::Writable for UFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UFSR to value 0"]
impl crate::Resettable for UFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
