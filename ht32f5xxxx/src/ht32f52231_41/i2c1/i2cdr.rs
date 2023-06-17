#[doc = "Register `I2CDR` reader"]
pub struct R(crate::R<I2CDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2CDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2CDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2CDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2CDR` writer"]
pub struct W(crate::W<I2CDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2CDR_SPEC>;
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
impl From<crate::W<I2CDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2CDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - DATA"]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA` writer - DATA"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, I2CDR_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cdr](index.html) module"]
pub struct I2CDR_SPEC;
impl crate::RegisterSpec for I2CDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2cdr::R](R) reader structure"]
impl crate::Readable for I2CDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2cdr::W](W) writer structure"]
impl crate::Writable for I2CDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2CDR to value 0"]
impl crate::Resettable for I2CDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
