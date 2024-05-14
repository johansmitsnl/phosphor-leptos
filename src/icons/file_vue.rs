//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="office", feature ="editor", feature ="development"))]
#[component]
pub fn FileVue(
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
                <path d="M213.66,82.34l-56-56A8,8,0,0,0,152,24h-96a16,16,0,0,0-16,16v76a4,4,0,0,0,4,4H212a4,4,0,0,0,4-4V88A8,8,0,0,0,213.66,82.34ZM152,88V44l44,44ZM87.36,155,67.47,210.69a8,8,0,0,1-15.08,0L32.5,155A8.21,8.21,0,0,1,37,144.55a8,8,0,0,1,10.46,4.76l12.47,34.9,12.47-34.9a8,8,0,0,1,10.46-4.76A8.22,8.22,0,0,1,87.36,155ZM184,160v12h15.73a8.19,8.19,0,0,1,8.26,7.47,8,8,0,0,1-8,8.53H184v12h23.73a8.18,8.18,0,0,1,8.26,7.47,8,8,0,0,1-8,8.53H176a8,8,0,0,1-8-8V152a8,8,0,0,1,8-8h31.74a8.18,8.18,0,0,1,8.26,7.47,8,8,0,0,1-8,8.53Zm-32-8v37.45c0,14.14-11.07,26.12-25.22,26.54A26,26,0,0,1,100,190V152.27a8.18,8.18,0,0,1,7.47-8.25,8,8,0,0,1,8.54,8v37.65A10.23,10.23,0,0,0,125.27,200,10,10,0,0,0,136,190V152.27a8.18,8.18,0,0,1,7.47-8.25A8,8,0,0,1,152,152Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M208,88H152V32Z" opacity="0.2"></path>
    <path d="M87.54,154.69l-20,56a8,8,0,0,1-15.07,0l-20-56a8,8,0,0,1,15.07-5.38L60,184.21l12.47-34.9a8,8,0,0,1,15.07,5.38ZM208,160a8,8,0,0,0,0-16H176a8,8,0,0,0-8,8v56a8,8,0,0,0,8,8h32a8,8,0,0,0,0-16H184V188h16a8,8,0,0,0,0-16H184V160Zm-64-16a8,8,0,0,0-8,8v38a10,10,0,0,1-20,0V152a8,8,0,0,0-16,0v38a26,26,0,0,0,52,0V152A8,8,0,0,0,144,144Zm72-56v24a8,8,0,0,1-16,0V96H152a8,8,0,0,1-8-8V40H56v72a8,8,0,0,1-16,0V40A16,16,0,0,1,56,24h96a8,8,0,0,1,5.66,2.34l56,56A8,8,0,0,1,216,88Zm-27.31-8L160,51.31V80Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M83.77,153.35l-20,56a4,4,0,0,1-7.54,0l-20-56a4,4,0,0,1,7.54-2.7L60,196.11l16.23-45.46a4,4,0,0,1,7.54,2.7ZM208,156a4,4,0,0,0,0-8H176a4,4,0,0,0-4,4v56a4,4,0,0,0,4,4h32a4,4,0,0,0,0-8H180V184h20a4,4,0,0,0,0-8H180V156Zm-64-8a4,4,0,0,0-4,4v38a14,14,0,0,1-28,0V152a4,4,0,0,0-8,0v38a22,22,0,0,0,44,0V152A4,4,0,0,0,144,148Zm68-60v24a4,4,0,0,1-8,0V92H152a4,4,0,0,1-4-4V36H56a4,4,0,0,0-4,4v72a4,4,0,0,1-8,0V40A12,12,0,0,1,56,28h96a4,4,0,0,1,2.83,1.17l56,56A4,4,0,0,1,212,88Zm-13.66-4L156,41.65V84Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M87.3,156l-20,56a12,12,0,0,1-22.6,0l-20-56A12,12,0,0,1,47.3,148L56,172.32,64.7,148A12,12,0,0,1,87.3,156ZM212,196H192v-4h12a12,12,0,0,0,0-24H192v-4h20a12,12,0,0,0,0-24H180a12,12,0,0,0-12,12v56a12,12,0,0,0,12,12h32a12,12,0,0,0,0-24Zm-68-56a12,12,0,0,0-12,12v38a6,6,0,0,1-12,0V152a12,12,0,0,0-24,0v38a30,30,0,0,0,60,0V152A12,12,0,0,0,144,140ZM36,108V40A20,20,0,0,1,56,20h96a12,12,0,0,1,8.49,3.52l56,56A12,12,0,0,1,220,88v20a12,12,0,0,1-24,0v-4H148a12,12,0,0,1-12-12V44H60v64a12,12,0,0,1-24,0ZM160,80h23L160,57Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M85.65,154l-20,56a6,6,0,0,1-11.3,0l-20-56a6,6,0,1,1,11.3-4L60,190.16,74.35,150a6,6,0,0,1,11.3,4ZM208,158a6,6,0,0,0,0-12H176a6,6,0,0,0-6,6v56a6,6,0,0,0,6,6h32a6,6,0,0,0,0-12H182V186h18a6,6,0,0,0,0-12H182V158Zm-64-12a6,6,0,0,0-6,6v38a12,12,0,0,1-24,0V152a6,6,0,0,0-12,0v38a24,24,0,0,0,48,0V152A6,6,0,0,0,144,146Zm70-58v24a6,6,0,0,1-12,0V94H152a6,6,0,0,1-6-6V38H56a2,2,0,0,0-2,2v72a6,6,0,0,1-12,0V40A14,14,0,0,1,56,26h96a6,6,0,0,1,4.25,1.76l56,56A6,6,0,0,1,214,88Zm-20.48-6L158,46.48V82Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M87.54,154.69l-20,56a8,8,0,0,1-15.07,0l-20-56a8,8,0,0,1,15.07-5.38L60,184.21l12.47-34.9a8,8,0,0,1,15.07,5.38ZM208,160a8,8,0,0,0,0-16H176a8,8,0,0,0-8,8v56a8,8,0,0,0,8,8h32a8,8,0,0,0,0-16H184V188h16a8,8,0,0,0,0-16H184V160Zm-64-16a8,8,0,0,0-8,8v38a10,10,0,0,1-20,0V152a8,8,0,0,0-16,0v38a26,26,0,0,0,52,0V152A8,8,0,0,0,144,144Zm72-56v24a8,8,0,0,1-16,0V96H152a8,8,0,0,1-8-8V40H56v72a8,8,0,0,1-16,0V40A16,16,0,0,1,56,24h96a8,8,0,0,1,5.66,2.34l56,56A8,8,0,0,1,216,88Zm-27.31-8L160,51.31V80Z"></path>
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