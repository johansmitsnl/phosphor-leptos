//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="media"))]
#[component]
pub fn MusicNotes(
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
                <path d="M212.92,17.71a7.89,7.89,0,0,0-6.86-1.46l-128,32A8,8,0,0,0,72,56V166.1A36,36,0,1,0,88,196V102.25l112-28V134.1A36,36,0,1,0,216,164V24A8,8,0,0,0,212.92,17.71Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,164a28,28,0,1,1-28-28A28,28,0,0,1,208,164ZM52,168a28,28,0,1,0,28,28A28,28,0,0,0,52,168Z"
        opacity="0.2"
    ></path>
    <path d="M212.92,17.69a8,8,0,0,0-6.86-1.45l-128,32A8,8,0,0,0,72,56V166.08A36,36,0,1,0,88,196V110.25l112-28v51.83A36,36,0,1,0,216,164V24A8,8,0,0,0,212.92,17.69ZM52,216a20,20,0,1,1,20-20A20,20,0,0,1,52,216ZM88,93.75V62.25l112-28v31.5ZM180,184a20,20,0,1,1,20-20A20,20,0,0,1,180,184Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M210.46,20.85a4,4,0,0,0-3.43-.73l-128,32A4,4,0,0,0,76,56V174.87A32,32,0,1,0,84,196V107.12l120-30v65.75A32,32,0,1,0,212,164V24A4,4,0,0,0,210.46,20.85ZM52,220a24,24,0,1,1,24-24A24,24,0,0,1,52,220Zm128-32a24,24,0,1,1,24-24A24,24,0,0,1,180,188ZM84,98.88V59.12l120-30V68.88Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M215.38,14.54a12,12,0,0,0-10.29-2.18l-128,32A12,12,0,0,0,68,56V159.35A40,40,0,1,0,92,196V113.37l104-26v40A40,40,0,1,0,220,164V24A12,12,0,0,0,215.38,14.54ZM52,212a16,16,0,1,1,16-16A16,16,0,0,1,52,212ZM92,88.63V65.37l104-26V62.63ZM180,180a16,16,0,1,1,16-16A16,16,0,0,1,180,180Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M211.69,19.27a6,6,0,0,0-5.15-1.09l-128,32A6,6,0,0,0,74,56V170.11A34,34,0,1,0,86,196V108.68l116-29v58.43A34,34,0,1,0,214,164V24A6,6,0,0,0,211.69,19.27ZM52,218a22,22,0,1,1,22-22A22,22,0,0,1,52,218ZM86,96.32V60.68l116-29V67.32ZM180,186a22,22,0,1,1,22-22A22,22,0,0,1,180,186Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M212.92,17.69a8,8,0,0,0-6.86-1.45l-128,32A8,8,0,0,0,72,56V166.08A36,36,0,1,0,88,196V110.25l112-28v51.83A36,36,0,1,0,216,164V24A8,8,0,0,0,212.92,17.69ZM52,216a20,20,0,1,1,20-20A20,20,0,0,1,52,216ZM88,93.75V62.25l112-28v31.5ZM180,184a20,20,0,1,1,20-20A20,20,0,0,1,180,184Z"></path>
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