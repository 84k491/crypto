use crate::signal::{Side, Signal};
use crate::order::{Order, Trade};
use crate::position::Position;
use crate::statistics::Statistics;

pub struct PortfolioManager {
    currency_depo: f32,
    absolute_commodity_limit: f32,

    pos: Option<Position>,
}

pub struct MockTradingGateway {
    pm: PortfolioManager,
    stat: Statistics,

    last_price: f32,
}

impl MockTradingGateway {
    pub fn new() -> Self {
        let init_depo = 1000f32;
        let pm = PortfolioManager{
            currency_depo: init_depo.clone(),
            absolute_commodity_limit: 0.1f32,
            pos: None,
        };

        return MockTradingGateway {
            pm,
            stat: Statistics::new(init_depo),
            last_price: 0f32,
        };
    }

    pub fn get_statistics(&self) -> &Statistics {
        return &self.stat;
    }

    fn modify_position(&mut self, desired: f32, price: f32) {
        let qty = match &self.pm.pos {
            Some(pos) => pos.commodity_qty.clone(),
            None => 0f32,
        };
        let qty_delta = desired - qty;
        let currency_delta = qty_delta * price as f32 * -1.0f32;

        // send order, get trade on this line

        if self.pm.pos.as_ref().is_none() {
            self.pm.pos = Some(Position {
                commodity_qty: 0f32,
                currency_delta: 0f32 });
        }
        let pos: &mut Position = self.pm.pos.as_mut().unwrap();

        pos.commodity_qty += qty_delta;
        pos.currency_delta += currency_delta;
        self.pm.currency_depo += currency_delta;

        if pos.commodity_qty == 0f32 {
            self.stat.on_position_close(pos);
            self.pm.pos = None;
        }
    }

    pub fn on_signal(&mut self, s: Signal) {
        let sign = match s.side {
            Side::Buy => 1f32,
            Side::Sell => -1f32,
        };
        let desired_qty = self.pm.absolute_commodity_limit * sign;
        if self.pm.pos.is_some() {
            self.modify_position(0f32, s.price);
        }
        self.modify_position(desired_qty, s.price);
        self.last_price = s.price;
    }

    pub fn close_position(&mut self) {
        self.modify_position(0f32, self.last_price);
    }

    fn send_order(order: Order) -> Option<Trade> {
        return None;
    }
}

impl Drop for MockTradingGateway {
    fn drop(&mut self) {
        self.close_position();
    }
}
