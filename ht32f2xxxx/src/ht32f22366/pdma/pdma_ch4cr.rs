#[doc = "Register `PDMA_CH4CR` reader"]
pub struct R(crate::R<PDMA_CH4CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_CH4CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_CH4CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_CH4CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_CH4CR` writer"]
pub struct W(crate::W<PDMA_CH4CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_CH4CR_SPEC>;
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
impl From<crate::W<PDMA_CH4CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_CH4CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHEN` reader - CHEN"]
pub type CHEN_R = crate::BitReader;
#[doc = "Field `CHEN` writer - CHEN"]
pub type CHEN_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_CH4CR_SPEC, O>;
#[doc = "Field `SWTRIG` reader - SWTRIG"]
pub type SWTRIG_R = crate::BitReader;
#[doc = "Field `SWTRIG` writer - SWTRIG"]
pub type SWTRIG_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_CH4CR_SPEC, O>;
#[doc = "Field `DWIDTH` reader - DWIDTH"]
pub type DWIDTH_R = crate::FieldReader;
#[doc = "Field `DWIDTH` writer - DWIDTH"]
pub type DWIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, PDMA_CH4CR_SPEC, 2, O>;
#[doc = "Field `DSTAINC` reader - DSTAINC"]
pub type DSTAINC_R = crate::BitReader;
#[doc = "Field `DSTAINC` writer - DSTAINC"]
pub type DSTAINC_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_CH4CR_SPEC, O>;
#[doc = "Field `DSTAMOD` reader - DSTAMOD"]
pub type DSTAMOD_R = crate::BitReader;
#[doc = "Field `DSTAMOD` writer - DSTAMOD"]
pub type DSTAMOD_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_CH4CR_SPEC, O>;
#[doc = "Field `SRCAINC` reader - SRCAINC"]
pub type SRCAINC_R = crate::BitReader;
#[doc = "Field `SRCAINC` writer - SRCAINC"]
pub type SRCAINC_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_CH4CR_SPEC, O>;
#[doc = "Field `SRCAMOD` reader - SRCAMOD"]
pub type SRCAMOD_R = crate::BitReader;
#[doc = "Field `SRCAMOD` writer - SRCAMOD"]
pub type SRCAMOD_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_CH4CR_SPEC, O>;
#[doc = "Field `CHPRI` reader - CHPRI"]
pub type CHPRI_R = crate::FieldReader;
#[doc = "Field `CHPRI` writer - CHPRI"]
pub type CHPRI_W<'a, const O: u8> = crate::FieldWriter<'a, PDMA_CH4CR_SPEC, 2, O>;
#[doc = "Field `FIXAEN` reader - FIXAEN"]
pub type FIXAEN_R = crate::BitReader;
#[doc = "Field `FIXAEN` writer - FIXAEN"]
pub type FIXAEN_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_CH4CR_SPEC, O>;
#[doc = "Field `AUTORL` reader - AUTORL"]
pub type AUTORL_R = crate::BitReader;
#[doc = "Field `AUTORL` writer - AUTORL"]
pub type AUTORL_W<'a, const O: u8> = crate::BitWriter<'a, PDMA_CH4CR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CHEN"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SWTRIG"]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DWIDTH"]
    #[inline(always)]
    pub fn dwidth(&self) -> DWIDTH_R {
        DWIDTH_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - DSTAINC"]
    #[inline(always)]
    pub fn dstainc(&self) -> DSTAINC_R {
        DSTAINC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DSTAMOD"]
    #[inline(always)]
    pub fn dstamod(&self) -> DSTAMOD_R {
        DSTAMOD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SRCAINC"]
    #[inline(always)]
    pub fn srcainc(&self) -> SRCAINC_R {
        SRCAINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SRCAMOD"]
    #[inline(always)]
    pub fn srcamod(&self) -> SRCAMOD_R {
        SRCAMOD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - CHPRI"]
    #[inline(always)]
    pub fn chpri(&self) -> CHPRI_R {
        CHPRI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - FIXAEN"]
    #[inline(always)]
    pub fn fixaen(&self) -> FIXAEN_R {
        FIXAEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AUTORL"]
    #[inline(always)]
    pub fn autorl(&self) -> AUTORL_R {
        AUTORL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CHEN"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<0> {
        CHEN_W::new(self)
    }
    #[doc = "Bit 1 - SWTRIG"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig(&mut self) -> SWTRIG_W<1> {
        SWTRIG_W::new(self)
    }
    #[doc = "Bits 2:3 - DWIDTH"]
    #[inline(always)]
    #[must_use]
    pub fn dwidth(&mut self) -> DWIDTH_W<2> {
        DWIDTH_W::new(self)
    }
    #[doc = "Bit 4 - DSTAINC"]
    #[inline(always)]
    #[must_use]
    pub fn dstainc(&mut self) -> DSTAINC_W<4> {
        DSTAINC_W::new(self)
    }
    #[doc = "Bit 5 - DSTAMOD"]
    #[inline(always)]
    #[must_use]
    pub fn dstamod(&mut self) -> DSTAMOD_W<5> {
        DSTAMOD_W::new(self)
    }
    #[doc = "Bit 6 - SRCAINC"]
    #[inline(always)]
    #[must_use]
    pub fn srcainc(&mut self) -> SRCAINC_W<6> {
        SRCAINC_W::new(self)
    }
    #[doc = "Bit 7 - SRCAMOD"]
    #[inline(always)]
    #[must_use]
    pub fn srcamod(&mut self) -> SRCAMOD_W<7> {
        SRCAMOD_W::new(self)
    }
    #[doc = "Bits 8:9 - CHPRI"]
    #[inline(always)]
    #[must_use]
    pub fn chpri(&mut self) -> CHPRI_W<8> {
        CHPRI_W::new(self)
    }
    #[doc = "Bit 10 - FIXAEN"]
    #[inline(always)]
    #[must_use]
    pub fn fixaen(&mut self) -> FIXAEN_W<10> {
        FIXAEN_W::new(self)
    }
    #[doc = "Bit 11 - AUTORL"]
    #[inline(always)]
    #[must_use]
    pub fn autorl(&mut self) -> AUTORL_W<11> {
        AUTORL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA_CH4CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch4cr](index.html) module"]
pub struct PDMA_CH4CR_SPEC;
impl crate::RegisterSpec for PDMA_CH4CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_ch4cr::R](R) reader structure"]
impl crate::Readable for PDMA_CH4CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_ch4cr::W](W) writer structure"]
impl crate::Writable for PDMA_CH4CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDMA_CH4CR to value 0"]
impl crate::Resettable for PDMA_CH4CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
