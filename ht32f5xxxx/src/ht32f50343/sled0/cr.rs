#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEDEN` reader - SLEDEN"]
pub type SLEDEN_R = crate::BitReader;
#[doc = "Field `SLEDEN` writer - SLEDEN"]
pub type SLEDEN_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `OEN` reader - OEN"]
pub type OEN_R = crate::BitReader;
#[doc = "Field `OEN` writer - OEN"]
pub type OEN_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `OINV` reader - OINV"]
pub type OINV_R = crate::BitReader;
#[doc = "Field `OINV` writer - OINV"]
pub type OINV_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `SYNCEN` reader - SYNCEN"]
pub type SYNCEN_R = crate::BitReader;
#[doc = "Field `SYNCEN` writer - SYNCEN"]
pub type SYNCEN_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `INTEN` reader - INTEN"]
pub type INTEN_R = crate::BitReader;
#[doc = "Field `INTEN` writer - INTEN"]
pub type INTEN_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `RSTSTS` reader - RSTSTS"]
pub type RSTSTS_R = crate::BitReader;
#[doc = "Field `RSTSTS` writer - RSTSTS"]
pub type RSTSTS_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `IDLESTS` reader - IDLESTS"]
pub type IDLESTS_R = crate::BitReader;
#[doc = "Field `IDLESTS` writer - IDLESTS"]
pub type IDLESTS_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `SYNCSTS` reader - SYNCSTS"]
pub type SYNCSTS_R = crate::BitReader;
#[doc = "Field `SYNCSTS` writer - SYNCSTS"]
pub type SYNCSTS_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `RSTSTR` reader - RSTSTR"]
pub type RSTSTR_R = crate::BitReader;
#[doc = "Field `RSTSTR` writer - RSTSTR"]
pub type RSTSTR_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - SLEDEN"]
    #[inline(always)]
    pub fn sleden(&self) -> SLEDEN_R {
        SLEDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OEN"]
    #[inline(always)]
    pub fn oen(&self) -> OEN_R {
        OEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OINV"]
    #[inline(always)]
    pub fn oinv(&self) -> OINV_R {
        OINV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SYNCEN"]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - INTEN"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - RSTSTS"]
    #[inline(always)]
    pub fn rststs(&self) -> RSTSTS_R {
        RSTSTS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IDLESTS"]
    #[inline(always)]
    pub fn idlests(&self) -> IDLESTS_R {
        IDLESTS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SYNCSTS"]
    #[inline(always)]
    pub fn syncsts(&self) -> SYNCSTS_R {
        SYNCSTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - RSTSTR"]
    #[inline(always)]
    pub fn rststr(&self) -> RSTSTR_R {
        RSTSTR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SLEDEN"]
    #[inline(always)]
    #[must_use]
    pub fn sleden(&mut self) -> SLEDEN_W<0> {
        SLEDEN_W::new(self)
    }
    #[doc = "Bit 1 - OEN"]
    #[inline(always)]
    #[must_use]
    pub fn oen(&mut self) -> OEN_W<1> {
        OEN_W::new(self)
    }
    #[doc = "Bit 2 - OINV"]
    #[inline(always)]
    #[must_use]
    pub fn oinv(&mut self) -> OINV_W<2> {
        OINV_W::new(self)
    }
    #[doc = "Bit 3 - SYNCEN"]
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SYNCEN_W<3> {
        SYNCEN_W::new(self)
    }
    #[doc = "Bit 4 - DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<4> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 5 - INTEN"]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<5> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 8 - RSTSTS"]
    #[inline(always)]
    #[must_use]
    pub fn rststs(&mut self) -> RSTSTS_W<8> {
        RSTSTS_W::new(self)
    }
    #[doc = "Bit 9 - IDLESTS"]
    #[inline(always)]
    #[must_use]
    pub fn idlests(&mut self) -> IDLESTS_W<9> {
        IDLESTS_W::new(self)
    }
    #[doc = "Bit 10 - SYNCSTS"]
    #[inline(always)]
    #[must_use]
    pub fn syncsts(&mut self) -> SYNCSTS_W<10> {
        SYNCSTS_W::new(self)
    }
    #[doc = "Bit 15 - RSTSTR"]
    #[inline(always)]
    #[must_use]
    pub fn rststr(&mut self) -> RSTSTR_W<15> {
        RSTSTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
