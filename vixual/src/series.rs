use crate::plotable::Plotable;
use leptos::prelude::*;

#[derive(Clone)]
pub struct SeriesInner<X, Y> {
    x: Vec<X>,
    y: Vec<Y>,
}

#[derive(Clone, Copy)]
pub struct Series<X, Y>
where
    X: Plotable,
    Y: Plotable,
{
    data: RwSignal<SeriesInner<X, Y>>,
}

impl<X, Y> Series<X, Y>
where
    X: Plotable,
    Y: Plotable,
{
    pub fn new(x: Vec<X>, y: Vec<Y>) -> Self {
        let data = SeriesInner { x, y };
        Self {
            data: RwSignal::new(data),
        }
    }
    pub fn clear(&self) {
        self.data.update(|d| {
            d.x.clear();
            d.y.clear();
        });
    }
    /// This method updated the underlying signal to the series. Every push
    /// is an access to a Signal. If you need to call this in a loop
    /// it is recommended to use `append`
    pub fn push(&self, x: X, y: Y) {
        self.data.update(move |d| {
            d.x.push(x);
            d.y.push(y);
        });
    }

    pub fn append(&self, x: &mut Vec<X>, y: &mut Vec<Y>) {
        self.data.update(move |d| {
            d.x.append(x);
            d.y.append(y);
        });
    }

    pub fn replace_data(&self, x: Vec<X>, y: Vec<Y>) {
        self.data.update(move |d| {
            d.x = x;
            d.y = y;
        });
    }

    pub fn get_x(&self) -> Vec<X> {
        self.data.get().x
    }
    pub fn get_y(&self) -> Vec<Y> {
        self.data.get().y
    }
}
