use crate::position::Position;

pub struct Statistics {
    init_depo: f32,
    profit_positions: u32,
    loss_positions: u32,
    total_currency_delta: f32,
    best_profit: f32,
    worst_loss: f32,
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
    }

    pub fn print(&self) {
        let procent_delta = 100f32 * self.total_currency_delta / self.init_depo;
        println!("Statistics:");
        println!(" Initial depo: {}", self.init_depo);
        println!(" Final depo: {}", self.init_depo + self.total_currency_delta);
        println!(" Depo persent delta: {}%", procent_delta);
        println!(" Depo currency delta: {}", self.total_currency_delta);
        println!(" =====");
        println!(" Profit positions: {}", self.profit_positions);
        println!(" Loss positions: {}", self.loss_positions);
        println!(" Best profit: {}", self.best_profit);
        println!(" Worst loss: {}", self.worst_loss);
        println!(" =====");
        println!(" Win rate: {}%", (100 * self.profit_positions) / (self.profit_positions + self.loss_positions));
    }
}
