use std::collections::LinkedList;
use crate::candle::Candle;
use crate::signal::{Side, Signal};
use plotters::backend::BitMapBackend;
use plotters::prelude::*;

static OUT_FILE_NAME: &str = "plot.bmp";

pub struct Visualizer {
    candles: LinkedList::<Candle>,
    symbol: String,
    signals: LinkedList::<Signal>,

    additional_series_1: LinkedList::<(u64, f32)>,
    additional_series_2: LinkedList::<(u64, f32)>,
}

impl Visualizer {
    pub fn new(symbol: &str, candles: &LinkedList::<Candle>) -> Visualizer {
        return Visualizer {
            candles: (*candles).clone(),
            symbol: symbol.to_owned(),
            signals: LinkedList::new(),
            additional_series_1: LinkedList::<(u64, f32)>::new(),
            additional_series_2: LinkedList::<(u64, f32)>::new(),
        };
    }

    pub fn set_signals(&mut self, signals: &LinkedList::<Signal>) {
        self.signals = signals.clone();
    }

    pub fn set_additional_1(&mut self, series: LinkedList::<(u64, f32)>) {
        self.additional_series_1 = series;
    }

    pub fn set_additional_2(&mut self, series: LinkedList::<(u64, f32)>) {
        self.additional_series_2 = series;
    }

    fn chart_bounds(&self) -> ((f32, f32), (f32, f32)) {
        let first_candle = self.candles.front().expect("Candles list is empty");
        let mut x_max = first_candle.close_ts as f32;
        let mut x_min = first_candle.close_ts as f32;
        let mut y_max = first_candle.close_price;
        let mut y_min = first_candle.close_price;

        self.candles.iter().for_each(|c| {
            if (c.close_ts as f32) < x_min {
                x_min = c.close_ts as f32;
            }
            if (c.close_ts as f32) > x_max {
                x_max = c.close_ts as f32;
            }
            if c.close_price > y_max {
                y_max = c.close_price;
            }
            if c.close_price < y_min {
                y_min = c.close_price;
            }
        });

        return ((x_min, x_max), (y_min, y_max));
    }

    fn side_to_style(s: &Side) -> plotters::style::ShapeStyle {
        match s {
            Side::Buy => { Into::<ShapeStyle>::into(&GREEN).filled() }
            Side::Sell => { Into::<ShapeStyle>::into(&RED).filled() }
        }
    }

    pub fn draw(&self) {
        let root = BitMapBackend::new(OUT_FILE_NAME, (10920, 1080)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let ((x_min, x_max), (y_min, y_max)) = self.chart_bounds();

        let mut chart = ChartBuilder::on(&root)
                .caption(&self.symbol, ("sans-serif", 50).into_font())
                .margin(20)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(x_min..x_max, y_min..y_max).unwrap();

        let prices: LinkedList<(f32, f32)> =
            self.candles.iter().map(|c| (c.close_ts as f32, c.close_price)).collect();

        chart.configure_mesh().draw().unwrap();

        chart.draw_series(LineSeries::new(prices, &BLACK,)).unwrap()
            .label("price")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

        let add_ser1 = self.additional_series_1.iter().map(|(t, v)| {(*t as f32, *v)});
        chart.draw_series(LineSeries::new(add_ser1, &BLUE,)).unwrap()
            .label("slow_sma")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

        let add_ser2 = self.additional_series_2.iter().map(|(t, v)| {(*t as f32, *v)});
        chart.draw_series(LineSeries::new(add_ser2, &MAGENTA,)).unwrap()
            .label("quick_sma")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

        chart.draw_series(
            self.signals.iter().map(
                |s| {
                    Circle::new((s.ts as f32, s.price), 3, Visualizer::side_to_style(&s.side))
                }))
            .unwrap()
            .label("trades")
            .legend(|(x, y)| Circle::new((x + 10, y), 5, Into::<ShapeStyle>::into(&GREEN).filled()));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw().unwrap();

        root.present().unwrap();

    }
}
