# Vixual

**Vixual** es una biblioteca de visualización de datos escrita en Rust, diseñada para crear gráficos interactivos y reactivos utilizando el framework web [Leptos](https://github.com/leptos-rs/leptos). La biblioteca permite renderizar gráficos SVG directamente en aplicaciones web, aprovechando el poder de WebAssembly (WASM) y la reactividad de Leptos.

## Características

- **Gráficos SVG nativos**: Renderizado de gráficos utilizando SVG puro, sin dependencias de JavaScript
- **Soporte para WebAssembly**: Compilación a WASM para máximo rendimiento en el navegador
- **Reactividad integrada**: Integración nativa con el sistema de señales de Leptos
- **Tipos de gráficos**:
  - Gráficos de barras (`BarPlot`)
  - Gráficos de líneas (`LinePlot`)
- **Soporte para fechas**: Integración con `chrono` para ejes temporales
- **Paletas de colores personalizables**: Sistema de colores con presets y personalización
- **Leyendas interactivas**: Componente de leyenda automático
- **Responsive**: Los gráficos se adaptan al tamaño del contenedor padre
- **SSR (Server-Side Rendering)**: Soporte completo para renderizado del lado del servidor

## Estructura del Proyecto

El repositorio contiene un workspace de Cargo con dos crates principales:

### 1. `vixual/` - Biblioteca Principal

El crate principal que contiene todos los componentes de visualización:

- **`plotable.rs`**: Trait fundamental que define qué tipos de datos pueden ser plotados
- **`series.rs`**: Estructura `Series<X, Y>` para manejar conjuntos de datos
- **`palette.rs`**: Sistema de paletas de colores (`ColorPalette`, `Hex`)
- **`legend.rs`**: Componente `Legend` y trait `LegendSource`
- **`bar_plot.rs`**: Implementación de gráficos de barras
- **`line_plot.rs`**: Implementación de gráficos de líneas

### 2. `showcase/` - Aplicación de Demostración

Una aplicación web completa que demuestra todas las capacidades de la biblioteca:

- **Home**: Página de inicio con navegación
- **Bar Plot**: Demostración interactiva de gráficos de barras
- **Line Plot**: Demostración interactiva de gráficos de líneas
- **Chrono Plot**: Ejemplo de uso con fechas y tiempos

## Instalación

### Prerrequisitos

1. **Rust toolchain**:
   ```bash
   rustup toolchain install nightly
   rustup target add wasm32-unknown-unknown
   ```

2. **cargo-leptos**:
   ```bash
   cargo install cargo-leptos --locked
   ```

3. **Sass** (opcional, para estilos):
   ```bash
   npm install -g sass
   ```

## Uso

### Dependencias en tu Proyecto

Agrega `vixual` a tu `Cargo.toml`:

```toml
[dependencies]
vixual = { path = "../vixual", features = ["bar_plot", "line_plot", "chrono"] }
leptos = { version = "0.8" }
```

### Features Disponibles

- `bar_plot`: Habilita el componente `BarPlot`
- `line_plot`: Habilita el componente `LinePlot`
- `chrono`: Habilita soporte para tipos de fecha de `chrono`

### Ejemplo Básico - Gráfico de Barras

```rust
use vixual::bar_plot::{BarPlot, BarPlotConfig};
use vixual::series::Series;
use vixual::palette::ColorPalette;
use leptos::prelude::*;

#[component]
fn MyApp() -> impl IntoView {
    let series = Signal::stored(vec![
        Series::new(
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec![10.0, 20.0, 15.0],
        ),
        Series::new(
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec![12.0, 18.0, 22.0],
        ),
    ]);

    let config = BarPlotConfig::builder()
        .with_series(series)
        .title("Mi Gráfico de Barras")
        .x_label("Categorías")
        .y_label("Valores")
        .palette(ColorPalette::default())
        .build();

    let config_sig = Signal::derive(move || config.clone());

    view! {
        <div style="height: 400px;">
            <BarPlot config=config_sig />
        </div>
    }
}
```

### Ejemplo Básico - Gráfico de Líneas

```rust
use vixual::line_plot::{LinePlot, LinePlotConfig};
use vixual::series::Series;
use vixual::legend::Legend;
use leptos::prelude::*;

#[component]
fn MyApp() -> impl IntoView {
    let series = Signal::stored(vec![
        Series::with_label(
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![10.0, 15.0, 13.0, 17.0, 20.0],
            "Serie A",
        ),
        Series::with_label(
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![8.0, 12.0, 16.0, 14.0, 18.0],
            "Serie B",
        ),
    ]);

    let config = LinePlotConfig::builder()
        .with_series(series)
        .title("Tendencia de Datos")
        .x_label("Tiempo")
        .y_label("Cantidad")
        .build();

    let config_sig = Signal::stored(config);

    view! {
        <div>
            <div style="height: 400px;">
                <LinePlot config=config_sig />
            </div>
            <Legend config=config_sig />
        </div>
    }
}
```

### Ejemplo con Fechas (Chrono)

```rust
use chrono::{Duration, Utc};
use vixual::line_plot::{LinePlot, LinePlotConfig};
use vixual::series::Series;

#[component]
fn ChronoPlot() -> impl IntoView {
    let now = Utc::now();
    
    let series = Signal::stored(vec![
        Series::new(
            vec![
                now - Duration::days(5),
                now - Duration::days(4),
                now - Duration::days(3),
                now - Duration::days(2),
                now - Duration::days(1),
                now,
            ],
            vec![10.0, 15.0, 8.0, 12.0, 20.0, 25.0],
        ),
    ]);

    let config = LinePlotConfig::builder()
        .with_series(series)
        .title("Datos Temporales")
        .x_label("Fecha")
        .y_label("Valor")
        .continuous_x_tick_formatter(|v| {
            chrono::DateTime::from_timestamp_millis(*v as i64)
                .unwrap_or_default()
                .format("%Y-%m-%d")
                .to_string()
        })
        .build();

    let config_sig = Signal::derive(move || config.clone());

    view! {
        <div style="height: 400px;">
            <LinePlot config=config_sig />
        </div>
    }
}
```

## Reference

### Trait Plotable

El trait fundamental que define qué tipos pueden ser utilizados como datos en los gráficos:

```rust
pub trait Plotable: PartialOrd + Clone + Send + Sync + 'static {
    fn to_f64(&self) -> f64;
    fn from_f64(f: f64) -> Self;
    fn to_plot_string(&self) -> String;
    fn data_type() -> DataType;
}
```

**Tipos implementados por defecto:**
- `f64`, `i32` - Tipos numéricos continuos
- `String`, `&'static str` - Tipos discretos
- `chrono::NaiveDateTime`, `chrono::DateTime<Utc>`, `chrono::DateTime<Local>`, `chrono::DateTime<FixedOffset>`, `chrono::NaiveDate` - Tipos de fecha (con feature `chrono`)

### Series<X, Y>

Estructura que contiene pares de valores (x, y) con una etiqueta opcional:

```rust
impl<X: Plotable, Y: Plotable> Series<X, Y> {
    pub fn new(x: Vec<X>, y: Vec<Y>) -> Self
    pub fn with_label(x: Vec<X>, y: Vec<Y>, label: impl Into<String>) -> Self
    pub fn push(&self, x: X, y: Y)
    pub fn append(&self, x: &mut Vec<X>, y: &mut Vec<Y>)
    pub fn replace_data(&self, x: Vec<X>, y: Vec<Y>)
    pub fn get_x(&self) -> Vec<X>
    pub fn get_y(&self) -> Vec<Y>
    pub fn get_label(&self) -> Option<String>
    pub fn set_label(&self, label: impl Into<String>)
    pub fn clear(&self)
}
```

### ColorPalette

Sistema de gestión de colores para los gráficos:

```rust
pub struct ColorPalette {
    pub background: RwSignal<Option<Hex>>,
    pub colors: RwSignal<Vec<Hex>>,
}

impl ColorPalette {
    pub fn get_color(&self, index: usize) -> Hex
    pub fn set_colors(&self, colors: Vec<Hex>)
    pub fn set_background(&self, background: Option<Hex>)
    pub fn set_from_presets(&self, name: &str) // "classic", "modern", "grayscale"
}
```

**Paletas preset:**
- `"classic"`: Azul, naranja, verde, rojo, púrpura, marrón
- `"modern"`: Azul oscuro, naranja quemado, verde azulado, rosa, amarillo, azul claro
- `"grayscale"`: Escala de grises de 4 niveles

### BarPlotConfig & BarPlot

Configuración y componente para gráficos de barras:

```rust
pub struct BarPlotConfigBuilder<X: Plotable, Y: Plotable> {
    pub fn new() -> Self
    pub fn palette(mut self, palette: impl Into<Signal<ColorPalette>>) -> Self
    pub fn with_series(mut self, series: impl Into<Signal<Vec<Series<X, Y>>>>) -> Self
    pub fn title(mut self, title: impl Into<Signal<String>>) -> Self
    pub fn x_label(mut self, x_label: impl Into<Signal<String>>) -> Self
    pub fn y_label(mut self, y_label: impl Into<Signal<String>>) -> Self
    pub fn margin_top/bottom/left/right(mut self, value: impl Into<Signal<f64>>) -> Self
    pub fn bar_relative_width(mut self, width: impl Into<Signal<f64>>) -> Self
    pub fn x_formatter/y_formatter(mut self, formatter: impl Fn(&T) -> String + ...) -> Self
    pub fn x_tick_formatter/y_tick_formatter(mut self, formatter: impl Fn(&T) -> String + ...) -> Self
    pub fn build(self) -> BarPlotConfig<X, Y>
}

#[component]
pub fn BarPlot<X: Plotable, Y: Plotable>(
    config: Signal<BarPlotConfig<X, Y>>,
    #[prop(optional, into)] width: Option<Signal<String>>,
    #[prop(optional, into)] height: Option<Signal<String>>,
) -> impl IntoView
```

### LinePlotConfig & LinePlot

Configuración y componente para gráficos de líneas:

```rust
pub struct LinePlotConfigBuilder<X: Plotable, Y: Plotable> {
    pub fn new() -> Self
    pub fn palette(mut self, palette: impl Into<Signal<ColorPalette>>) -> Self
    pub fn with_series(mut self, series: impl Into<Signal<Vec<Series<X, Y>>>>) -> Self
    pub fn title(mut self, title: impl Into<Signal<String>>) -> Self
    pub fn x_label(mut self, x_label: impl Into<Signal<String>>) -> Self
    pub fn y_label(mut self, y_label: impl Into<Signal<String>>) -> Self
    pub fn margin_top/bottom/left/right(mut self, value: impl Into<Signal<f64>>) -> Self
    pub fn x_formatter/y_formatter(mut self, formatter: impl Fn(&T) -> String + ...) -> Self
    pub fn x_tick_formatter/y_tick_formatter(mut self, formatter: impl Fn(&T) -> String + ...) -> Self
    pub fn continuous_x_tick_formatter(mut self, formatter: impl Fn(&f64) -> String + ...) -> Self
    pub fn build(self) -> LinePlotConfig<X, Y>
}

#[component]
pub fn LinePlot<X: Plotable, Y: Plotable>(
    config: Signal<LinePlotConfig<X, Y>>,
    #[prop(optional, into)] width: Option<Signal<String>>,
    #[prop(optional, into)] height: Option<Signal<String>>,
) -> impl IntoView
```

### Legend

Componente para mostrar leyendas de gráficos:

```rust
#[component]
pub fn Legend<S: LegendSource>(
    config: Signal<S>,
    #[prop(optional, into)] width: Option<Signal<String>>,
    #[prop(optional, into)] height: Option<Signal<String>>,
) -> impl IntoView
```

## Ejecución del Showcase

Para ejecutar la aplicación de demostración en modo desarrollo:

```bash
just watch
```

La aplicación estará disponible en `http://127.0.0.1:3000`


## informacion

- [Leptos](https://github.com/leptos-rs/leptos) 
- [cargo-leptos](https://github.com/akesson/cargo-leptos)
- [Axum](https://github.com/tokio-rs/axum)
- [Chrono](https://github.com/chronotope/chrono)

## Enlaces

- **Repositorio**: [https://github.com/ixpantia/vixual](https://github.com/ixpantia/vixual)
- **Documentación de Leptos**: [https://leptos.dev/](https://leptos.dev/)