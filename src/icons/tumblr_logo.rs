//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="brand", feature ="communication"))]
#[component]
pub fn TumblrLogo(
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
                <path d="M152,120v48a8,8,0,0,0,8,8h32a8,8,0,0,1,8,8v48a8,8,0,0,1-8,8H152a64.07,64.07,0,0,1-64-64V120H64a8,8,0,0,1-8-8V72a8,8,0,0,1,8-8,40,40,0,0,0,40-40,8,8,0,0,1,8-8h32a8,8,0,0,1,8,8V64h40a8,8,0,0,1,8,8v40a8,8,0,0,1-8,8Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M144,112v56a16,16,0,0,0,16,16h32v48H152a56,56,0,0,1-56-56V112H64V72h0a48,48,0,0,0,48-48h32V72h48v40Z"
        opacity="0.2"
    ></path>
    <path d="M192,120a8,8,0,0,0,8-8V72a8,8,0,0,0-8-8H152V24a8,8,0,0,0-8-8H112a8,8,0,0,0-8,8A40,40,0,0,1,64,64a8,8,0,0,0-8,8v40a8,8,0,0,0,8,8H88v56a64.07,64.07,0,0,0,64,64h40a8,8,0,0,0,8-8V184a8,8,0,0,0-8-8H160a8,8,0,0,1-8-8V120Zm-32,72h24v32H152a48.05,48.05,0,0,1-48-48V112a8,8,0,0,0-8-8H72V79.43A56.13,56.13,0,0,0,119.43,32H136V72a8,8,0,0,0,8,8h40v24H144a8,8,0,0,0-8,8v56A24,24,0,0,0,160,192Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M192,116a4,4,0,0,0,4-4V72a4,4,0,0,0-4-4H148V24a4,4,0,0,0-4-4H112a4,4,0,0,0-4,4A44.05,44.05,0,0,1,64,68a4,4,0,0,0-4,4v40a4,4,0,0,0,4,4H92v60a60.07,60.07,0,0,0,60,60h40a4,4,0,0,0,4-4V184a4,4,0,0,0-4-4H160a12,12,0,0,1-12-12V116Zm-32,72h28v40H152a52.06,52.06,0,0,1-52-52V112a4,4,0,0,0-4-4H68V75.85A52.09,52.09,0,0,0,115.85,28H140V72a4,4,0,0,0,4,4h44v32H144a4,4,0,0,0-4,4v56A20,20,0,0,0,160,188Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M192,124a12,12,0,0,0,12-12V72a12,12,0,0,0-12-12H156V24a12,12,0,0,0-12-12H112a12,12,0,0,0-12,12A36,36,0,0,1,64,60,12,12,0,0,0,52,72v40a12,12,0,0,0,12,12H84v52a68.07,68.07,0,0,0,68,68h40a12,12,0,0,0,12-12V184a12,12,0,0,0-12-12H160a4,4,0,0,1-4-4V124Zm-32,72h20v24H152a44.05,44.05,0,0,1-44-44V112a12,12,0,0,0-12-12H76V82.79A60.18,60.18,0,0,0,122.79,36H132V72a12,12,0,0,0,12,12h36v16H144a12,12,0,0,0-12,12v56A28,28,0,0,0,160,196Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M192,118a6,6,0,0,0,6-6V72a6,6,0,0,0-6-6H150V24a6,6,0,0,0-6-6H112a6,6,0,0,0-6,6A42,42,0,0,1,64,66a6,6,0,0,0-6,6v40a6,6,0,0,0,6,6H90v58a62.07,62.07,0,0,0,62,62h40a6,6,0,0,0,6-6V184a6,6,0,0,0-6-6H160a10,10,0,0,1-10-10V118Zm-32,72h26v36H152a50.06,50.06,0,0,1-50-50V112a6,6,0,0,0-6-6H70V77.67A54.12,54.12,0,0,0,117.67,30H138V72a6,6,0,0,0,6,6h42v28H144a6,6,0,0,0-6,6v56A22,22,0,0,0,160,190Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M192,120a8,8,0,0,0,8-8V72a8,8,0,0,0-8-8H152V24a8,8,0,0,0-8-8H112a8,8,0,0,0-8,8A40,40,0,0,1,64,64a8,8,0,0,0-8,8v40a8,8,0,0,0,8,8H88v56a64.07,64.07,0,0,0,64,64h40a8,8,0,0,0,8-8V184a8,8,0,0,0-8-8H160a8,8,0,0,1-8-8V120Zm-32,72h24v32H152a48.05,48.05,0,0,1-48-48V112a8,8,0,0,0-8-8H72V79.43A56.13,56.13,0,0,0,119.43,32H136V72a8,8,0,0,0,8,8h40v24H144a8,8,0,0,0-8,8v56A24,24,0,0,0,160,192Z"></path>
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