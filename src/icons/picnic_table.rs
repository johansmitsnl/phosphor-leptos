//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="map", feature ="nature"))]
#[component]
pub fn PicnicTable(
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
                <path d="M146.85,96l14.54,32H94.61l14.54-32ZM232,56V200a16,16,0,0,1-16,16H40a16,16,0,0,1-16-16V56A16,16,0,0,1,40,40H216A16,16,0,0,1,232,56Zm-24,80a8,8,0,0,0-8-8H179L164.42,96H176a8,8,0,0,0,0-16H80a8,8,0,0,0,0,16H91.58L77,128H56a8,8,0,0,0,0,16H69.76l-13,28.69a8,8,0,1,0,14.56,6.62l16-35.31h81.34l16.05,35.31a8,8,0,0,0,14.56-6.62l-13-28.69H200A8,8,0,0,0,208,136Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M224,192H32L96,64h64Z" opacity="0.2"></path>
    <path d="M248,128H200.94l-28-56H192a8,8,0,0,0,0-16H64a8,8,0,0,0,0,16H83.06l-28,56H8a8,8,0,0,0,0,16H47.06L24.84,188.42a8,8,0,0,0,3.58,10.73A7.9,7.9,0,0,0,32,200a8,8,0,0,0,7.17-4.42L64.94,144H191.06l25.78,51.58A8,8,0,0,0,224,200a7.9,7.9,0,0,0,3.57-.85,8,8,0,0,0,3.58-10.73L208.94,144H248a8,8,0,0,0,0-16ZM72.94,128l28-56h54.12l28,56Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M248,132H198.47l-32-64H192a4,4,0,0,0,0-8H64a4,4,0,0,0,0,8H89.53l-32,64H8a4,4,0,0,0,0,8H53.53L28.42,190.21a4,4,0,0,0,1.79,5.37A4.12,4.12,0,0,0,32,196a4,4,0,0,0,3.58-2.21L62.47,140H193.53l26.89,53.79A4,4,0,0,0,224,196a4.12,4.12,0,0,0,1.79-.42,4,4,0,0,0,1.79-5.37L202.47,140H248a4,4,0,0,0,0-8ZM66.47,132l32-64h59.06l32,64Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M244,124H203.42l-24-48H192a12,12,0,0,0,0-24H64a12,12,0,0,0,0,24H76.58l-24,48H12a12,12,0,0,0,0,24H40.58L21.27,186.63a12,12,0,1,0,21.46,10.73L67.42,148H188.58l24.69,49.36a12,12,0,1,0,21.46-10.73L215.42,148H244a12,12,0,0,0,0-24ZM79.42,124l24-48h49.16l24,48Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M248,130H199.71l-30-60H192a6,6,0,0,0,0-12H64a6,6,0,0,0,0,12H86.29l-30,60H8a6,6,0,0,0,0,12H50.29L26.63,189.32a6,6,0,0,0,10.74,5.36L63.71,142H192.29l26.34,52.68a6,6,0,1,0,10.74-5.36L205.71,142H248a6,6,0,0,0,0-12ZM69.71,130l30-60h56.58l30,60Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M248,128H200.94l-28-56H192a8,8,0,0,0,0-16H64a8,8,0,0,0,0,16H83.06l-28,56H8a8,8,0,0,0,0,16H47.06L24.84,188.42a8,8,0,0,0,3.58,10.73A7.9,7.9,0,0,0,32,200a8,8,0,0,0,7.17-4.42L64.94,144H191.06l25.78,51.58A8,8,0,0,0,224,200a7.9,7.9,0,0,0,3.57-.85,8,8,0,0,0,3.58-10.73L208.94,144H248a8,8,0,0,0,0-16ZM72.94,128l28-56h54.12l28,56Z"></path>
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