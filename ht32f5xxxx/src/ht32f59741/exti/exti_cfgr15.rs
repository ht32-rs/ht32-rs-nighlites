#[doc = "Register `EXTI_CFGR15` reader"]
pub struct R(crate::R<EXTI_CFGR15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_CFGR15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_CFGR15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_CFGR15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_CFGR15` writer"]
pub struct W(crate::W<EXTI_CFGR15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_CFGR15_SPEC>;
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
impl From<crate::W<EXTI_CFGR15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_CFGR15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBCNT` reader - DBCNT"]
pub type DBCNT_R = crate::FieldReader<u16>;
#[doc = "Field `DBCNT` writer - DBCNT"]
pub type DBCNT_W<'a, const O: u8> = crate::FieldWriter<'a, EXTI_CFGR15_SPEC, 16, O, u16>;
#[doc = "Field `SRCTYPE` reader - SRCTYPE"]
pub type SRCTYPE_R = crate::FieldReader;
#[doc = "Field `SRCTYPE` writer - SRCTYPE"]
pub type SRCTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, EXTI_CFGR15_SPEC, 3, O>;
#[doc = "Field `DBEN` reader - DBEN"]
pub type DBEN_R = crate::BitReader;
#[doc = "Field `DBEN` writer - DBEN"]
pub type DBEN_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_CFGR15_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - DBCNT"]
    #[inline(always)]
    pub fn dbcnt(&self) -> DBCNT_R {
        DBCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:30 - SRCTYPE"]
    #[inline(always)]
    pub fn srctype(&self) -> SRCTYPE_R {
        SRCTYPE_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - DBEN"]
    #[inline(always)]
    pub fn dben(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - DBCNT"]
    #[inline(always)]
    #[must_use]
    pub fn dbcnt(&mut self) -> DBCNT_W<0> {
        DBCNT_W::new(self)
    }
    #[doc = "Bits 28:30 - SRCTYPE"]
    #[inline(always)]
    #[must_use]
    pub fn srctype(&mut self) -> SRCTYPE_W<28> {
        SRCTYPE_W::new(self)
    }
    #[doc = "Bit 31 - DBEN"]
    #[inline(always)]
    #[must_use]
    pub fn dben(&mut self) -> DBEN_W<31> {
        DBEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI_CFGR15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr15](index.html) module"]
pub struct EXTI_CFGR15_SPEC;
impl crate::RegisterSpec for EXTI_CFGR15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_cfgr15::R](R) reader structure"]
impl crate::Readable for EXTI_CFGR15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_cfgr15::W](W) writer structure"]
impl crate::Writable for EXTI_CFGR15_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTI_CFGR15 to value 0"]
impl crate::Resettable for EXTI_CFGR15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
