#[doc = "Register `I2CADDR` reader"]
pub struct R(crate::R<I2CADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2CADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2CADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2CADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2CADDR` writer"]
pub struct W(crate::W<I2CADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2CADDR_SPEC>;
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
impl From<crate::W<I2CADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2CADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - ADDR"]
pub type ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - ADDR"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, I2CADDR_SPEC, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - ADDR"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2caddr](index.html) module"]
pub struct I2CADDR_SPEC;
impl crate::RegisterSpec for I2CADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2caddr::R](R) reader structure"]
impl crate::Readable for I2CADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2caddr::W](W) writer structure"]
impl crate::Writable for I2CADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2CADDR to value 0"]
impl crate::Resettable for I2CADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
