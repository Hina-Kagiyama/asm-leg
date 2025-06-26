use std::fmt::{Debug, Display};

#[derive(Clone, Copy)]
pub enum Reg {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    P,
    I,
    O,
}

impl Debug for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reg::R0 => write!(f, "$0"),
            Reg::R1 => write!(f, "$1"),
            Reg::R2 => write!(f, "$2"),
            Reg::R3 => write!(f, "$3"),
            Reg::R4 => write!(f, "$4"),
            Reg::R5 => write!(f, "$5"),
            Reg::P => write!(f, "$pc"),
            Reg::I => write!(f, "$in"),
            Reg::O => write!(f, "$out"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Val {
    Reg(Reg),
    Im(u8),
}

impl Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Reg(reg) => write!(f, "{reg:?}"),
            Val::Im(x) => write!(f, "{x}"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Bop {
    Add,
    Sub,
    And,
    Or,
    Xor,
}

impl Debug for Bop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bop::Add => write!(f, "add"),
            Bop::Sub => write!(f, "sub"),
            Bop::And => write!(f, "and"),
            Bop::Or => write!(f, "or"),
            Bop::Xor => write!(f, "xor"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Uop {
    Not,
}

impl Debug for Uop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Uop::Not => write!(f, "not"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Cmp {
    Eq,
    Neq,
    Lt,
    Leq,
    Gt,
    Geq,
}

impl Debug for Cmp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cmp::Eq => write!(f, "eq"),
            Cmp::Neq => write!(f, "neq"),
            Cmp::Lt => write!(f, "lt"),
            Cmp::Leq => write!(f, "leq"),
            Cmp::Gt => write!(f, "gt"),
            Cmp::Geq => write!(f, "geq"),
        }
    }
}

pub enum Stmt {
    Bin(Bop, Val, Val, Reg),
    Un(Uop, Val, Reg),
    Save {
        addr: Val,
        val: Val,
    },
    Load {
        addr: Val,
        reg: Reg,
    },
    Label(String),
    Br {
        label: String,
        lhs: Val,
        rhs: Val,
        cmp: Cmp,
    },
}

impl Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Bin(bop, val, val1, reg) => write!(f, "{reg:?} <- {val:?} {bop:?} {val1:?}"),
            Stmt::Un(uop, val, reg) => write!(f, "{reg:?} <- {uop:?} {val:?}"),
            Stmt::Save { addr, val } => write!(f, "[{addr:?}] <- {val:?}"),
            Stmt::Load { addr, reg } => write!(f, "{reg:?} <- [{addr:?}]"),
            Stmt::Label(l) => write!(f, "{l}:"),
            Stmt::Br {
                label,
                lhs,
                rhs,
                cmp,
            } => write!(f, "{lhs:?} {cmp:?} {rhs:?} ? {label}"),
        }
    }
}

pub trait ToNum {
    fn to_num(self) -> u8;
}
impl ToNum for Reg {
    fn to_num(self) -> u8 {
        match self {
            Reg::R0 => 0,
            Reg::R1 => 1,
            Reg::R2 => 2,
            Reg::R3 => 3,
            Reg::R4 => 4,
            Reg::R5 => 5,
            Reg::P => 6,
            Reg::I => 7,
            Reg::O => 7,
        }
    }
}
impl ToNum for Bop {
    fn to_num(self) -> u8 {
        match self {
            Bop::Add => 0,
            Bop::Sub => 1,
            Bop::And => 2,
            Bop::Or => 3,
            Bop::Xor => 5,
        }
    }
}
impl ToNum for Uop {
    fn to_num(self) -> u8 {
        match self {
            Uop::Not => 4,
        }
    }
}
impl ToNum for Val {
    fn to_num(self) -> u8 {
        match self {
            Val::Reg(reg) => reg.to_num(),
            Val::Im(x) => x,
        }
    }
}
impl ToNum for Cmp {
    fn to_num(self) -> u8 {
        match self {
            Cmp::Eq => 16,
            Cmp::Neq => 17,
            Cmp::Lt => 18,
            Cmp::Leq => 19,
            Cmp::Gt => 20,
            Cmp::Geq => 21,
        }
    }
}

impl Val {
    pub fn is_reg(&self) -> bool {
        matches!(self, Self::Reg(..))
    }
    pub fn is_im(&self) -> bool {
        matches!(self, Self::Im(..))
    }
}
fn fix_u<T: ToNum + Copy>(op: &T, x: &Val) -> u8 {
    op.to_num() + x.is_im().then_some(128).unwrap_or(0)
}
fn fix_b<T: ToNum + Copy>(op: &T, l: &Val, r: &Val) -> u8 {
    op.to_num() + l.is_im().then_some(128).unwrap_or(0) + r.is_im().then_some(64).unwrap_or(0)
}

const LOAD: u8 = 6;
const SAVE: u8 = 7;

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Bin(bop, val, val1, reg) => write!(
                f,
                "{} {} {} {}",
                fix_b(bop, val, val1),
                val.to_num(),
                val1.to_num(),
                reg.to_num()
            ),
            Stmt::Un(uop, val, reg) => {
                write!(f, "{} {} 0 {}", fix_u(uop, val), val.to_num(), reg.to_num())
            }
            Stmt::Save { addr, val } => write!(f, "{SAVE} {} {} 0", addr.to_num(), val.to_num()),
            Stmt::Load { addr, reg } => write!(f, "{LOAD} {} 0 {}", addr.to_num(), reg.to_num()),
            Stmt::Label(l) => write!(f, "label {l}"),
            Stmt::Br {
                label,
                lhs,
                rhs,
                cmp,
            } => {
                write!(
                    f,
                    "{} {} {} {}",
                    fix_b(cmp, lhs, rhs),
                    lhs.to_num(),
                    rhs.to_num(),
                    label
                )
            }
        }
    }
}
