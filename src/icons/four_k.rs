//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "media"))]
#[component]
pub fn FourK(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Fill => view! {
                <path d="M79.55,136,96,113v23ZM232,56V200a16,16,0,0,1-16,16H40a16,16,0,0,1-16-16V56A16,16,0,0,1,40,40H216A16,16,0,0,1,232,56ZM124,144a8,8,0,0,0-8-8h-4V88a8,8,0,0,0-14.51-4.65l-40,56A8,8,0,0,0,64,152H96v16a8,8,0,0,0,16,0V152h4A8,8,0,0,0,124,144Zm49.59-22.23,24.48-28.56a8,8,0,0,0-12.14-10.42L157.8,115.6s0,0,0,0L152,122.37V88a8,8,0,0,0-16,0v80a8,8,0,0,0,16,0V147l10.62-12.39,22.52,37.55a8,8,0,1,0,13.72-8.24Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M224,48V208H32V48Z" opacity="0.2"></path>
    <path d="M24,48a8,8,0,0,1,8-8H224a8,8,0,0,1,0,16H32A8,8,0,0,1,24,48ZM224,200H32a8,8,0,0,0,0,16H224a8,8,0,0,0,0-16ZM144,72a8,8,0,0,0-8,8v96a8,8,0,0,0,16,0V147l13.09-15,28,48A8,8,0,1,0,206.91,172l-30.7-52.63L206,85.27a8,8,0,1,0-12-10.54l-42,48V80A8,8,0,0,0,144,72ZM88,176V160H40a8,8,0,0,1-6.31-12.91l56-72A8,8,0,0,1,104,80v64h8a8,8,0,0,1,0,16h-8v16a8,8,0,0,1-16,0Zm0-32V103.32L56.36,144Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M28,48a4,4,0,0,1,4-4H224a4,4,0,0,1,0,8H32A4,4,0,0,1,28,48ZM224,204H32a4,4,0,0,0,0,8H224a4,4,0,0,0,0-8ZM144,76a4,4,0,0,0-4,4v96a4,4,0,0,0,8,0V145.5l17.75-20.28L196.54,178a4,4,0,1,0,6.92-4L171.3,118.87,203,82.63a4,4,0,1,0-6-5.26l-49,56V80A4,4,0,0,0,144,76ZM92,176V156H40a4,4,0,0,1-3.16-6.46l56-72A4,4,0,0,1,100,80v68h12a4,4,0,0,1,0,8H100v20a4,4,0,0,1-8,0Zm0-28V91.66L48.18,148Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M20,44A12,12,0,0,1,32,32H224a12,12,0,0,1,0,24H32A12,12,0,0,1,20,44ZM224,200H32a12,12,0,0,0,0,24H224a12,12,0,0,0,0-24ZM148,68a12,12,0,0,0-12,12v96a12,12,0,0,0,24,0V148.51l8.44-9.65,25.19,43.19A12,12,0,0,0,214.37,170l-29.26-50.14L213,87.9A12,12,0,1,0,195,72.1l-35,40V80A12,12,0,0,0,148,68ZM80,176V164H36a12,12,0,0,1-9.47-19.37l56-72A12,12,0,0,1,104,80v60h4a12,12,0,0,1,0,24h-4v12a12,12,0,0,1-24,0Zm0-36V115L60.54,140Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M26,48a6,6,0,0,1,6-6H224a6,6,0,0,1,0,12H32A6,6,0,0,1,26,48ZM224,202H32a6,6,0,0,0,0,12H224a6,6,0,0,0,0-12ZM144,74a6,6,0,0,0-6,6v96a6,6,0,0,0,12,0V146.25l15.42-17.62L194.82,179a6,6,0,0,0,5.19,3,5.91,5.91,0,0,0,3-.82,6,6,0,0,0,2.16-8.2L173.76,119.1,204.52,84a6,6,0,0,0-9-7.9L150,128V80A6,6,0,0,0,144,74ZM90,176V158H40a6,6,0,0,1-4.74-9.68l56-72A6,6,0,0,1,102,80v66h10a6,6,0,0,1,0,12H102v18a6,6,0,0,1-12,0Zm0-30V97.49L52.27,146Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M24,48a8,8,0,0,1,8-8H224a8,8,0,0,1,0,16H32A8,8,0,0,1,24,48ZM224,200H32a8,8,0,0,0,0,16H224a8,8,0,0,0,0-16ZM144,72a8,8,0,0,0-8,8v96a8,8,0,0,0,16,0V147l13.09-15,28,48A8,8,0,1,0,206.91,172l-30.7-52.63L206,85.27a8,8,0,1,0-12-10.54l-42,48V80A8,8,0,0,0,144,72ZM88,176V160H40a8,8,0,0,1-6.31-12.91l56-72A8,8,0,0,1,104,80v64h8a8,8,0,0,1,0,16h-8v16a8,8,0,0,1-16,0Zm0-32V103.32L56.36,144Z"></path>
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
