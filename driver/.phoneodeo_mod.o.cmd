savedcmd_/home/recurse/driver/phoneodeo_mod.o :=  gcc-12 -Wp,-MMD,/home/recurse/driver/.phoneodeo_mod.o.d -nostdinc -I/usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include -I./arch/arm/include/generated -I/usr/src/linux-headers-6.6.31+rpt-common-rpi/include -I./include -I/usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/uapi -I./arch/arm/include/generated/uapi -I/usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi -I./include/generated/uapi -include /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/compiler-version.h -include /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kconfig.h -include /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/compiler_types.h -D__KERNEL__ -mlittle-endian -D__LINUX_ARM_ARCH__=6 -fmacro-prefix-map=/usr/src/linux-headers-6.6.31+rpt-common-rpi/= -std=gnu11 -fshort-wchar -funsigned-char -fno-common -fno-PIE -fno-strict-aliasing -fno-dwarf2-cfi-asm -mno-fdpic -fno-ipa-sra -mtp=cp15 -mabi=aapcs-linux -mfpu=vfp -funwind-tables -marm -Wa,-mno-warn-deprecated -march=armv6k -mtune=arm1136j-s -msoft-float -Uarm -fno-delete-null-pointer-checks -O2 -fno-allow-store-data-races -fstack-protector-strong -ftrivial-auto-var-init=zero -fno-stack-clash-protection -pg -fno-strict-overflow -fno-stack-check -fconserve-stack -Wall -Wundef -Werror=implicit-function-declaration -Werror=implicit-int -Werror=return-type -Werror=strict-prototypes -Wno-format-security -Wno-trigraphs -Wno-frame-address -Wno-address-of-packed-member -Wframe-larger-than=1024 -Wno-main -Wno-unused-but-set-variable -Wno-unused-const-variable -Wno-dangling-pointer -Wvla -Wno-pointer-sign -Wcast-function-type -Wno-array-bounds -Wno-alloc-size-larger-than -Wimplicit-fallthrough=5 -Werror=date-time -Werror=incompatible-pointer-types -Werror=designated-init -Wenum-conversion -Wno-unused-but-set-variable -Wno-unused-const-variable -Wno-restrict -Wno-packed-not-aligned -Wno-format-overflow -Wno-format-truncation -Wno-stringop-overflow -Wno-stringop-truncation -Wno-missing-field-initializers -Wno-type-limits -Wno-shift-negative-value -Wno-maybe-uninitialized -Wno-sign-compare -mstack-protector-guard=tls -mstack-protector-guard-offset=1192  -DMODULE  -DKBUILD_BASENAME='"phoneodeo_mod"' -DKBUILD_MODNAME='"phoneodeo_mod"' -D__KBUILD_MODNAME=kmod_phoneodeo_mod -c -o /home/recurse/driver/phoneodeo_mod.o /home/recurse/driver/phoneodeo_mod.c  

source_/home/recurse/driver/phoneodeo_mod.o := /home/recurse/driver/phoneodeo_mod.c

