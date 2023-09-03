use std::collections::LinkedList;
use crate::candle::Candle;
use crate::signal::Signal;
use crate::signal::Side;

pub struct Strategy {
    candles: LinkedList::<Candle>,
    signals: LinkedList::<Signal>,

    signal_iter: i32,
    side_iter: Side,
}

impl Strategy {
    pub fn new() -> Strategy {
        return Strategy{
            candles: LinkedList::<Candle>::new(),
            signals: LinkedList::<Signal>::new(),
            signal_iter: 0,
            side_iter: Side::Buy,
        };
    }

    pub fn push_candle(&mut self, candle: &Candle) -> Option<Signal> {
        self.candles.push_back(candle.clone());

        self.signal_iter += 1;
        if self.signal_iter == 100 {
            self.signal_iter = 0;
            let signal = Signal{ts: candle.close_ts, price: candle.close_price, side: self.side_iter.clone()};
            match self.side_iter {
                Side::Buy => { self.side_iter = Side::Sell; }
                Side::Sell => { self.side_iter = Side::Buy; }
            }
            self.signals.push_back(signal.clone());
            return Some(signal);
        }
        return None;
    }

    pub fn get_signals(&self) -> &LinkedList::<Signal> {
        return &self.signals;
    }

    pub fn get_candles(&self) -> &LinkedList::<Candle> {
        return &self.candles;
    }
}
