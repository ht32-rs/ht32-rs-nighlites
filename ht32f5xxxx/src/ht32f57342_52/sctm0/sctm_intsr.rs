#[doc = "Register `SCTM_INTSR` reader"]
pub struct R(crate::R<SCTM_INTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTM_INTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTM_INTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTM_INTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTM_INTSR` writer"]
pub struct W(crate::W<SCTM_INTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTM_INTSR_SPEC>;
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
impl From<crate::W<SCTM_INTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTM_INTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0CCIF` reader - CH0CCIF"]
pub type CH0CCIF_R = crate::BitReader;
#[doc = "Field `CH0CCIF` writer - CH0CCIF"]
pub type CH0CCIF_W<'a, const O: u8> = crate::BitWriter<'a, SCTM_INTSR_SPEC, O>;
#[doc = "Field `CH0OCF` reader - CH0OCF"]
pub type CH0OCF_R = crate::BitReader;
#[doc = "Field `CH0OCF` writer - CH0OCF"]
pub type CH0OCF_W<'a, const O: u8> = crate::BitWriter<'a, SCTM_INTSR_SPEC, O>;
#[doc = "Field `UEVIF` reader - UEVIF"]
pub type UEVIF_R = crate::BitReader;
#[doc = "Field `UEVIF` writer - UEVIF"]
pub type UEVIF_W<'a, const O: u8> = crate::BitWriter<'a, SCTM_INTSR_SPEC, O>;
#[doc = "Field `TEVIF` reader - TEVIF"]
pub type TEVIF_R = crate::BitReader;
#[doc = "Field `TEVIF` writer - TEVIF"]
pub type TEVIF_W<'a, const O: u8> = crate::BitWriter<'a, SCTM_INTSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CH0CCIF"]
    #[inline(always)]
    pub fn ch0ccif(&self) -> CH0CCIF_R {
        CH0CCIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CH0OCF"]
    #[inline(always)]
    pub fn ch0ocf(&self) -> CH0OCF_R {
        CH0OCF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - UEVIF"]
    #[inline(always)]
    pub fn uevif(&self) -> UEVIF_R {
        UEVIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - TEVIF"]
    #[inline(always)]
    pub fn tevif(&self) -> TEVIF_R {
        TEVIF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CCIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccif(&mut self) -> CH0CCIF_W<0> {
        CH0CCIF_W::new(self)
    }
    #[doc = "Bit 4 - CH0OCF"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ocf(&mut self) -> CH0OCF_W<4> {
        CH0OCF_W::new(self)
    }
    #[doc = "Bit 8 - UEVIF"]
    #[inline(always)]
    #[must_use]
    pub fn uevif(&mut self) -> UEVIF_W<8> {
        UEVIF_W::new(self)
    }
    #[doc = "Bit 10 - TEVIF"]
    #[inline(always)]
    #[must_use]
    pub fn tevif(&mut self) -> TEVIF_W<10> {
        TEVIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCTM_INTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctm_intsr](index.html) module"]
pub struct SCTM_INTSR_SPEC;
impl crate::RegisterSpec for SCTM_INTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sctm_intsr::R](R) reader structure"]
impl crate::Readable for SCTM_INTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctm_intsr::W](W) writer structure"]
impl crate::Writable for SCTM_INTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTM_INTSR to value 0"]
impl crate::Resettable for SCTM_INTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
