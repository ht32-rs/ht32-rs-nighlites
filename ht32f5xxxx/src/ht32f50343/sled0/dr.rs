#[doc = "Register `DR` reader"]
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR` writer"]
pub struct W(crate::W<DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR_SPEC>;
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
impl From<crate::W<DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CP` reader - CP"]
pub type CP_R = crate::FieldReader;
#[doc = "Field `CP` writer - CP"]
pub type CP_W<'a, const O: u8> = crate::FieldWriter<'a, DR_SPEC, 2, O>;
#[doc = "Field `BRD` reader - BRD"]
pub type BRD_R = crate::FieldReader;
#[doc = "Field `BRD` writer - BRD"]
pub type BRD_W<'a, const O: u8> = crate::FieldWriter<'a, DR_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:1 - CP"]
    #[inline(always)]
    pub fn cp(&self) -> CP_R {
        CP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:12 - BRD"]
    #[inline(always)]
    pub fn brd(&self) -> BRD_R {
        BRD_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CP"]
    #[inline(always)]
    #[must_use]
    pub fn cp(&mut self) -> CP_W<0> {
        CP_W::new(self)
    }
    #[doc = "Bits 8:12 - BRD"]
    #[inline(always)]
    #[must_use]
    pub fn brd(&mut self) -> BRD_W<8> {
        BRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](index.html) module"]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr::R](R) reader structure"]
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr::W](W) writer structure"]
impl crate::Writable for DR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
