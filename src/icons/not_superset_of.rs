//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="finance", feature ="development"))]
#[component]
pub fn NotSupersetOf(
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
                <path d="M164.09,98.9A24,24,0,0,1,144,136H131.63ZM224,48V208a16,16,0,0,1-16,16H48a16,16,0,0,1-16-16V48A16,16,0,0,1,48,32H208A16,16,0,0,1,224,48ZM189.27,58a8,8,0,0,0-11.29.75L162.42,76.51A39.82,39.82,0,0,0,144,72H80a8,8,0,0,0,0,16h64a23.87,23.87,0,0,1,7.36,1.16l-41,46.84H80a8,8,0,0,0,0,16H96.37L66,186.73a8,8,0,0,0,12,10.54L89.63,184H176a8,8,0,0,0,0-16H103.63l14-16H144a40,40,0,0,0,30.87-65.41L190,69.27A8,8,0,0,0,189.27,58Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M208,104a56,56,0,0,1-56,56H56V48h96A56,56,0,0,1,208,104Z" opacity="0.2"></path>
    <path d="M208,192H80.63l21.82-24H152A64,64,0,0,0,199.54,61.2l14.38-15.82a8,8,0,0,0-11.84-10.76L187.43,50.73A63.66,63.66,0,0,0,152,40H56a8,8,0,0,0,0,16h96a47.72,47.72,0,0,1,24.51,6.75L95.37,152H56a8,8,0,0,0,0,16H80.82L42.08,210.62a8,8,0,1,0,11.84,10.76L66.08,208H208a8,8,0,0,0,0-16ZM188.71,73.12A48,48,0,0,1,152,152H117Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M208,196H71.59l29.09-32H152A60,60,0,0,0,194.07,61.27L211,42.69A4,4,0,0,0,205,37.31L188,56a59.73,59.73,0,0,0-36-12H56a4,4,0,0,0,0,8h96a51.75,51.75,0,0,1,30.6,10L97.14,156H56a4,4,0,0,0,0,8H89.87L45,213.31A4,4,0,1,0,51,218.69L64.31,204H208a4,4,0,0,0,0-8ZM188.69,67.19A52,52,0,0,1,152,156H108Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M208,188H89.67l14.55-16H152A68,68,0,0,0,204.86,61.29l12-13.22a12,12,0,0,0-17.76-16.14l-12.4,13.63A67.62,67.62,0,0,0,152,36H56a12,12,0,0,0,0,24h96a43.74,43.74,0,0,1,18.06,3.89L93.6,148H56a12,12,0,0,0,0,24H71.78L39.12,207.93a12,12,0,1,0,17.76,16.14l11-12.07H208a12,12,0,0,0,0-24ZM188.44,79.36A44,44,0,0,1,152,148H126Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M208,194H76.11l25.45-28H152A62,62,0,0,0,196.81,61.22L212.44,44A6,6,0,1,0,203.56,36L187.73,53.38A61.61,61.61,0,0,0,152,42H56a6,6,0,0,0,0,12h96a49.67,49.67,0,0,1,27.59,8.33L96.25,154H56a6,6,0,0,0,0,12H85.35L43.56,212A6,6,0,0,0,52.44,220L65.2,206H208a6,6,0,0,0,0-12ZM188.73,70.12A50,50,0,0,1,152,154H112.47Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M208,192H80.63l21.82-24H152A64,64,0,0,0,199.54,61.2l14.38-15.82a8,8,0,0,0-11.84-10.76L187.43,50.73A63.66,63.66,0,0,0,152,40H56a8,8,0,0,0,0,16h96a47.72,47.72,0,0,1,24.51,6.75L95.37,152H56a8,8,0,0,0,0,16H80.82L42.08,210.62a8,8,0,1,0,11.84,10.76L66.08,208H208a8,8,0,0,0,0-16ZM188.71,73.12A48,48,0,0,1,152,152H117Z"></path>
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