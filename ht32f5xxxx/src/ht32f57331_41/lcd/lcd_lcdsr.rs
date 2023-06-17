#[doc = "Register `LCD_LCDSR` reader"]
pub struct R(crate::R<LCD_LCDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_LCDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_LCDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_LCDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_LCDSR` writer"]
pub struct W(crate::W<LCD_LCDSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_LCDSR_SPEC>;
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
impl From<crate::W<LCD_LCDSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_LCDSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDENS` reader - LCDENS"]
pub type LCDENS_R = crate::BitReader;
#[doc = "Field `LCDENS` writer - LCDENS"]
pub type LCDENS_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDSR_SPEC, O>;
#[doc = "Field `SOF` reader - SOF"]
pub type SOF_R = crate::BitReader;
#[doc = "Field `SOF` writer - SOF"]
pub type SOF_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDSR_SPEC, O>;
#[doc = "Field `UDR` reader - UDR"]
pub type UDR_R = crate::BitReader;
#[doc = "Field `UDR` writer - UDR"]
pub type UDR_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDSR_SPEC, O>;
#[doc = "Field `UDD` reader - UDD"]
pub type UDD_R = crate::BitReader;
#[doc = "Field `UDD` writer - UDD"]
pub type UDD_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDSR_SPEC, O>;
#[doc = "Field `RDY` reader - RDY"]
pub type RDY_R = crate::BitReader;
#[doc = "Field `RDY` writer - RDY"]
pub type RDY_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDSR_SPEC, O>;
#[doc = "Field `FCRSF` reader - FCRSF"]
pub type FCRSF_R = crate::BitReader;
#[doc = "Field `FCRSF` writer - FCRSF"]
pub type FCRSF_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - LCDENS"]
    #[inline(always)]
    pub fn lcdens(&self) -> LCDENS_R {
        LCDENS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UDR"]
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UDD"]
    #[inline(always)]
    pub fn udd(&self) -> UDD_R {
        UDD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RDY"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FCRSF"]
    #[inline(always)]
    pub fn fcrsf(&self) -> FCRSF_R {
        FCRSF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCDENS"]
    #[inline(always)]
    #[must_use]
    pub fn lcdens(&mut self) -> LCDENS_W<0> {
        LCDENS_W::new(self)
    }
    #[doc = "Bit 1 - SOF"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<1> {
        SOF_W::new(self)
    }
    #[doc = "Bit 2 - UDR"]
    #[inline(always)]
    #[must_use]
    pub fn udr(&mut self) -> UDR_W<2> {
        UDR_W::new(self)
    }
    #[doc = "Bit 3 - UDD"]
    #[inline(always)]
    #[must_use]
    pub fn udd(&mut self) -> UDD_W<3> {
        UDD_W::new(self)
    }
    #[doc = "Bit 4 - RDY"]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RDY_W<4> {
        RDY_W::new(self)
    }
    #[doc = "Bit 5 - FCRSF"]
    #[inline(always)]
    #[must_use]
    pub fn fcrsf(&mut self) -> FCRSF_W<5> {
        FCRSF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_LCDSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_lcdsr](index.html) module"]
pub struct LCD_LCDSR_SPEC;
impl crate::RegisterSpec for LCD_LCDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_lcdsr::R](R) reader structure"]
impl crate::Readable for LCD_LCDSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_lcdsr::W](W) writer structure"]
impl crate::Writable for LCD_LCDSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCD_LCDSR to value 0"]
impl crate::Resettable for LCD_LCDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
