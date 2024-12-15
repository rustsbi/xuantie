use core::arch::asm;

/// D-cache clean all dirty items instruction.
///
/// Clears all L1 D-cache table items, write all dirty items to next level storage.
///
/// # Permissions
///
/// Can run on M mode, or S mode if applicable.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906, C907, E907 and E906 cores.
#[inline]
pub unsafe fn dcache_call() {
    // th.dcache.call
    asm!(".insn i 0x0B, 0, x0, x0, 0x001")
}

/// D-cache invalid all items instruction.
///
/// Invalidates all L1 D-cache table items. This instruction only operates on the current hart.
///
/// # Permissions
///
/// Can run on M mode, or S mode if applicable.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906, C907, E907 and E906 cores.
#[inline]
pub unsafe fn dcache_iall() {
    // th.dcache.iall
    asm!(".insn i 0x0B, 0, x0, x0, 0x002")
}

/// D-cache clean all dirty and invalid item instruction.
///
/// Writes all L1 D-cache dirty items to next level storage, and invalidate all L1 D-cache table items.
///
/// # Permissions
///
/// Can run on M mode, or S mode if applicable.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906, C907, E907 and E906 cores.
#[inline]
pub unsafe fn dcache_ciall() {
    // th.dcache.ciall
    asm!(".insn i 0x0B, 0, x0, x0, 0x003")
}

/// I-cache invalid all items instruction.
///
/// Invalidates all I-cache table items. This instruction only operates on the current hart.
///
/// # Permissions
///
/// Can run on M mode, or S mode if applicable.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906, C907, E907, E906 and E902 cores.
#[inline]
pub unsafe fn icache_iall() {
    // th.icache.iall
    asm!(".insn i 0x0B, 0, x0, x0, 0x010")
}

/// I-cache broadcast all harts to invalid all items instruction.
///
/// Invalidates all I-cache table items, and broadcast other harts to invalid all I-cache items.
/// This operation operates on I-cache on all harts.
///
/// # Permissions
///
/// Can run on M or S mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906 and C907 cores.
#[inline]
pub unsafe fn icache_ialls() {
    // th.icache.ialls
    asm!(".insn i 0x0B, 0, x0, x0, 0x011")
}

/// L2-cache clean all dirty items instruction.
///
/// Clears all L2-cache table items, write all dirty items to next level storage.
///
/// # Permissions
///
/// Must run on M or S mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910 core.
#[inline]
pub unsafe fn l2cache_call() {
    // th.l2cache.call
    asm!(".insn i 0x0B, 0, x0, x0, 0x015")
}

/// L2-cache invalid all items instruction.
///
/// Invalidates all L2-cache table items.
///
/// # Permissions
///
/// Can run on M or S mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910 and C906 cores.
#[inline]
pub unsafe fn l2cache_iall() {
    // th.l2cache.iall
    asm!(".insn i 0x0B, 0, x0, x0, 0x016")
}

/// L2-cache clean all dirty and invalid item instruction.
///
/// Writes all L2-cache dirty items to next level storage, and invalidate all L2-cache table items.
///
/// # Permissions
///
/// Can run on M or S mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910 core.
#[inline]
pub unsafe fn l2cache_ciall() {
    // th.l2cache.ciall
    asm!(".insn i 0x0B, 0, x0, x0, 0x017")
}

/// D-cache clean dirty item on way and set instruction.
///
/// Writes D-cache dirty table item corresponding to given way and set to next level storage.
///
/// # Permissions
///
/// Can run on M mode, or S mode if applicable.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906, C907, E907 and E906 cores.
///
/// The C910 core has a 2-way set-associative D-cache. Input variable `rs1[31]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configured 32 Kibibytes, `w` equals 13;
/// when configured 64 Kibibytes, `w` equals 14.
///
/// The C906 core has a 4-way set-associative D-cache. Input variable `rs1[31:30]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configured 32 Kibibytes, `w` equals 13;
/// when configured 64 Kibibytes, `w` equals 14.
///
/// The C907 core has a 2-way set-associative D-cache. Input variable `rs1[31]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configured 32 Kibibytes, `w` equals 13;
/// when configured 64 Kibibytes, `w` equals 14.
///
/// The E907 core has a 2-way set-associative D-cache. Input variable `rs1[31]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configured 32 Kibibytes, `w` equals 13;
/// when configured 16 Kibibytes, `w` equals 12, and so on.
///
/// The E906 core has a 2-way set-associative D-cache. Input variable `rs1[31]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configured 32 Kibibytes, `w` equals 13;
/// when configured 16 Kibibytes, `w` equals 12, and so on.
#[inline]
pub unsafe fn dcache_csw(way_and_set: usize) {
    // th.dcache.csw {}
    asm!(".insn i 0x0B, 0, x0, {}, 0x021", in(reg) way_and_set)
}

