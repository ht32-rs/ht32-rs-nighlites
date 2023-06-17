#[doc = "Register `CKCU_CKST` reader"]
pub struct R(crate::R<CKCU_CKST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_CKST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_CKST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_CKST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_CKST` writer"]
pub struct W(crate::W<CKCU_CKST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_CKST_SPEC>;
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
impl From<crate::W<CKCU_CKST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_CKST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKSWST` reader - CKSWST"]
pub type CKSWST_R = crate::FieldReader;
#[doc = "Field `CKSWST` writer - CKSWST"]
pub type CKSWST_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_CKST_SPEC, 3, O>;
#[doc = "Field `PLLST` reader - PLLST"]
pub type PLLST_R = crate::FieldReader;
#[doc = "Field `PLLST` writer - PLLST"]
pub type PLLST_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_CKST_SPEC, 4, O>;
#[doc = "Field `HSEST` reader - HSEST"]
pub type HSEST_R = crate::FieldReader;
#[doc = "Field `HSEST` writer - HSEST"]
pub type HSEST_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_CKST_SPEC, 3, O>;
#[doc = "Field `HSIST` reader - HSIST"]
pub type HSIST_R = crate::FieldReader;
#[doc = "Field `HSIST` writer - HSIST"]
pub type HSIST_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_CKST_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:2 - CKSWST"]
    #[inline(always)]
    pub fn ckswst(&self) -> CKSWST_R {
        CKSWST_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - PLLST"]
    #[inline(always)]
    pub fn pllst(&self) -> PLLST_R {
        PLLST_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - HSEST"]
    #[inline(always)]
    pub fn hsest(&self) -> HSEST_R {
        HSEST_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:27 - HSIST"]
    #[inline(always)]
    pub fn hsist(&self) -> HSIST_R {
        HSIST_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CKSWST"]
    #[inline(always)]
    #[must_use]
    pub fn ckswst(&mut self) -> CKSWST_W<0> {
        CKSWST_W::new(self)
    }
    #[doc = "Bits 8:11 - PLLST"]
    #[inline(always)]
    #[must_use]
    pub fn pllst(&mut self) -> PLLST_W<8> {
        PLLST_W::new(self)
    }
    #[doc = "Bits 16:18 - HSEST"]
    #[inline(always)]
    #[must_use]
    pub fn hsest(&mut self) -> HSEST_W<16> {
        HSEST_W::new(self)
    }
    #[doc = "Bits 24:27 - HSIST"]
    #[inline(always)]
    #[must_use]
    pub fn hsist(&mut self) -> HSIST_W<24> {
        HSIST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_CKST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_ckst](index.html) module"]
pub struct CKCU_CKST_SPEC;
impl crate::RegisterSpec for CKCU_CKST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_ckst::R](R) reader structure"]
impl crate::Readable for CKCU_CKST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_ckst::W](W) writer structure"]
impl crate::Writable for CKCU_CKST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_CKST to value 0"]
impl crate::Resettable for CKCU_CKST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
