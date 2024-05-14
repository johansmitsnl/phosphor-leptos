//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="brand", feature ="communication"))]
#[component]
pub fn LinktreeLogo(
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
                <path d="M200,24H56A16,16,0,0,0,40,40V216a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V40A16,16,0,0,0,200,24ZM136,200a8,8,0,0,1-16,0V160a8,8,0,0,1,16,0Zm48-80H147.31l26.35,26.34a8,8,0,0,1-11.32,11.32L128,123.31,93.66,157.66a8,8,0,0,1-11.32-11.32L108.69,120H72a8,8,0,0,1,0-16h36.69L82.34,77.66A8,8,0,0,1,93.66,66.34L120,92.69V56a8,8,0,0,1,16,0V92.69l26.34-26.35a8,8,0,0,1,11.32,11.32L147.31,104H184a8,8,0,0,1,0,16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M208,104a80,80,0,1,1-80-80A80,80,0,0,1,208,104Z" opacity="0.2"></path>
    <path d="M136,160v72a8,8,0,0,1-16,0V160a8,8,0,0,1,16,0Zm72-64H147.31l42.35-42.34a8,8,0,0,0-11.32-11.32L136,84.69V24a8,8,0,0,0-16,0V84.69L77.66,42.34A8,8,0,0,0,66.34,53.66L108.69,96H48a8,8,0,0,0,0,16h60.69L66.34,154.34a8,8,0,0,0,11.32,11.32L128,115.31l50.34,50.35a8,8,0,0,0,11.32-11.32L147.31,112H208a8,8,0,0,0,0-16Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M132,160v72a4,4,0,0,1-8,0V160a4,4,0,0,1,8,0Zm76-60H137.66l49.17-49.17a4,4,0,1,0-5.66-5.66L132,94.34V24a4,4,0,0,0-8,0V94.34L74.83,45.17a4,4,0,0,0-5.66,5.66L118.34,100H48a4,4,0,0,0,0,8h70.34L69.17,157.17a4,4,0,0,0,5.66,5.66L128,109.66l53.17,53.17a4,4,0,0,0,5.66-5.66L137.66,108H208a4,4,0,0,0,0-8Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M140,164v68a12,12,0,0,1-24,0V164a12,12,0,0,1,24,0Zm68-72H157l35.52-35.51a12,12,0,0,0-17-17L140,75V24a12,12,0,0,0-24,0V75L80.49,39.51a12,12,0,0,0-17,17L99,92H48a12,12,0,0,0,0,24H99L63.51,151.51a12,12,0,0,0,17,17L128,121l47.51,47.52a12,12,0,0,0,17-17L157,116h51a12,12,0,0,0,0-24Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M134,160v72a6,6,0,0,1-12,0V160a6,6,0,0,1,12,0Zm74-62H142.48l45.76-45.76a6,6,0,0,0-8.48-8.48L134,89.52V24a6,6,0,0,0-12,0V89.52L76.24,43.76a6,6,0,0,0-8.48,8.48L113.52,98H48a6,6,0,0,0,0,12h65.52L67.76,155.76a6,6,0,1,0,8.48,8.48L128,112.48l51.76,51.76a6,6,0,0,0,8.48-8.48L142.48,110H208a6,6,0,0,0,0-12Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M136,160v72a8,8,0,0,1-16,0V160a8,8,0,0,1,16,0Zm72-64H147.31l42.35-42.34a8,8,0,0,0-11.32-11.32L136,84.69V24a8,8,0,0,0-16,0V84.69L77.66,42.34A8,8,0,0,0,66.34,53.66L108.69,96H48a8,8,0,0,0,0,16h60.69L66.34,154.34a8,8,0,0,0,11.32,11.32L128,115.31l50.34,50.35a8,8,0,0,0,11.32-11.32L147.31,112H208a8,8,0,0,0,0-16Z"></path>
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