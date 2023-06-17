#[doc = "Register `EXTI_WAKUPPOLR` reader"]
pub struct R(crate::R<EXTI_WAKUPPOLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_WAKUPPOLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_WAKUPPOLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_WAKUPPOLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_WAKUPPOLR` writer"]
pub struct W(crate::W<EXTI_WAKUPPOLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_WAKUPPOLR_SPEC>;
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
impl From<crate::W<EXTI_WAKUPPOLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_WAKUPPOLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI0POL` reader - EXTI0POL"]
pub type EXTI0POL_R = crate::BitReader;
#[doc = "Field `EXTI0POL` writer - EXTI0POL"]
pub type EXTI0POL_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_WAKUPPOLR_SPEC, O>;
#[doc = "Field `EXTI1POL` reader - EXTI1POL"]
pub type EXTI1POL_R = crate::BitReader;
#[doc = "Field `EXTI1POL` writer - EXTI1POL"]
pub type EXTI1POL_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_WAKUPPOLR_SPEC, O>;
#[doc = "Field `EXTI2POL` reader - EXTI2POL"]
pub type EXTI2POL_R = crate::BitReader;
#[doc = "Field `EXTI2POL` writer - EXTI2POL"]
pub type EXTI2POL_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_WAKUPPOLR_SPEC, O>;
#[doc = "Field `EXTI3POL` reader - EXTI3POL"]
pub type EXTI3POL_R = crate::BitReader;
#[doc = "Field `EXTI3POL` writer - EXTI3POL"]
pub type EXTI3POL_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_WAKUPPOLR_SPEC, O>;
#[doc = "Field `EXTI4POL` reader - EXTI4POL"]
pub type EXTI4POL_R = crate::BitReader;
#[doc = "Field `EXTI4POL` writer - EXTI4POL"]
pub type EXTI4POL_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_WAKUPPOLR_SPEC, O>;
#[doc = "Field `EXTI5POL` reader - EXTI5POL"]
pub type EXTI5POL_R = crate::BitReader;
#[doc = "Field `EXTI5POL` writer - EXTI5POL"]
pub type EXTI5POL_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_WAKUPPOLR_SPEC, O>;
#[doc = "Field `EXTI6POL` reader - EXTI6POL"]
pub type EXTI6POL_R = crate::BitReader;
#[doc = "Field `EXTI6POL` writer - EXTI6POL"]
pub type EXTI6POL_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_WAKUPPOLR_SPEC, O>;
#[doc = "Field `EXTI7POL` reader - EXTI7POL"]
pub type EXTI7POL_R = crate::BitReader;
#[doc = "Field `EXTI7POL` writer - EXTI7POL"]
pub type EXTI7POL_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_WAKUPPOLR_SPEC, O>;
#[doc = "Field `EXTI8POL` reader - EXTI8POL"]
pub type EXTI8POL_R = crate::BitReader;
#[doc = "Field `EXTI8POL` writer - EXTI8POL"]
pub type EXTI8POL_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_WAKUPPOLR_SPEC, O>;
#[doc = "Field `EXTI9POL` reader - EXTI9POL"]
pub type EXTI9POL_R = crate::BitReader;
#[doc = "Field `EXTI9POL` writer - EXTI9POL"]
pub type EXTI9POL_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_WAKUPPOLR_SPEC, O>;
#[doc = "Field `EXTI10POL` reader - EXTI10POL"]
pub type EXTI10POL_R = crate::BitReader;
#[doc = "Field `EXTI10POL` writer - EXTI10POL"]
pub type EXTI10POL_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_WAKUPPOLR_SPEC, O>;
#[doc = "Field `EXTI11POL` reader - EXTI11POL"]
pub type EXTI11POL_R = crate::BitReader;
#[doc = "Field `EXTI11POL` writer - EXTI11POL"]
pub type EXTI11POL_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_WAKUPPOLR_SPEC, O>;
#[doc = "Field `EXTI12POL` reader - EXTI12POL"]
pub type EXTI12POL_R = crate::BitReader;
#[doc = "Field `EXTI12POL` writer - EXTI12POL"]
pub type EXTI12POL_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_WAKUPPOLR_SPEC, O>;
#[doc = "Field `EXTI13POL` reader - EXTI13POL"]
pub type EXTI13POL_R = crate::BitReader;
#[doc = "Field `EXTI13POL` writer - EXTI13POL"]
pub type EXTI13POL_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_WAKUPPOLR_SPEC, O>;
#[doc = "Field `EXTI14POL` reader - EXTI14POL"]
pub type EXTI14POL_R = crate::BitReader;
#[doc = "Field `EXTI14POL` writer - EXTI14POL"]
pub type EXTI14POL_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_WAKUPPOLR_SPEC, O>;
#[doc = "Field `EXTI15POL` reader - EXTI15POL"]
pub type EXTI15POL_R = crate::BitReader;
#[doc = "Field `EXTI15POL` writer - EXTI15POL"]
pub type EXTI15POL_W<'a, const O: u8> = crate::BitWriter<'a, EXTI_WAKUPPOLR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - EXTI0POL"]
    #[inline(always)]
    pub fn exti0pol(&self) -> EXTI0POL_R {
        EXTI0POL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1POL"]
    #[inline(always)]
    pub fn exti1pol(&self) -> EXTI1POL_R {
        EXTI1POL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI2POL"]
    #[inline(always)]
    pub fn exti2pol(&self) -> EXTI2POL_R {
        EXTI2POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI3POL"]
    #[inline(always)]
    pub fn exti3pol(&self) -> EXTI3POL_R {
        EXTI3POL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI4POL"]
    #[inline(always)]
    pub fn exti4pol(&self) -> EXTI4POL_R {
        EXTI4POL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI5POL"]
    #[inline(always)]
    pub fn exti5pol(&self) -> EXTI5POL_R {
        EXTI5POL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI6POL"]
    #[inline(always)]
    pub fn exti6pol(&self) -> EXTI6POL_R {
        EXTI6POL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI7POL"]
    #[inline(always)]
    pub fn exti7pol(&self) -> EXTI7POL_R {
        EXTI7POL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI8POL"]
    #[inline(always)]
    pub fn exti8pol(&self) -> EXTI8POL_R {
        EXTI8POL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI9POL"]
    #[inline(always)]
    pub fn exti9pol(&self) -> EXTI9POL_R {
        EXTI9POL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI10POL"]
    #[inline(always)]
    pub fn exti10pol(&self) -> EXTI10POL_R {
        EXTI10POL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI11POL"]
    #[inline(always)]
    pub fn exti11pol(&self) -> EXTI11POL_R {
        EXTI11POL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTI12POL"]
    #[inline(always)]
    pub fn exti12pol(&self) -> EXTI12POL_R {
        EXTI12POL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTI13POL"]
    #[inline(always)]
    pub fn exti13pol(&self) -> EXTI13POL_R {
        EXTI13POL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTI14POL"]
    #[inline(always)]
    pub fn exti14pol(&self) -> EXTI14POL_R {
        EXTI14POL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTI15POL"]
    #[inline(always)]
    pub fn exti15pol(&self) -> EXTI15POL_R {
        EXTI15POL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0POL"]
    #[inline(always)]
    #[must_use]
    pub fn exti0pol(&mut self) -> EXTI0POL_W<0> {
        EXTI0POL_W::new(self)
    }
    #[doc = "Bit 1 - EXTI1POL"]
    #[inline(always)]
    #[must_use]
    pub fn exti1pol(&mut self) -> EXTI1POL_W<1> {
        EXTI1POL_W::new(self)
    }
    #[doc = "Bit 2 - EXTI2POL"]
    #[inline(always)]
    #[must_use]
    pub fn exti2pol(&mut self) -> EXTI2POL_W<2> {
        EXTI2POL_W::new(self)
    }
    #[doc = "Bit 3 - EXTI3POL"]
    #[inline(always)]
    #[must_use]
    pub fn exti3pol(&mut self) -> EXTI3POL_W<3> {
        EXTI3POL_W::new(self)
    }
    #[doc = "Bit 4 - EXTI4POL"]
    #[inline(always)]
    #[must_use]
    pub fn exti4pol(&mut self) -> EXTI4POL_W<4> {
        EXTI4POL_W::new(self)
    }
    #[doc = "Bit 5 - EXTI5POL"]
    #[inline(always)]
    #[must_use]
    pub fn exti5pol(&mut self) -> EXTI5POL_W<5> {
        EXTI5POL_W::new(self)
    }
    #[doc = "Bit 6 - EXTI6POL"]
    #[inline(always)]
    #[must_use]
    pub fn exti6pol(&mut self) -> EXTI6POL_W<6> {
        EXTI6POL_W::new(self)
    }
    #[doc = "Bit 7 - EXTI7POL"]
    #[inline(always)]
    #[must_use]
    pub fn exti7pol(&mut self) -> EXTI7POL_W<7> {
        EXTI7POL_W::new(self)
    }
    #[doc = "Bit 8 - EXTI8POL"]
    #[inline(always)]
    #[must_use]
    pub fn exti8pol(&mut self) -> EXTI8POL_W<8> {
        EXTI8POL_W::new(self)
    }
    #[doc = "Bit 9 - EXTI9POL"]
    #[inline(always)]
    #[must_use]
    pub fn exti9pol(&mut self) -> EXTI9POL_W<9> {
        EXTI9POL_W::new(self)
    }
    #[doc = "Bit 10 - EXTI10POL"]
    #[inline(always)]
    #[must_use]
    pub fn exti10pol(&mut self) -> EXTI10POL_W<10> {
        EXTI10POL_W::new(self)
    }
    #[doc = "Bit 11 - EXTI11POL"]
    #[inline(always)]
    #[must_use]
    pub fn exti11pol(&mut self) -> EXTI11POL_W<11> {
        EXTI11POL_W::new(self)
    }
    #[doc = "Bit 12 - EXTI12POL"]
    #[inline(always)]
    #[must_use]
    pub fn exti12pol(&mut self) -> EXTI12POL_W<12> {
        EXTI12POL_W::new(self)
    }
    #[doc = "Bit 13 - EXTI13POL"]
    #[inline(always)]
    #[must_use]
    pub fn exti13pol(&mut self) -> EXTI13POL_W<13> {
        EXTI13POL_W::new(self)
    }
    #[doc = "Bit 14 - EXTI14POL"]
    #[inline(always)]
    #[must_use]
    pub fn exti14pol(&mut self) -> EXTI14POL_W<14> {
        EXTI14POL_W::new(self)
    }
    #[doc = "Bit 15 - EXTI15POL"]
    #[inline(always)]
    #[must_use]
    pub fn exti15pol(&mut self) -> EXTI15POL_W<15> {
        EXTI15POL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI_WAKUPPOLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_wakuppolr](index.html) module"]
pub struct EXTI_WAKUPPOLR_SPEC;
impl crate::RegisterSpec for EXTI_WAKUPPOLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_wakuppolr::R](R) reader structure"]
impl crate::Readable for EXTI_WAKUPPOLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_wakuppolr::W](W) writer structure"]
impl crate::Writable for EXTI_WAKUPPOLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTI_WAKUPPOLR to value 0"]
impl crate::Resettable for EXTI_WAKUPPOLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
