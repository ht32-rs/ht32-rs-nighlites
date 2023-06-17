#[doc = "Register `CKCU_APBPCSR2` reader"]
pub struct R(crate::R<CKCU_APBPCSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_APBPCSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_APBPCSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_APBPCSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_APBPCSR2` writer"]
pub struct W(crate::W<CKCU_APBPCSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_APBPCSR2_SPEC>;
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
impl From<crate::W<CKCU_APBPCSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_APBPCSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFEPCLK` reader - AFEPCLK"]
pub type AFEPCLK_R = crate::FieldReader;
#[doc = "Field `AFEPCLK` writer - AFEPCLK"]
pub type AFEPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR2_SPEC, 2, O>;
#[doc = "Field `DACPCLK` reader - DACPCLK"]
pub type DACPCLK_R = crate::FieldReader;
#[doc = "Field `DACPCLK` writer - DACPCLK"]
pub type DACPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR2_SPEC, 2, O>;
#[doc = "Field `MIDIPCLK` reader - MIDIPCLK"]
pub type MIDIPCLK_R = crate::FieldReader;
#[doc = "Field `MIDIPCLK` writer - MIDIPCLK"]
pub type MIDIPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_APBPCSR2_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - AFEPCLK"]
    #[inline(always)]
    pub fn afepclk(&self) -> AFEPCLK_R {
        AFEPCLK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DACPCLK"]
    #[inline(always)]
    pub fn dacpclk(&self) -> DACPCLK_R {
        DACPCLK_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - MIDIPCLK"]
    #[inline(always)]
    pub fn midipclk(&self) -> MIDIPCLK_R {
        MIDIPCLK_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AFEPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn afepclk(&mut self) -> AFEPCLK_W<0> {
        AFEPCLK_W::new(self)
    }
    #[doc = "Bits 2:3 - DACPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn dacpclk(&mut self) -> DACPCLK_W<2> {
        DACPCLK_W::new(self)
    }
    #[doc = "Bits 8:9 - MIDIPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn midipclk(&mut self) -> MIDIPCLK_W<8> {
        MIDIPCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_APBPCSR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_apbpcsr2](index.html) module"]
pub struct CKCU_APBPCSR2_SPEC;
impl crate::RegisterSpec for CKCU_APBPCSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_apbpcsr2::R](R) reader structure"]
impl crate::Readable for CKCU_APBPCSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_apbpcsr2::W](W) writer structure"]
impl crate::Writable for CKCU_APBPCSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_APBPCSR2 to value 0"]
impl crate::Resettable for CKCU_APBPCSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
