use std::collections::LinkedList;
use crate::candle::Candle;
use crate::signal::Signal;
use crate::signal::Side;

pub struct Strategy {
    candles: LinkedList::<Candle>,
    signals: LinkedList::<Signal>,

    slow_sma: SimpleMovingAverage,
    quick_sma: SimpleMovingAverage,
    side_iter: Side,
    slow_above: Option<bool>,

    slow_sma_res: LinkedList::<(u64, f32)>,
    quick_sma_res: LinkedList::<(u64, f32)>,
}

// static DAY: u32 = HOUR * 24;
static HOUR: u32 = 3600000;
// static MINUTE: u32 = 60000;
// static SECOND: u32 = 1000;

impl Strategy {
    pub fn new() -> Strategy {
        return Strategy{
            candles: LinkedList::<Candle>::new(),
            signals: LinkedList::<Signal>::new(),
            slow_sma: SimpleMovingAverage::new(3 * HOUR),
            quick_sma: SimpleMovingAverage::new(1 * HOUR),
            side_iter: Side::Buy,
            slow_above: None,
            slow_sma_res: LinkedList::<(u64, f32)>::new(),
            quick_sma_res: LinkedList::<(u64, f32)>::new(),
        };
    }

    fn flip_side(&mut self) -> Side {
        match self.side_iter {
            Side::Buy => { self.side_iter = Side::Sell; }
            Side::Sell => { self.side_iter = Side::Buy; }
        }
        return self.side_iter.clone();
    }

    pub fn push_candle(&mut self, candle: &Candle) -> Option<Signal> {
        self.candles.push_back(candle.clone());

        let slow_avg = self.slow_sma.push_price(candle.close_ts, candle.close_price);
        let quick_avg = self.quick_sma.push_price(candle.close_ts, candle.close_price);
        if slow_avg.is_some() {
            self.slow_sma_res.push_back((candle.close_ts, slow_avg.unwrap()));
        }
        if quick_avg.is_some() {
            self.quick_sma_res.push_back((candle.close_ts, quick_avg.unwrap()));
        }

        if slow_avg.is_none() || quick_avg.is_none() {
            return None;
        }
        if self.slow_above.is_none() {
            self.slow_above = Some(slow_avg > quick_avg);
            self.side_iter = if self.slow_above.unwrap() {Side::Sell} else {Side::Buy};
        }

        if (slow_avg > quick_avg && !self.slow_above.unwrap()) ||
           (slow_avg < quick_avg && self.slow_above.unwrap()) {
            let signal = Signal{
                ts: candle.close_ts,
                price: candle.close_price,
                side: self.flip_side()};
            self.slow_above = Some(slow_avg > quick_avg);
            self.signals.push_back(signal.clone());
            return Some(signal);
        }

        return None;
    }

    pub fn get_slow_sma_results(&self) -> &LinkedList::<(u64, f32)> {
        return &self.slow_sma_res;
    }

    pub fn get_quick_sma_results(&self) -> &LinkedList::<(u64, f32)> {
        return &self.quick_sma_res;
    }

    pub fn get_signals(&self) -> &LinkedList::<Signal> {
        return &self.signals;
    }

    pub fn get_candles(&self) -> &LinkedList::<Candle> {
        return &self.candles;
    }
}

pub struct SimpleMovingAverage {
    window_size_us: u32,
    prices: LinkedList<(u64, f32)>,
}

impl SimpleMovingAverage {
    pub fn new(window_size_us: u32) -> SimpleMovingAverage {
        return SimpleMovingAverage{ window_size_us, prices: LinkedList::<(u64, f32)>::new() };
    }

    pub fn push_price(&mut self, ts: u64, price: f32) -> Option<f32> {
        self.prices.push_back((ts, price));

        let time_delta = self.prices.back().unwrap().0 - self.prices.front().unwrap().0;
        let is_ready =
            time_delta >= self.window_size_us as u64;
        if is_ready {
            self.prices.pop_front();
            let res: f32 =
                self.prices
                .iter()
                .map(|(_, p)| *p)
                .reduce(|acc, p| {acc + p})
                .unwrap() / self.prices.len() as f32;

            return Some(res);
        }
        else {
            return None;
        }
    }
}
