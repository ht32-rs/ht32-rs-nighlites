#[doc = "Register `PFINER` reader"]
pub struct R(crate::R<PFINER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFINER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFINER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFINER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFINER` writer"]
pub struct W(crate::W<PFINER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFINER_SPEC>;
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
impl From<crate::W<PFINER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFINER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFINEN0` reader - PFINEN0"]
pub type PFINEN0_R = crate::BitReader;
#[doc = "Field `PFINEN0` writer - PFINEN0"]
pub type PFINEN0_W<'a, const O: u8> = crate::BitWriter<'a, PFINER_SPEC, O>;
#[doc = "Field `PFINEN1` reader - PFINEN1"]
pub type PFINEN1_R = crate::BitReader;
#[doc = "Field `PFINEN1` writer - PFINEN1"]
pub type PFINEN1_W<'a, const O: u8> = crate::BitWriter<'a, PFINER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PFINEN0"]
    #[inline(always)]
    pub fn pfinen0(&self) -> PFINEN0_R {
        PFINEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PFINEN1"]
    #[inline(always)]
    pub fn pfinen1(&self) -> PFINEN1_R {
        PFINEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PFINEN0"]
    #[inline(always)]
    #[must_use]
    pub fn pfinen0(&mut self) -> PFINEN0_W<0> {
        PFINEN0_W::new(self)
    }
    #[doc = "Bit 1 - PFINEN1"]
    #[inline(always)]
    #[must_use]
    pub fn pfinen1(&mut self) -> PFINEN1_W<1> {
        PFINEN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PFINER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfiner](index.html) module"]
pub struct PFINER_SPEC;
impl crate::RegisterSpec for PFINER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfiner::R](R) reader structure"]
impl crate::Readable for PFINER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfiner::W](W) writer structure"]
impl crate::Writable for PFINER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFINER to value 0"]
impl crate::Resettable for PFINER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
