//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="arrows"))]
#[component]
pub fn ArrowElbowUpRight(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Fill => view! {
                <path d="M221.66,85.66l-48,48A8,8,0,0,1,160,128V88H80V224a8,8,0,0,1-16,0V80a8,8,0,0,1,8-8h88V32a8,8,0,0,1,13.66-5.66l48,48A8,8,0,0,1,221.66,85.66Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M216,80l-48,48V32Z" opacity="0.2"></path>
    <path d="M221.66,74.34l-48-48A8,8,0,0,0,160,32V72H72a8,8,0,0,0-8,8V224a8,8,0,0,0,16,0V88h80v40a8,8,0,0,0,13.66,5.66l48-48A8,8,0,0,0,221.66,74.34ZM176,108.69V51.31L204.69,80Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M218.83,82.83l-48,48a4,4,0,0,1-5.66-5.66L206.34,84H76V224a4,4,0,0,1-8,0V80a4,4,0,0,1,4-4H206.34L165.17,34.83a4,4,0,0,1,5.66-5.66l48,48A4,4,0,0,1,218.83,82.83Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M224.49,88.49l-48,48a12,12,0,0,1-17-17L187,92H84V224a12,12,0,0,1-24,0V80A12,12,0,0,1,72,68H187L159.51,40.49a12,12,0,1,1,17-17l48,48A12,12,0,0,1,224.49,88.49Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M220.24,84.24l-48,48a6,6,0,0,1-8.48-8.48L201.51,86H78V224a6,6,0,0,1-12,0V80a6,6,0,0,1,6-6H201.51L163.76,36.24a6,6,0,0,1,8.48-8.48l48,48A6,6,0,0,1,220.24,84.24Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M221.66,85.66l-48,48a8,8,0,0,1-11.32-11.32L196.69,88H80V224a8,8,0,0,1-16,0V80a8,8,0,0,1,8-8H196.69L162.34,37.66a8,8,0,0,1,11.32-11.32l48,48A8,8,0,0,1,221.66,85.66Z"></path>
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