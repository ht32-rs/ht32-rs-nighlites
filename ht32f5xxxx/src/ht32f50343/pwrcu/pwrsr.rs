#[doc = "Register `PWRSR` reader"]
pub struct R(crate::R<PWRSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRSR` writer"]
pub struct W(crate::W<PWRSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRSR_SPEC>;
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
impl From<crate::W<PWRSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORF` reader - PORF"]
pub type PORF_R = crate::BitReader;
#[doc = "Field `PORF` writer - PORF"]
pub type PORF_W<'a, const O: u8> = crate::BitWriter<'a, PWRSR_SPEC, O>;
#[doc = "Field `WUPF0` reader - WUPF0"]
pub type WUPF0_R = crate::BitReader;
#[doc = "Field `WUPF0` writer - WUPF0"]
pub type WUPF0_W<'a, const O: u8> = crate::BitWriter<'a, PWRSR_SPEC, O>;
#[doc = "Field `WUPF1` reader - WUPF1"]
pub type WUPF1_R = crate::BitReader;
#[doc = "Field `WUPF1` writer - WUPF1"]
pub type WUPF1_W<'a, const O: u8> = crate::BitWriter<'a, PWRSR_SPEC, O>;
impl R {
    #[doc = "Bit 4 - PORF"]
    #[inline(always)]
    pub fn porf(&self) -> PORF_R {
        PORF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - WUPF0"]
    #[inline(always)]
    pub fn wupf0(&self) -> WUPF0_R {
        WUPF0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WUPF1"]
    #[inline(always)]
    pub fn wupf1(&self) -> WUPF1_R {
        WUPF1_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - PORF"]
    #[inline(always)]
    #[must_use]
    pub fn porf(&mut self) -> PORF_W<4> {
        PORF_W::new(self)
    }
    #[doc = "Bit 8 - WUPF0"]
    #[inline(always)]
    #[must_use]
    pub fn wupf0(&mut self) -> WUPF0_W<8> {
        WUPF0_W::new(self)
    }
    #[doc = "Bit 9 - WUPF1"]
    #[inline(always)]
    #[must_use]
    pub fn wupf1(&mut self) -> WUPF1_W<9> {
        WUPF1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWRSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrsr](index.html) module"]
pub struct PWRSR_SPEC;
impl crate::RegisterSpec for PWRSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrsr::R](R) reader structure"]
impl crate::Readable for PWRSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrsr::W](W) writer structure"]
impl crate::Writable for PWRSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRSR to value 0"]
impl crate::Resettable for PWRSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
