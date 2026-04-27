pub trait Plotable: PartialOrd + Clone + Send + Sync + 'static {
    fn to_f64(&self) -> f64;
    fn to_string(&self) -> String;
}

impl Plotable for f64 {
    fn to_f64(&self) -> f64 {
        *self
    }
    fn to_string(&self) -> String {
        ToString::to_string(self)
    }
}

impl Plotable for i32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn to_string(&self) -> String {
        ToString::to_string(self)
    }
}

impl Plotable for String {
    fn to_f64(&self) -> f64 {
        self.parse().unwrap_or(0.0)
    }
    fn to_string(&self) -> String {
        self.clone()
    }
}

impl Plotable for &'static str {
    fn to_f64(&self) -> f64 {
        self.parse().unwrap_or(0.0)
    }
    fn to_string(&self) -> String {
        ToString::to_string(self)
    }
}
