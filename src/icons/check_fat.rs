//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="system"))]
#[component]
pub fn CheckFat(
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
                <path d="M243.31,90.91l-128.4,128.4a16,16,0,0,1-22.62,0l-71.62-72a16,16,0,0,1,0-22.61l20-20a16,16,0,0,1,22.58,0L104,144.22l96.76-95.57a16,16,0,0,1,22.59,0l19.95,19.54A16,16,0,0,1,243.31,90.91Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M237.66,85.26l-128.4,128.4a8,8,0,0,1-11.32,0l-71.6-72a8,8,0,0,1,0-11.31l24-24a8,8,0,0,1,11.32,0L104,147.43l98.34-97.09a8,8,0,0,1,11.32,0l24,23.6A8,8,0,0,1,237.66,85.26Z"
        opacity="0.2"
    ></path>
    <path d="M243.28,68.24l-24-23.56a16,16,0,0,0-22.59,0L104,136.23l-36.69-35.6a16,16,0,0,0-22.58.05l-24,24a16,16,0,0,0,0,22.61l71.62,72a16,16,0,0,0,22.63,0L243.33,90.91A16,16,0,0,0,243.28,68.24ZM103.62,208,32,136l24-24a.6.6,0,0,1,.08.08l42.35,41.09a8,8,0,0,0,11.19,0L208.06,56,232,79.6Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M240.47,71.09l-24-23.58a12,12,0,0,0-17,0L104,141.83l-39.5-38.32a12,12,0,0,0-17,0l-24,24a12,12,0,0,0,0,17l71.61,72a12,12,0,0,0,17,0L240.49,88.08A12,12,0,0,0,240.47,71.09Zm-5.63,11.34L106.43,210.83a4,4,0,0,1-5.65,0l-71.61-72a4,4,0,0,1,0-5.66l24-24A3.94,3.94,0,0,1,56,108a4.11,4.11,0,0,1,2.89,1.21l42.35,41.08a4,4,0,0,0,5.59,0l98.37-97.1a4,4,0,0,1,5.68,0l24,23.58A4,4,0,0,1,234.84,82.43Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M246.15,65.46l-.07-.08L222.15,41.85a20,20,0,0,0-28.23-.05l-90,88.83L70.06,97.78a20,20,0,0,0-28.21.08l-24,24a20,20,0,0,0,0,28.26l71.62,72a20,20,0,0,0,28.29,0L246.15,93.74A20,20,0,0,0,246.15,65.46ZM103.61,202.33,37.64,136,56,117.61,95.65,156a12,12,0,0,0,16.78-.08L208,61.61l18.32,18Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M241.87,69.66l-24-23.56a14,14,0,0,0-19.77,0L104,139,65.9,102.1a14,14,0,0,0-19.8,0l-24,24a14,14,0,0,0,0,19.79l71.62,72a14,14,0,0,0,19.8,0L241.91,89.5A14,14,0,0,0,241.87,69.66ZM233.42,81,105,209.41a2,2,0,0,1-2.81,0l-71.62-72a2,2,0,0,1,0-2.82l24-24A2,2,0,0,1,56,110a2.12,2.12,0,0,1,1.5.64l42.35,41.08a6,6,0,0,0,8.39,0l98.37-97.11a2,2,0,0,1,2.87,0l24,23.56A2,2,0,0,1,233.42,81Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M243.28,68.24l-24-23.56a16,16,0,0,0-22.59,0L104,136.23l-36.69-35.6a16,16,0,0,0-22.58.05l-24,24a16,16,0,0,0,0,22.61l71.62,72a16,16,0,0,0,22.63,0L243.33,90.91A16,16,0,0,0,243.28,68.24ZM103.62,208,32,136l24-24a.6.6,0,0,1,.08.08l42.35,41.09a8,8,0,0,0,11.19,0L208.06,56,232,79.6Z"></path>
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