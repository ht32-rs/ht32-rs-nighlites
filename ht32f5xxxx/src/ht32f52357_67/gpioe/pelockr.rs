#[doc = "Register `PELOCKR` reader"]
pub struct R(crate::R<PELOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PELOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PELOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PELOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PELOCKR` writer"]
pub struct W(crate::W<PELOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PELOCKR_SPEC>;
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
impl From<crate::W<PELOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PELOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PELOCK0` reader - PELOCK0"]
pub type PELOCK0_R = crate::BitReader;
#[doc = "Field `PELOCK0` writer - PELOCK0"]
pub type PELOCK0_W<'a, const O: u8> = crate::BitWriter<'a, PELOCKR_SPEC, O>;
#[doc = "Field `PELOCK1` reader - PELOCK1"]
pub type PELOCK1_R = crate::BitReader;
#[doc = "Field `PELOCK1` writer - PELOCK1"]
pub type PELOCK1_W<'a, const O: u8> = crate::BitWriter<'a, PELOCKR_SPEC, O>;
#[doc = "Field `PELOCK2` reader - PELOCK2"]
pub type PELOCK2_R = crate::BitReader;
#[doc = "Field `PELOCK2` writer - PELOCK2"]
pub type PELOCK2_W<'a, const O: u8> = crate::BitWriter<'a, PELOCKR_SPEC, O>;
#[doc = "Field `PELOCK3` reader - PELOCK3"]
pub type PELOCK3_R = crate::BitReader;
#[doc = "Field `PELOCK3` writer - PELOCK3"]
pub type PELOCK3_W<'a, const O: u8> = crate::BitWriter<'a, PELOCKR_SPEC, O>;
#[doc = "Field `PELKEY` reader - PELKEY"]
pub type PELKEY_R = crate::FieldReader<u16>;
#[doc = "Field `PELKEY` writer - PELKEY"]
pub type PELKEY_W<'a, const O: u8> = crate::FieldWriter<'a, PELOCKR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - PELOCK0"]
    #[inline(always)]
    pub fn pelock0(&self) -> PELOCK0_R {
        PELOCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PELOCK1"]
    #[inline(always)]
    pub fn pelock1(&self) -> PELOCK1_R {
        PELOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PELOCK2"]
    #[inline(always)]
    pub fn pelock2(&self) -> PELOCK2_R {
        PELOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PELOCK3"]
    #[inline(always)]
    pub fn pelock3(&self) -> PELOCK3_R {
        PELOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:31 - PELKEY"]
    #[inline(always)]
    pub fn pelkey(&self) -> PELKEY_R {
        PELKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - PELOCK0"]
    #[inline(always)]
    #[must_use]
    pub fn pelock0(&mut self) -> PELOCK0_W<0> {
        PELOCK0_W::new(self)
    }
    #[doc = "Bit 1 - PELOCK1"]
    #[inline(always)]
    #[must_use]
    pub fn pelock1(&mut self) -> PELOCK1_W<1> {
        PELOCK1_W::new(self)
    }
    #[doc = "Bit 2 - PELOCK2"]
    #[inline(always)]
    #[must_use]
    pub fn pelock2(&mut self) -> PELOCK2_W<2> {
        PELOCK2_W::new(self)
    }
    #[doc = "Bit 3 - PELOCK3"]
    #[inline(always)]
    #[must_use]
    pub fn pelock3(&mut self) -> PELOCK3_W<3> {
        PELOCK3_W::new(self)
    }
    #[doc = "Bits 16:31 - PELKEY"]
    #[inline(always)]
    #[must_use]
    pub fn pelkey(&mut self) -> PELKEY_W<16> {
        PELKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PELOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pelockr](index.html) module"]
pub struct PELOCKR_SPEC;
impl crate::RegisterSpec for PELOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pelockr::R](R) reader structure"]
impl crate::Readable for PELOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pelockr::W](W) writer structure"]
impl crate::Writable for PELOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PELOCKR to value 0"]
impl crate::Resettable for PELOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
