//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="design"))]
#[component]
pub fn DiamondsFour(
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
                <path d="M82.34,69.66a8,8,0,0,1,0-11.32l40-40a8,8,0,0,1,11.32,0l40,40a8,8,0,0,1,0,11.32l-40,40a8,8,0,0,1-11.32,0Zm51.32,76.68a8,8,0,0,0-11.32,0l-40,40a8,8,0,0,0,0,11.32l40,40a8,8,0,0,0,11.32,0l40-40a8,8,0,0,0,0-11.32Zm104-24-40-40a8,8,0,0,0-11.32,0l-40,40a8,8,0,0,0,0,11.32l40,40a8,8,0,0,0,11.32,0l40-40A8,8,0,0,0,237.66,122.34Zm-128,0-40-40a8,8,0,0,0-11.32,0l-40,40a8,8,0,0,0,0,11.32l40,40a8,8,0,0,0,11.32,0l40-40A8,8,0,0,0,109.66,122.34Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M128,104,88,64l40-40,40,40ZM88,192l40,40,40-40-40-40ZM192,88l-40,40,40,40,40-40ZM64,88,24,128l40,40,40-40Z"
        opacity="0.2"
    ></path>
    <path d="M122.34,109.66a8,8,0,0,0,11.32,0l40-40a8,8,0,0,0,0-11.32l-40-40a8,8,0,0,0-11.32,0l-40,40a8,8,0,0,0,0,11.32ZM128,35.31,156.69,64,128,92.69,99.31,64Zm5.66,111a8,8,0,0,0-11.32,0l-40,40a8,8,0,0,0,0,11.32l40,40a8,8,0,0,0,11.32,0l40-40a8,8,0,0,0,0-11.32ZM128,220.69,99.31,192,128,163.31,156.69,192Zm109.66-98.35-40-40a8,8,0,0,0-11.32,0l-40,40a8,8,0,0,0,0,11.32l40,40a8,8,0,0,0,11.32,0l40-40A8,8,0,0,0,237.66,122.34ZM192,156.69,163.31,128,192,99.31,220.69,128Zm-82.34-34.35-40-40a8,8,0,0,0-11.32,0l-40,40a8,8,0,0,0,0,11.32l40,40a8,8,0,0,0,11.32,0l40-40A8,8,0,0,0,109.66,122.34ZM64,156.69,35.31,128,64,99.31,92.69,128Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M125.17,106.83a4,4,0,0,0,5.66,0l40-40a4,4,0,0,0,0-5.66l-40-40a4,4,0,0,0-5.66,0l-40,40a4,4,0,0,0,0,5.66ZM128,29.66,162.34,64,128,98.34,93.66,64Zm2.83,119.51a4,4,0,0,0-5.66,0l-40,40a4,4,0,0,0,0,5.66l40,40a4,4,0,0,0,5.66,0l40-40a4,4,0,0,0,0-5.66ZM128,226.34,93.66,192,128,157.66,162.34,192ZM234.83,125.17l-40-40a4,4,0,0,0-5.66,0l-40,40a4,4,0,0,0,0,5.66l40,40a4,4,0,0,0,5.66,0l40-40A4,4,0,0,0,234.83,125.17ZM192,162.34,157.66,128,192,93.66,226.34,128Zm-85.17-37.17-40-40a4,4,0,0,0-5.66,0l-40,40a4,4,0,0,0,0,5.66l40,40a4,4,0,0,0,5.66,0l40-40A4,4,0,0,0,106.83,125.17ZM64,162.34,29.66,128,64,93.66,98.34,128Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M119.51,108.49a12,12,0,0,0,17,0l38-38a12,12,0,0,0,0-17l-38-38a12,12,0,0,0-17,0l-38,38a12,12,0,0,0,0,17ZM128,41l21,21L128,83,107,62Zm8.49,106.54a12,12,0,0,0-17,0l-38,38a12,12,0,0,0,0,17l38,38a12,12,0,0,0,17,0l38-38a12,12,0,0,0,0-17ZM128,215l-21-21,21-21,21,21Zm-19.51-95.52-38-38a12,12,0,0,0-17,0l-38,38a12,12,0,0,0,0,17l38,38a12,12,0,0,0,17,0l38-38A12,12,0,0,0,108.49,119.51ZM62,149,41,128l21-21,21,21Zm178.49-29.52-38-38a12,12,0,0,0-17,0l-38,38a12,12,0,0,0,0,17l38,38a12,12,0,0,0,17,0l38-38A12,12,0,0,0,240.49,119.51ZM194,149l-21-21,21-21,21,21Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M123.76,108.24a6,6,0,0,0,8.48,0l40-40a6,6,0,0,0,0-8.48l-40-40a6,6,0,0,0-8.48,0l-40,40a6,6,0,0,0,0,8.48ZM128,32.49,159.51,64,128,95.51,96.49,64Zm4.24,115.27a6,6,0,0,0-8.48,0l-40,40a6,6,0,0,0,0,8.48l40,40a6,6,0,0,0,8.48,0l40-40a6,6,0,0,0,0-8.48ZM128,223.51,96.49,192,128,160.49,159.51,192Zm108.24-99.75-40-40a6,6,0,0,0-8.48,0l-40,40a6,6,0,0,0,0,8.48l40,40a6,6,0,0,0,8.48,0l40-40A6,6,0,0,0,236.24,123.76ZM192,159.51,160.49,128,192,96.49,223.51,128Zm-83.76-35.75-40-40a6,6,0,0,0-8.48,0l-40,40a6,6,0,0,0,0,8.48l40,40a6,6,0,0,0,8.48,0l40-40A6,6,0,0,0,108.24,123.76ZM64,159.51,32.49,128,64,96.49,95.51,128Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M122.34,109.66a8,8,0,0,0,11.32,0l40-40a8,8,0,0,0,0-11.32l-40-40a8,8,0,0,0-11.32,0l-40,40a8,8,0,0,0,0,11.32ZM128,35.31,156.69,64,128,92.69,99.31,64Zm5.66,111a8,8,0,0,0-11.32,0l-40,40a8,8,0,0,0,0,11.32l40,40a8,8,0,0,0,11.32,0l40-40a8,8,0,0,0,0-11.32ZM128,220.69,99.31,192,128,163.31,156.69,192Zm109.66-98.35-40-40a8,8,0,0,0-11.32,0l-40,40a8,8,0,0,0,0,11.32l40,40a8,8,0,0,0,11.32,0l40-40A8,8,0,0,0,237.66,122.34ZM192,156.69,163.31,128,192,99.31,220.69,128Zm-82.34-34.35-40-40a8,8,0,0,0-11.32,0l-40,40a8,8,0,0,0,0,11.32l40,40a8,8,0,0,0,11.32,0l40-40A8,8,0,0,0,109.66,122.34ZM64,156.69,35.31,128,64,99.31,92.69,128Z"></path>
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