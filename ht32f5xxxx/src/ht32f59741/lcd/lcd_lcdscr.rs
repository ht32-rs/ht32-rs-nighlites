#[doc = "Register `LCD_LCDSCR` reader"]
pub struct R(crate::R<LCD_LCDSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_LCDSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_LCDSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_LCDSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_LCDSCR` writer"]
pub struct W(crate::W<LCD_LCDSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_LCDSCR_SPEC>;
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
impl From<crate::W<LCD_LCDSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_LCDSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFC` reader - SOFC"]
pub type SOFC_R = crate::BitReader;
#[doc = "Field `SOFC` writer - SOFC"]
pub type SOFC_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDSCR_SPEC, O>;
#[doc = "Field `UDDC` reader - UDDC"]
pub type UDDC_R = crate::BitReader;
#[doc = "Field `UDDC` writer - UDDC"]
pub type UDDC_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDSCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - SOFC"]
    #[inline(always)]
    pub fn sofc(&self) -> SOFC_R {
        SOFC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UDDC"]
    #[inline(always)]
    pub fn uddc(&self) -> UDDC_R {
        UDDC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SOFC"]
    #[inline(always)]
    #[must_use]
    pub fn sofc(&mut self) -> SOFC_W<0> {
        SOFC_W::new(self)
    }
    #[doc = "Bit 1 - UDDC"]
    #[inline(always)]
    #[must_use]
    pub fn uddc(&mut self) -> UDDC_W<1> {
        UDDC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_LCDSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_lcdscr](index.html) module"]
pub struct LCD_LCDSCR_SPEC;
impl crate::RegisterSpec for LCD_LCDSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_lcdscr::R](R) reader structure"]
impl crate::Readable for LCD_LCDSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_lcdscr::W](W) writer structure"]
impl crate::Writable for LCD_LCDSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCD_LCDSCR to value 0"]
impl crate::Resettable for LCD_LCDSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
