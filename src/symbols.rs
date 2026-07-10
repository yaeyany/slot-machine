#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Symbol {
    Cherry{payout: u32, weight: u32},
    Bar{payout: u32, weight: u32},
    Gold{payout: u32, weight: u32},
    Diamond{payout: u32, weight: u32},
    Seven{payout: u32, weight: u32}
}

impl Symbol {
    pub fn display(&self) -> &'static str {
        match self {
            Symbol::Cherry { .. } => "🍒",
            Symbol::Bar { .. } => "🍫",
            Symbol::Gold { .. } => "🧈",
            Symbol::Diamond { .. } => "💎",
            Symbol::Seven { .. } => "7️⃣",
        }
    }
}

impl Symbol {
    pub fn payout(&self) -> u32 {
        match self {
            Symbol::Cherry { payout, .. } => *payout,
            Symbol::Bar { payout, .. } => *payout,
            Symbol::Gold { payout, .. } => *payout,
            Symbol::Diamond { payout, .. } => *payout,
            Symbol::Seven { payout, .. } => *payout,
        }
    }
}

pub const CHERRY: Symbol = Symbol::Cherry { payout: 2, weight: 50 };
pub const BAR: Symbol = Symbol::Bar { payout: 3, weight: 30 };
pub const GOLD: Symbol = Symbol::Gold { payout: 5, weight: 20 };
pub const DIAMOND: Symbol = Symbol::Diamond { payout: 10, weight: 10 };
pub const SEVEN: Symbol = Symbol::Seven { payout: 100, weight: 1 };

pub const ALL_SYMBOLS: [Symbol; 5] = [CHERRY, BAR, GOLD, DIAMOND, SEVEN];
pub const WEIGHTS: [u32; 5] = [50, 30, 20, 10, 1];
