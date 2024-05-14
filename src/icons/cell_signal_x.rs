//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "system"))]
#[component]
pub fn CellSignalX(
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
                <path d="M213.66,194.34a8,8,0,0,1-11.32,11.32L184,187.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L172.69,176l-18.35-18.34a8,8,0,0,1,11.32-11.32L184,164.69l18.34-18.35a8,8,0,0,1,11.32,11.32L195.31,176ZM157.41,120.1a32,32,0,0,1,23.92,8.05,4,4,0,0,0,5.34,0,31.88,31.88,0,0,1,17.77-8,4,4,0,0,0,3.56-4V40.46a16.41,16.41,0,0,0-9.18-14.93,16,16,0,0,0-18.14,3.16l-160,160a16,16,0,0,0-3.17,18.13A16.4,16.4,0,0,0,32.46,216h93.6a4,4,0,0,0,3.78-5.3,32,32,0,0,1,6.31-32,4,4,0,0,0,0-5.34,32,32,0,0,1,21.26-53.23Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M200,40V160l-45.66,45.66a8,8,0,0,1-5.65,2.34H32a8,8,0,0,1-5.66-13.66l160-160A8,8,0,0,1,200,40Z"
        opacity="0.2"
    ></path>
    <path d="M213.66,194.34a8,8,0,0,1-11.32,11.32L184,187.31l-18.35,18.35a8,8,0,0,1-11.31-11.32L172.68,176l-18.34-18.35a8,8,0,0,1,11.31-11.31L184,164.68l18.34-18.34a8,8,0,0,1,11.32,11.31L195.31,176ZM120,200H32L192,40v72a8,8,0,0,0,16,0V40a16,16,0,0,0-27.31-11.32l-160,160A16,16,0,0,0,32,216h88a8,8,0,0,0,0-16Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M210.83,197.17a4,4,0,0,1-5.66,5.66L184,181.66l-21.17,21.17a4,4,0,0,1-5.66-5.66L178.34,176l-21.17-21.17a4,4,0,0,1,5.66-5.66L184,170.34l21.17-21.17a4,4,0,0,1,5.66,5.66L189.66,176ZM160,116a4,4,0,0,0,4-4V72a4,4,0,0,0-8,0v40A4,4,0,0,0,160,116Zm40,0a4,4,0,0,0,4-4V32a4,4,0,0,0-8,0v80A4,4,0,0,0,200,116Zm-80-8a4,4,0,0,0-4,4v88a4,4,0,0,0,8,0V112A4,4,0,0,0,120,108ZM80,148a4,4,0,0,0-4,4v48a4,4,0,0,0,8,0V152A4,4,0,0,0,80,148ZM40,188a4,4,0,0,0-4,4v8a4,4,0,0,0,8,0v-8A4,4,0,0,0,40,188Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M216.49,191.51a12,12,0,0,1-17,17L184,193l-15.51,15.52a12,12,0,0,1-17-17L167,176l-15.52-15.51a12,12,0,0,1,17-17L184,159l15.51-15.52a12,12,0,0,1,17,17L201,176ZM160,120a12,12,0,0,0,12-12V72a12,12,0,0,0-24,0v36A12,12,0,0,0,160,120Zm40,0a12,12,0,0,0,12-12V32a12,12,0,0,0-24,0v76A12,12,0,0,0,200,120Zm-80-20a12,12,0,0,0-12,12v88a12,12,0,0,0,24,0V112A12,12,0,0,0,120,100ZM80,140a12,12,0,0,0-12,12v48a12,12,0,0,0,24,0V152A12,12,0,0,0,80,140ZM40,180a12,12,0,0,0-12,12v8a12,12,0,0,0,24,0v-8A12,12,0,0,0,40,180Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M212.24,195.76a6,6,0,1,1-8.48,8.48L184,184.48l-19.76,19.76a6,6,0,0,1-8.48-8.48L175.52,176l-19.76-19.76a6,6,0,0,1,8.48-8.48L184,167.52l19.76-19.76a6,6,0,0,1,8.48,8.48L192.48,176ZM160,118a6,6,0,0,0,6-6V72a6,6,0,0,0-12,0v40A6,6,0,0,0,160,118Zm40,0a6,6,0,0,0,6-6V32a6,6,0,0,0-12,0v80A6,6,0,0,0,200,118Zm-80-12a6,6,0,0,0-6,6v88a6,6,0,0,0,12,0V112A6,6,0,0,0,120,106ZM80,146a6,6,0,0,0-6,6v48a6,6,0,0,0,12,0V152A6,6,0,0,0,80,146ZM40,186a6,6,0,0,0-6,6v8a6,6,0,0,0,12,0v-8A6,6,0,0,0,40,186Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M213.66,194.34a8,8,0,0,1-11.32,11.32L184,187.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L172.69,176l-18.35-18.34a8,8,0,0,1,11.32-11.32L184,164.69l18.34-18.35a8,8,0,0,1,11.32,11.32L195.31,176ZM160,120a8,8,0,0,0,8-8V72a8,8,0,0,0-16,0v40A8,8,0,0,0,160,120Zm40,0a8,8,0,0,0,8-8V32a8,8,0,0,0-16,0v80A8,8,0,0,0,200,120Zm-80-16a8,8,0,0,0-8,8v88a8,8,0,0,0,16,0V112A8,8,0,0,0,120,104ZM80,144a8,8,0,0,0-8,8v48a8,8,0,0,0,16,0V152A8,8,0,0,0,80,144ZM40,184a8,8,0,0,0-8,8v8a8,8,0,0,0,16,0v-8A8,8,0,0,0,40,184Z"></path>
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
