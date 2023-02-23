trait Common {
    fn calc(self) -> u32;
}

struct Score {
    math: u32,
    english: u32,
}

impl Common for Score {
    fn calc(self) -> u32 {
        self.math + self.english
    }
}

struct Profit {
    cost: u32,    // 成本
    revenue: u32, // 营业额
}

impl Common for Profit {
    fn calc(self) -> u32 {
        self.cost - self.revenue
    }
}

pub fn main() {
    let score = Score {
        math: 100,
        english: 120,
    };

    let profit = Profit {
        cost: 190,
        revenue: 120,
    };

    println!("{}", score.calc());
    println!("{}", profit.calc());
}
