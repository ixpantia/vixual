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

#[cfg(feature = "chrono")]
impl Plotable for chrono::NaiveDateTime {
    fn to_f64(&self) -> f64 {
        self.and_utc().timestamp_millis() as f64
    }
    fn from_f64(f: f64) -> Self {
        chrono::DateTime::from_timestamp_millis(f as i64)
            .unwrap_or_default()
            .naive_utc()
    }
    fn to_plot_string(&self) -> String {
        self.format("%Y-%m-%d %H:%M:%S").to_string()
    }
    fn data_type() -> DataType {
        DataType::Continuous
    }
}

#[cfg(feature = "chrono")]
impl Plotable for chrono::DateTime<chrono::Utc> {
    fn to_f64(&self) -> f64 {
        self.timestamp_millis() as f64
    }
    fn from_f64(f: f64) -> Self {
        chrono::DateTime::from_timestamp_millis(f as i64)
            .unwrap_or_default()
            .with_timezone(&chrono::Utc)
    }
    fn to_plot_string(&self) -> String {
        self.format("%Y-%m-%d %H:%M:%S").to_string()
    }
    fn data_type() -> DataType {
        DataType::Continuous
    }
}

#[cfg(feature = "chrono")]
impl Plotable for chrono::DateTime<chrono::Local> {
    fn to_f64(&self) -> f64 {
        self.timestamp_millis() as f64
    }
    fn from_f64(f: f64) -> Self {
        chrono::DateTime::from_timestamp_millis(f as i64)
            .unwrap_or_default()
            .with_timezone(&chrono::Local)
    }
    fn to_plot_string(&self) -> String {
        self.format("%Y-%m-%d %H:%M:%S").to_string()
    }
    fn data_type() -> DataType {
        DataType::Continuous
    }
}

#[cfg(feature = "chrono")]
impl Plotable for chrono::DateTime<chrono::FixedOffset> {
    fn to_f64(&self) -> f64 {
        self.timestamp_millis() as f64
    }
    fn from_f64(f: f64) -> Self {
        chrono::DateTime::from_timestamp_millis(f as i64)
            .unwrap_or_default()
            .with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())
    }
    fn to_plot_string(&self) -> String {
        self.format("%Y-%m-%d %H:%M:%S").to_string()
    }
    fn data_type() -> DataType {
        DataType::Continuous
    }
}

#[cfg(feature = "chrono")]
impl Plotable for chrono::NaiveDate {
    fn to_f64(&self) -> f64 {
        self.and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis() as f64
    }
    fn from_f64(f: f64) -> Self {
        chrono::DateTime::from_timestamp_millis(f as i64)
            .unwrap_or_default()
            .naive_utc()
            .date()
    }
    fn to_plot_string(&self) -> String {
        self.format("%Y-%m-%d").to_string()
    }
    fn data_type() -> DataType {
        DataType::Continuous
    }
}
