//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "objects", feature = "map", feature = "system"))]
#[component]
pub fn FlagBanner(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Fill => view! {
                <path d="M239.22,59.44l-45.63,95.82a3.54,3.54,0,0,1-.16.34l-34.21,71.84a8,8,0,1,1-14.44-6.88L173.62,160H40a8,8,0,0,1-5.66-13.66L76.69,104,34.34,61.66A8,8,0,0,1,40,48H232a8,8,0,0,1,7.22,11.44Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M232,56l-45.71,96H40l48-48L40,56Z" opacity="0.2"></path>
    <path d="M238.76,51.73A8,8,0,0,0,232,48H40a8,8,0,0,0-5.66,13.66L76.69,104,34.34,146.34A8,8,0,0,0,40,160H173.62l-28.84,60.56a8,8,0,1,0,14.44,6.88l80-168A8,8,0,0,0,238.76,51.73ZM181.23,144H59.31l34.35-34.34a8,8,0,0,0,0-11.32L59.31,64h160Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M235.38,53.86A4,4,0,0,0,232,52H40a4,4,0,0,0-2.83,6.83L82.34,104,37.17,149.17A4,4,0,0,0,40,156H180l-31.56,66.28a4,4,0,0,0,1.89,5.33A3.92,3.92,0,0,0,152,228a4,4,0,0,0,3.61-2.28l80-168A4,4,0,0,0,235.38,53.86ZM183.76,148H49.66l41.17-41.17a4,4,0,0,0,0-5.66L49.66,60h176Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M242.15,49.59A12,12,0,0,0,232,44H40a12,12,0,0,0-8.48,20.49L71,104,31.52,143.51A12,12,0,0,0,40,164H167.28l-26.11,54.84a12,12,0,1,0,21.66,10.32l80-168A12,12,0,0,0,242.15,49.59ZM178.71,140H69l27.52-27.52a12,12,0,0,0,0-17L69,68H213Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M237.07,52.8A6,6,0,0,0,232,50H40a6,6,0,0,0-4.24,10.24L79.51,104,35.76,147.76A6,6,0,0,0,40,158H176.78l-30.2,63.42a6,6,0,0,0,10.84,5.16l80-168A6,6,0,0,0,237.07,52.8ZM182.5,146h-128l37.75-37.76a6,6,0,0,0,0-8.48L54.49,62h168Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M238.76,51.73A8,8,0,0,0,232,48H40a8,8,0,0,0-5.66,13.66L76.69,104,34.34,146.34A8,8,0,0,0,40,160H173.62l-28.84,60.56a8,8,0,1,0,14.44,6.88l80-168A8,8,0,0,0,238.76,51.73ZM181.23,144H59.31l34.35-34.34a8,8,0,0,0,0-11.32L59.31,64h160Z"></path>
}.into_view()
        }
    });

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };
    let height = size.clone();

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=move || size.get()
            height=move || height.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
            id=move || id.get().unwrap_or(TextProp::from(""))
            class=move || class.get().unwrap_or(TextProp::from(""))
        >
            {body}
        </svg>
    }
}
