//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="design", feature ="office", feature ="editor"))]
#[component]
pub fn StackMinus(
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
                <path d="M230.91,124A8,8,0,0,1,228,134.91l-96,56a8,8,0,0,1-8.06,0l-96-56A8,8,0,0,1,36,121.09l92,53.65,92-53.65A8,8,0,0,1,230.91,124ZM28,86.91l96,56a8,8,0,0,0,8.06,0l96-56a8,8,0,0,0,0-13.82l-96-56a8,8,0,0,0-8.06,0l-96,56a8,8,0,0,0,0,13.82ZM232,192H184a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16Zm-92,23.76-12,7L36,169.09A8,8,0,0,0,28,182.91l96,56a8,8,0,0,0,8.06,0l16-9.33A8,8,0,1,0,140,215.76Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M224,80l-96,56L32,80l96-56Z" opacity="0.2"></path>
    <path d="M240,200a8,8,0,0,1-8,8H184a8,8,0,0,1,0-16h48A8,8,0,0,1,240,200Zm-20-78.91-92,53.65L36,121.09A8,8,0,0,0,28,134.91l96,56a8,8,0,0,0,8.06,0l96-56A8,8,0,1,0,220,121.09ZM24,80a8,8,0,0,1,4-6.91l96-56a8,8,0,0,1,8.06,0l96,56a8,8,0,0,1,0,13.82l-96,56a8,8,0,0,1-8.06,0l-96-56A8,8,0,0,1,24,80Zm23.88,0L128,126.74,208.12,80,128,33.26ZM140,215.76l-12,7L36,169.09A8,8,0,0,0,28,182.91l96,56a8,8,0,0,0,8.06,0l16-9.33A8,8,0,1,0,140,215.76Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M236,200a4,4,0,0,1-4,4H184a4,4,0,0,1,0-8h48A4,4,0,0,1,236,200Zm-14-75.46-94,54.83L34,124.54a4,4,0,0,0-4,6.92l96,56a4,4,0,0,0,4,0l96-56a4,4,0,1,0-4-6.92ZM28,80a4,4,0,0,1,2-3.46l96-56a4,4,0,0,1,4,0l96,56a4,4,0,0,1,0,6.92l-96,56a4,4,0,0,1-4,0l-96-56A4,4,0,0,1,28,80Zm11.94,0L128,131.37,216.06,80,128,28.63ZM142,219.21l-14,8.16L34,172.54a4,4,0,0,0-4,6.92l96,56a4,4,0,0,0,4,0l16-9.34a4,4,0,1,0-4-6.91Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M240,200a12,12,0,0,1-12,12H188a12,12,0,0,1,0-24h40A12,12,0,0,1,240,200ZM218,117.63,128,170.11,38.05,117.63A12,12,0,0,0,26,138.37l96,56a12,12,0,0,0,12.1,0l96-56A12,12,0,1,0,218,117.63ZM20,80a12,12,0,0,1,6-10.37l96-56a12.06,12.06,0,0,1,12.1,0l96,56a12,12,0,0,1,0,20.74l-96,56a12,12,0,0,1-12.1,0l-96-56A12,12,0,0,1,20,80Zm35.82,0L128,122.11,200.18,80,128,37.89ZM138,212.3,128,218.11,38.05,165.63A12,12,0,0,0,26,186.37l96,56a12,12,0,0,0,12.1,0l16-9.34A12,12,0,1,0,138,212.3Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M238,200a6,6,0,0,1-6,6H184a6,6,0,0,1,0-12h48A6,6,0,0,1,238,200Zm-17-77.18-93,54.23L35,122.82a6,6,0,0,0-6,10.36l96,56a6,6,0,0,0,6,0l96-56a6,6,0,0,0-6-10.36ZM26,80a6,6,0,0,1,3-5.18l96-56a6,6,0,0,1,6,0l96,56a6,6,0,0,1,0,10.36l-96,56a6,6,0,0,1-6,0l-96-56A6,6,0,0,1,26,80Zm17.91,0L128,129.05,212.09,80,128,31ZM141,217.48l-13,7.57L35,170.82a6,6,0,0,0-6,10.36l96,56a6,6,0,0,0,6,0l16-9.33a6,6,0,0,0-6-10.37Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M230.91,124A8,8,0,0,1,228,134.91l-96,56a8,8,0,0,1-8.06,0l-96-56A8,8,0,0,1,36,121.09l92,53.65,92-53.65A8,8,0,0,1,230.91,124ZM24,80a8,8,0,0,1,4-6.91l96-56a8,8,0,0,1,8.06,0l96,56a8,8,0,0,1,0,13.82l-96,56a8,8,0,0,1-8.06,0l-96-56A8,8,0,0,1,24,80Zm23.88,0L128,126.74,208.12,80,128,33.26ZM232,192H184a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16Zm-92,23.76-12,7L36,169.09A8,8,0,0,0,28,182.91l96,56a8,8,0,0,0,8.06,0l16-9.33A8,8,0,1,0,140,215.76Z"></path>
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