//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="design"))]
#[component]
pub fn CirclesFour(
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
                <path d="M120,80A40,40,0,1,1,80,40,40,40,0,0,1,120,80Zm56,40a40,40,0,1,0-40-40A40,40,0,0,0,176,120ZM80,136a40,40,0,1,0,40,40A40,40,0,0,0,80,136Zm96,0a40,40,0,1,0,40,40A40,40,0,0,0,176,136Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M112,80A32,32,0,1,1,80,48,32,32,0,0,1,112,80Zm64,32a32,32,0,1,0-32-32A32,32,0,0,0,176,112ZM80,144a32,32,0,1,0,32,32A32,32,0,0,0,80,144Zm96,0a32,32,0,1,0,32,32A32,32,0,0,0,176,144Z"
        opacity="0.2"
    ></path>
    <path d="M80,40a40,40,0,1,0,40,40A40,40,0,0,0,80,40Zm0,64a24,24,0,1,1,24-24A24,24,0,0,1,80,104Zm96,16a40,40,0,1,0-40-40A40,40,0,0,0,176,120Zm0-64a24,24,0,1,1-24,24A24,24,0,0,1,176,56ZM80,136a40,40,0,1,0,40,40A40,40,0,0,0,80,136Zm0,64a24,24,0,1,1,24-24A24,24,0,0,1,80,200Zm96-64a40,40,0,1,0,40,40A40,40,0,0,0,176,136Zm0,64a24,24,0,1,1,24-24A24,24,0,0,1,176,200Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M80,44a36,36,0,1,0,36,36A36,36,0,0,0,80,44Zm0,64a28,28,0,1,1,28-28A28,28,0,0,1,80,108Zm96,8a36,36,0,1,0-36-36A36,36,0,0,0,176,116Zm0-64a28,28,0,1,1-28,28A28,28,0,0,1,176,52ZM80,140a36,36,0,1,0,36,36A36,36,0,0,0,80,140Zm0,64a28,28,0,1,1,28-28A28,28,0,0,1,80,204Zm96-64a36,36,0,1,0,36,36A36,36,0,0,0,176,140Zm0,64a28,28,0,1,1,28-28A28,28,0,0,1,176,204Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M78,36a42,42,0,1,0,42,42A42,42,0,0,0,78,36Zm0,60A18,18,0,1,1,96,78,18,18,0,0,1,78,96Zm100,24a42,42,0,1,0-42-42A42,42,0,0,0,178,120Zm0-60a18,18,0,1,1-18,18A18,18,0,0,1,178,60ZM78,136a42,42,0,1,0,42,42A42,42,0,0,0,78,136Zm0,60a18,18,0,1,1,18-18A18,18,0,0,1,78,196Zm100-60a42,42,0,1,0,42,42A42,42,0,0,0,178,136Zm0,60a18,18,0,1,1,18-18A18,18,0,0,1,178,196Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M80,42a38,38,0,1,0,38,38A38,38,0,0,0,80,42Zm0,64a26,26,0,1,1,26-26A26,26,0,0,1,80,106Zm96,12a38,38,0,1,0-38-38A38,38,0,0,0,176,118Zm0-64a26,26,0,1,1-26,26A26,26,0,0,1,176,54ZM80,138a38,38,0,1,0,38,38A38,38,0,0,0,80,138Zm0,64a26,26,0,1,1,26-26A26,26,0,0,1,80,202Zm96-64a38,38,0,1,0,38,38A38,38,0,0,0,176,138Zm0,64a26,26,0,1,1,26-26A26,26,0,0,1,176,202Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M80,40a40,40,0,1,0,40,40A40,40,0,0,0,80,40Zm0,64a24,24,0,1,1,24-24A24,24,0,0,1,80,104Zm96,16a40,40,0,1,0-40-40A40,40,0,0,0,176,120Zm0-64a24,24,0,1,1-24,24A24,24,0,0,1,176,56ZM80,136a40,40,0,1,0,40,40A40,40,0,0,0,80,136Zm0,64a24,24,0,1,1,24-24A24,24,0,0,1,80,200Zm96-64a40,40,0,1,0,40,40A40,40,0,0,0,176,136Zm0,64a24,24,0,1,1,24-24A24,24,0,0,1,176,200Z"></path>
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