/// D-cache invalid item for way and set instruction.
///
/// Invalidate D-cache dirty table item corresponding to given way and set.
///
/// # Permissions
///
/// Can run on M mode, or S mode if applicable.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906, C907, E907 and E906 cores.
///
/// The C910 core has a 2-way set-associative D-cache. Input variable `rs1[31]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configured 32 Kibibytes, `w` equals 13;
/// when configured 64 Kibibytes, `w` equals 14.
///
/// The C906 core has a 4-way set-associative D-cache. Input variable `rs1[31:30]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configured 32 Kibibytes, `w` equals 13;
/// when configured 64 Kibibytes, `w` equals 14.
///
/// The C907 core has a 2-way set-associative D-cache. Input variable `rs1[31]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configured 32 Kibibytes, `w` equals 13;
/// when configured 64 Kibibytes, `w` equals 14.
///
/// The E907 core has a 2-way set-associative D-cache. Input variable `rs1[31]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configured 32 Kibibytes, `w` equals 13;
/// when configured 16 Kibibytes, `w` equals 12, and so on.
///
/// The E906 core has a 2-way set-associative D-cache. Input variable `rs1[31]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configured 32 Kibibytes, `w` equals 13;
/// when configured 16 Kibibytes, `w` equals 12, and so on.
#[inline]
pub unsafe fn dcache_isw(way_and_set: usize) {
    // th.dcache.isw
    asm!(".insn i 0x0B, 0, x0, {}, 0x022", in(reg) way_and_set)
}

/// D-cache clean dirty and invalid for way and set instruction.
///
/// Writes L1 D-cache dirty item corresponding to given way and set to next level storage,
/// and invalidate this table item.
/// This instruction only operates on the current hart.
///
/// # Permissions
///
/// Can run on M mode, or S mode if applicable.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906, C907, E907 and E906 cores.
///
/// The C910 core has a 2-way set-associative D-cache. Input variable `rs1[31]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configured 32 Kibibytes, `w` equals 13;
/// when configured 64 Kibibytes, `w` equals 14.
///
/// The C906 core has a 4-way set-associative D-cache. Input variable `rs1[31:30]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configured 32 Kibibytes, `w` equals 13;
/// when configured 64 Kibibytes, `w` equals 14.
///
/// The C907 core has a 2-way set-associative D-cache. Input variable `rs1[31]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configured 32 Kibibytes, `w` equals 13;
/// when configured 64 Kibibytes, `w` equals 14.
///
/// The E907 core has a 2-way set-associative D-cache. Input variable `rs1[31]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configured 32 Kibibytes, `w` equals 13;
/// when configured 16 Kibibytes, `w` equals 12, and so on.
///
/// The E906 core has a 2-way set-associative D-cache. Input variable `rs1[31]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configured 32 Kibibytes, `w` equals 13;
/// when configured 16 Kibibytes, `w` equals 12, and so on.
#[inline]
pub unsafe fn dcache_cisw(way_and_set: usize) {
    // th.dcache.cisw
    asm!(".insn i 0x0B, 0, x0, {}, 0x023", in(reg) way_and_set)
}

/// L1 D-cache clean dirty item for virtual address instruction.
///
/// Writes D-cache table item corresponding to virtual address `va` to next level storage.
/// This operation effects on L1-cache on all harts.
///
/// # Permissions
///
/// Can run on M, S or U mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception, or load page fault exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, and `mxstatus.ucme = 1`, this instruction can be run on U mode.
/// - When `mxstatus.theadisaee = 1`, and `mxstatus.ucme = 0`,
///   this instruction will raise illegal instruction when being run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906 and C907 cores.
/// On Xuantie C906 User Manual, this instruction is named `DCACHE.CVA`.
#[inline]
pub unsafe fn dcache_cval1(va: usize) {
    // th.dcache.cval1
    asm!(".insn i 0x0B, 0, x0, {}, 0x024", in(reg) va)
}

/// D-cache clean dirty item for virtual address instruction.
///
/// Writes D-cache and L2-cache table item corresponding to virtual address `va` to next level storage.
/// This operation effects on all harts and the L2-cache.
///
/// # Permissions
///
/// Can run on M or S mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception, or load page fault exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, and `mxstatus.ucme = 1`, this instruction can be run on U mode.
/// - When `mxstatus.theadisaee = 1`, and `mxstatus.ucme = 0`,
///   this instruction will raise illegal instruction when being run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910 and C907 cores.
///
/// The Xuantie C906 User Manual names `DCACHE.CVAL1` as `DCACHE.CVA`; to clean dirty item on
/// C906 you may need to use function [`dcache_cval1`] on this library.
#[inline]
pub unsafe fn dcache_cva(va: usize) {
    // th.dcache.cva
    asm!(".insn i 0x0B, 0, x0, {}, 0x025", in(reg) va)
}

/// D-cache invalid item for virtual address instruction.
///
/// Invalidates D-cache or L2-cache (if applicable) table item corresponding to virtual address `va`.
///
/// This instruction operates on the current hart. If applicable, this instruction will
/// operates on L2-cache, and decide whether to broadcast to other harts according to
/// the share attribute of the virtual address.
///
/// # Permissions
///
/// Can run on M or S mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception, or load page fault exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, this instruction will raise illegal instruction when being run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906 and C907 cores.
#[inline]
pub unsafe fn dcache_iva(va: usize) {
    // th.dcache.iva
    asm!(".insn i 0x0B, 0, x0, {}, 0x026", in(reg) va)
}

