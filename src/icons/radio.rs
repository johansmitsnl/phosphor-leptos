//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "communication", feature = "media", feature = "objects"))]
#[component]
pub fn Radio(
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
                <path d="M216,64H86.51L194.3,31.67a8,8,0,0,0-4.6-15.33l-160,48h0A8,8,0,0,0,24,72V192a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V80A16,16,0,0,0,216,64ZM104,176H64a8,8,0,0,1,0-16h40a8,8,0,0,1,0,16Zm0-32H64a8,8,0,0,1,0-16h40a8,8,0,0,1,0,16Zm0-32H64a8,8,0,0,1,0-16h40a8,8,0,0,1,0,16Zm64,56a32,32,0,1,1,32-32A32,32,0,0,1,168,168Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M216,72H32V192a8,8,0,0,0,8,8H216a8,8,0,0,0,8-8V80A8,8,0,0,0,216,72Zm-56,96a32,32,0,1,1,32-32A32,32,0,0,1,160,168Z"
        opacity="0.2"
    ></path>
    <path d="M104,168a8,8,0,0,1-8,8H64a8,8,0,0,1,0-16H96A8,8,0,0,1,104,168Zm-8-40H64a8,8,0,0,0,0,16H96a8,8,0,0,0,0-16Zm0-32H64a8,8,0,0,0,0,16H96a8,8,0,0,0,0-16ZM232,80V192a16,16,0,0,1-16,16H40a16,16,0,0,1-16-16V72a8,8,0,0,1,5.7-7.66l160-48a8,8,0,0,1,4.6,15.33L86.51,64H216A16,16,0,0,1,232,80ZM216,192V80H40V192H216Zm-16-56a40,40,0,1,1-40-40A40,40,0,0,1,200,136Zm-16,0a24,24,0,1,0-24,24A24,24,0,0,0,184,136Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M100,168a4,4,0,0,1-4,4H64a4,4,0,0,1,0-8H96A4,4,0,0,1,100,168Zm-4-36H64a4,4,0,0,0,0,8H96a4,4,0,0,0,0-8ZM228,80V192a12,12,0,0,1-12,12H40a12,12,0,0,1-12-12V72a4,4,0,0,1,2.85-3.81l160-48a4,4,0,0,1,2.3,7.66L59.25,68H216A12,12,0,0,1,228,80Zm-8,0a4,4,0,0,0-4-4H36V192a4,4,0,0,0,4,4H216a4,4,0,0,0,4-4Zm-24,56a36,36,0,1,1-36-36A36,36,0,0,1,196,136Zm-8,0a28,28,0,1,0-28,28A28,28,0,0,0,188,136ZM96,100H64a4,4,0,0,0,0,8H96a4,4,0,0,0,0-8Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M160,172a36,36,0,1,0-36-36A36,36,0,0,0,160,172Zm0-48a12,12,0,1,1-12,12A12,12,0,0,1,160,124Zm56-64H113.76l81.69-24.5a12,12,0,0,0-6.9-23l-160,48A12,12,0,0,0,20,72V192a20,20,0,0,0,20,20H216a20,20,0,0,0,20-20V80A20,20,0,0,0,216,60Zm-4,128H44V84H212ZM60,116a12,12,0,0,1,12-12H96a12,12,0,0,1,0,24H72A12,12,0,0,1,60,116Zm0,40a12,12,0,0,1,12-12H96a12,12,0,0,1,0,24H72A12,12,0,0,1,60,156Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M102,104a6,6,0,0,1-6,6H64a6,6,0,0,1,0-12H96A6,6,0,0,1,102,104Zm-6,26H64a6,6,0,0,0,0,12H96a6,6,0,0,0,0-12Zm0,32H64a6,6,0,0,0,0,12H96a6,6,0,0,0,0-12ZM230,80V192a14,14,0,0,1-14,14H40a14,14,0,0,1-14-14V72a6,6,0,0,1,4.28-5.75l160-48a6,6,0,0,1,3.44,11.5L72.88,66H216A14,14,0,0,1,230,80Zm-12,0a2,2,0,0,0-2-2H38V192a2,2,0,0,0,2,2H216a2,2,0,0,0,2-2Zm-20,56a38,38,0,1,1-38-38A38,38,0,0,1,198,136Zm-12,0a26,26,0,1,0-26,26A26,26,0,0,0,186,136Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M104,168a8,8,0,0,1-8,8H64a8,8,0,0,1,0-16H96A8,8,0,0,1,104,168Zm-8-40H64a8,8,0,0,0,0,16H96a8,8,0,0,0,0-16Zm0-32H64a8,8,0,0,0,0,16H96a8,8,0,0,0,0-16ZM232,80V192a16,16,0,0,1-16,16H40a16,16,0,0,1-16-16V72a8,8,0,0,1,5.7-7.66l160-48a8,8,0,0,1,4.6,15.33L86.51,64H216A16,16,0,0,1,232,80ZM216,192V80H40V192H216Zm-16-56a40,40,0,1,1-40-40A40,40,0,0,1,200,136Zm-16,0a24,24,0,1,0-24,24A24,24,0,0,0,184,136Z"></path>
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
