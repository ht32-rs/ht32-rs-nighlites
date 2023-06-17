#[doc = "Register `CKCU_MCUDBGCR` reader"]
pub struct R(crate::R<CKCU_MCUDBGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_MCUDBGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_MCUDBGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_MCUDBGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_MCUDBGCR` writer"]
pub struct W(crate::W<CKCU_MCUDBGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_MCUDBGCR_SPEC>;
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
impl From<crate::W<CKCU_MCUDBGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_MCUDBGCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBSLP` reader - DBSLP"]
pub type DBSLP_R = crate::BitReader;
#[doc = "Field `DBSLP` writer - DBSLP"]
pub type DBSLP_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_MCUDBGCR_SPEC, O>;
#[doc = "Field `DBDSLP1` reader - DBDSLP1"]
pub type DBDSLP1_R = crate::BitReader;
#[doc = "Field `DBDSLP1` writer - DBDSLP1"]
pub type DBDSLP1_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_MCUDBGCR_SPEC, O>;
#[doc = "Field `DBPD` reader - DBPD"]
pub type DBPD_R = crate::BitReader;
#[doc = "Field `DBPD` writer - DBPD"]
pub type DBPD_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_MCUDBGCR_SPEC, O>;
#[doc = "Field `DBWDT` reader - DBWDT"]
pub type DBWDT_R = crate::BitReader;
#[doc = "Field `DBWDT` writer - DBWDT"]
pub type DBWDT_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_MCUDBGCR_SPEC, O>;
#[doc = "Field `DBGPTM0` reader - DBGPTM0"]
pub type DBGPTM0_R = crate::BitReader;
#[doc = "Field `DBGPTM0` writer - DBGPTM0"]
pub type DBGPTM0_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_MCUDBGCR_SPEC, O>;
#[doc = "Field `DBGPTM1` reader - DBGPTM1"]
pub type DBGPTM1_R = crate::BitReader;
#[doc = "Field `DBGPTM1` writer - DBGPTM1"]
pub type DBGPTM1_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_MCUDBGCR_SPEC, O>;
#[doc = "Field `DBUSART` reader - DBUSART"]
pub type DBUSART_R = crate::BitReader;
#[doc = "Field `DBUSART` writer - DBUSART"]
pub type DBUSART_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_MCUDBGCR_SPEC, O>;
#[doc = "Field `DBSPI` reader - DBSPI"]
pub type DBSPI_R = crate::BitReader;
#[doc = "Field `DBSPI` writer - DBSPI"]
pub type DBSPI_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_MCUDBGCR_SPEC, O>;
#[doc = "Field `DBDSLP2` reader - DBDSLP2"]
pub type DBDSLP2_R = crate::BitReader;
#[doc = "Field `DBDSLP2` writer - DBDSLP2"]
pub type DBDSLP2_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_MCUDBGCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - DBSLP"]
    #[inline(always)]
    pub fn dbslp(&self) -> DBSLP_R {
        DBSLP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DBDSLP1"]
    #[inline(always)]
    pub fn dbdslp1(&self) -> DBDSLP1_R {
        DBDSLP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DBPD"]
    #[inline(always)]
    pub fn dbpd(&self) -> DBPD_R {
        DBPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DBWDT"]
    #[inline(always)]
    pub fn dbwdt(&self) -> DBWDT_R {
        DBWDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - DBGPTM0"]
    #[inline(always)]
    pub fn dbgptm0(&self) -> DBGPTM0_R {
        DBGPTM0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DBGPTM1"]
    #[inline(always)]
    pub fn dbgptm1(&self) -> DBGPTM1_R {
        DBGPTM1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DBUSART"]
    #[inline(always)]
    pub fn dbusart(&self) -> DBUSART_R {
        DBUSART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - DBSPI"]
    #[inline(always)]
    pub fn dbspi(&self) -> DBSPI_R {
        DBSPI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - DBDSLP2"]
    #[inline(always)]
    pub fn dbdslp2(&self) -> DBDSLP2_R {
        DBDSLP2_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBSLP"]
    #[inline(always)]
    #[must_use]
    pub fn dbslp(&mut self) -> DBSLP_W<0> {
        DBSLP_W::new(self)
    }
    #[doc = "Bit 1 - DBDSLP1"]
    #[inline(always)]
    #[must_use]
    pub fn dbdslp1(&mut self) -> DBDSLP1_W<1> {
        DBDSLP1_W::new(self)
    }
    #[doc = "Bit 2 - DBPD"]
    #[inline(always)]
    #[must_use]
    pub fn dbpd(&mut self) -> DBPD_W<2> {
        DBPD_W::new(self)
    }
    #[doc = "Bit 3 - DBWDT"]
    #[inline(always)]
    #[must_use]
    pub fn dbwdt(&mut self) -> DBWDT_W<3> {
        DBWDT_W::new(self)
    }
    #[doc = "Bit 6 - DBGPTM0"]
    #[inline(always)]
    #[must_use]
    pub fn dbgptm0(&mut self) -> DBGPTM0_W<6> {
        DBGPTM0_W::new(self)
    }
    #[doc = "Bit 7 - DBGPTM1"]
    #[inline(always)]
    #[must_use]
    pub fn dbgptm1(&mut self) -> DBGPTM1_W<7> {
        DBGPTM1_W::new(self)
    }
    #[doc = "Bit 8 - DBUSART"]
    #[inline(always)]
    #[must_use]
    pub fn dbusart(&mut self) -> DBUSART_W<8> {
        DBUSART_W::new(self)
    }
    #[doc = "Bit 10 - DBSPI"]
    #[inline(always)]
    #[must_use]
    pub fn dbspi(&mut self) -> DBSPI_W<10> {
        DBSPI_W::new(self)
    }
    #[doc = "Bit 14 - DBDSLP2"]
    #[inline(always)]
    #[must_use]
    pub fn dbdslp2(&mut self) -> DBDSLP2_W<14> {
        DBDSLP2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_MCUDBGCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_mcudbgcr](index.html) module"]
pub struct CKCU_MCUDBGCR_SPEC;
impl crate::RegisterSpec for CKCU_MCUDBGCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_mcudbgcr::R](R) reader structure"]
impl crate::Readable for CKCU_MCUDBGCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_mcudbgcr::W](W) writer structure"]
impl crate::Writable for CKCU_MCUDBGCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_MCUDBGCR to value 0"]
impl crate::Resettable for CKCU_MCUDBGCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
