#[doc = "Register `I2S_SR` reader"]
pub struct R(crate::R<I2S_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_SR` writer"]
pub struct W(crate::W<I2S_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_SR_SPEC>;
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
impl From<crate::W<I2S_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFTL` reader - TXFTL"]
pub type TXFTL_R = crate::BitReader;
#[doc = "Field `TXFTL` writer - TXFTL"]
pub type TXFTL_W<'a, const O: u8> = crate::BitWriter<'a, I2S_SR_SPEC, O>;
#[doc = "Field `TXFUD` reader - TXFUD"]
pub type TXFUD_R = crate::BitReader;
#[doc = "Field `TXFUD` writer - TXFUD"]
pub type TXFUD_W<'a, const O: u8> = crate::BitWriter<'a, I2S_SR_SPEC, O>;
#[doc = "Field `TXFOV` reader - TXFOV"]
pub type TXFOV_R = crate::BitReader;
#[doc = "Field `TXFOV` writer - TXFOV"]
pub type TXFOV_W<'a, const O: u8> = crate::BitWriter<'a, I2S_SR_SPEC, O>;
#[doc = "Field `TXFEMT` reader - TXFEMT"]
pub type TXFEMT_R = crate::BitReader;
#[doc = "Field `TXFEMT` writer - TXFEMT"]
pub type TXFEMT_W<'a, const O: u8> = crate::BitWriter<'a, I2S_SR_SPEC, O>;
#[doc = "Field `TXFFUL` reader - TXFFUL"]
pub type TXFFUL_R = crate::BitReader;
#[doc = "Field `TXFFUL` writer - TXFFUL"]
pub type TXFFUL_W<'a, const O: u8> = crate::BitWriter<'a, I2S_SR_SPEC, O>;
#[doc = "Field `RXFTL` reader - RXFTL"]
pub type RXFTL_R = crate::BitReader;
#[doc = "Field `RXFTL` writer - RXFTL"]
pub type RXFTL_W<'a, const O: u8> = crate::BitWriter<'a, I2S_SR_SPEC, O>;
#[doc = "Field `RXFUD` reader - RXFUD"]
pub type RXFUD_R = crate::BitReader;
#[doc = "Field `RXFUD` writer - RXFUD"]
pub type RXFUD_W<'a, const O: u8> = crate::BitWriter<'a, I2S_SR_SPEC, O>;
#[doc = "Field `RXFOV` reader - RXFOV"]
pub type RXFOV_R = crate::BitReader;
#[doc = "Field `RXFOV` writer - RXFOV"]
pub type RXFOV_W<'a, const O: u8> = crate::BitWriter<'a, I2S_SR_SPEC, O>;
#[doc = "Field `RXFEMT` reader - RXFEMT"]
pub type RXFEMT_R = crate::BitReader;
#[doc = "Field `RXFEMT` writer - RXFEMT"]
pub type RXFEMT_W<'a, const O: u8> = crate::BitWriter<'a, I2S_SR_SPEC, O>;
#[doc = "Field `RXFFUL` reader - RXFFUL"]
pub type RXFFUL_R = crate::BitReader;
#[doc = "Field `RXFFUL` writer - RXFFUL"]
pub type RXFFUL_W<'a, const O: u8> = crate::BitWriter<'a, I2S_SR_SPEC, O>;
#[doc = "Field `CHS` reader - CHS"]
pub type CHS_R = crate::BitReader;
#[doc = "Field `CHS` writer - CHS"]
pub type CHS_W<'a, const O: u8> = crate::BitWriter<'a, I2S_SR_SPEC, O>;
#[doc = "Field `TXBUSY` reader - TXBUSY"]
pub type TXBUSY_R = crate::BitReader;
#[doc = "Field `TXBUSY` writer - TXBUSY"]
pub type TXBUSY_W<'a, const O: u8> = crate::BitWriter<'a, I2S_SR_SPEC, O>;
#[doc = "Field `CLKRDY` reader - CLKRDY"]
pub type CLKRDY_R = crate::BitReader;
#[doc = "Field `CLKRDY` writer - CLKRDY"]
pub type CLKRDY_W<'a, const O: u8> = crate::BitWriter<'a, I2S_SR_SPEC, O>;
#[doc = "Field `TXFS` reader - TXFS"]
pub type TXFS_R = crate::FieldReader;
#[doc = "Field `TXFS` writer - TXFS"]
pub type TXFS_W<'a, const O: u8> = crate::FieldWriter<'a, I2S_SR_SPEC, 4, O>;
#[doc = "Field `RXFS` reader - RXFS"]
pub type RXFS_R = crate::FieldReader;
#[doc = "Field `RXFS` writer - RXFS"]
pub type RXFS_W<'a, const O: u8> = crate::FieldWriter<'a, I2S_SR_SPEC, 4, O>;
impl R {
    #[doc = "Bit 0 - TXFTL"]
    #[inline(always)]
    pub fn txftl(&self) -> TXFTL_R {
        TXFTL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXFUD"]
    #[inline(always)]
    pub fn txfud(&self) -> TXFUD_R {
        TXFUD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXFOV"]
    #[inline(always)]
    pub fn txfov(&self) -> TXFOV_R {
        TXFOV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXFEMT"]
    #[inline(always)]
    pub fn txfemt(&self) -> TXFEMT_R {
        TXFEMT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXFFUL"]
    #[inline(always)]
    pub fn txfful(&self) -> TXFFUL_R {
        TXFFUL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - RXFTL"]
    #[inline(always)]
    pub fn rxftl(&self) -> RXFTL_R {
        RXFTL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXFUD"]
    #[inline(always)]
    pub fn rxfud(&self) -> RXFUD_R {
        RXFUD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RXFOV"]
    #[inline(always)]
    pub fn rxfov(&self) -> RXFOV_R {
        RXFOV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXFEMT"]
    #[inline(always)]
    pub fn rxfemt(&self) -> RXFEMT_R {
        RXFEMT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RXFFUL"]
    #[inline(always)]
    pub fn rxfful(&self) -> RXFFUL_R {
        RXFFUL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - CHS"]
    #[inline(always)]
    pub fn chs(&self) -> CHS_R {
        CHS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TXBUSY"]
    #[inline(always)]
    pub fn txbusy(&self) -> TXBUSY_R {
        TXBUSY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CLKRDY"]
    #[inline(always)]
    pub fn clkrdy(&self) -> CLKRDY_R {
        CLKRDY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 24:27 - TXFS"]
    #[inline(always)]
    pub fn txfs(&self) -> TXFS_R {
        TXFS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - RXFS"]
    #[inline(always)]
    pub fn rxfs(&self) -> RXFS_R {
        RXFS_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TXFTL"]
    #[inline(always)]
    #[must_use]
    pub fn txftl(&mut self) -> TXFTL_W<0> {
        TXFTL_W::new(self)
    }
    #[doc = "Bit 1 - TXFUD"]
    #[inline(always)]
    #[must_use]
    pub fn txfud(&mut self) -> TXFUD_W<1> {
        TXFUD_W::new(self)
    }
    #[doc = "Bit 2 - TXFOV"]
    #[inline(always)]
    #[must_use]
    pub fn txfov(&mut self) -> TXFOV_W<2> {
        TXFOV_W::new(self)
    }
    #[doc = "Bit 3 - TXFEMT"]
    #[inline(always)]
    #[must_use]
    pub fn txfemt(&mut self) -> TXFEMT_W<3> {
        TXFEMT_W::new(self)
    }
    #[doc = "Bit 4 - TXFFUL"]
    #[inline(always)]
    #[must_use]
    pub fn txfful(&mut self) -> TXFFUL_W<4> {
        TXFFUL_W::new(self)
    }
    #[doc = "Bit 8 - RXFTL"]
    #[inline(always)]
    #[must_use]
    pub fn rxftl(&mut self) -> RXFTL_W<8> {
        RXFTL_W::new(self)
    }
    #[doc = "Bit 9 - RXFUD"]
    #[inline(always)]
    #[must_use]
    pub fn rxfud(&mut self) -> RXFUD_W<9> {
        RXFUD_W::new(self)
    }
    #[doc = "Bit 10 - RXFOV"]
    #[inline(always)]
    #[must_use]
    pub fn rxfov(&mut self) -> RXFOV_W<10> {
        RXFOV_W::new(self)
    }
    #[doc = "Bit 11 - RXFEMT"]
    #[inline(always)]
    #[must_use]
    pub fn rxfemt(&mut self) -> RXFEMT_W<11> {
        RXFEMT_W::new(self)
    }
    #[doc = "Bit 12 - RXFFUL"]
    #[inline(always)]
    #[must_use]
    pub fn rxfful(&mut self) -> RXFFUL_W<12> {
        RXFFUL_W::new(self)
    }
    #[doc = "Bit 16 - CHS"]
    #[inline(always)]
    #[must_use]
    pub fn chs(&mut self) -> CHS_W<16> {
        CHS_W::new(self)
    }
    #[doc = "Bit 17 - TXBUSY"]
    #[inline(always)]
    #[must_use]
    pub fn txbusy(&mut self) -> TXBUSY_W<17> {
        TXBUSY_W::new(self)
    }
    #[doc = "Bit 18 - CLKRDY"]
    #[inline(always)]
    #[must_use]
    pub fn clkrdy(&mut self) -> CLKRDY_W<18> {
        CLKRDY_W::new(self)
    }
    #[doc = "Bits 24:27 - TXFS"]
    #[inline(always)]
    #[must_use]
    pub fn txfs(&mut self) -> TXFS_W<24> {
        TXFS_W::new(self)
    }
    #[doc = "Bits 28:31 - RXFS"]
    #[inline(always)]
    #[must_use]
    pub fn rxfs(&mut self) -> RXFS_W<28> {
        RXFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S_SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_sr](index.html) module"]
pub struct I2S_SR_SPEC;
impl crate::RegisterSpec for I2S_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_sr::R](R) reader structure"]
impl crate::Readable for I2S_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_sr::W](W) writer structure"]
impl crate::Writable for I2S_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2S_SR to value 0"]
impl crate::Resettable for I2S_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
