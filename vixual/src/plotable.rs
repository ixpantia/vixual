#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Continuous,
    Discrete,
}

pub trait Plotable: PartialOrd + Clone + Send + Sync + 'static {
    fn to_f64(&self) -> f64;
    fn from_f64(f: f64) -> Self;
    fn to_plot_string(&self) -> String;
    fn data_type() -> DataType;
}

impl Plotable for f64 {
    fn to_f64(&self) -> f64 {
        *self
    }
    fn from_f64(f: f64) -> Self {
        f
    }
    fn to_plot_string(&self) -> String {
        ToString::to_string(self)
    }
    fn data_type() -> DataType {
        DataType::Continuous
    }
}

impl Plotable for i32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn from_f64(f: f64) -> Self {
        f as i32
    }
    fn to_plot_string(&self) -> String {
        ToString::to_string(self)
    }
    fn data_type() -> DataType {
        DataType::Continuous
    }
}

impl Plotable for String {
    fn to_f64(&self) -> f64 {
        self.parse().unwrap_or(0.0)
    }
    fn from_f64(f: f64) -> Self {
        f.to_string()
    }
    fn to_plot_string(&self) -> String {
        self.clone()
    }
    fn data_type() -> DataType {
        DataType::Discrete
    }
}

impl Plotable for &'static str {
    fn to_f64(&self) -> f64 {
        self.parse().unwrap_or(0.0)
    }
    fn from_f64(_f: f64) -> Self {
        ""
    }
    fn to_plot_string(&self) -> String {
        ToString::to_string(self)
    }
    fn data_type() -> DataType {
        DataType::Discrete
    }
}
