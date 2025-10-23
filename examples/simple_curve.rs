//! Simple Curve Plotting Example
//! 
//! This example demonstrates how to create a basic curve plot using gpui-plot.
//! It plots a sine wave function: y = sin(x) over the range [0, 2π].
//! 
//! The example is designed to compile on Linux, macOS, and Windows.

use gpui::{
    div, prelude::*, px, size, App, Application, Bounds, Entity, Hsla, Window,
    WindowBounds, WindowOptions,
};
use gpui_plot::figure::axes::AxesModel;
use gpui_plot::figure::figure::{FigureModel, FigureView};
use gpui_plot::figure::grid::GridModel;
use gpui_plot::geometry::{
    point2, AxesBounds, AxisRange, GeometryAxes, Line,
};
use parking_lot::RwLock;
use std::sync::Arc;

/// Main application view containing the curve plot
struct CurvePlotView {
    model: Arc<RwLock<FigureModel>>,
    axes_model: Arc<RwLock<AxesModel<f64, f64>>>,
    figure: Entity<FigureView>,
}

impl CurvePlotView {
    fn new(_window: &mut Window, cx: &mut App) -> Self {
        // Create the main figure model
        let model = FigureModel::new("Simple Curve Plot - y = sin(x)".to_string());
        let model = Arc::new(RwLock::new(model));

        // Set up axes bounds: x from 0 to 2π, y from -1.5 to 1.5 for the sine wave
        let x_range = AxisRange::new(0.0, 2.0 * std::f64::consts::PI);
        let y_range = AxisRange::new(-1.5, 1.5);
        let axes_bounds = AxesBounds::new(x_range, y_range);
        
        // Create a grid with 10 divisions on each axis
        let grid = GridModel::from_numbers(10, 8);
        let axes_model = Arc::new(RwLock::new(AxesModel::new(axes_bounds, grid)));

        // Create the figure view
        let figure = cx.new(|_| FigureView::new(model.clone()));

        Self {
            model,
            axes_model,
            figure,
        }
    }
}

impl Render for CurvePlotView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Defer rendering to trigger updates
        let id = cx.entity_id();
        cx.defer(move |app| app.notify(id));

        // Clear existing plots and create new curve
        let mut model = self.model.write();
        model.clear_plots();
        model.add_plot_with(|plot| {
            plot.add_axes_with(self.axes_model.clone(), |axes| {
                axes.clear_elements();
                
                // Create the sine curve
                let sine_curve = SineCurve::new();
                axes.plot(sine_curve);
            });
        });

        // Return the main UI layout
        div()
            .size_full()
            .flex_col()
            .bg(gpui::white())
            .text_color(gpui::black())
            .child(self.figure.clone())
    }
}

/// A simple sine wave curve generator
#[derive(Clone)]
struct SineCurve;

impl SineCurve {
    fn new() -> Self {
        Self
    }
}

impl GeometryAxes for SineCurve {
    type X = f64;
    type Y = f64;

    fn render_axes(&mut self, cx: &mut gpui_plot::figure::axes::AxesContext<Self::X, Self::Y>) {
        // Generate a sine wave curve
        let mut line = Line::new().color(Hsla::blue());
        
        // Sample the sine function from 0 to 2π with fine resolution
        let step = 0.05;
        let mut x = 0.0;
        let end = 2.0 * std::f64::consts::PI;
        
        while x <= end {
            let y = x.sin();
            line.add_point(point2(x, y));
            x += step;
        }
        
        // Render the line to the axes context
        line.render_axes(cx);
        
        // Optionally add a cosine curve for comparison
        let mut cosine_line = Line::new().color(Hsla::red());
        x = 0.0;
        while x <= end {
            let y = x.cos();
            cosine_line.add_point(point2(x, y));
            x += step;
        }
        cosine_line.render_axes(cx);
    }
}

fn main() {
    // Initialize the GPUI application
    Application::new().run(|cx: &mut App| {
        // Create a centered window
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
        
        // Open the main window with our curve plot
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| CurvePlotView::new(window, cx)),
        )
        .unwrap();
    });
}