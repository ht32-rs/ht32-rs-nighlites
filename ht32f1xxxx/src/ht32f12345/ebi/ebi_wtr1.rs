#[doc = "Register `EBI_WTR1` reader"]
pub struct R(crate::R<EBI_WTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBI_WTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EBI_WTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EBI_WTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EBI_WTR1` writer"]
pub struct W(crate::W<EBI_WTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBI_WTR1_SPEC>;
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
impl From<crate::W<EBI_WTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EBI_WTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRSETUP` reader - WRSETUP"]
pub type WRSETUP_R = crate::FieldReader;
#[doc = "Field `WRSETUP` writer - WRSETUP"]
pub type WRSETUP_W<'a, const O: u8> = crate::FieldWriter<'a, EBI_WTR1_SPEC, 4, O>;
#[doc = "Field `WRSTRB` reader - WRSTRB"]
pub type WRSTRB_R = crate::FieldReader;
#[doc = "Field `WRSTRB` writer - WRSTRB"]
pub type WRSTRB_W<'a, const O: u8> = crate::FieldWriter<'a, EBI_WTR1_SPEC, 6, O>;
#[doc = "Field `WRHOLD` reader - WRHOLD"]
pub type WRHOLD_R = crate::FieldReader;
#[doc = "Field `WRHOLD` writer - WRHOLD"]
pub type WRHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, EBI_WTR1_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - WRSETUP"]
    #[inline(always)]
    pub fn wrsetup(&self) -> WRSETUP_R {
        WRSETUP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - WRSTRB"]
    #[inline(always)]
    pub fn wrstrb(&self) -> WRSTRB_R {
        WRSTRB_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - WRHOLD"]
    #[inline(always)]
    pub fn wrhold(&self) -> WRHOLD_R {
        WRHOLD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - WRSETUP"]
    #[inline(always)]
    #[must_use]
    pub fn wrsetup(&mut self) -> WRSETUP_W<0> {
        WRSETUP_W::new(self)
    }
    #[doc = "Bits 8:13 - WRSTRB"]
    #[inline(always)]
    #[must_use]
    pub fn wrstrb(&mut self) -> WRSTRB_W<8> {
        WRSTRB_W::new(self)
    }
    #[doc = "Bits 16:19 - WRHOLD"]
    #[inline(always)]
    #[must_use]
    pub fn wrhold(&mut self) -> WRHOLD_W<16> {
        WRHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBI_WTR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_wtr1](index.html) module"]
pub struct EBI_WTR1_SPEC;
impl crate::RegisterSpec for EBI_WTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ebi_wtr1::R](R) reader structure"]
impl crate::Readable for EBI_WTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ebi_wtr1::W](W) writer structure"]
impl crate::Writable for EBI_WTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EBI_WTR1 to value 0"]
impl crate::Resettable for EBI_WTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
