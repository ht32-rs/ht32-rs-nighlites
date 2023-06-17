#[doc = "Register `INTSR` reader"]
pub struct R(crate::R<INTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSR` writer"]
pub struct W(crate::W<INTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSR_SPEC>;
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
impl From<crate::W<INTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0CCIF` reader - CH0CCIF"]
pub type CH0CCIF_R = crate::BitReader;
#[doc = "Field `CH0CCIF` writer - CH0CCIF"]
pub type CH0CCIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `CH1CCIF` reader - CH1CCIF"]
pub type CH1CCIF_R = crate::BitReader;
#[doc = "Field `CH1CCIF` writer - CH1CCIF"]
pub type CH1CCIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `CH2CCIF` reader - CH2CCIF"]
pub type CH2CCIF_R = crate::BitReader;
#[doc = "Field `CH2CCIF` writer - CH2CCIF"]
pub type CH2CCIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `CH3CCIF` reader - CH3CCIF"]
pub type CH3CCIF_R = crate::BitReader;
#[doc = "Field `CH3CCIF` writer - CH3CCIF"]
pub type CH3CCIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `CH0OCF` reader - CH0OCF"]
pub type CH0OCF_R = crate::BitReader;
#[doc = "Field `CH0OCF` writer - CH0OCF"]
pub type CH0OCF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `CH1OCF` reader - CH1OCF"]
pub type CH1OCF_R = crate::BitReader;
#[doc = "Field `CH1OCF` writer - CH1OCF"]
pub type CH1OCF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `CH2OCF` reader - CH2OCF"]
pub type CH2OCF_R = crate::BitReader;
#[doc = "Field `CH2OCF` writer - CH2OCF"]
pub type CH2OCF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `CH3OCF` reader - CH3OCF"]
pub type CH3OCF_R = crate::BitReader;
#[doc = "Field `CH3OCF` writer - CH3OCF"]
pub type CH3OCF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `UEV1IF` reader - UEV1IF"]
pub type UEV1IF_R = crate::BitReader;
#[doc = "Field `UEV1IF` writer - UEV1IF"]
pub type UEV1IF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `UEV2IF` reader - UEV2IF"]
pub type UEV2IF_R = crate::BitReader;
#[doc = "Field `UEV2IF` writer - UEV2IF"]
pub type UEV2IF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `TEVIF` reader - TEVIF"]
pub type TEVIF_R = crate::BitReader;
#[doc = "Field `TEVIF` writer - TEVIF"]
pub type TEVIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `BRK0IF` reader - BRK0IF"]
pub type BRK0IF_R = crate::BitReader;
#[doc = "Field `BRK0IF` writer - BRK0IF"]
pub type BRK0IF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `BRK1IF` reader - BRK1IF"]
pub type BRK1IF_R = crate::BitReader;
#[doc = "Field `BRK1IF` writer - BRK1IF"]
pub type BRK1IF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `OVERIF` reader - OVERIF"]
pub type OVERIF_R = crate::BitReader;
#[doc = "Field `OVERIF` writer - OVERIF"]
pub type OVERIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `UNDERIF` reader - UNDERIF"]
pub type UNDERIF_R = crate::BitReader;
#[doc = "Field `UNDERIF` writer - UNDERIF"]
pub type UNDERIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `CH0CDIF` reader - CH0CDIF"]
pub type CH0CDIF_R = crate::BitReader;
#[doc = "Field `CH0CDIF` writer - CH0CDIF"]
pub type CH0CDIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `CH1CDIF` reader - CH1CDIF"]
pub type CH1CDIF_R = crate::BitReader;
#[doc = "Field `CH1CDIF` writer - CH1CDIF"]
pub type CH1CDIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `CH2CDIF` reader - CH2CDIF"]
pub type CH2CDIF_R = crate::BitReader;
#[doc = "Field `CH2CDIF` writer - CH2CDIF"]
pub type CH2CDIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
#[doc = "Field `CH3CDIF` reader - CH3CDIF"]
pub type CH3CDIF_R = crate::BitReader;
#[doc = "Field `CH3CDIF` writer - CH3CDIF"]
pub type CH3CDIF_W<'a, const O: u8> = crate::BitWriter<'a, INTSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CH0CCIF"]
    #[inline(always)]
    pub fn ch0ccif(&self) -> CH0CCIF_R {
        CH0CCIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1CCIF"]
    #[inline(always)]
    pub fn ch1ccif(&self) -> CH1CCIF_R {
        CH1CCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2CCIF"]
    #[inline(always)]
    pub fn ch2ccif(&self) -> CH2CCIF_R {
        CH2CCIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3CCIF"]
    #[inline(always)]
    pub fn ch3ccif(&self) -> CH3CCIF_R {
        CH3CCIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH0OCF"]
    #[inline(always)]
    pub fn ch0ocf(&self) -> CH0OCF_R {
        CH0OCF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH1OCF"]
    #[inline(always)]
    pub fn ch1ocf(&self) -> CH1OCF_R {
        CH1OCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH2OCF"]
    #[inline(always)]
    pub fn ch2ocf(&self) -> CH2OCF_R {
        CH2OCF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH3OCF"]
    #[inline(always)]
    pub fn ch3ocf(&self) -> CH3OCF_R {
        CH3OCF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UEV1IF"]
    #[inline(always)]
    pub fn uev1if(&self) -> UEV1IF_R {
        UEV1IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UEV2IF"]
    #[inline(always)]
    pub fn uev2if(&self) -> UEV2IF_R {
        UEV2IF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TEVIF"]
    #[inline(always)]
    pub fn tevif(&self) -> TEVIF_R {
        TEVIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK0IF"]
    #[inline(always)]
    pub fn brk0if(&self) -> BRK0IF_R {
        BRK0IF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRK1IF"]
    #[inline(always)]
    pub fn brk1if(&self) -> BRK1IF_R {
        BRK1IF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OVERIF"]
    #[inline(always)]
    pub fn overif(&self) -> OVERIF_R {
        OVERIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - UNDERIF"]
    #[inline(always)]
    pub fn underif(&self) -> UNDERIF_R {
        UNDERIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 20 - CH0CDIF"]
    #[inline(always)]
    pub fn ch0cdif(&self) -> CH0CDIF_R {
        CH0CDIF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CH1CDIF"]
    #[inline(always)]
    pub fn ch1cdif(&self) -> CH1CDIF_R {
        CH1CDIF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CH2CDIF"]
    #[inline(always)]
    pub fn ch2cdif(&self) -> CH2CDIF_R {
        CH2CDIF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CH3CDIF"]
    #[inline(always)]
    pub fn ch3cdif(&self) -> CH3CDIF_R {
        CH3CDIF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CCIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccif(&mut self) -> CH0CCIF_W<0> {
        CH0CCIF_W::new(self)
    }
    #[doc = "Bit 1 - CH1CCIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ccif(&mut self) -> CH1CCIF_W<1> {
        CH1CCIF_W::new(self)
    }
    #[doc = "Bit 2 - CH2CCIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ccif(&mut self) -> CH2CCIF_W<2> {
        CH2CCIF_W::new(self)
    }
    #[doc = "Bit 3 - CH3CCIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ccif(&mut self) -> CH3CCIF_W<3> {
        CH3CCIF_W::new(self)
    }
    #[doc = "Bit 4 - CH0OCF"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ocf(&mut self) -> CH0OCF_W<4> {
        CH0OCF_W::new(self)
    }
    #[doc = "Bit 5 - CH1OCF"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ocf(&mut self) -> CH1OCF_W<5> {
        CH1OCF_W::new(self)
    }
    #[doc = "Bit 6 - CH2OCF"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ocf(&mut self) -> CH2OCF_W<6> {
        CH2OCF_W::new(self)
    }
    #[doc = "Bit 7 - CH3OCF"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ocf(&mut self) -> CH3OCF_W<7> {
        CH3OCF_W::new(self)
    }
    #[doc = "Bit 8 - UEV1IF"]
    #[inline(always)]
    #[must_use]
    pub fn uev1if(&mut self) -> UEV1IF_W<8> {
        UEV1IF_W::new(self)
    }
    #[doc = "Bit 9 - UEV2IF"]
    #[inline(always)]
    #[must_use]
    pub fn uev2if(&mut self) -> UEV2IF_W<9> {
        UEV2IF_W::new(self)
    }
    #[doc = "Bit 10 - TEVIF"]
    #[inline(always)]
    #[must_use]
    pub fn tevif(&mut self) -> TEVIF_W<10> {
        TEVIF_W::new(self)
    }
    #[doc = "Bit 11 - BRK0IF"]
    #[inline(always)]
    #[must_use]
    pub fn brk0if(&mut self) -> BRK0IF_W<11> {
        BRK0IF_W::new(self)
    }
    #[doc = "Bit 12 - BRK1IF"]
    #[inline(always)]
    #[must_use]
    pub fn brk1if(&mut self) -> BRK1IF_W<12> {
        BRK1IF_W::new(self)
    }
    #[doc = "Bit 13 - OVERIF"]
    #[inline(always)]
    #[must_use]
    pub fn overif(&mut self) -> OVERIF_W<13> {
        OVERIF_W::new(self)
    }
    #[doc = "Bit 14 - UNDERIF"]
    #[inline(always)]
    #[must_use]
    pub fn underif(&mut self) -> UNDERIF_W<14> {
        UNDERIF_W::new(self)
    }
    #[doc = "Bit 20 - CH0CDIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cdif(&mut self) -> CH0CDIF_W<20> {
        CH0CDIF_W::new(self)
    }
    #[doc = "Bit 21 - CH1CDIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cdif(&mut self) -> CH1CDIF_W<21> {
        CH1CDIF_W::new(self)
    }
    #[doc = "Bit 22 - CH2CDIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch2cdif(&mut self) -> CH2CDIF_W<22> {
        CH2CDIF_W::new(self)
    }
    #[doc = "Bit 23 - CH3CDIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch3cdif(&mut self) -> CH3CDIF_W<23> {
        CH3CDIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intsr](index.html) module"]
pub struct INTSR_SPEC;
impl crate::RegisterSpec for INTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intsr::R](R) reader structure"]
impl crate::Readable for INTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intsr::W](W) writer structure"]
impl crate::Writable for INTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTSR to value 0"]
impl crate::Resettable for INTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
