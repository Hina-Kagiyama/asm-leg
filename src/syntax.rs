use std::{
    fmt::{Debug, Display},
    time::{SystemTime, UNIX_EPOCH},
};

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
    Shl,
    Shr,
    Rol,
    Ror,
    Ashr,
    Mul,
    Div,
}

impl Debug for Bop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bop::Add => write!(f, "add"),
            Bop::Sub => write!(f, "sub"),
            Bop::And => write!(f, "and"),
            Bop::Or => write!(f, "or"),
            Bop::Xor => write!(f, "xor"),
            Bop::Shl => write!(f, "shl"),
            Bop::Shr => write!(f, "shr"),
            Bop::Rol => write!(f, "rol"),
            Bop::Ror => write!(f, "ror"),
            Bop::Ashr => write!(f, "ashr"),
            Bop::Mul => write!(f, "mul"),
            Bop::Div => write!(f, "div"),
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

impl Cmp {
    pub fn inv(self) -> Self {
        match self {
            Cmp::Eq => Cmp::Neq,
            Cmp::Neq => Cmp::Eq,
            Cmp::Lt => Cmp::Geq,
            Cmp::Leq => Cmp::Gt,
            Cmp::Gt => Cmp::Leq,
            Cmp::Geq => Cmp::Lt,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Cond {
    pub(crate) lhs: Val,
    pub(crate) rhs: Val,
    pub(crate) cmp: Cmp,
}

impl Debug for Cond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { lhs, rhs, cmp } = self;
        write!(f, "{lhs:?} {cmp:?} {rhs:?}")
    }
}

pub enum Stmt {
    Bin(Bop, Val, Val, Reg),
    Un(Uop, Val, Reg),
    Hf(Reg),
    Call(String),
    Args,
    Ret,
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
        cond: Cond,
    },
    While {
        cond: Cond,
        block: Vec<Stmt>,
    },
    If {
        cond: Cond,
        yes: Vec<Stmt>,
        no: Vec<Stmt>,
    },
    Loop {
        block: Vec<Stmt>,
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
            Stmt::Br { label, cond } => write!(f, "{cond:?} ? {label}"),
            Stmt::While { cond, block } => {
                write!(
                    f,
                    "while {cond:?} do\n{}\ndone",
                    block
                        .iter()
                        .map(|x| format!("{x:?}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
            Stmt::If { cond, yes, no } => write!(
                f,
                "if {cond:?} then\n{}\nelse\n{}\ndone",
                yes.iter()
                    .map(|x| format!("{x:?}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
                no.iter()
                    .map(|x| format!("{x:?}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            Stmt::Loop { block } => {
                write!(
                    f,
                    "loop\n{}\ndone",
                    block
                        .iter()
                        .map(|x| format!("{x:?}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
            Stmt::Hf(reg) => write!(f, "{reg:?} <- $high"),
            Stmt::Call(s) => write!(f, "call {s}"),
            Stmt::Ret => write!(f, "ret"),
            Stmt::Args => write!(f, "args"),
        }
    }
}

pub trait ToNum {
    fn to_num(self) -> u8;
}
impl ToNum for u8 {
    fn to_num(self) -> u8 {
        self
    }
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
            Bop::Shl => 8,
            Bop::Shr => 9,
            Bop::Rol => 10,
            Bop::Ror => 11,
            Bop::Ashr => 12,
            Bop::Mul => 13,
            Bop::Div => 14,
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
const HF: u8 = 15;

fn get_stamp() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u32
}

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
            Stmt::Save { addr, val } => {
                write!(
                    f,
                    "{} {} {} 0",
                    fix_b(&SAVE, addr, val),
                    addr.to_num(),
                    val.to_num()
                )
            }
            Stmt::Load { addr, reg } => write!(
                f,
                "{} {} 0 {}",
                LOAD + if addr.is_im() { 128 } else { 0 },
                addr.to_num(),
                reg.to_num()
            ),
            Stmt::Label(l) => write!(f, "label {l}"),
            Stmt::Br { label, cond } => {
                write!(
                    f,
                    "{} {} {} {}",
                    fix_b(&cond.cmp, &cond.lhs, &cond.rhs),
                    cond.lhs.to_num(),
                    cond.rhs.to_num(),
                    label
                )
            }
            Stmt::While { cond, block } => {
                let stamp = get_stamp();
                let jeq = Cmp::Eq.to_num();
                write!(
                    f,
                    "label L_{stamp:X}\n{} {} {} E_{stamp:X}\n{}\n{jeq} 0 0 L_{stamp:X}\nlabel E_{stamp:X}",
                    fix_b(&cond.cmp.inv(), &cond.lhs, &cond.rhs),
                    cond.lhs.to_num(),
                    cond.rhs.to_num(),
                    block
                        .iter()
                        .map(|x| format!("{x}"))
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
            }
            Stmt::If { cond, yes, no } => {
                let stamp = get_stamp();
                let jeq = Cmp::Eq.to_num();
                write!(
                    f,
                    "{} {} {} T_{stamp:X}\n{}{jeq} 0 0 D_{stamp:X}\nlabel T_{stamp:X}\n{}\nlabel D_{stamp:X}",
                    fix_b(&cond.cmp, &cond.lhs, &cond.rhs),
                    cond.lhs.to_num(),
                    cond.rhs.to_num(),
                    if no.len() > 0 {
                        no.iter()
                            .map(|x| format!("{x}"))
                            .collect::<Vec<_>>()
                            .join("\n")
                            + "\n"
                    } else {
                        "".to_string()
                    },
                    yes.iter()
                        .map(|x| format!("{x}"))
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
            }
            Stmt::Loop { block } => {
                let stamp = get_stamp();
                let jeq = Cmp::Eq.to_num();
                write!(
                    f,
                    "label L_{stamp:X}\n{}\n{jeq} 0 0 L_{stamp:X}",
                    block
                        .iter()
                        .map(|x| format!("{x}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
            Stmt::Hf(reg) => {
                write!(f, "{HF} 0 0 {}", reg.to_num())
            }
            Stmt::Args => {
                writeln!(f, "65 5 1 5\n{SAVE} 5 1 0")?;
                writeln!(f, "65 5 1 5\n{SAVE} 5 2 0")?;
                writeln!(f, "65 5 1 5\n{SAVE} 5 3 0")?;
                write!(f, "65 5 1 5\n{SAVE} 5 4 0")
            }
            Stmt::Call(s) => {
                writeln!(f, "64 6 8 0\n{SAVE} 5 0 0\n64 {s} 0 6")
            }
            Stmt::Ret => {
                writeln!(f, "{LOAD} 5 0 0\n64 5 1 5")?;
                writeln!(f, "{LOAD} 5 0 4\n64 5 1 5")?;
                writeln!(f, "{LOAD} 5 0 3\n64 5 1 5")?;
                writeln!(f, "{LOAD} 5 0 2\n64 5 1 5")?;
                write!(f, "{LOAD} 5 0 1\n64 5 1 5\n64 0 0 6")
            }
        }
    }
}