/// D-cache clean dirty and invalid for virtual address instruction.
///
/// Write D-cache or L2-cache (if applicable) table item corresponding to virtual address `va`
/// to next level storage, and invalidate this table item.
///
/// This instruction operates on the current hart. If applicable, this instruction will
/// operates on L2-cache, and decide whether to broadcast to other harts according to
/// the share attribute of the virtual address.
///
/// # Permissions
///
/// Can run on M or S mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception, or load page fault exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, and `mxstatus.ucme = 1`, this instruction can be run on U mode.
/// - When `mxstatus.theadisaee = 1`, and `mxstatus.ucme = 0`,
///   this instruction will raise illegal instruction when being run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906 and C907 cores.
#[inline]
pub unsafe fn dcache_civa(va: usize) {
    // th.dcache.civa
    asm!(".insn i 0x0B, 0, x0, {}, 0x027", in(reg) va)
}

/// L1 D-cache clean dirty item for physical address instruction.
///
/// Writes D-cache table item corresponding to physical address `pa` to next level storage.
/// This operation effects on L1-cache for all harts.
///
/// # Permissions
///
/// Can run on M mode, or S mode if applicable.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906, C907, E907 and E906 cores.
/// On Xuantie C906 User Manual, Xuantie E907 User Manual and Xuantie E906 User Manual,
/// this instruction is named `DCACHE.CPA`.
#[inline]
pub unsafe fn dcache_cpal1(pa: usize) {
    // th.dcache.cpal1
    asm!(".insn i 0x0B, 0, x0, {}, 0x028", in(reg) pa)
}

/// D-cache clean dirty item for physical address instruction.
///
/// Writes D-cache and L2-cache table item corresponding to physical address `pa` to next level storage.
/// This operation effects on all harts and the L2-cache.
///
/// # Permissions
///
/// Can run on M or S mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910 and C907 cores.
///
/// The Xuantie C906 User Manual, Xuantie E907 User Manual and Xuantie E906 User Manual
/// names `DCACHE.CPAL1` as `DCACHE.CPA`; to clean dirty item on
/// these cores you may need to use function [`dcache_cpal1`] on this library.
#[inline]
pub unsafe fn dcache_cpa(pa: usize) {
    // th.dcache.cpa
    asm!(".insn i 0x0B, 0, x0, {}, 0x029", in(reg) pa)
}

#[inline]
/// D-cache invalid item for physical address instruction.
///
/// Invalidates D-cache table item corresponding to physical address `pa`.
///
/// # Permissions
///
/// Can run on M mode, or S mode if applicable.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906, C907, E907 and E906 cores.
pub unsafe fn dcache_ipa(pa: usize) {
    // th.dcache.ipa
    asm!(".insn i 0x0B, 0, x0, {}, 0x02A", in(reg) pa)
}

/// D-cache clean dirty and invalid for physical address instruction.
///
/// Writes D-cache or L2-cache (if applicable) table item corresponding to physical address `pa`
/// to next level storage, and invalidate this table item.
/// If applicable, this instruction operates on all harts and the L2-cache.
///
/// # Permissions
///
/// Can run on M mode, or S mode if applicable.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906, C907, E907 and E906 cores.
#[inline]
pub unsafe fn dcache_cipa(pa: usize) {
    // th.dcache.cipa
    asm!(".insn i 0x0B, 0, x0, {}, 0x02B", in(reg) pa)
}

/// I-cache invalid item for virtual address instruction.
///
/// Invalidates the I-cache table item corresponding to virtual address `va`.
///
/// This instruction operates on the current hart. If applicable, this instruction will
/// operates on L2-cache, and decide whether to broadcast to other harts according to
/// the share attribute of the virtual address.
///
/// # Permissions
///
/// Can run on M, S or U mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception, or load page fault exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, and `mxstatus.ucme = 1`, this instruction can be run on U mode.
/// - When `mxstatus.theadisaee = 1`, and `mxstatus.ucme = 0`,
///   this instruction will raise illegal instruction when being run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906 and C907 cores.
#[inline]
pub unsafe fn icache_iva(va: usize) {
    // th.icache.iva
    asm!(".insn i 0x0B, 0, x0, {}, 0x030", in(reg) va)
}

/// I-cache invalid item for physical address instruction.
///
/// Invalidates I-cache table item corresponding to physical address `pa`.
/// If applicable, this instruction operates on all harts.
///
/// # Permissions
///
/// Can run on M mode, or S mode if applicable.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906, C907, E907, E906 and E902 cores.
#[inline]
pub unsafe fn icache_ipa(pa: usize) {
    // th.icache.ipa
    asm!(".insn i 0x0B, 0, x0, {}, 0x038", in(reg) pa)
}
