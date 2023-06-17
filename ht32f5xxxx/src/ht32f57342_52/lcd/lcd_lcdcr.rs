#[doc = "Register `LCD_LCDCR` reader"]
pub struct R(crate::R<LCD_LCDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_LCDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_LCDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_LCDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_LCDCR` writer"]
pub struct W(crate::W<LCD_LCDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_LCDCR_SPEC>;
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
impl From<crate::W<LCD_LCDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_LCDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDEN` reader - LCDEN"]
pub type LCDEN_R = crate::BitReader;
#[doc = "Field `LCDEN` writer - LCDEN"]
pub type LCDEN_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDCR_SPEC, O>;
#[doc = "Field `LCDPR` reader - LCDPR"]
pub type LCDPR_R = crate::BitReader;
#[doc = "Field `LCDPR` writer - LCDPR"]
pub type LCDPR_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDCR_SPEC, O>;
#[doc = "Field `DTYC` reader - DTYC"]
pub type DTYC_R = crate::FieldReader;
#[doc = "Field `DTYC` writer - DTYC"]
pub type DTYC_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_LCDCR_SPEC, 3, O>;
#[doc = "Field `BIAS` reader - BIAS"]
pub type BIAS_R = crate::FieldReader;
#[doc = "Field `BIAS` writer - BIAS"]
pub type BIAS_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_LCDCR_SPEC, 2, O>;
#[doc = "Field `TYPE` reader - TYPE"]
pub type TYPE_R = crate::BitReader;
#[doc = "Field `TYPE` writer - TYPE"]
pub type TYPE_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDCR_SPEC, O>;
#[doc = "Field `MUXCOM4` reader - MUXCOM4"]
pub type MUXCOM4_R = crate::BitReader;
#[doc = "Field `MUXCOM4` writer - MUXCOM4"]
pub type MUXCOM4_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDCR_SPEC, O>;
#[doc = "Field `MUXCOM5` reader - MUXCOM5"]
pub type MUXCOM5_R = crate::BitReader;
#[doc = "Field `MUXCOM5` writer - MUXCOM5"]
pub type MUXCOM5_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDCR_SPEC, O>;
#[doc = "Field `MUXCOM6` reader - MUXCOM6"]
pub type MUXCOM6_R = crate::BitReader;
#[doc = "Field `MUXCOM6` writer - MUXCOM6"]
pub type MUXCOM6_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDCR_SPEC, O>;
#[doc = "Field `MUXCOM7` reader - MUXCOM7"]
pub type MUXCOM7_R = crate::BitReader;
#[doc = "Field `MUXCOM7` writer - MUXCOM7"]
pub type MUXCOM7_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDCR_SPEC, O>;
#[doc = "Field `DSTAO` reader - DSTAO"]
pub type DSTAO_R = crate::BitReader;
#[doc = "Field `DSTAO` writer - DSTAO"]
pub type DSTAO_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDCR_SPEC, O>;
#[doc = "Field `HRLEN` reader - HRLEN"]
pub type HRLEN_R = crate::BitReader;
#[doc = "Field `HRLEN` writer - HRLEN"]
pub type HRLEN_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDCR_SPEC, O>;
#[doc = "Field `MMASK` reader - MMASK"]
pub type MMASK_R = crate::BitReader;
#[doc = "Field `MMASK` writer - MMASK"]
pub type MMASK_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - LCDEN"]
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCDPR"]
    #[inline(always)]
    pub fn lcdpr(&self) -> LCDPR_R {
        LCDPR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - DTYC"]
    #[inline(always)]
    pub fn dtyc(&self) -> DTYC_R {
        DTYC_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - BIAS"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - TYPE"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MUXCOM4"]
    #[inline(always)]
    pub fn muxcom4(&self) -> MUXCOM4_R {
        MUXCOM4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MUXCOM5"]
    #[inline(always)]
    pub fn muxcom5(&self) -> MUXCOM5_R {
        MUXCOM5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MUXCOM6"]
    #[inline(always)]
    pub fn muxcom6(&self) -> MUXCOM6_R {
        MUXCOM6_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MUXCOM7"]
    #[inline(always)]
    pub fn muxcom7(&self) -> MUXCOM7_R {
        MUXCOM7_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - DSTAO"]
    #[inline(always)]
    pub fn dstao(&self) -> DSTAO_R {
        DSTAO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - HRLEN"]
    #[inline(always)]
    pub fn hrlen(&self) -> HRLEN_R {
        HRLEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - MMASK"]
    #[inline(always)]
    pub fn mmask(&self) -> MMASK_R {
        MMASK_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCDEN"]
    #[inline(always)]
    #[must_use]
    pub fn lcden(&mut self) -> LCDEN_W<0> {
        LCDEN_W::new(self)
    }
    #[doc = "Bit 1 - LCDPR"]
    #[inline(always)]
    #[must_use]
    pub fn lcdpr(&mut self) -> LCDPR_W<1> {
        LCDPR_W::new(self)
    }
    #[doc = "Bits 2:4 - DTYC"]
    #[inline(always)]
    #[must_use]
    pub fn dtyc(&mut self) -> DTYC_W<2> {
        DTYC_W::new(self)
    }
    #[doc = "Bits 5:6 - BIAS"]
    #[inline(always)]
    #[must_use]
    pub fn bias(&mut self) -> BIAS_W<5> {
        BIAS_W::new(self)
    }
    #[doc = "Bit 7 - TYPE"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TYPE_W<7> {
        TYPE_W::new(self)
    }
    #[doc = "Bit 8 - MUXCOM4"]
    #[inline(always)]
    #[must_use]
    pub fn muxcom4(&mut self) -> MUXCOM4_W<8> {
        MUXCOM4_W::new(self)
    }
    #[doc = "Bit 9 - MUXCOM5"]
    #[inline(always)]
    #[must_use]
    pub fn muxcom5(&mut self) -> MUXCOM5_W<9> {
        MUXCOM5_W::new(self)
    }
    #[doc = "Bit 10 - MUXCOM6"]
    #[inline(always)]
    #[must_use]
    pub fn muxcom6(&mut self) -> MUXCOM6_W<10> {
        MUXCOM6_W::new(self)
    }
    #[doc = "Bit 11 - MUXCOM7"]
    #[inline(always)]
    #[must_use]
    pub fn muxcom7(&mut self) -> MUXCOM7_W<11> {
        MUXCOM7_W::new(self)
    }
    #[doc = "Bit 14 - DSTAO"]
    #[inline(always)]
    #[must_use]
    pub fn dstao(&mut self) -> DSTAO_W<14> {
        DSTAO_W::new(self)
    }
    #[doc = "Bit 15 - HRLEN"]
    #[inline(always)]
    #[must_use]
    pub fn hrlen(&mut self) -> HRLEN_W<15> {
        HRLEN_W::new(self)
    }
    #[doc = "Bit 24 - MMASK"]
    #[inline(always)]
    #[must_use]
    pub fn mmask(&mut self) -> MMASK_W<24> {
        MMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_LCDCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_lcdcr](index.html) module"]
pub struct LCD_LCDCR_SPEC;
impl crate::RegisterSpec for LCD_LCDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_lcdcr::R](R) reader structure"]
impl crate::Readable for LCD_LCDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_lcdcr::W](W) writer structure"]
impl crate::Writable for LCD_LCDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCD_LCDCR to value 0"]
impl crate::Resettable for LCD_LCDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
