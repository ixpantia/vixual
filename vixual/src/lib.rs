use leptos::{either::Either, prelude::*};

pub mod legend;
pub mod palette;
pub mod plotable;
pub mod series;

#[cfg(feature = "bar_plot")]
pub mod bar_plot;

#[cfg(feature = "line_plot")]
pub mod line_plot;

/// Render the children only on the client. This is useful for plots
/// or visalizations that depend on data like a specific element's size.
#[component(transparent)]
pub(crate) fn OnClient<C>(children: TypedChildrenFn<C>) -> impl IntoView
where
    C: IntoView + 'static,
{
    let (is_mounted, set_is_mounted) = signal(false);
    Effect::new(move || set_is_mounted.set(true));
    let children = children.into_inner();

    move || match is_mounted.get() {
        true => Either::Left(children()),
        false => Either::Right(()),
    }
}
