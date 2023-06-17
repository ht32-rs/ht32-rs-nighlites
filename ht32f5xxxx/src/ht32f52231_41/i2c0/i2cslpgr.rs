#[doc = "Register `I2CSLPGR` reader"]
pub struct R(crate::R<I2CSLPGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2CSLPGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2CSLPGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2CSLPGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2CSLPGR` writer"]
pub struct W(crate::W<I2CSLPGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2CSLPGR_SPEC>;
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
impl From<crate::W<I2CSLPGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2CSLPGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLPG` reader - SLPG"]
pub type SLPG_R = crate::FieldReader<u16>;
#[doc = "Field `SLPG` writer - SLPG"]
pub type SLPG_W<'a, const O: u8> = crate::FieldWriter<'a, I2CSLPGR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - SLPG"]
    #[inline(always)]
    pub fn slpg(&self) -> SLPG_R {
        SLPG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SLPG"]
    #[inline(always)]
    #[must_use]
    pub fn slpg(&mut self) -> SLPG_W<0> {
        SLPG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CSLPGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cslpgr](index.html) module"]
pub struct I2CSLPGR_SPEC;
impl crate::RegisterSpec for I2CSLPGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2cslpgr::R](R) reader structure"]
impl crate::Readable for I2CSLPGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2cslpgr::W](W) writer structure"]
impl crate::Writable for I2CSLPGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2CSLPGR to value 0"]
impl crate::Resettable for I2CSLPGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
