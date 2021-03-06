//! Abstractions over the Tegra Security Co-Processor registers.

use register::mmio::ReadWrite;

use crate::memory_map::TSEC;

/// A pointer to the TSEC register block that can be accessed by dereferencing it.
pub const REGISTERS: *const Registers = TSEC as *const Registers;

// TODO: Bitfields.

// TODO: Convert this struct to register_structs! format.

// FIXME: Are really all of them ReadWrite?

/// Representation of the TSEC registers.
#[allow(non_snake_case)]
#[repr(C)]
pub struct Registers {
    pub TSEC_THI_INCR_SYNCPT: ReadWrite<u32>,
    pub TSEC_THI_INCR_SYNCPT_CTRL: ReadWrite<u32>,
    pub TSEC_THI_INCR_SYNCPT_ERR: ReadWrite<u32>,
    pub TSEC_THI_CTXSW_INCR_SYNCPT: ReadWrite<u32>,
    _reserved4: [ReadWrite<u8>; 0x10],
    pub TSEC_THI_CTXSW: ReadWrite<u32>,
    pub TSEC_THI_CTXSW_NEXT: ReadWrite<u32>,
    pub TSEC_THI_CONT_SYNCPT_EOF: ReadWrite<u32>,
    pub TSEC_THI_CONT_SYNCPT_L1: ReadWrite<u32>,
    pub TSEC_THI_STREAMID0: ReadWrite<u32>,
    pub TSEC_THI_STREAMID1: ReadWrite<u32>,
    pub TSEC_THI_THI_SEC: ReadWrite<u32>,
    _reserved11: [ReadWrite<u8>; 0x4],
    pub TSEC_THI_METHOD0: ReadWrite<u32>,
    pub TSEC_THI_METHOD1: ReadWrite<u32>,
    _reserved13: [ReadWrite<u8>; 0x18],
    pub TSEC_THI_CONTEXT_SWITCH: ReadWrite<u32>,
    _reserved14: [ReadWrite<u8>; 0x14],
    pub TSEC_THI_INT_STATUS: ReadWrite<u32>,
    pub TSEC_THI_INT_MASK: ReadWrite<u32>,
    pub TSEC_THI_CONFIG0: ReadWrite<u32>,
    pub TSEC_THI_DBG_MISC: ReadWrite<u32>,
    pub TSEC_THI_SLCG_OVERRIDE_HIGH_A: ReadWrite<u32>,
    pub TSEC_THI_SLCG_OVERRIDE_LOW_A: ReadWrite<u32>,
    _reserved20: [ReadWrite<u8>; 0xD70],
    pub TSEC_THI_CLK_OVERRIDE: ReadWrite<u32>,
    _reserved21: [ReadWrite<u8>; 0x1FC],
    pub FALCON_IRQSSET: ReadWrite<u32>,
    pub FALCON_IRQSCLR: ReadWrite<u32>,
    pub FALCON_IRQSTAT: ReadWrite<u32>,
    pub FALCON_IRQMODE: ReadWrite<u32>,
    pub FALCON_IRQMSET: ReadWrite<u32>,
    pub FALCON_IRQMCLR: ReadWrite<u32>,
    pub FALCON_IRQMASK: ReadWrite<u32>,
    pub FALCON_IRQDEST: ReadWrite<u32>,
    pub FALCON_GPTMRINT: ReadWrite<u32>,
    pub FALCON_GPTMRVAL: ReadWrite<u32>,
    pub FALCON_GPTMRCTL: ReadWrite<u32>,
    pub FALCON_PTIMER0: ReadWrite<u32>,
    pub FALCON_PTIMER1: ReadWrite<u32>,
    pub FALCON_WDTMRVAL: ReadWrite<u32>,
    pub FALCON_WDTMRCTL: ReadWrite<u32>,
    pub FALCON_IRQDEST2: ReadWrite<u32>,
    pub FALCON_MAILBOX0: ReadWrite<u32>,
    pub FALCON_MAILBOX1: ReadWrite<u32>,
    pub FALCON_ITFEN: ReadWrite<u32>,
    pub FALCON_IDLESTATE: ReadWrite<u32>,
    pub FALCON_CURCTX: ReadWrite<u32>,
    pub FALCON_NXTCTX: ReadWrite<u32>,
    pub FALCON_CTXACK: ReadWrite<u32>,
    pub FALCON_FHSTATE: ReadWrite<u32>,
    pub FALCON_PRIVSTATE: ReadWrite<u32>,
    pub FALCON_MTHDDATA: ReadWrite<u32>,
    pub FALCON_MTHDID: ReadWrite<u32>,
    pub FALCON_MTHDWDAT: ReadWrite<u32>,
    pub FALCON_MTHDCOUNT: ReadWrite<u32>,
    pub FALCON_MTHDPOP: ReadWrite<u32>,
    pub FALCON_MTHDRAMSZ: ReadWrite<u32>,
    pub FALCON_SFTRESET: ReadWrite<u32>,
    pub FALCON_OS: ReadWrite<u32>,
    pub FALCON_RM: ReadWrite<u32>,
    pub FALCON_SOFT_PM: ReadWrite<u32>,
    pub FALCON_SOFT_MODE: ReadWrite<u32>,
    pub FALCON_DEBUG1: ReadWrite<u32>,
    pub FALCON_DEBUGINFO: ReadWrite<u32>,
    pub FALCON_IBRKPT1: ReadWrite<u32>,
    pub FALCON_IBRKPT2: ReadWrite<u32>,
    pub FALCON_CGCTL: ReadWrite<u32>,
    pub FALCON_ENGCTL: ReadWrite<u32>,
    pub FALCON_PMM: ReadWrite<u32>,
    pub FALCON_ADDR: ReadWrite<u32>,
    pub FALCON_IBRKPT3: ReadWrite<u32>,
    pub FALCON_IBRKPT4: ReadWrite<u32>,
    pub FALCON_IBRKPT5: ReadWrite<u32>,
    _reserved68: [ReadWrite<u8>; 0x14],
    pub FALCON_EXCI: ReadWrite<u32>,
    pub FALCON_SVEC_SPR: ReadWrite<u32>,
    pub FALCON_RSTAT0: ReadWrite<u32>,
    pub FALCON_RSTAT3: ReadWrite<u32>,
    pub FALCON_UNK_E0: ReadWrite<u32>,
    _reserved73: [ReadWrite<u8>; 0x1C],
    pub FALCON_CPUCTL: ReadWrite<u32>,
    pub FALCON_BOOTVEC: ReadWrite<u32>,
    pub FALCON_HWCFG: ReadWrite<u32>,
    pub FALCON_DMACTL: ReadWrite<u32>,
    pub FALCON_DMATRFBASE: ReadWrite<u32>,
    pub FALCON_DMATRFMOFFS: ReadWrite<u32>,
    pub FALCON_DMATRFCMD: ReadWrite<u32>,
    pub FALCON_DMATRFFBOFFS: ReadWrite<u32>,
    pub FALCON_DMAPOLL_FB: ReadWrite<u32>,
    pub FALCON_DMAPOLL_CP: ReadWrite<u32>,
    pub FALCON_DBG_STATE: ReadWrite<u32>,
    pub FALCON_HWCFG1: ReadWrite<u32>,
    pub FALCON_CPUCTL_ALIAS: ReadWrite<u32>,
    _reserved86: [ReadWrite<u8>; 0x4],
    pub FALCON_STACKCFG: ReadWrite<u32>,
    _reserved87: [ReadWrite<u8>; 0x4],
    pub FALCON_IMCTL: ReadWrite<u32>,
    pub FALCON_IMSTAT: ReadWrite<u32>,
    pub FALCON_TRACEIDX: ReadWrite<u32>,
    pub FALCON_TRACEPC: ReadWrite<u32>,
    pub FALCON_IMFILLRNG0: ReadWrite<u32>,
    pub FALCON_IMFILLRNG1: ReadWrite<u32>,
    pub FALCON_IMFILLCTL: ReadWrite<u32>,
    pub FALCON_IMCTL_DEBUG: ReadWrite<u32>,
    pub FALCON_CMEMBASE: ReadWrite<u32>,
    pub FALCON_DMEMAPERT: ReadWrite<u32>,
    pub FALCON_EXTERRADDR: ReadWrite<u32>,
    pub FALCON_EXTERRSTAT: ReadWrite<u32>,
    _reserved99: [ReadWrite<u8>; 0xC],
    pub FALCON_CG2: ReadWrite<u32>,
    pub FALCON_IMEMC0: ReadWrite<u32>,
    pub FALCON_IMEMD0: ReadWrite<u32>,
    pub FALCON_IMEMT0: ReadWrite<u32>,
    _reserved103: [ReadWrite<u8>; 0x4],
    pub FALCON_IMEMC1: ReadWrite<u32>,
    pub FALCON_IMEMD1: ReadWrite<u32>,
    pub FALCON_IMEMT1: ReadWrite<u32>,
    _reserved106: [ReadWrite<u8>; 0x4],
    pub FALCON_IMEMC2: ReadWrite<u32>,
    pub FALCON_IMEMD2: ReadWrite<u32>,
    pub FALCON_IMEMT2: ReadWrite<u32>,
    _reserved109: [ReadWrite<u8>; 0x4],
    pub FALCON_IMEMC3: ReadWrite<u32>,
    pub FALCON_IMEMD3: ReadWrite<u32>,
    pub FALCON_IMEMT3: ReadWrite<u32>,
    _reserved112: [ReadWrite<u8>; 0x4],
    pub FALCON_DMEMC0: ReadWrite<u32>,
    pub FALCON_DMEMD0: ReadWrite<u32>,
    pub FALCON_DMEMC1: ReadWrite<u32>,
    pub FALCON_DMEMD1: ReadWrite<u32>,
    pub FALCON_DMEMC2: ReadWrite<u32>,
    pub FALCON_DMEMD2: ReadWrite<u32>,
    pub FALCON_DMEMC3: ReadWrite<u32>,
    pub FALCON_DMEMD3: ReadWrite<u32>,
    pub FALCON_DMEMC4: ReadWrite<u32>,
    pub FALCON_DMEMD4: ReadWrite<u32>,
    pub FALCON_DMEMC5: ReadWrite<u32>,
    pub FALCON_DMEMD5: ReadWrite<u32>,
    pub FALCON_DMEMC6: ReadWrite<u32>,
    pub FALCON_DMEMD6: ReadWrite<u32>,
    pub FALCON_DMEMC7: ReadWrite<u32>,
    pub FALCON_DMEMD7: ReadWrite<u32>,
    pub FALCON_ICD_CMD: ReadWrite<u32>,
    pub FALCON_ICD_ADDR: ReadWrite<u32>,
    pub FALCON_ICD_WDATA: ReadWrite<u32>,
    pub FALCON_ICD_RDATA: ReadWrite<u32>,
    _reserved132: [ReadWrite<u8>; 0x30],
    pub FALCON_SCTL: ReadWrite<u32>,
    pub FALCON_SSTAT: ReadWrite<u32>,
    pub FALCON_UNK_248: ReadWrite<u32>,
    pub FALCON_UNK_24C: ReadWrite<u32>,
    pub FALCON_UNK_250: ReadWrite<u32>,
    _reserved137: [ReadWrite<u8>; 0xC],
    pub FALCON_UNK_260: ReadWrite<u32>,
    _reserved138: [ReadWrite<u8>; 0x1C],
    pub FALCON_SPROT_IMEM: ReadWrite<u32>,
    pub FALCON_SPROT_DMEM: ReadWrite<u32>,
    pub FALCON_SPROT_CPUCTL: ReadWrite<u32>,
    pub FALCON_SPROT_MISC: ReadWrite<u32>,
    pub FALCON_SPROT_IRQ: ReadWrite<u32>,
    pub FALCON_SPROT_MTHD: ReadWrite<u32>,
    pub FALCON_SPROT_SCTL: ReadWrite<u32>,
    pub FALCON_SPROT_WDTMR: ReadWrite<u32>,
    _reserved146: [ReadWrite<u8>; 0x20],
    pub FALCON_DMAINFO_FINISHED_FBRD_LOW: ReadWrite<u32>,
    pub FALCON_DMAINFO_FINISHED_FBRD_HIGH: ReadWrite<u32>,
    pub FALCON_DMAINFO_FINISHED_FBWR_LOW: ReadWrite<u32>,
    pub FALCON_DMAINFO_FINISHED_FBWR_HIGH: ReadWrite<u32>,
    pub FALCON_DMAINFO_CURRENT_FBRD_LOW: ReadWrite<u32>,
    pub FALCON_DMAINFO_CURRENT_FBRD_HIGH: ReadWrite<u32>,
    pub FALCON_DMAINFO_CURRENT_FBWR_LOW: ReadWrite<u32>,
    pub FALCON_DMAINFO_CURRENT_FBWR_HIGH: ReadWrite<u32>,
    pub FALCON_DMAINFO_CTL: ReadWrite<u32>,
    _reserved155: [ReadWrite<u8>; 0x11C],
    pub TSEC_SCP_CTL0: ReadWrite<u32>,
    pub TSEC_SCP_CTL1: ReadWrite<u32>,
    pub TSEC_SCP_CTL_STAT: ReadWrite<u32>,
    pub TSEC_SCP_CTL_LOCK: ReadWrite<u32>,
    pub TSEC_SCP_UNK_10: ReadWrite<u32>,
    pub TSEC_SCP_UNK_14: ReadWrite<u32>,
    pub TSEC_SCP_CTL_PKEY: ReadWrite<u32>,
    pub TSEC_SCP_UNK_1C: ReadWrite<u32>,
    pub TSEC_SCP_SEQ_CTL: ReadWrite<u32>,
    pub TSEC_SCP_SEQ_VAL: ReadWrite<u32>,
    pub TSEC_SCP_SEQ_STAT: ReadWrite<u32>,
    _reserved166: [ReadWrite<u8>; 0x4],
    pub TSEC_SCP_INSN_STAT: ReadWrite<u32>,
    _reserved167: [ReadWrite<u8>; 0x1c],
    pub TSEC_SCP_UNK_50: ReadWrite<u32>,
    pub TSEC_SCP_AUTH_STAT: ReadWrite<u32>,
    pub TSEC_SCP_AES_STAT: ReadWrite<u32>,
    _reserved170: [ReadWrite<u8>; 0x14],
    pub TSEC_SCP_UNK_70: ReadWrite<u32>,
    _reserved171: [ReadWrite<u8>; 0xC],
    pub TSEC_SCP_IRQSTAT: ReadWrite<u32>,
    pub TSEC_SCP_IRQMASK: ReadWrite<u32>,
    _reserved173: [ReadWrite<u8>; 0x8],
    pub TSEC_SCP_ACL_ERR: ReadWrite<u32>,
    pub TSEC_SCP_UNK_94: ReadWrite<u32>,
    pub TSEC_SCP_INSN_ERR: ReadWrite<u32>,
    _reserved176: [ReadWrite<u8>; 0x64],
    pub TSEC_TRNG_CLK_LIMIT_LOW: ReadWrite<u32>,
    pub TSEC_TRNG_CLK_LIMIT_HIGH: ReadWrite<u32>,
    pub TSEC_TRNG_UNK_08: ReadWrite<u32>,
    pub TSEC_TRNG_TEST_CTL: ReadWrite<u32>,
    pub TSEC_TRNG_TEST_CFG0: ReadWrite<u32>,
    pub TSEC_TRNG_TEST_SEED0: ReadWrite<u32>,
    pub TSEC_TRNG_TEST_CFG1: ReadWrite<u32>,
    pub TSEC_TRNG_TEST_SEED1: ReadWrite<u32>,
    pub TSEC_TRNG_UNK_20: ReadWrite<u32>,
    pub TSEC_TRNG_UNK_24: ReadWrite<u32>,
    pub TSEC_TRNG_UNK_28: ReadWrite<u32>,
    pub TSEC_TRNG_CTL: ReadWrite<u32>,
    _reserved188: [ReadWrite<u8>; 0xD0],
    pub TSEC_TFBIF_CTL: ReadWrite<u32>,
    pub TSEC_TFBIF_MCCIF_FIFOCTRL: ReadWrite<u32>,
    pub TSEC_TFBIF_THROTTLE: ReadWrite<u32>,
    pub TSEC_TFBIF_DBG_STAT0: ReadWrite<u32>,
    pub TSEC_TFBIF_DBG_STAT1: ReadWrite<u32>,
    pub TSEC_TFBIF_DBG_RDCOUNT_LO: ReadWrite<u32>,
    pub TSEC_TFBIF_DBG_RDCOUNT_HI: ReadWrite<u32>,
    pub TSEC_TFBIF_DBG_WRCOUNT_LO: ReadWrite<u32>,
    pub TSEC_TFBIF_DBG_WRCOUNT_HI: ReadWrite<u32>,
    pub TSEC_TFBIF_DBG_R32COUNT: ReadWrite<u32>,
    pub TSEC_TFBIF_DBG_R64COUNT: ReadWrite<u32>,
    pub TSEC_TFBIF_DBG_R128COUNT: ReadWrite<u32>,
    pub TSEC_TFBIF_UNK_30: ReadWrite<u32>,
    pub TSEC_TFBIF_MCCIF_FIFOCTRL1: ReadWrite<u32>,
    pub TSEC_TFBIF_WRR_RDP: ReadWrite<u32>,
    _reserved203: [ReadWrite<u8>; 0x4],
    pub TSEC_TFBIF_SPROT_EMEM: ReadWrite<u32>,
    pub TSEC_TFBIF_TRANSCFG: ReadWrite<u32>,
    pub TSEC_TFBIF_REGIONCFG: ReadWrite<u32>,
    pub TSEC_TFBIF_ACTMON_ACTIVE_MASK: ReadWrite<u32>,
    pub TSEC_TFBIF_ACTMON_ACTIVE_BORPS: ReadWrite<u32>,
    pub TSEC_TFBIF_ACTMON_ACTIVE_WEIGHT: ReadWrite<u32>,
    _reserved209: [ReadWrite<u8>; 0x8],
    pub TSEC_TFBIF_ACTMON_MCB_MASK: ReadWrite<u32>,
    pub TSEC_TFBIF_ACTMON_MCB_BORPS: ReadWrite<u32>,
    pub TSEC_TFBIF_ACTMON_MCB_WEIGHT: ReadWrite<u32>,
    _reserved212: [ReadWrite<u8>; 0x4],
    pub TSEC_TFBIF_THI_TRANSPROP: ReadWrite<u32>,
    _reserved213: [ReadWrite<u8>; 0x5C],
    pub TSEC_CG: ReadWrite<u32>,
    _reserved214: [ReadWrite<u8>; 0x2C],
    pub TSEC_BAR0_CTL: ReadWrite<u32>,
    pub TSEC_BAR0_ADDR: ReadWrite<u32>,
    pub TSEC_BAR0_DATA: ReadWrite<u32>,
    pub TSEC_BAR0_TIMEOUT: ReadWrite<u32>,
    _reserved218: [ReadWrite<u8>; 0xF0],
    pub TSEC_TEGRA_FALCON_IP_VER: ReadWrite<u32>,
    pub TSEC_TEGRA_UNK_04: ReadWrite<u32>,
    pub TSEC_TEGRA_UNK_08: ReadWrite<u32>,
    pub TSEC_TEGRA_UNK_0C: ReadWrite<u32>,
    pub TSEC_TEGRA_UNK_10: ReadWrite<u32>,
    pub TSEC_TEGRA_UNK_14: ReadWrite<u32>,
    pub TSEC_TEGRA_UNK_18: ReadWrite<u32>,
    pub TSEC_TEGRA_UNK_1C: ReadWrite<u32>,
    pub TSEC_TEGRA_UNK_20: ReadWrite<u32>,
    pub TSEC_TEGRA_UNK_24: ReadWrite<u32>,
    pub TSEC_TEGRA_UNK_28: ReadWrite<u32>,
    pub TSEC_TEGRA_UNK_2C: ReadWrite<u32>,
    pub TSEC_TEGRA_UNK_30: ReadWrite<u32>,
    pub TSEC_TEGRA_UNK_34: ReadWrite<u32>,
    pub TSEC_TEGRA_CTL: ReadWrite<u32>,
}
