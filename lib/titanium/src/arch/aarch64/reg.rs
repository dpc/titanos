def_reg!(
    mpidr_el1, u64, ro,
    AFF0(7, 0),
    );

def_reg!(
    ttbr0_el1, u64, rw,
    BADDR(47, 0),
    ASID(63, 48),
    );

def_reg!(
    ttbr1_el1, u64, rw,
    BADDR(47, 0),
    ASID(63, 48),
    );

def_reg!(
    sctlr_el1, u64, rw,
    M(0, 0),
    );

def_reg!(
    mair_el1, u64, rw,
    );

def_reg!(
    tcr_el1, u64, rw,
    TBI1(38, 38),
    TBI0(37, 37),
    IPS(34, 32),
    TG1(31, 30),
    SH1(29, 28),
    ORGN1(27, 26),
    IRGN1(25, 24),
    EPD1(23, 23),
    A1(22, 22),
    T1SZ(21, 16),
    TG0(15, 14),
    S0(13, 12),
    ORGN0(11, 10),
    IRGN0(9, 8),
    EPD0(7, 7),
    T0SZ(5, 0),
    );

pub const TG1_16K : u64 = 0x1;
pub const TG1_4K : u64 = 0x2;
pub const TG1_64K : u64 = 0x3;

pub const TG0_4K : u64 = 0x0;
pub const TG0_64K : u64 = 0x1;
pub const TG0_16K : u64 = 0x2;

def_reg!(
    daif, u64, rw,
    F(0, 0),
    I(1, 1),
    A(2, 2),
    D(3, 3),
    );

def_reg!(
    vbar_el1, u64, rw,
    );


