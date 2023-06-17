#[doc = "Register `DIV_CR` reader"]
pub struct R(crate::R<DIV_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV_CR` writer"]
pub struct W(crate::W<DIV_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_CR_SPEC>;
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
impl From<crate::W<DIV_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - START"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - START"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, DIV_CR_SPEC, O>;
#[doc = "Field `ZEF` reader - ZEF"]
pub type ZEF_R = crate::BitReader;
#[doc = "Field `ZEF` writer - ZEF"]
pub type ZEF_W<'a, const O: u8> = crate::BitWriter<'a, DIV_CR_SPEC, O>;
#[doc = "Field `COM` reader - COM"]
pub type COM_R = crate::BitReader;
#[doc = "Field `COM` writer - COM"]
pub type COM_W<'a, const O: u8> = crate::BitWriter<'a, DIV_CR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - ZEF"]
    #[inline(always)]
    pub fn zef(&self) -> ZEF_R {
        ZEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COM"]
    #[inline(always)]
    pub fn com(&self) -> COM_R {
        COM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - START"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 2 - ZEF"]
    #[inline(always)]
    #[must_use]
    pub fn zef(&mut self) -> ZEF_W<2> {
        ZEF_W::new(self)
    }
    #[doc = "Bit 3 - COM"]
    #[inline(always)]
    #[must_use]
    pub fn com(&mut self) -> COM_W<3> {
        COM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DIV_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_cr](index.html) module"]
pub struct DIV_CR_SPEC;
impl crate::RegisterSpec for DIV_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_cr::R](R) reader structure"]
impl crate::Readable for DIV_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div_cr::W](W) writer structure"]
impl crate::Writable for DIV_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIV_CR to value 0"]
impl crate::Resettable for DIV_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