deps_/home/recurse/driver/phoneodeo_mod.o := \
    $(wildcard include/config/OF) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/compiler-version.h \
    $(wildcard include/config/CC_VERSION_TEXT) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kconfig.h \
    $(wildcard include/config/CPU_BIG_ENDIAN) \
    $(wildcard include/config/BOOGER) \
    $(wildcard include/config/FOO) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/compiler_types.h \
    $(wildcard include/config/DEBUG_INFO_BTF) \
    $(wildcard include/config/PAHOLE_HAS_BTF_TAG) \
    $(wildcard include/config/FUNCTION_ALIGNMENT) \
    $(wildcard include/config/CC_IS_GCC) \
    $(wildcard include/config/X86_64) \
    $(wildcard include/config/ARM64) \
    $(wildcard include/config/HAVE_ARCH_COMPILER_H) \
    $(wildcard include/config/CC_HAS_ASM_INLINE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/compiler_attributes.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/compiler-gcc.h \
    $(wildcard include/config/RETPOLINE) \
    $(wildcard include/config/GCC_ASM_GOTO_OUTPUT_WORKAROUND) \
    $(wildcard include/config/ARCH_USE_BUILTIN_BSWAP) \
    $(wildcard include/config/SHADOW_CALL_STACK) \
    $(wildcard include/config/KCOV) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/moduleparam.h \
    $(wildcard include/config/ALPHA) \
    $(wildcard include/config/IA64) \
    $(wildcard include/config/PPC64) \
    $(wildcard include/config/SYSFS) \
    $(wildcard include/config/MODULES) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/init.h \
    $(wildcard include/config/HAVE_ARCH_PREL32_RELOCATIONS) \
    $(wildcard include/config/STRICT_KERNEL_RWX) \
    $(wildcard include/config/STRICT_MODULE_RWX) \
    $(wildcard include/config/LTO_CLANG) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/build_bug.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/compiler.h \
    $(wildcard include/config/TRACE_BRANCH_PROFILING) \
    $(wildcard include/config/PROFILE_ALL_BRANCHES) \
    $(wildcard include/config/OBJTOOL) \
  arch/arm/include/generated/asm/rwonce.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/rwonce.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kasan-checks.h \
    $(wildcard include/config/KASAN_GENERIC) \
    $(wildcard include/config/KASAN_SW_TAGS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/types.h \
    $(wildcard include/config/HAVE_UID16) \
    $(wildcard include/config/UID16) \
    $(wildcard include/config/ARCH_DMA_ADDR_T_64BIT) \
    $(wildcard include/config/PHYS_ADDR_T_64BIT) \
    $(wildcard include/config/64BIT) \
    $(wildcard include/config/ARCH_32BIT_USTAT_F_TINODE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/types.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/uapi/asm/types.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/int-ll64.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/asm-generic/int-ll64.h \
  arch/arm/include/generated/uapi/asm/bitsperlong.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitsperlong.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/asm-generic/bitsperlong.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/posix_types.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/stddef.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/stddef.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/uapi/asm/posix_types.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/asm-generic/posix_types.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kcsan-checks.h \
    $(wildcard include/config/KCSAN) \
    $(wildcard include/config/KCSAN_WEAK_MEMORY) \
    $(wildcard include/config/KCSAN_IGNORE_ATOMICS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/stringify.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kernel.h \
    $(wildcard include/config/PREEMPT_VOLUNTARY_BUILD) \
    $(wildcard include/config/PREEMPT_DYNAMIC) \
    $(wildcard include/config/HAVE_PREEMPT_DYNAMIC_CALL) \
    $(wildcard include/config/HAVE_PREEMPT_DYNAMIC_KEY) \
    $(wildcard include/config/PREEMPT_) \
    $(wildcard include/config/DEBUG_ATOMIC_SLEEP) \
    $(wildcard include/config/SMP) \
    $(wildcard include/config/MMU) \
    $(wildcard include/config/PROVE_LOCKING) \
    $(wildcard include/config/TRACING) \
    $(wildcard include/config/FTRACE_MCOUNT_RECORD) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/stdarg.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/align.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/const.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/vdso/const.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/const.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/limits.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/limits.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/vdso/limits.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/linkage.h \
    $(wildcard include/config/ARCH_USE_SYM_ANNOTATIONS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/export.h \
    $(wildcard include/config/MODVERSIONS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/linkage.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/container_of.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/bitops.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/bits.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/vdso/bits.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/typecheck.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/kernel.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/sysinfo.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/generic-non-atomic.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/barrier.h \
    $(wildcard include/config/CPU_32v6K) \
    $(wildcard include/config/THUMB2_KERNEL) \
    $(wildcard include/config/CPU_XSC3) \
    $(wildcard include/config/CPU_FA526) \
    $(wildcard include/config/ARM_HEAVY_MB) \
    $(wildcard include/config/ARM_DMA_MEM_BUFFERABLE) \
    $(wildcard include/config/CPU_SPECTRE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/barrier.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/bitops.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/irqflags.h \
    $(wildcard include/config/TRACE_IRQFLAGS) \
    $(wildcard include/config/PREEMPT_RT) \
    $(wildcard include/config/IRQSOFF_TRACER) \
    $(wildcard include/config/PREEMPT_TRACER) \
    $(wildcard include/config/DEBUG_IRQFLAGS) \
    $(wildcard include/config/TRACE_IRQFLAGS_SUPPORT) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/cleanup.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/irqflags.h \
    $(wildcard include/config/CPU_V7M) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/ptrace.h \
    $(wildcard include/config/ARM_THUMB) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/uapi/asm/ptrace.h \
    $(wildcard include/config/CPU_ENDIAN_BE8) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/hwcap.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/uapi/asm/hwcap.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/irqflags.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/percpu.h \
    $(wildcard include/config/CPU_V6) \
    $(wildcard include/config/ARM_HAS_GROUP_RELOCS) \
    $(wildcard include/config/ARM_MODULE_PLTS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/insn.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/percpu.h \
    $(wildcard include/config/DEBUG_PREEMPT) \
    $(wildcard include/config/HAVE_SETUP_PER_CPU_AREA) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/threads.h \
    $(wildcard include/config/NR_CPUS) \
    $(wildcard include/config/BASE_SMALL) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/percpu-defs.h \
    $(wildcard include/config/DEBUG_FORCE_WEAK_PER_CPU) \
    $(wildcard include/config/AMD_MEM_ENCRYPT) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/non-atomic.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/non-instrumented-non-atomic.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/builtin-__fls.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/builtin-__ffs.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/builtin-fls.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/builtin-ffs.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/ffz.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/fls64.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/sched.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/hweight.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/arch_hweight.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/const_hweight.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/lock.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/atomic.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/atomic.h \
    $(wildcard include/config/GENERIC_ATOMIC64) \
    $(wildcard include/config/ARM_LPAE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/prefetch.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/processor.h \
    $(wildcard include/config/HAVE_HW_BREAKPOINT) \
    $(wildcard include/config/BINFMT_ELF_FDPIC) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/hw_breakpoint.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/unified.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/vdso/processor.h \
    $(wildcard include/config/ARM_ERRATA_754327) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/cache.h \
    $(wildcard include/config/ARM_L1_CACHE_SHIFT) \
    $(wildcard include/config/AEABI) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/cmpxchg.h \
    $(wildcard include/config/CPU_SA1100) \
    $(wildcard include/config/CPU_SA110) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/cmpxchg-local.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/atomic/atomic-arch-fallback.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/atomic/atomic-long.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/atomic/atomic-instrumented.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/instrumented.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kmsan-checks.h \
    $(wildcard include/config/KMSAN) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/instrumented-lock.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/le.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/uapi/asm/byteorder.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/byteorder/little_endian.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/byteorder/little_endian.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/swab.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/swab.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/swab.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/uapi/asm/swab.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/byteorder/generic.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bitops/ext2-atomic-setbit.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/hex.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kstrtox.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/log2.h \
    $(wildcard include/config/ARCH_HAS_ILOG2_U32) \
    $(wildcard include/config/ARCH_HAS_ILOG2_U64) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/math.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/div64.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/compiler.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/div64.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/minmax.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/panic.h \
    $(wildcard include/config/PANIC_TIMEOUT) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/printk.h \
    $(wildcard include/config/MESSAGE_LOGLEVEL_DEFAULT) \
    $(wildcard include/config/CONSOLE_LOGLEVEL_DEFAULT) \
    $(wildcard include/config/CONSOLE_LOGLEVEL_QUIET) \
    $(wildcard include/config/EARLY_PRINTK) \
    $(wildcard include/config/PRINTK) \
    $(wildcard include/config/PRINTK_INDEX) \
    $(wildcard include/config/DYNAMIC_DEBUG) \
    $(wildcard include/config/DYNAMIC_DEBUG_CORE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kern_levels.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/ratelimit_types.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/param.h \
  arch/arm/include/generated/uapi/asm/param.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/param.h \
    $(wildcard include/config/HZ) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/asm-generic/param.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/spinlock_types_raw.h \
    $(wildcard include/config/DEBUG_SPINLOCK) \
    $(wildcard include/config/DEBUG_LOCK_ALLOC) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/spinlock_types_up.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/lockdep_types.h \
    $(wildcard include/config/PROVE_RAW_LOCK_NESTING) \
    $(wildcard include/config/LOCKDEP) \
    $(wildcard include/config/LOCK_STAT) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/once_lite.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sprintf.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/static_call_types.h \
    $(wildcard include/config/HAVE_STATIC_CALL) \
    $(wildcard include/config/HAVE_STATIC_CALL_INLINE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/instruction_pointer.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/slab.h \
    $(wildcard include/config/DEBUG_SLAB) \
    $(wildcard include/config/DEBUG_OBJECTS) \
    $(wildcard include/config/SLUB_TINY) \
    $(wildcard include/config/FAILSLAB) \
    $(wildcard include/config/MEMCG_KMEM) \
    $(wildcard include/config/KFENCE) \
    $(wildcard include/config/SLAB) \
    $(wildcard include/config/SLUB) \
    $(wildcard include/config/RANDOM_KMALLOC_CACHES) \
    $(wildcard include/config/ZONE_DMA) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/cache.h \
    $(wildcard include/config/ARCH_HAS_CACHE_LINE_SIZE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/gfp.h \
    $(wildcard include/config/HIGHMEM) \
    $(wildcard include/config/ZONE_DMA32) \
    $(wildcard include/config/ZONE_DEVICE) \
    $(wildcard include/config/NUMA) \
    $(wildcard include/config/COMPACTION) \
    $(wildcard include/config/CONTIG_ALLOC) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/gfp_types.h \
    $(wildcard include/config/KASAN_HW_TAGS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/mmzone.h \
    $(wildcard include/config/ARCH_FORCE_MAX_ORDER) \
    $(wildcard include/config/CMA) \
    $(wildcard include/config/MEMORY_ISOLATION) \
    $(wildcard include/config/ZSMALLOC) \
    $(wildcard include/config/UNACCEPTED_MEMORY) \
    $(wildcard include/config/SWAP) \
    $(wildcard include/config/NUMA_BALANCING) \
    $(wildcard include/config/TRANSPARENT_HUGEPAGE) \
    $(wildcard include/config/LRU_GEN) \
    $(wildcard include/config/LRU_GEN_STATS) \
    $(wildcard include/config/MEMCG) \
    $(wildcard include/config/SPARSEMEM) \
    $(wildcard include/config/MEMORY_HOTPLUG) \
    $(wildcard include/config/MEMORY_FAILURE) \
    $(wildcard include/config/FLATMEM) \
    $(wildcard include/config/PAGE_EXTENSION) \
    $(wildcard include/config/DEFERRED_STRUCT_PAGE_INIT) \
    $(wildcard include/config/HAVE_MEMORYLESS_NODES) \
    $(wildcard include/config/SPARSEMEM_VMEMMAP) \
    $(wildcard include/config/SPARSEMEM_EXTREME) \
    $(wildcard include/config/HAVE_ARCH_PFN_VALID) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/spinlock.h \
    $(wildcard include/config/PREEMPTION) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/preempt.h \
    $(wildcard include/config/PREEMPT_COUNT) \
    $(wildcard include/config/TRACE_PREEMPT_TOGGLE) \
    $(wildcard include/config/PREEMPT_NOTIFIERS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/list.h \
    $(wildcard include/config/LIST_HARDENED) \
    $(wildcard include/config/DEBUG_LIST) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/poison.h \
    $(wildcard include/config/ILLEGAL_POINTER_VALUE) \
  arch/arm/include/generated/asm/preempt.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/preempt.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/thread_info.h \
    $(wildcard include/config/THREAD_INFO_IN_TASK) \
    $(wildcard include/config/GENERIC_ENTRY) \
    $(wildcard include/config/HAVE_ARCH_WITHIN_STACK_FRAMES) \
    $(wildcard include/config/HARDENED_USERCOPY) \
    $(wildcard include/config/BUG) \
    $(wildcard include/config/SH) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/bug.h \
    $(wildcard include/config/GENERIC_BUG) \
    $(wildcard include/config/BUG_ON_DATA_CORRUPTION) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/bug.h \
    $(wildcard include/config/DEBUG_BUGVERBOSE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/opcodes.h \
    $(wildcard include/config/CPU_ENDIAN_BE32) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/bug.h \
    $(wildcard include/config/GENERIC_BUG_RELATIVE_POINTERS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/instrumentation.h \
    $(wildcard include/config/NOINSTR_VALIDATION) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/restart_block.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/time64.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/math64.h \
    $(wildcard include/config/ARCH_SUPPORTS_INT128) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/vdso/math64.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/vdso/time64.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/time.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/time_types.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/errno.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/errno.h \
  arch/arm/include/generated/uapi/asm/errno.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/asm-generic/errno.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/asm-generic/errno-base.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/current.h \
    $(wildcard include/config/CURRENT_POINTER_IN_TPIDRURO) \
    $(wildcard include/config/CC_IS_CLANG) \
    $(wildcard include/config/CLANG_VERSION) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/thread_info.h \
    $(wildcard include/config/KASAN) \
    $(wildcard include/config/VMAP_STACK) \
    $(wildcard include/config/ARM_THUMBEE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/fpstate.h \
    $(wildcard include/config/VFPv3) \
    $(wildcard include/config/IWMMXT) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/page.h \
    $(wildcard include/config/CPU_COPY_V4WT) \
    $(wildcard include/config/CPU_COPY_V4WB) \
    $(wildcard include/config/CPU_COPY_FEROCEON) \
    $(wildcard include/config/CPU_COPY_FA) \
    $(wildcard include/config/CPU_XSCALE) \
    $(wildcard include/config/CPU_COPY_V6) \
    $(wildcard include/config/KUSER_HELPERS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/glue.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/pgtable-2level-types.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/memory.h \
    $(wildcard include/config/NEED_MACH_MEMORY_H) \
    $(wildcard include/config/PAGE_OFFSET) \
    $(wildcard include/config/DRAM_BASE) \
    $(wildcard include/config/DRAM_SIZE) \
    $(wildcard include/config/XIP_KERNEL) \
    $(wildcard include/config/HAVE_TCM) \
    $(wildcard include/config/ARM_PATCH_PHYS_VIRT) \
    $(wildcard include/config/PHYS_OFFSET) \
    $(wildcard include/config/DEBUG_VIRTUAL) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sizes.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/kasan_def.h \
    $(wildcard include/config/KASAN_SHADOW_OFFSET) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/getorder.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/memory_model.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/pfn.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/traps.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/bottom_half.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/lockdep.h \
    $(wildcard include/config/DEBUG_LOCKING_API_SELFTESTS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/smp.h \
    $(wildcard include/config/UP_LATE_INIT) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/cpumask.h \
    $(wildcard include/config/FORCE_NR_CPUS) \
    $(wildcard include/config/HOTPLUG_CPU) \
    $(wildcard include/config/DEBUG_PER_CPU_MAPS) \
    $(wildcard include/config/CPUMASK_OFFSTACK) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/bitmap.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/find.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/string.h \
    $(wildcard include/config/BINARY_PRINTF) \
    $(wildcard include/config/FORTIFY_SOURCE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/err.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/overflow.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/string.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/string.h \
    $(wildcard include/config/BCM2835_FAST_MEMCPY) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/numa.h \
    $(wildcard include/config/NODES_SHIFT) \
    $(wildcard include/config/NUMA_KEEP_MEMINFO) \
    $(wildcard include/config/HAVE_ARCH_NODE_DEV_GROUP) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/smp_types.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/llist.h \
    $(wildcard include/config/ARCH_HAVE_NMI_SAFE_CMPXCHG) \
  arch/arm/include/generated/asm/mmiowb.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/mmiowb.h \
    $(wildcard include/config/MMIOWB) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/spinlock_types.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/rwlock_types.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/spinlock_up.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/rwlock.h \
    $(wildcard include/config/PREEMPT) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/spinlock_api_up.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/list_nulls.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/wait.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/wait.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/seqlock.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/mutex.h \
    $(wildcard include/config/MUTEX_SPIN_ON_OWNER) \
    $(wildcard include/config/DEBUG_MUTEXES) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/osq_lock.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/debug_locks.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/nodemask.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/random.h \
    $(wildcard include/config/VMGENID) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/random.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/ioctl.h \
  arch/arm/include/generated/uapi/asm/ioctl.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/ioctl.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/asm-generic/ioctl.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/irqnr.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/irqnr.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/prandom.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/once.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/jump_label.h \
    $(wildcard include/config/JUMP_LABEL) \
    $(wildcard include/config/HAVE_ARCH_JUMP_LABEL_RELATIVE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/jump_label.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/percpu.h \
    $(wildcard include/config/NEED_PER_CPU_PAGE_FIRST_CHUNK) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/mmdebug.h \
    $(wildcard include/config/DEBUG_VM) \
    $(wildcard include/config/DEBUG_VM_IRQSOFF) \
    $(wildcard include/config/DEBUG_VM_PGFLAGS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/pageblock-flags.h \
    $(wildcard include/config/HUGETLB_PAGE) \
    $(wildcard include/config/HUGETLB_PAGE_SIZE_VARIABLE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/page-flags-layout.h \
  include/generated/bounds.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/mm_types.h \
    $(wildcard include/config/HAVE_ALIGNED_STRUCT_PAGE) \
    $(wildcard include/config/USERFAULTFD) \
    $(wildcard include/config/PER_VMA_LOCK) \
    $(wildcard include/config/ANON_VMA_NAME) \
    $(wildcard include/config/SCHED_MM_CID) \
    $(wildcard include/config/HAVE_ARCH_COMPAT_MMAP_BASES) \
    $(wildcard include/config/MEMBARRIER) \
    $(wildcard include/config/AIO) \
    $(wildcard include/config/MMU_NOTIFIER) \
    $(wildcard include/config/ARCH_WANT_BATCHED_UNMAP_TLB_FLUSH) \
    $(wildcard include/config/IOMMU_SVA) \
    $(wildcard include/config/KSM) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/mm_types_task.h \
    $(wildcard include/config/SPLIT_PTLOCK_CPUS) \
    $(wildcard include/config/ARCH_ENABLE_SPLIT_PMD_PTLOCK) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/auxvec.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/auxvec.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/auxvec.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/uapi/asm/auxvec.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kref.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/refcount.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/rbtree.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/rbtree_types.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/rcupdate.h \
    $(wildcard include/config/PREEMPT_RCU) \
    $(wildcard include/config/TINY_RCU) \
    $(wildcard include/config/RCU_STRICT_GRACE_PERIOD) \
    $(wildcard include/config/RCU_LAZY) \
    $(wildcard include/config/TASKS_RCU_GENERIC) \
    $(wildcard include/config/RCU_STALL_COMMON) \
    $(wildcard include/config/NO_HZ_FULL) \
    $(wildcard include/config/KVM_XFER_TO_GUEST_WORK) \
    $(wildcard include/config/RCU_NOCB_CPU) \
    $(wildcard include/config/TASKS_RCU) \
    $(wildcard include/config/TASKS_TRACE_RCU) \
    $(wildcard include/config/TASKS_RUDE_RCU) \
    $(wildcard include/config/TREE_RCU) \
    $(wildcard include/config/DEBUG_OBJECTS_RCU_HEAD) \
    $(wildcard include/config/PROVE_RCU) \
    $(wildcard include/config/ARCH_WEAK_RELEASE_ACQUIRE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/context_tracking_irq.h \
    $(wildcard include/config/CONTEXT_TRACKING_IDLE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/rcutiny.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/maple_tree.h \
    $(wildcard include/config/MAPLE_RCU_DISABLED) \
    $(wildcard include/config/DEBUG_MAPLE_TREE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/rwsem.h \
    $(wildcard include/config/RWSEM_SPIN_ON_OWNER) \
    $(wildcard include/config/DEBUG_RWSEMS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/completion.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/swait.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/uprobes.h \
    $(wildcard include/config/UPROBES) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/workqueue.h \
    $(wildcard include/config/DEBUG_OBJECTS_WORK) \
    $(wildcard include/config/FREEZER) \
    $(wildcard include/config/WQ_WATCHDOG) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/timer.h \
    $(wildcard include/config/DEBUG_OBJECTS_TIMERS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/ktime.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/time.h \
    $(wildcard include/config/POSIX_TIMERS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/time32.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/timex.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/timex.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/timex.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/vdso/time32.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/vdso/time.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/jiffies.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/vdso/jiffies.h \
  include/generated/timeconst.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/vdso/ktime.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/timekeeping.h \
    $(wildcard include/config/GENERIC_CMOS_UPDATE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/clocksource_ids.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/debugobjects.h \
    $(wildcard include/config/DEBUG_OBJECTS_FREE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/percpu_counter.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/mmu.h \
    $(wildcard include/config/CPU_HAS_ASID) \
    $(wildcard include/config/VDSO) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/page-flags.h \
    $(wildcard include/config/ARCH_USES_PG_UNCACHED) \
    $(wildcard include/config/PAGE_IDLE_FLAG) \
    $(wildcard include/config/ARCH_USES_PG_ARCH_X) \
    $(wildcard include/config/HUGETLB_PAGE_OPTIMIZE_VMEMMAP) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/local_lock.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/local_lock_internal.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/memory_hotplug.h \
    $(wildcard include/config/HAVE_ARCH_NODEDATA_EXTENSION) \
    $(wildcard include/config/ARCH_HAS_ADD_PAGES) \
    $(wildcard include/config/MEMORY_HOTREMOVE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/notifier.h \
    $(wildcard include/config/TREE_SRCU) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/srcu.h \
    $(wildcard include/config/TINY_SRCU) \
    $(wildcard include/config/NEED_SRCU_NMI_SAFE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/rcu_segcblist.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/srcutiny.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/topology.h \
    $(wildcard include/config/USE_PERCPU_NUMA_NODE_ID) \
    $(wildcard include/config/SCHED_SMT) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/arch_topology.h \
    $(wildcard include/config/ACPI_CPPC_LIB) \
    $(wildcard include/config/GENERIC_ARCH_TOPOLOGY) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/topology.h \
    $(wildcard include/config/ARM_CPU_TOPOLOGY) \
    $(wildcard include/config/BL_SWITCHER) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/topology.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/percpu-refcount.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/hash.h \
    $(wildcard include/config/HAVE_ARCH_HASH) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kasan.h \
    $(wildcard include/config/KASAN_STACK) \
    $(wildcard include/config/KASAN_VMALLOC) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kasan-enabled.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/static_key.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/soc.h \
    $(wildcard include/config/PM_SLEEP) \
    $(wildcard include/config/SND_SOC_COMPRESS) \
    $(wildcard include/config/DMI) \
    $(wildcard include/config/SND_SOC_AC97_BUS) \
    $(wildcard include/config/SND_SOC_TOPOLOGY) \
    $(wildcard include/config/PCI) \
    $(wildcard include/config/DEBUG_FS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/of.h \
    $(wildcard include/config/OF_DYNAMIC) \
    $(wildcard include/config/SPARC) \
    $(wildcard include/config/OF_PROMTREE) \
    $(wildcard include/config/OF_KOBJ) \
    $(wildcard include/config/OF_NUMA) \
    $(wildcard include/config/OF_OVERLAY) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kobject.h \
    $(wildcard include/config/UEVENT_HELPER) \
    $(wildcard include/config/DEBUG_KOBJECT_RELEASE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sysfs.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kernfs.h \
    $(wildcard include/config/KERNFS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/idr.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/radix-tree.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/xarray.h \
    $(wildcard include/config/XARRAY_MULTI) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sched/mm.h \
    $(wildcard include/config/MMU_LAZY_TLB_REFCOUNT) \
    $(wildcard include/config/ARCH_HAS_MEMBARRIER_CALLBACKS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sched.h \
    $(wildcard include/config/VIRT_CPU_ACCOUNTING_NATIVE) \
    $(wildcard include/config/SCHED_INFO) \
    $(wildcard include/config/SCHEDSTATS) \
    $(wildcard include/config/SCHED_CORE) \
    $(wildcard include/config/FAIR_GROUP_SCHED) \
    $(wildcard include/config/RT_GROUP_SCHED) \
    $(wildcard include/config/RT_MUTEXES) \
    $(wildcard include/config/UCLAMP_TASK) \
    $(wildcard include/config/UCLAMP_BUCKETS_COUNT) \
    $(wildcard include/config/KMAP_LOCAL) \
    $(wildcard include/config/CGROUP_SCHED) \
    $(wildcard include/config/BLK_DEV_IO_TRACE) \
    $(wildcard include/config/COMPAT_BRK) \
    $(wildcard include/config/CGROUPS) \
    $(wildcard include/config/BLK_CGROUP) \
    $(wildcard include/config/PSI) \
    $(wildcard include/config/PAGE_OWNER) \
    $(wildcard include/config/EVENTFD) \
    $(wildcard include/config/CPU_SUP_INTEL) \
    $(wildcard include/config/TASK_DELAY_ACCT) \
    $(wildcard include/config/STACKPROTECTOR) \
    $(wildcard include/config/ARCH_HAS_SCALED_CPUTIME) \
    $(wildcard include/config/VIRT_CPU_ACCOUNTING_GEN) \
    $(wildcard include/config/POSIX_CPUTIMERS) \
    $(wildcard include/config/POSIX_CPU_TIMERS_TASK_WORK) \
    $(wildcard include/config/KEYS) \
    $(wildcard include/config/SYSVIPC) \
    $(wildcard include/config/DETECT_HUNG_TASK) \
    $(wildcard include/config/IO_URING) \
    $(wildcard include/config/AUDIT) \
    $(wildcard include/config/AUDITSYSCALL) \
    $(wildcard include/config/UBSAN) \
    $(wildcard include/config/UBSAN_TRAP) \
    $(wildcard include/config/TASK_XACCT) \
    $(wildcard include/config/CPUSETS) \
    $(wildcard include/config/X86_CPU_RESCTRL) \
    $(wildcard include/config/FUTEX) \
    $(wildcard include/config/COMPAT) \
    $(wildcard include/config/PERF_EVENTS) \
    $(wildcard include/config/RSEQ) \
    $(wildcard include/config/FAULT_INJECTION) \
    $(wildcard include/config/LATENCYTOP) \
    $(wildcard include/config/KUNIT) \
    $(wildcard include/config/FUNCTION_GRAPH_TRACER) \
    $(wildcard include/config/BCACHE) \
    $(wildcard include/config/LIVEPATCH) \
    $(wildcard include/config/SECURITY) \
    $(wildcard include/config/BPF_SYSCALL) \
    $(wildcard include/config/GCC_PLUGIN_STACKLEAK) \
    $(wildcard include/config/X86_MCE) \
    $(wildcard include/config/KRETPROBES) \
    $(wildcard include/config/RETHOOK) \
    $(wildcard include/config/ARCH_HAS_PARANOID_L1D_FLUSH) \
    $(wildcard include/config/RV) \
    $(wildcard include/config/USER_EVENTS) \
    $(wildcard include/config/ARCH_TASK_STRUCT_ON_STACK) \
    $(wildcard include/config/PREEMPT_NONE) \
    $(wildcard include/config/PREEMPT_VOLUNTARY) \
    $(wildcard include/config/DEBUG_RSEQ) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/sched.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/pid.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/rculist.h \
    $(wildcard include/config/PROVE_RCU_LIST) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sem.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/sem.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/ipc.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/uidgid.h \
    $(wildcard include/config/MULTIUSER) \
    $(wildcard include/config/USER_NS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/highuid.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/rhashtable-types.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/ipc.h \
  arch/arm/include/generated/uapi/asm/ipcbuf.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/asm-generic/ipcbuf.h \
  arch/arm/include/generated/uapi/asm/sembuf.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/asm-generic/sembuf.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/shm.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/shm.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/asm-generic/hugetlb_encode.h \
  arch/arm/include/generated/uapi/asm/shmbuf.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/asm-generic/shmbuf.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/shmparam.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kmsan_types.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/plist.h \
    $(wildcard include/config/DEBUG_PLIST) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/hrtimer.h \
    $(wildcard include/config/HIGH_RES_TIMERS) \
    $(wildcard include/config/TIME_LOW_RES) \
    $(wildcard include/config/TIMERFD) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/hrtimer_defs.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/timerqueue.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/seccomp.h \
    $(wildcard include/config/SECCOMP) \
    $(wildcard include/config/HAVE_ARCH_SECCOMP_FILTER) \
    $(wildcard include/config/SECCOMP_FILTER) \
    $(wildcard include/config/CHECKPOINT_RESTORE) \
    $(wildcard include/config/SECCOMP_CACHE_DEBUG) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/seccomp.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/seccomp.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/seccomp.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/unistd.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/unistd.h \
    $(wildcard include/config/OABI_COMPAT) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/uapi/asm/unistd.h \
  arch/arm/include/generated/uapi/asm/unistd-eabi.h \
  arch/arm/include/generated/asm/unistd-nr.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/resource.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/resource.h \
  arch/arm/include/generated/uapi/asm/resource.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/resource.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/asm-generic/resource.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/latencytop.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sched/prio.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sched/types.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/signal_types.h \
    $(wildcard include/config/OLD_SIGACTION) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/signal.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/signal.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/uapi/asm/signal.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/asm-generic/signal-defs.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/uapi/asm/sigcontext.h \
  arch/arm/include/generated/uapi/asm/siginfo.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/asm-generic/siginfo.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/syscall_user_dispatch.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/task_io_accounting.h \
    $(wildcard include/config/TASK_IO_ACCOUNTING) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/posix-timers.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/alarmtimer.h \
    $(wildcard include/config/RTC_CLASS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/rseq.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kcsan.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/rv.h \
    $(wildcard include/config/RV_REACTORS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/livepatch_sched.h \
  arch/arm/include/generated/asm/kmap_size.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/kmap_size.h \
    $(wildcard include/config/DEBUG_KMAP_LOCAL) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sync_core.h \
    $(wildcard include/config/ARCH_HAS_SYNC_CORE_BEFORE_USERMODE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kobject_ns.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/stat.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/uapi/asm/stat.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/stat.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/mod_devicetable.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/mei.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/mei_uuid.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/uuid.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/property.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/fwnode.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/platform_device.h \
    $(wildcard include/config/HAS_IOMEM) \
    $(wildcard include/config/SUSPEND) \
    $(wildcard include/config/HIBERNATE_CALLBACKS) \
    $(wildcard include/config/SUPERH) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/device.h \
    $(wildcard include/config/GENERIC_MSI_IRQ) \
    $(wildcard include/config/ENERGY_MODEL) \
    $(wildcard include/config/PINCTRL) \
    $(wildcard include/config/DMA_OPS) \
    $(wildcard include/config/DMA_DECLARE_COHERENT) \
    $(wildcard include/config/DMA_CMA) \
    $(wildcard include/config/SWIOTLB) \
    $(wildcard include/config/SWIOTLB_DYNAMIC) \
    $(wildcard include/config/ARCH_HAS_SYNC_DMA_FOR_DEVICE) \
    $(wildcard include/config/ARCH_HAS_SYNC_DMA_FOR_CPU) \
    $(wildcard include/config/ARCH_HAS_SYNC_DMA_FOR_CPU_ALL) \
    $(wildcard include/config/DMA_OPS_BYPASS) \
    $(wildcard include/config/DEVTMPFS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/dev_printk.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/ratelimit.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/energy_model.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sched/cpufreq.h \
    $(wildcard include/config/CPU_FREQ) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sched/topology.h \
    $(wildcard include/config/SCHED_DEBUG) \
    $(wildcard include/config/SCHED_CLUSTER) \
    $(wildcard include/config/SCHED_MC) \
    $(wildcard include/config/CPU_FREQ_GOV_SCHEDUTIL) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sched/idle.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/ioport.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/klist.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/pm.h \
    $(wildcard include/config/VT_CONSOLE_SLEEP) \
    $(wildcard include/config/CXL_SUSPEND) \
    $(wildcard include/config/PM) \
    $(wildcard include/config/PM_CLK) \
    $(wildcard include/config/PM_GENERIC_DOMAINS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/device/bus.h \
    $(wildcard include/config/ACPI) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/device/class.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/device/driver.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/module.h \
    $(wildcard include/config/MODULES_TREE_LOOKUP) \
    $(wildcard include/config/STACKTRACE_BUILD_ID) \
    $(wildcard include/config/ARCH_USES_CFI_TRAPS) \
    $(wildcard include/config/MODULE_SIG) \
    $(wildcard include/config/KALLSYMS) \
    $(wildcard include/config/TRACEPOINTS) \
    $(wildcard include/config/BPF_EVENTS) \
    $(wildcard include/config/EVENT_TRACING) \
    $(wildcard include/config/KPROBES) \
    $(wildcard include/config/MODULE_UNLOAD) \
    $(wildcard include/config/CONSTRUCTORS) \
    $(wildcard include/config/FUNCTION_ERROR_INJECTION) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/buildid.h \
    $(wildcard include/config/CRASH_CORE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kmod.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/umh.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sysctl.h \
    $(wildcard include/config/SYSCTL) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/sysctl.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/elf.h \
    $(wildcard include/config/ARCH_USE_GNU_PROPERTY) \
    $(wildcard include/config/ARCH_HAVE_ELF_PROT) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/elf.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/vdso_datapage.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/vdso/datapage.h \
    $(wildcard include/config/ARCH_HAS_VDSO_DATA) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/vdso/clocksource.h \
    $(wildcard include/config/GENERIC_GETTIMEOFDAY) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/vdso/processor.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/vdso/gettimeofday.h \
    $(wildcard include/config/ARM_ARCH_TIMER) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/vdso/cp15.h \
    $(wildcard include/config/CPU_CP15) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/user.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/elf.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/elf-em.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/rbtree_latch.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/error-injection.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/error-injection.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/tracepoint-defs.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/dynamic_debug.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/module.h \
    $(wildcard include/config/ARM_UNWIND) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/module.h \
    $(wildcard include/config/HAVE_MOD_ARCH_SPECIFIC) \
    $(wildcard include/config/MODULES_USE_ELF_REL) \
    $(wildcard include/config/MODULES_USE_ELF_RELA) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/unwind.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/device.h \
    $(wildcard include/config/ARM_DMA_USE_IOMMU) \
    $(wildcard include/config/ARCH_OMAP) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/pm_wakeup.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/interrupt.h \
    $(wildcard include/config/IRQ_FORCED_THREADING) \
    $(wildcard include/config/GENERIC_IRQ_PROBE) \
    $(wildcard include/config/PROC_FS) \
    $(wildcard include/config/IRQ_TIMINGS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/irqreturn.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/hardirq.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/context_tracking_state.h \
    $(wildcard include/config/CONTEXT_TRACKING_USER) \
    $(wildcard include/config/CONTEXT_TRACKING) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/ftrace_irq.h \
    $(wildcard include/config/HWLAT_TRACER) \
    $(wildcard include/config/OSNOISE_TRACER) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/vtime.h \
    $(wildcard include/config/VIRT_CPU_ACCOUNTING) \
    $(wildcard include/config/IRQ_TIME_ACCOUNTING) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/hardirq.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/irq.h \
    $(wildcard include/config/SPARSE_IRQ) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/hardirq.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/irq.h \
    $(wildcard include/config/GENERIC_IRQ_EFFECTIVE_AFF_MASK) \
    $(wildcard include/config/GENERIC_IRQ_IPI) \
    $(wildcard include/config/IRQ_DOMAIN_HIERARCHY) \
    $(wildcard include/config/DEPRECATED_IRQ_CPU_ONOFFLINE) \
    $(wildcard include/config/GENERIC_IRQ_MIGRATION) \
    $(wildcard include/config/GENERIC_PENDING_IRQ) \
    $(wildcard include/config/HARDIRQS_SW_RESEND) \
    $(wildcard include/config/GENERIC_IRQ_LEGACY) \
    $(wildcard include/config/GENERIC_IRQ_MULTI_HANDLER) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/irqhandler.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/io.h \
    $(wildcard include/config/HAS_IOPORT_MAP) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/io.h \
    $(wildcard include/config/PCMCIA) \
    $(wildcard include/config/NEED_MACH_IO_H) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/pci_iomap.h \
    $(wildcard include/config/NO_GENERIC_PCI_IOPORT_MAP) \
    $(wildcard include/config/GENERIC_PCI_IOMAP) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/io.h \
    $(wildcard include/config/GENERIC_IOMAP) \
    $(wildcard include/config/TRACE_MMIO_ACCESS) \
    $(wildcard include/config/GENERIC_IOREMAP) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/logic_pio.h \
    $(wildcard include/config/INDIRECT_PIO) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/vmalloc.h \
    $(wildcard include/config/HAVE_ARCH_HUGE_VMALLOC) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/vmalloc.h \
  arch/arm/include/generated/asm/irq_regs.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/irq_regs.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/irqdesc.h \
    $(wildcard include/config/GENERIC_IRQ_DEBUGFS) \
    $(wildcard include/config/IRQ_DOMAIN) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/hw_irq.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/sections.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/sections.h \
    $(wildcard include/config/HAVE_FUNCTION_DESCRIPTORS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/regmap.h \
    $(wildcard include/config/REGMAP) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/delay.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/delay.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/iopoll.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/core.h \
    $(wildcard include/config/SND_DYNAMIC_MINORS) \
    $(wildcard include/config/SND_MAX_CARDS) \
    $(wildcard include/config/SND_MAJOR) \
    $(wildcard include/config/SND_CTL_FAST_LOOKUP) \
    $(wildcard include/config/SND_DEBUG) \
    $(wildcard include/config/SND_MIXER_OSS) \
    $(wildcard include/config/SND_OSSEMUL) \
    $(wildcard include/config/ISA_DMA_API) \
    $(wildcard include/config/SND_VERBOSE_PRINTK) \
    $(wildcard include/config/SND_DEBUG_VERBOSE) \
    $(wildcard include/config/GAMEPORT) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/pcm.h \
    $(wildcard include/config/SND_PCM_OSS) \
    $(wildcard include/config/SND_VERBOSE_PROCFS) \
    $(wildcard include/config/SND_PCM_XRUN_DEBUG) \
    $(wildcard include/config/X86) \
    $(wildcard include/config/PPC) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/asound.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/sound/asound.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/memalloc.h \
    $(wildcard include/config/GENERIC_ALLOCATOR) \
    $(wildcard include/config/SND_DMA_SGBUF) \
    $(wildcard include/config/HAS_DMA) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/dma-direction.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/minors.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/poll.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/fs.h \
    $(wildcard include/config/READ_ONLY_THP_FOR_FS) \
    $(wildcard include/config/FS_POSIX_ACL) \
    $(wildcard include/config/CGROUP_WRITEBACK) \
    $(wildcard include/config/IMA) \
    $(wildcard include/config/FILE_LOCKING) \
    $(wildcard include/config/FSNOTIFY) \
    $(wildcard include/config/FS_ENCRYPTION) \
    $(wildcard include/config/FS_VERITY) \
    $(wildcard include/config/EPOLL) \
    $(wildcard include/config/UNICODE) \
    $(wildcard include/config/QUOTA) \
    $(wildcard include/config/FS_DAX) \
    $(wildcard include/config/BLOCK) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/wait_bit.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/kdev_t.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/kdev_t.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/dcache.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/rculist_bl.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/list_bl.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/bit_spinlock.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/lockref.h \
    $(wildcard include/config/ARCH_USE_CMPXCHG_LOCKREF) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/stringhash.h \
    $(wildcard include/config/DCACHE_WORD_ACCESS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/path.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/list_lru.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/shrinker.h \
    $(wildcard include/config/SHRINKER_DEBUG) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/capability.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/capability.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/semaphore.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/fcntl.h \
    $(wildcard include/config/ARCH_32BIT_OFF_T) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/fcntl.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/uapi/asm/fcntl.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/asm-generic/fcntl.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/openat2.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/migrate_mode.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/percpu-rwsem.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/rcuwait.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sched/signal.h \
    $(wildcard include/config/SCHED_AUTOGROUP) \
    $(wildcard include/config/BSD_PROCESS_ACCT) \
    $(wildcard include/config/TASKSTATS) \
    $(wildcard include/config/STACK_GROWSUP) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/signal.h \
    $(wildcard include/config/DYNAMIC_SIGFRAME) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sched/jobctl.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sched/task.h \
    $(wildcard include/config/HAVE_EXIT_THREAD) \
    $(wildcard include/config/ARCH_WANTS_DYNAMIC_TASK_STRUCT) \
    $(wildcard include/config/HAVE_ARCH_THREAD_STRUCT_WHITELIST) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/uaccess.h \
    $(wildcard include/config/ARCH_HAS_SUBPAGE_FAULTS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/fault-inject-usercopy.h \
    $(wildcard include/config/FAULT_INJECTION_USERCOPY) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/uaccess.h \
    $(wildcard include/config/CPU_SW_DOMAIN_PAN) \
    $(wildcard include/config/CPU_USE_DOMAINS) \
    $(wildcard include/config/HAVE_EFFICIENT_UNALIGNED_ACCESS) \
    $(wildcard include/config/UACCESS_WITH_MEMCPY) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/domain.h \
    $(wildcard include/config/IO_36) \
    $(wildcard include/config/CPU_CP15_MMU) \
  arch/arm/include/generated/asm/unaligned.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/unaligned.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/unaligned/packed_struct.h \
  arch/arm/include/generated/asm/extable.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/extable.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/access_ok.h \
    $(wildcard include/config/ALTERNATE_USER_ADDRESS_SPACE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/cred.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/key.h \
    $(wildcard include/config/KEY_NOTIFICATIONS) \
    $(wildcard include/config/NET) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/assoc_array.h \
    $(wildcard include/config/ASSOCIATIVE_ARRAY) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sched/user.h \
    $(wildcard include/config/VFIO_PCI_ZDEV_KVM) \
    $(wildcard include/config/IOMMUFD) \
    $(wildcard include/config/WATCH_QUEUE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/rcu_sync.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/delayed_call.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/errseq.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/ioprio.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sched/rt.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/iocontext.h \
    $(wildcard include/config/BLK_ICQ) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/ioprio.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/fs_types.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/mount.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/mnt_idmapping.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/fs.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/quota.h \
    $(wildcard include/config/QUOTA_NETLINK_INTERFACE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/dqblk_xfs.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/dqblk_v1.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/dqblk_v2.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/dqblk_qtree.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/projid.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/quota.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/poll.h \
  arch/arm/include/generated/uapi/asm/poll.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/asm-generic/poll.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/eventpoll.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/mm.h \
    $(wildcard include/config/HAVE_ARCH_MMAP_RND_BITS) \
    $(wildcard include/config/HAVE_ARCH_MMAP_RND_COMPAT_BITS) \
    $(wildcard include/config/MEM_SOFT_DIRTY) \
    $(wildcard include/config/ARCH_USES_HIGH_VMA_FLAGS) \
    $(wildcard include/config/ARCH_HAS_PKEYS) \
    $(wildcard include/config/X86_USER_SHADOW_STACK) \
    $(wildcard include/config/PARISC) \
    $(wildcard include/config/SPARC64) \
    $(wildcard include/config/ARM64_MTE) \
    $(wildcard include/config/HAVE_ARCH_USERFAULTFD_MINOR) \
    $(wildcard include/config/SHMEM) \
    $(wildcard include/config/MIGRATION) \
    $(wildcard include/config/ARCH_HAS_PTE_SPECIAL) \
    $(wildcard include/config/ARCH_HAS_PTE_DEVMAP) \
    $(wildcard include/config/DEBUG_VM_RB) \
    $(wildcard include/config/PAGE_POISONING) \
    $(wildcard include/config/INIT_ON_ALLOC_DEFAULT_ON) \
    $(wildcard include/config/INIT_ON_FREE_DEFAULT_ON) \
    $(wildcard include/config/DEBUG_PAGEALLOC) \
    $(wildcard include/config/ARCH_WANT_OPTIMIZE_DAX_VMEMMAP) \
    $(wildcard include/config/HUGETLBFS) \
    $(wildcard include/config/MAPPING_DIRTY_HELPERS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/mmap_lock.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/range.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/page_ext.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/stacktrace.h \
    $(wildcard include/config/ARCH_STACKWALK) \
    $(wildcard include/config/STACKTRACE) \
    $(wildcard include/config/HAVE_RELIABLE_STACKTRACE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/stackdepot.h \
    $(wildcard include/config/STACKDEPOT_ALWAYS_INIT) \
    $(wildcard include/config/STACKDEPOT) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/page_ref.h \
    $(wildcard include/config/DEBUG_PAGE_REF) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/pgtable.h \
    $(wildcard include/config/PGTABLE_LEVELS) \
    $(wildcard include/config/HIGHPTE) \
    $(wildcard include/config/ARCH_HAS_NONLEAF_PMD_YOUNG) \
    $(wildcard include/config/GUP_GET_PXX_LOW_HIGH) \
    $(wildcard include/config/ARCH_WANT_PMD_MKWRITE) \
    $(wildcard include/config/HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD) \
    $(wildcard include/config/HAVE_ARCH_SOFT_DIRTY) \
    $(wildcard include/config/ARCH_ENABLE_THP_MIGRATION) \
    $(wildcard include/config/HAVE_ARCH_HUGE_VMAP) \
    $(wildcard include/config/X86_ESPFIX64) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/pgtable.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/proc-fns.h \
    $(wildcard include/config/BIG_LITTLE) \
    $(wildcard include/config/HARDEN_BRANCH_PREDICTOR) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/glue-proc.h \
    $(wildcard include/config/CPU_ARM7TDMI) \
    $(wildcard include/config/CPU_ARM720T) \
    $(wildcard include/config/CPU_ARM740T) \
    $(wildcard include/config/CPU_ARM9TDMI) \
    $(wildcard include/config/CPU_ARM920T) \
    $(wildcard include/config/CPU_ARM922T) \
    $(wildcard include/config/CPU_ARM925T) \
    $(wildcard include/config/CPU_ARM926T) \
    $(wildcard include/config/CPU_ARM940T) \
    $(wildcard include/config/CPU_ARM946E) \
    $(wildcard include/config/CPU_ARM1020) \
    $(wildcard include/config/CPU_ARM1020E) \
    $(wildcard include/config/CPU_ARM1022) \
    $(wildcard include/config/CPU_ARM1026) \
    $(wildcard include/config/CPU_MOHAWK) \
    $(wildcard include/config/CPU_FEROCEON) \
    $(wildcard include/config/CPU_V6K) \
    $(wildcard include/config/CPU_PJ4B) \
    $(wildcard include/config/CPU_V7) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/pgtable-nopud.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/pgtable-nop4d.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/pgtable-hwdef.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/pgtable-2level-hwdef.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/tlbflush.h \
    $(wildcard include/config/SMP_ON_UP) \
    $(wildcard include/config/CPU_TLB_V4WT) \
    $(wildcard include/config/CPU_TLB_FA) \
    $(wildcard include/config/CPU_TLB_V4WBI) \
    $(wildcard include/config/CPU_TLB_FEROCEON) \
    $(wildcard include/config/CPU_TLB_V4WB) \
    $(wildcard include/config/CPU_TLB_V6) \
    $(wildcard include/config/CPU_TLB_V7) \
    $(wildcard include/config/ARM_ERRATA_720789) \
    $(wildcard include/config/ARM_ERRATA_798181) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/arch/arm/include/asm/pgtable-2level.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/asm-generic/pgtable_uffd.h \
    $(wildcard include/config/HAVE_ARCH_USERFAULTFD_WP) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/page_table_check.h \
    $(wildcard include/config/PAGE_TABLE_CHECK) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/memremap.h \
    $(wildcard include/config/DEVICE_PRIVATE) \
    $(wildcard include/config/PCI_P2PDMA) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/huge_mm.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/sched/coredump.h \
    $(wildcard include/config/CORE_DUMP_DEFAULT_ELF_HEADERS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/vmstat.h \
    $(wildcard include/config/VM_EVENT_COUNTERS) \
    $(wildcard include/config/DEBUG_TLBFLUSH) \
    $(wildcard include/config/PER_VMA_LOCK_STATS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/vm_event_item.h \
    $(wildcard include/config/MEMORY_BALLOON) \
    $(wildcard include/config/BALLOON_COMPACTION) \
    $(wildcard include/config/ZSWAP) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/pm_qos.h \
    $(wildcard include/config/CPU_IDLE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/uio.h \
    $(wildcard include/config/ARCH_HAS_UACCESS_FLUSHCACHE) \
    $(wildcard include/config/ARCH_HAS_COPY_MC) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/linux/uio.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/pcm_oss.h \
    $(wildcard include/config/SND_PCM_OSS_PLUGINS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/compress_driver.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/sound/compress_offload.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/sound/compress_params.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/control.h \
    $(wildcard include/config/SND_CTL_DEBUG) \
    $(wildcard include/config/SND_CTL_LED) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/nospec.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/ac97_codec.h \
    $(wildcard include/config/SND_AC97_POWER_SAVE) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/ac97/regs.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/info.h \
    $(wildcard include/config/SND_PROC_FS) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/seq_file.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/string_helpers.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/ctype.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/linux/string_choices.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/soc-dapm.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/soc-topology.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/uapi/sound/asoc.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/soc-dpcm.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/soc-dai.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/soc-component.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/soc-card.h \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/soc-jack.h \
    $(wildcard include/config/GPIOLIB) \
  /usr/src/linux-headers-6.6.31+rpt-common-rpi/include/sound/initval.h \
    $(wildcard include/config/PNP) \

/home/recurse/driver/phoneodeo_mod.o: $(deps_/home/recurse/driver/phoneodeo_mod.o)

$(deps_/home/recurse/driver/phoneodeo_mod.o):
