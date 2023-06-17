#[doc = "Register `I2SIER` reader"]
pub struct R(crate::R<I2SIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SIER` writer"]
pub struct W(crate::W<I2SIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SIER_SPEC>;
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
impl From<crate::W<I2SIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFTLIEN` reader - TXFTLIEN"]
pub type TXFTLIEN_R = crate::BitReader;
#[doc = "Field `TXFTLIEN` writer - TXFTLIEN"]
pub type TXFTLIEN_W<'a, const O: u8> = crate::BitWriter<'a, I2SIER_SPEC, O>;
#[doc = "Field `TXUDIEN` reader - TXUDIEN"]
pub type TXUDIEN_R = crate::BitReader;
#[doc = "Field `TXUDIEN` writer - TXUDIEN"]
pub type TXUDIEN_W<'a, const O: u8> = crate::BitWriter<'a, I2SIER_SPEC, O>;
#[doc = "Field `TXOVIEN` reader - TXOVIEN"]
pub type TXOVIEN_R = crate::BitReader;
#[doc = "Field `TXOVIEN` writer - TXOVIEN"]
pub type TXOVIEN_W<'a, const O: u8> = crate::BitWriter<'a, I2SIER_SPEC, O>;
#[doc = "Field `RXFTLIEN` reader - RXFTLIEN"]
pub type RXFTLIEN_R = crate::BitReader;
#[doc = "Field `RXFTLIEN` writer - RXFTLIEN"]
pub type RXFTLIEN_W<'a, const O: u8> = crate::BitWriter<'a, I2SIER_SPEC, O>;
#[doc = "Field `RXUDIEN` reader - RXUDIEN"]
pub type RXUDIEN_R = crate::BitReader;
#[doc = "Field `RXUDIEN` writer - RXUDIEN"]
pub type RXUDIEN_W<'a, const O: u8> = crate::BitWriter<'a, I2SIER_SPEC, O>;
#[doc = "Field `RXOVIEN` reader - RXOVIEN"]
pub type RXOVIEN_R = crate::BitReader;
#[doc = "Field `RXOVIEN` writer - RXOVIEN"]
pub type RXOVIEN_W<'a, const O: u8> = crate::BitWriter<'a, I2SIER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - TXFTLIEN"]
    #[inline(always)]
    pub fn txftlien(&self) -> TXFTLIEN_R {
        TXFTLIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXUDIEN"]
    #[inline(always)]
    pub fn txudien(&self) -> TXUDIEN_R {
        TXUDIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXOVIEN"]
    #[inline(always)]
    pub fn txovien(&self) -> TXOVIEN_R {
        TXOVIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFTLIEN"]
    #[inline(always)]
    pub fn rxftlien(&self) -> RXFTLIEN_R {
        RXFTLIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXUDIEN"]
    #[inline(always)]
    pub fn rxudien(&self) -> RXUDIEN_R {
        RXUDIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXOVIEN"]
    #[inline(always)]
    pub fn rxovien(&self) -> RXOVIEN_R {
        RXOVIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXFTLIEN"]
    #[inline(always)]
    #[must_use]
    pub fn txftlien(&mut self) -> TXFTLIEN_W<0> {
        TXFTLIEN_W::new(self)
    }
    #[doc = "Bit 1 - TXUDIEN"]
    #[inline(always)]
    #[must_use]
    pub fn txudien(&mut self) -> TXUDIEN_W<1> {
        TXUDIEN_W::new(self)
    }
    #[doc = "Bit 2 - TXOVIEN"]
    #[inline(always)]
    #[must_use]
    pub fn txovien(&mut self) -> TXOVIEN_W<2> {
        TXOVIEN_W::new(self)
    }
    #[doc = "Bit 4 - RXFTLIEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxftlien(&mut self) -> RXFTLIEN_W<4> {
        RXFTLIEN_W::new(self)
    }
    #[doc = "Bit 5 - RXUDIEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxudien(&mut self) -> RXUDIEN_W<5> {
        RXUDIEN_W::new(self)
    }
    #[doc = "Bit 6 - RXOVIEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxovien(&mut self) -> RXOVIEN_W<6> {
        RXOVIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2SIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sier](index.html) module"]
pub struct I2SIER_SPEC;
impl crate::RegisterSpec for I2SIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sier::R](R) reader structure"]
impl crate::Readable for I2SIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2sier::W](W) writer structure"]
impl crate::Writable for I2SIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SIER to value 0"]
impl crate::Resettable for I2SIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
