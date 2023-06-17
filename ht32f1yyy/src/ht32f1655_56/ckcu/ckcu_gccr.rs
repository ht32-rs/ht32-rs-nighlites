#[doc = "Register `CKCU_GCCR` reader"]
pub struct R(crate::R<CKCU_GCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_GCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_GCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_GCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_GCCR` writer"]
pub struct W(crate::W<CKCU_GCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_GCCR_SPEC>;
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
impl From<crate::W<CKCU_GCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_GCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW` reader - SW"]
pub type SW_R = crate::FieldReader;
#[doc = "Field `SW` writer - SW"]
pub type SW_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_GCCR_SPEC, 2, O>;
#[doc = "Field `PLLEN` reader - PLLEN"]
pub type PLLEN_R = crate::BitReader;
#[doc = "Field `PLLEN` writer - PLLEN"]
pub type PLLEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_GCCR_SPEC, O>;
#[doc = "Field `HSEEN` reader - HSEEN"]
pub type HSEEN_R = crate::BitReader;
#[doc = "Field `HSEEN` writer - HSEEN"]
pub type HSEEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_GCCR_SPEC, O>;
#[doc = "Field `HSIEN` reader - HSIEN"]
pub type HSIEN_R = crate::BitReader;
#[doc = "Field `HSIEN` writer - HSIEN"]
pub type HSIEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_GCCR_SPEC, O>;
#[doc = "Field `CKMEN` reader - CKMEN"]
pub type CKMEN_R = crate::BitReader;
#[doc = "Field `CKMEN` writer - CKMEN"]
pub type CKMEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_GCCR_SPEC, O>;
#[doc = "Field `PSRCEN` reader - PSRCEN"]
pub type PSRCEN_R = crate::BitReader;
#[doc = "Field `PSRCEN` writer - PSRCEN"]
pub type PSRCEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_GCCR_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - SW"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 9 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSEEN"]
    #[inline(always)]
    pub fn hseen(&self) -> HSEEN_R {
        HSEEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSIEN"]
    #[inline(always)]
    pub fn hsien(&self) -> HSIEN_R {
        HSIEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - CKMEN"]
    #[inline(always)]
    pub fn ckmen(&self) -> CKMEN_R {
        CKMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PSRCEN"]
    #[inline(always)]
    pub fn psrcen(&self) -> PSRCEN_R {
        PSRCEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SW"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    #[doc = "Bit 9 - PLLEN"]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PLLEN_W<9> {
        PLLEN_W::new(self)
    }
    #[doc = "Bit 10 - HSEEN"]
    #[inline(always)]
    #[must_use]
    pub fn hseen(&mut self) -> HSEEN_W<10> {
        HSEEN_W::new(self)
    }
    #[doc = "Bit 11 - HSIEN"]
    #[inline(always)]
    #[must_use]
    pub fn hsien(&mut self) -> HSIEN_W<11> {
        HSIEN_W::new(self)
    }
    #[doc = "Bit 16 - CKMEN"]
    #[inline(always)]
    #[must_use]
    pub fn ckmen(&mut self) -> CKMEN_W<16> {
        CKMEN_W::new(self)
    }
    #[doc = "Bit 17 - PSRCEN"]
    #[inline(always)]
    #[must_use]
    pub fn psrcen(&mut self) -> PSRCEN_W<17> {
        PSRCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_GCCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_gccr](index.html) module"]
pub struct CKCU_GCCR_SPEC;
impl crate::RegisterSpec for CKCU_GCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_gccr::R](R) reader structure"]
impl crate::Readable for CKCU_GCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_gccr::W](W) writer structure"]
impl crate::Writable for CKCU_GCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_GCCR to value 0"]
impl crate::Resettable for CKCU_GCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
