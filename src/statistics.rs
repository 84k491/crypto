use crate::position::Position;

pub struct Statistics {
    init_depo: f32,
    profit_positions: u32,
    loss_positions: u32,
    total_currency_delta: f32,
    best_profit: f32,
    worst_loss: f32,
    highest_depo: f32,
    lowest_depo: f32,
}

impl Statistics {
    pub fn new(initial_depo: f32) -> Statistics {
        return Statistics {
            init_depo: initial_depo,
            profit_positions: 0,
            loss_positions: 0,
            total_currency_delta: 0f32,
            best_profit: 0f32,
            worst_loss: 0f32,
            highest_depo: initial_depo,
            lowest_depo: initial_depo,
        }
    }

    pub fn on_position_close(&mut self, pos: &Position) {
        if pos.currency_delta > 0f32 {
            self.profit_positions += 1;
            if self.best_profit < pos.currency_delta {
                self.best_profit = pos.currency_delta;
            }
        }
        else {
            self.loss_positions += 1;
            if self.worst_loss > pos.currency_delta {
                self.worst_loss = pos.currency_delta;
            }
        }

        self.total_currency_delta += pos.currency_delta;

        if self.total_currency_delta + self.init_depo > self.highest_depo {
            self.highest_depo = self.total_currency_delta + self.init_depo;
        }
        if self.total_currency_delta + self.init_depo < self.lowest_depo {
            self.lowest_depo = self.total_currency_delta + self.init_depo;
        }
    }

    pub fn print(&self) {
        let depo_procent_delta = 100f32 * self.total_currency_delta / self.init_depo;
        let final_depo = self.init_depo + self.total_currency_delta;
        println!("Statistics:");
        println!(" Depo:");
        println!("  Initial depo: {}", self.init_depo);
        println!("  Final depo: {}", final_depo);
        println!("  Depo delta persent: {}%", depo_procent_delta);
        println!("  Depo delta: {}", self.total_currency_delta);
        println!("  Highest depo: {}", self.highest_depo);
        println!("  Lowest depo: {}", self.lowest_depo);
        println!("  Highest depo persent: {}%", 100f32 * (self.highest_depo - self.init_depo) / self.init_depo);
        println!("  Lowest depo percent: {}%", 100f32 * (self.lowest_depo - self.init_depo) / self.init_depo);
        println!(" Positions:");
        println!("  Profit positions: {}", self.profit_positions);
        println!("  Loss positions: {}", self.loss_positions);
        println!("  Best profit: {}", self.best_profit);
        println!("  Worst loss: {}", self.worst_loss);
        println!(" Result:");
        println!("  Win rate: {}%", (100 * self.profit_positions) / (self.profit_positions + self.loss_positions));
    }
}
