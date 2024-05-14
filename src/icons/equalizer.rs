//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="media", feature ="system"))]
#[component]
pub fn Equalizer(
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
                <path d="M80,96a8,8,0,0,1-8,8H24a8,8,0,0,1,0-16H72A8,8,0,0,1,80,96Zm72,24H104a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16Zm32-48h48a8,8,0,0,0,0-16H184a8,8,0,0,0,0,16ZM72,120H24a8,8,0,0,0-8,8v64a8,8,0,0,0,8,8H72a8,8,0,0,0,8-8V128A8,8,0,0,0,72,120ZM232,88H184a8,8,0,0,0-8,8v96a8,8,0,0,0,8,8h48a8,8,0,0,0,8-8V96A8,8,0,0,0,232,88Zm-80,64H104a8,8,0,0,0-8,8v32a8,8,0,0,0,8,8h48a8,8,0,0,0,8-8V160A8,8,0,0,0,152,152Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M232,64V192H24V80A16,16,0,0,1,40,64Z" opacity="0.2"></path>
    <path d="M80,96a8,8,0,0,1-8,8H24a8,8,0,0,1,0-16H72A8,8,0,0,1,80,96Zm-8,24H24a8,8,0,0,0,0,16H72a8,8,0,0,0,0-16Zm0,32H24a8,8,0,0,0,0,16H72a8,8,0,0,0,0-16Zm0,32H24a8,8,0,0,0,0,16H72a8,8,0,0,0,0-16Zm80-64H104a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16Zm0,32H104a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16Zm0,32H104a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16Zm80-96H184a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16ZM184,72h48a8,8,0,0,0,0-16H184a8,8,0,0,0,0,16Zm48,48H184a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16Zm0,32H184a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16Zm0,32H184a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M76,96a4,4,0,0,1-4,4H24a4,4,0,0,1,0-8H72A4,4,0,0,1,76,96Zm-4,28H24a4,4,0,0,0,0,8H72a4,4,0,0,0,0-8Zm0,32H24a4,4,0,0,0,0,8H72a4,4,0,0,0,0-8Zm0,32H24a4,4,0,0,0,0,8H72a4,4,0,0,0,0-8Zm80-64H104a4,4,0,0,0,0,8h48a4,4,0,0,0,0-8Zm0,32H104a4,4,0,0,0,0,8h48a4,4,0,0,0,0-8Zm0,32H104a4,4,0,0,0,0,8h48a4,4,0,0,0,0-8Zm80-96H184a4,4,0,0,0,0,8h48a4,4,0,0,0,0-8ZM184,68h48a4,4,0,0,0,0-8H184a4,4,0,0,0,0,8Zm48,56H184a4,4,0,0,0,0,8h48a4,4,0,0,0,0-8Zm0,32H184a4,4,0,0,0,0,8h48a4,4,0,0,0,0-8Zm0,32H184a4,4,0,0,0,0,8h48a4,4,0,0,0,0-8Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M80,108a12,12,0,0,1-12,12H28a12,12,0,0,1,0-24H68A12,12,0,0,1,80,108ZM68,136H28a12,12,0,0,0,0,24H68a12,12,0,0,0,0-24Zm0,40H28a12,12,0,0,0,0,24H68a12,12,0,0,0,0-24Zm80-40H108a12,12,0,0,0,0,24h40a12,12,0,0,0,0-24Zm0,40H108a12,12,0,0,0,0,24h40a12,12,0,0,0,0-24Zm40-96h40a12,12,0,0,0,0-24H188a12,12,0,0,0,0,24Zm40,16H188a12,12,0,0,0,0,24h40a12,12,0,0,0,0-24Zm0,40H188a12,12,0,0,0,0,24h40a12,12,0,0,0,0-24Zm0,40H188a12,12,0,0,0,0,24h40a12,12,0,0,0,0-24Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M78,96a6,6,0,0,1-6,6H24a6,6,0,0,1,0-12H72A6,6,0,0,1,78,96Zm-6,26H24a6,6,0,0,0,0,12H72a6,6,0,0,0,0-12Zm0,32H24a6,6,0,0,0,0,12H72a6,6,0,0,0,0-12Zm0,32H24a6,6,0,0,0,0,12H72a6,6,0,0,0,0-12Zm80-64H104a6,6,0,0,0,0,12h48a6,6,0,0,0,0-12Zm0,32H104a6,6,0,0,0,0,12h48a6,6,0,0,0,0-12Zm0,32H104a6,6,0,0,0,0,12h48a6,6,0,0,0,0-12Zm80-96H184a6,6,0,0,0,0,12h48a6,6,0,0,0,0-12ZM184,70h48a6,6,0,0,0,0-12H184a6,6,0,0,0,0,12Zm48,52H184a6,6,0,0,0,0,12h48a6,6,0,0,0,0-12Zm0,32H184a6,6,0,0,0,0,12h48a6,6,0,0,0,0-12Zm0,32H184a6,6,0,0,0,0,12h48a6,6,0,0,0,0-12Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M80,96a8,8,0,0,1-8,8H24a8,8,0,0,1,0-16H72A8,8,0,0,1,80,96Zm-8,24H24a8,8,0,0,0,0,16H72a8,8,0,0,0,0-16Zm0,32H24a8,8,0,0,0,0,16H72a8,8,0,0,0,0-16Zm0,32H24a8,8,0,0,0,0,16H72a8,8,0,0,0,0-16Zm80-64H104a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16Zm0,32H104a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16Zm0,32H104a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16Zm80-96H184a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16ZM184,72h48a8,8,0,0,0,0-16H184a8,8,0,0,0,0,16Zm48,48H184a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16Zm0,32H184a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16Zm0,32H184a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16Z"></path>
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