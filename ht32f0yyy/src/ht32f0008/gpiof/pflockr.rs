#[doc = "Register `PFLOCKR` reader"]
pub struct R(crate::R<PFLOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFLOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFLOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFLOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFLOCKR` writer"]
pub struct W(crate::W<PFLOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFLOCKR_SPEC>;
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
impl From<crate::W<PFLOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFLOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFLOCK0` reader - PFLOCK0"]
pub type PFLOCK0_R = crate::BitReader;
#[doc = "Field `PFLOCK0` writer - PFLOCK0"]
pub type PFLOCK0_W<'a, const O: u8> = crate::BitWriter<'a, PFLOCKR_SPEC, O>;
#[doc = "Field `PFLOCK1` reader - PFLOCK1"]
pub type PFLOCK1_R = crate::BitReader;
#[doc = "Field `PFLOCK1` writer - PFLOCK1"]
pub type PFLOCK1_W<'a, const O: u8> = crate::BitWriter<'a, PFLOCKR_SPEC, O>;
#[doc = "Field `PFLKEY` reader - PFLKEY"]
pub type PFLKEY_R = crate::FieldReader<u16>;
#[doc = "Field `PFLKEY` writer - PFLKEY"]
pub type PFLKEY_W<'a, const O: u8> = crate::FieldWriter<'a, PFLOCKR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - PFLOCK0"]
    #[inline(always)]
    pub fn pflock0(&self) -> PFLOCK0_R {
        PFLOCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PFLOCK1"]
    #[inline(always)]
    pub fn pflock1(&self) -> PFLOCK1_R {
        PFLOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:31 - PFLKEY"]
    #[inline(always)]
    pub fn pflkey(&self) -> PFLKEY_R {
        PFLKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - PFLOCK0"]
    #[inline(always)]
    #[must_use]
    pub fn pflock0(&mut self) -> PFLOCK0_W<0> {
        PFLOCK0_W::new(self)
    }
    #[doc = "Bit 1 - PFLOCK1"]
    #[inline(always)]
    #[must_use]
    pub fn pflock1(&mut self) -> PFLOCK1_W<1> {
        PFLOCK1_W::new(self)
    }
    #[doc = "Bits 16:31 - PFLKEY"]
    #[inline(always)]
    #[must_use]
    pub fn pflkey(&mut self) -> PFLKEY_W<16> {
        PFLKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PFLOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pflockr](index.html) module"]
pub struct PFLOCKR_SPEC;
impl crate::RegisterSpec for PFLOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pflockr::R](R) reader structure"]
impl crate::Readable for PFLOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pflockr::W](W) writer structure"]
impl crate::Writable for PFLOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFLOCKR to value 0"]
impl crate::Resettable for PFLOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
