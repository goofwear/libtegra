//! Abstractions over the PMC registers of the Tegra X1.
//!
//! See Chapter 12.6 in the Tegra X1 Technical Reference Manual
//! for details.

use register::mmio::*;

use crate::memory_map::PMC;

/// A pointer to the PMC register block that can be accessed by dereferencing it.
pub const REGISTERS: *const Registers = PMC as *const Registers;

// TODO: Bitfields.

// TODO: Convert this struct to register_structs! format.

/// Representation of the PMC registers.
#[allow(non_snake_case)]
#[repr(C)]
pub struct Registers {
    pub APBDEV_PMC_CNTRL_0: ReadWrite<u32>,
    pub APBDEV_PMC_SEC_DISABLE_0: ReadWrite<u32>,
    pub APBDEV_PMC_PMC_SWRST_0: ReadWrite<u32>,
    pub APBDEV_PMC_WAKE_MASK_0: ReadWrite<u32>,
    pub APBDEV_PMC_WAKE_LVL_0: ReadWrite<u32>,
    pub APBDEV_PMC_WAKE_STATUS_0: ReadWrite<u32>,
    pub APBDEV_PMC_SW_WAKE_STATUS_0: ReadWrite<u32>,
    pub APBDEV_PMC_DPD_PADS_ORIDE_0: ReadWrite<u32>,
    pub APBDEV_PMC_DPD_SAMPLE_0: ReadWrite<u32>,
    pub APBDEV_PMC_DPD_ENABLE_0: ReadWrite<u32>,
    pub APBDEV_PMC_PWRGATE_TIMER_OFF_0: ReadWrite<u32>,
    pub APBDEV_PMC_CLAMP_STATUS_0: ReadOnly<u32>,
    pub APBDEV_PMC_PWRGATE_TOGGLE_0: ReadWrite<u32>,
    pub APBDEV_PMC_REMOVE_CLAMPING_CMD_0: ReadWrite<u32>,
    pub APBDEV_PMC_PWRGATE_STATUS_0: ReadOnly<u32>,
    pub APBDEV_PMC_PWRGOOD_TIMER_0: ReadWrite<u32>,
    pub APBDEV_PMC_BLINK_TIMER_0: ReadWrite<u32>,
    pub APBDEV_PMC_NO_IOPOWER_0: ReadWrite<u32>,
    pub APBDEV_PMC_PWR_DET_0: ReadWrite<u32>,
    pub APBDEV_PMC_PWR_DET_LATCH_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH0_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH1_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH2_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH3_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH4_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH5_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH6_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH7_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH8_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH9_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH10_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH11_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH12_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH13_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH14_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH15_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH16_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH17_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH18_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH19_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH20_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH21_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH22_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH23_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH0_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH1_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH2_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH3_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH4_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH5_0: ReadWrite<u32>,
    pub APBDEV_PMC_CPUPWRGOOD_TIMER_0: ReadWrite<u32>,
    pub APBDEV_PMC_CPUPWROFF_TIMER_0: ReadWrite<u32>,
    pub APBDEV_PMC_PG_MASK_0: ReadWrite<u32>,
    pub APBDEV_PMC_PG_MASK_1_0: ReadWrite<u32>,
    pub APBDEV_PMC_AUTO_WAKE_LVL_0: ReadWrite<u32>,
    pub APBDEV_PMC_AUTO_WAKE_LVL_MASK_0: ReadWrite<u32>,
    pub APBDEV_PMC_WAKE_DELAY_0: ReadWrite<u32>,
    pub APBDEV_PMC_PWR_DET_VAL_0: ReadWrite<u32>,
    pub APBDEV_PMC_DDR_PWR_0: ReadWrite<u32>,
    pub APBDEV_PMC_USB_DEBOUNCE_DEL_0: ReadWrite<u32>,
    pub APBDEV_PMC_USB_AO_0: ReadWrite<u32>,
    pub APBDEV_PMC_CRYPTO_OP_0: ReadWrite<u32>,
    pub APBDEV_PMC_PLLP_WB0_OVERRIDE_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH24_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH25_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH26_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH27_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH28_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH29_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH30_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH31_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH32_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH33_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH34_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH35_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH36_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH37_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH38_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH39_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH40_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH41_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH42_0: ReadWrite<u32>,
    pub APBDEV_PMC_BONDOUT_MIRROR0_0: ReadWrite<u32>,
    pub APBDEV_PMC_BONDOUT_MIRROR1_0: ReadWrite<u32>,
    pub APBDEV_PMC_BONDOUT_MIRROR2_0: ReadWrite<u32>,
    pub APBDEV_PMC_SYS_33V_EN_0: ReadWrite<u32>,
    pub APBDEV_PMC_BONDOUT_MIRROR_ACCESS_0: ReadWrite<u32>,
    pub APBDEV_PMC_GATE_0: ReadWrite<u32>,
    pub APBDEV_PMC_WAKE2_MASK_0: ReadWrite<u32>,
    pub APBDEV_PMC_WAKE2_LVL_0: ReadWrite<u32>,
    pub APBDEV_PMC_WAKE2_STATUS_0: ReadWrite<u32>,
    pub APBDEV_PMC_SW_WAKE2_STATUS_0: ReadWrite<u32>,
    pub APBDEV_PMC_AUTO_WAKE2_LVL_MASK_0: ReadWrite<u32>,
    pub APBDEV_PMC_PG_MASK_2_0: ReadWrite<u32>,
    pub APBDEV_PMC_PG_MASK_CE1_0: ReadWrite<u32>,
    pub APBDEV_PMC_PG_MASK_CE2_0: ReadWrite<u32>,
    pub APBDEV_PMC_PG_MASK_CE3_0: ReadWrite<u32>,
    pub APBDEV_PMC_PWRGATE_TIMER_CE_0_0: ReadWrite<u32>,
    pub APBDEV_PMC_PWRGATE_TIMER_CE_1_0: ReadWrite<u32>,
    pub APBDEV_PMC_PWRGATE_TIMER_CE_2_0: ReadWrite<u32>,
    pub APBDEV_PMC_PWRGATE_TIMER_CE_3_0: ReadWrite<u32>,
    pub APBDEV_PMC_PWRGATE_TIMER_CE_4_0: ReadWrite<u32>,
    pub APBDEV_PMC_PWRGATE_TIMER_CE_5_0: ReadWrite<u32>,
    pub APBDEV_PMC_PWRGATE_TIMER_CE_6_0: ReadWrite<u32>,
    pub APBDEV_PMC_PCX_EDPD_CNTRL_0: ReadWrite<u32>,
    pub APBDEV_PMC_OSC_EDPD_OVER_0: ReadWrite<u32>,
    pub APBDEV_PMC_CLK_OUT_CNTRL_0: ReadWrite<u32>,
    pub APBDEV_PMC_SATA_PWRGT_0: ReadWrite<u32>,
    pub APBDEV_PMC_SENSOR_CTRL_0: ReadWrite<u32>,
    pub APBDEV_PMC_RST_STATUS_0: ReadWrite<u32>,
    pub APBDEV_PMC_IO_DPD_REQ_0: ReadWrite<u32>,
    pub APBDEV_PMC_IO_DPD_STATUS_0: ReadWrite<u32>,
    pub APBDEV_PMC_IO_DPD2_REQ_0: ReadWrite<u32>,
    pub APBDEV_PMC_IO_DPD2_STATUS_0: ReadWrite<u32>,
    pub APBDEV_PMC_SEL_DPD_TIM_0: ReadWrite<u32>,
    pub APBDEV_PMC_VDDP_SEL_0: ReadWrite<u32>,
    pub APBDEV_PMC_DDR_CFG_0: ReadWrite<u32>,
    _reserved0: [ReadWrite<u8>; 0x8],
    pub APBDEV_PMC_PLLM_WB0_OVERRIDE_FREQ_0: ReadWrite<u32>,
    _reserved1: [ReadWrite<u8>; 0x4],
    pub APBDEV_PMC_PWRGATE_TIMER_MULT_0: ReadWrite<u32>,
    pub APBDEV_PMC_DSI_SEL_DPD_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_UHSIC_TRIGGERS_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_UHSIC_SAVED_STATE_0: ReadWrite<u32>,
    _reserved2: [ReadWrite<u8>; 0x4],
    pub APBDEV_PMC_UTMIP_TERM_PAD_CFG_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_UHSIC_SLEEP_CFG_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_UHSIC_SLEEPWALK_CFG_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_SLEEPWALK_P0_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_SLEEPWALK_P1_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_SLEEPWALK_P2_0: ReadWrite<u32>,
    pub APBDEV_PMC_UHSIC_SLEEPWALK_P0_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_UHSIC_STATUS_0: ReadOnly<u32>,
    pub APBDEV_PMC_UTMIP_UHSIC_FAKE_0: ReadWrite<u32>,
    pub APBDEV_PMC_BONDOUT_MIRROR3_0: ReadWrite<u32>,
    pub APBDEV_PMC_BONDOUT_MIRROR4_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH6_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH7_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH43_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH44_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH45_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH46_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH47_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH48_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH49_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH50_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH51_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH52_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH53_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH54_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH55_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH0_ECO_0: ReadWrite<u32>,
    pub APBDEV_PMC_POR_DPD_CTRL_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH2_ECO_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_UHSIC_LINE_WAKEUP_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_BIAS_MASTER_CNTRL_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_MASTER_CONFIG_0: ReadWrite<u32>,
    pub APBDEV_PMC_TD_PWRGATE_INTER_PART_TIMER_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_UHSIC2_TRIGGERS_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_UHSIC2_SAVED_STATE_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_UHSIC2_SLEEP_CFG_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_UHSIC2_SLEEPWALK_CFG_0: ReadWrite<u32>,
    pub APBDEV_PMC_UHSIC2_SLEEPWALK_P1_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_UHSIC2_STATUS_0: ReadOnly<u32>,
    pub APBDEV_PMC_UTMIP_UHSIC2_FAKE_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_UHSIC2_LINE_WAKEUP_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_MASTER2_CONFIG_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_UHSIC_RPD_CFG_0: ReadWrite<u32>,
    pub APBDEV_PMC_PG_MASK_CE0_0: ReadWrite<u32>,
    pub APBDEV_PMC_PG_MASK3_0: ReadWrite<u32>,
    pub APBDEV_PMC_PG_MASK_4_0: ReadWrite<u32>,
    pub APBDEV_PMC_PLLM_WB0_OVERRIDE2_0: ReadWrite<u32>,
    pub APBDEV_PMC_TSC_MULT_0: ReadWrite<u32>,
    pub APBDEV_PMC_CPU_VSENSE_OVERRIDE_0: ReadWrite<u32>,
    pub APBDEV_PMC_GLB_AMAP_CFG_0: ReadWrite<u32>,
    pub APBDEV_PMC_STICKY_BITS_0: ReadWrite<u32>,
    pub APBDEV_PMC_SEC_DISABLE2_0: ReadWrite<u32>,
    pub APBDEV_PMC_WEAK_BIAS_0: ReadWrite<u32>,
    pub APBDEV_PMC_REG_SHORT_0: ReadWrite<u32>,
    pub APBDEV_PMC_PG_MASK_ANDOR_0: ReadWrite<u32>,
    pub APBDEV_PMC_GPU_RG_CNTRL_0: ReadWrite<u32>,
    pub APBDEV_PMC_SEC_DISABLE3_0: ReadWrite<u32>,
    pub APBDEV_PMC_PG_MASK_5_0: ReadWrite<u32>,
    pub APBDEV_PMC_PG_MASK_6_0: ReadWrite<u32>,
    _reserved3: [ReadWrite<u8>; 0x1C],
    pub APBDEV_PMC_SECURE_SCRATCH8_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH9_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH10_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH11_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH12_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH13_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH14_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH15_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH16_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH17_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH18_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH19_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH20_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH21_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH22_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH23_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH24_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH25_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH26_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH27_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH28_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH29_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH30_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH31_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH32_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH33_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH34_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH35_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH36_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH37_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH38_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH39_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH40_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH41_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH42_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH43_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH44_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH45_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH46_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH47_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH48_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH49_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH50_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH51_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH52_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH53_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH54_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH55_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH56_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH57_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH58_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH59_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH60_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH61_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH62_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH63_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH64_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH65_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH66_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH67_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH68_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH69_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH70_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH71_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH72_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH73_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH74_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH75_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH76_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH77_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH78_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH79_0: ReadWrite<u32>,
    _reserved4: [ReadWrite<u8>; 0x20],
    pub APBDEV_PMC_CNTRL2_0: ReadWrite<u32>,
    pub APBDEV_PMC_IO_DPD_OFF_MASK_0: ReadWrite<u32>,
    pub APBDEV_PMC_IO_DPD2_OFF_MASK_0: ReadWrite<u32>,
    pub APBDEV_PMC_EVENT_COUNTER_0: ReadWrite<u32>,
    pub APBDEV_PMC_FUSE_CONTROL_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH1_ECO_0: ReadWrite<u32>,
    _reserved5: [ReadWrite<u8>; 0x4],
    pub APBDEV_PMC_IO_DPD3_REQ_0: ReadWrite<u32>,
    pub APBDEV_PMC_IO_DPD3_STATUS_0: ReadWrite<u32>,
    pub APBDEV_PMC_IO_DPD4_REQ_0: ReadWrite<u32>,
    pub APBDEV_PMC_IO_DPD4_STATUS_0: ReadWrite<u32>,
    _reserved6: [ReadWrite<u8>; 0x8],
    pub APBDEV_PMC_DIRECT_THERMTRIP_CFG_0: ReadWrite<u32>,
    pub APBDEV_PMC_TSOSC_DELAY_0: ReadWrite<u32>,
    pub APBDEV_PMC_SET_SW_CLAMP_0: ReadWrite<u32>,
    pub APBDEV_PMC_DEBUG_AUTHENTICATION_0: ReadWrite<u32>,
    pub APBDEV_PMC_AOTAG_CFG_0: ReadWrite<u32>,
    pub APBDEV_PMC_AOTAG_THRESH1_CFG_0: ReadWrite<u32>,
    pub APBDEV_PMC_AOTAG_THRESH2_CFG_0: ReadWrite<u32>,
    pub APBDEV_PMC_AOTAG_THRESH3_CFG_0: ReadWrite<u32>,
    pub APBDEV_PMC_AOTAG_STATUS_0: ReadOnly<u32>,
    pub APBDEV_PMC_AOTAG_SECURITY_0: ReadWrite<u32>,
    pub APBDEV_PMC_TSENSOR_CONFIG0_0: ReadWrite<u32>,
    pub APBDEV_PMC_TSENSOR_CONFIG1_0: ReadWrite<u32>,
    pub APBDEV_PMC_TSENSOR_CONFIG2_0: ReadWrite<u32>,
    pub APBDEV_PMC_TSENSOR_STATUS0_0: ReadOnly<u32>,
    pub APBDEV_PMC_TSENSOR_STATUS1_0: ReadOnly<u32>,
    pub APBDEV_PMC_TSENSOR_STATUS2_0: ReadOnly<u32>,
    pub APBDEV_PMC_TSENSOR_PDIV_0: ReadWrite<u32>,
    pub APBDEV_PMC_AOTAG_INTR_EN_0: ReadWrite<u32>,
    pub APBDEV_PMC_AOTAG_INTR_DIS_0: WriteOnly<u32>,
    pub APBDEV_PMC_UTMIP_PAD_CFG0_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_PAD_CFG1_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_PAD_CFG2_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_PAD_CFG3_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_UHSIC_SLEEP_CFG1_0: ReadWrite<u32>,
    pub APBDEV_PMC_CC4_HVC_CONTROL_0: ReadWrite<u32>,
    pub APBDEV_PMC_WAKE_DEBOUNCE_EN_0: ReadWrite<u32>,
    pub APBDEV_PMC_RAMDUMP_CTL_STATUS_0: ReadWrite<u32>,
    pub APBDEV_PMC_UTMIP_SLEEPWALK_P3_0: ReadWrite<u32>,
    pub APBDEV_PMC_DDR_CNTRL_0: ReadWrite<u32>,
    _reserved7: [ReadWrite<u8>; 0xC8],
    pub APBDEV_PMC_SEC_DISABLE4_0: ReadWrite<u32>,
    pub APBDEV_PMC_SEC_DISABLE5_0: ReadWrite<u32>,
    pub APBDEV_PMC_SEC_DISABLE6_0: ReadWrite<u32>,
    pub APBDEV_PMC_SEC_DISABLE7_0: ReadWrite<u32>,
    pub APBDEV_PMC_SEC_DISABLE8_0: ReadWrite<u32>,
    _reserved8: [ReadWrite<u8>; 0x3C],
    pub APBDEV_PMC_SCRATCH56_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH57_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH58_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH59_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH60_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH61_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH62_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH63_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH64_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH65_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH66_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH67_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH68_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH69_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH70_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH71_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH72_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH73_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH74_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH75_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH76_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH77_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH78_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH79_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH80_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH81_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH82_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH83_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH84_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH85_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH86_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH87_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH88_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH89_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH90_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH91_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH92_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH93_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH94_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH95_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH96_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH97_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH98_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH99_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH100_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH101_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH102_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH103_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH104_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH105_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH106_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH107_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH108_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH109_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH110_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH111_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH112_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH113_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH114_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH115_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH116_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH117_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH118_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH119_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH120_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH121_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH122_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH123_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH124_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH125_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH126_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH127_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH128_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH129_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH130_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH131_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH132_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH133_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH134_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH135_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH136_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH137_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH138_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH139_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH140_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH141_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH142_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH143_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH144_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH145_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH146_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH147_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH148_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH149_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH150_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH151_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH152_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH153_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH154_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH155_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH156_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH157_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH158_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH159_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH160_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH161_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH162_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH163_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH164_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH165_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH166_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH167_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH168_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH169_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH170_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH171_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH172_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH173_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH174_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH175_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH176_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH177_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH178_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH179_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH180_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH181_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH182_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH183_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH184_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH185_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH186_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH187_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH188_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH189_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH190_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH191_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH192_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH193_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH194_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH195_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH196_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH197_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH198_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH199_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH200_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH201_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH202_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH203_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH204_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH205_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH206_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH207_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH208_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH209_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH210_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH211_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH212_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH213_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH214_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH215_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH216_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH217_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH218_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH219_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH220_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH221_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH222_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH223_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH224_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH225_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH226_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH227_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH228_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH229_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH230_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH231_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH232_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH233_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH234_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH235_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH236_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH237_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH238_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH239_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH240_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH241_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH242_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH243_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH244_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH245_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH246_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH247_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH248_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH249_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH250_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH251_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH252_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH253_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH254_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH255_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH256_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH257_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH258_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH259_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH260_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH261_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH262_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH263_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH264_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH265_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH266_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH267_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH268_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH269_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH270_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH271_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH272_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH273_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH274_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH275_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH276_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH277_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH278_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH279_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH280_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH281_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH282_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH283_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH284_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH285_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH286_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH287_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH288_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH289_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH290_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH291_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH292_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH293_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH294_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH295_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH296_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH297_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH298_0: ReadWrite<u32>,
    pub APBDEV_PMC_SCRATCH299_0: ReadWrite<u32>,
    _reserved9: [ReadWrite<u8>; 0xC8],
    pub APBDEV_PMC_SECURE_SCRATCH80_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH81_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH82_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH83_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH84_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH85_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH86_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH87_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH88_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH89_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH90_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH91_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH92_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH93_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH94_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH95_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH96_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH97_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH98_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH99_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH100_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH101_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH102_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH103_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH104_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH105_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH106_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH107_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH108_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH109_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH110_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH111_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH112_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH113_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH114_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH115_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH116_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH117_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH118_0: ReadWrite<u32>,
    pub APBDEV_PMC_SECURE_SCRATCH119_0: ReadWrite<u32>,
}

assert_eq_size!(Registers, [u8; 0xB38]);