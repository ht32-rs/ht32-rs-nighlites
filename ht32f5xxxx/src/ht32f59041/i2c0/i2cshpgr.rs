#[doc = "Register `I2CSHPGR` reader"]
pub struct R(crate::R<I2CSHPGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2CSHPGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2CSHPGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2CSHPGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2CSHPGR` writer"]
pub struct W(crate::W<I2CSHPGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2CSHPGR_SPEC>;
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
impl From<crate::W<I2CSHPGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2CSHPGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHPG` reader - SHPG"]
pub type SHPG_R = crate::FieldReader<u16>;
#[doc = "Field `SHPG` writer - SHPG"]
pub type SHPG_W<'a, const O: u8> = crate::FieldWriter<'a, I2CSHPGR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - SHPG"]
    #[inline(always)]
    pub fn shpg(&self) -> SHPG_R {
        SHPG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SHPG"]
    #[inline(always)]
    #[must_use]
    pub fn shpg(&mut self) -> SHPG_W<0> {
        SHPG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CSHPGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cshpgr](index.html) module"]
pub struct I2CSHPGR_SPEC;
impl crate::RegisterSpec for I2CSHPGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2cshpgr::R](R) reader structure"]
impl crate::Readable for I2CSHPGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2cshpgr::W](W) writer structure"]
impl crate::Writable for I2CSHPGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2CSHPGR to value 0"]
impl crate::Resettable for I2CSHPGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
