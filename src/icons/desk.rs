//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="commerce", feature ="objects", feature ="office"))]
#[component]
pub fn Desk(
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
                <path d="M248,64H8A8,8,0,0,0,8,80h8V192a8,8,0,0,0,16,0V144H224v48a8,8,0,0,0,16,0V80h8a8,8,0,0,0,0-16ZM80,112H56a8,8,0,0,1,0-16H80a8,8,0,0,1,0,16Zm56,8a8,8,0,0,1-16,0V88a8,8,0,0,1,16,0Zm64-8H176a8,8,0,0,1,0-16h24a8,8,0,0,1,0,16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M232,72v64H24V72Z" opacity="0.2"></path>
    <path d="M248,64H8A8,8,0,0,0,8,80h8V192a8,8,0,0,0,16,0V144H224v48a8,8,0,0,0,16,0V80h8a8,8,0,0,0,0-16ZM32,80h88v48H32Zm192,48H136V80h88ZM96,104a8,8,0,0,1-8,8H64a8,8,0,0,1,0-16H88A8,8,0,0,1,96,104Zm64,0a8,8,0,0,1,8-8h24a8,8,0,0,1,0,16H168A8,8,0,0,1,160,104Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M248,68H8a4,4,0,0,0,0,8H20V192a4,4,0,0,0,8,0V140H228v52a4,4,0,0,0,8,0V76h12a4,4,0,0,0,0-8ZM28,76h96v56H28Zm200,56H132V76h96ZM92,104a4,4,0,0,1-4,4H64a4,4,0,0,1,0-8H88A4,4,0,0,1,92,104Zm72,0a4,4,0,0,1,4-4h24a4,4,0,0,1,0,8H168A4,4,0,0,1,164,104Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M244,60H12a12,12,0,0,0,0,24h4V192a12,12,0,0,0,24,0V156H216v36a12,12,0,0,0,24,0V84h4a12,12,0,0,0,0-24ZM40,84h76v48H40Zm176,48H140V84h76ZM100,108a12,12,0,0,1-12,12H68a12,12,0,0,1,0-24H88A12,12,0,0,1,100,108Zm56,0a12,12,0,0,1,12-12h20a12,12,0,0,1,0,24H168A12,12,0,0,1,156,108Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M248,66H8A6,6,0,0,0,8,78H18V192a6,6,0,0,0,12,0V142H226v50a6,6,0,0,0,12,0V78h10a6,6,0,0,0,0-12ZM30,78h92v52H30Zm196,52H134V78h92ZM94,104a6,6,0,0,1-6,6H64a6,6,0,0,1,0-12H88A6,6,0,0,1,94,104Zm68,0a6,6,0,0,1,6-6h24a6,6,0,0,1,0,12H168A6,6,0,0,1,162,104Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M248,64H8A8,8,0,0,0,8,80h8V192a8,8,0,0,0,16,0V144H224v48a8,8,0,0,0,16,0V80h8a8,8,0,0,0,0-16ZM32,80h88v48H32Zm192,48H136V80h88ZM96,104a8,8,0,0,1-8,8H64a8,8,0,0,1,0-16H88A8,8,0,0,1,96,104Zm64,0a8,8,0,0,1,8-8h24a8,8,0,0,1,0,16H168A8,8,0,0,1,160,104Z"></path>
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