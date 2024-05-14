//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "map", feature = "objects"))]
#[component]
pub fn AirplaneLanding(
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
                <path d="M256,216a8,8,0,0,1-8,8H104a8,8,0,0,1,0-16H248A8,8,0,0,1,256,216Zm-24-24a8,8,0,0,0,8-8V148.32a40.13,40.13,0,0,0-29.28-38.54l-60.84-17-22.5-53.63a8,8,0,0,0-4.85-4.5l-5.47-1.82A16,16,0,0,0,96,48V77.39L66.13,68.88,55.52,39.51a8,8,0,0,0-5-4.87l-5.47-1.82A16,16,0,0,0,24,48v55.72a40.12,40.12,0,0,0,29.21,38.52L229.84,191.7A8,8,0,0,0,232,192Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,148.32V184L55.37,134.54A32,32,0,0,1,32,103.73V48a8,8,0,0,1,10.53-7.59L48,42.24,60,75.46,104,88V48a8,8,0,0,1,10.53-7.59L120,42.24l24,57.2,64.56,18A32,32,0,0,1,232,148.32Z"
        opacity="0.2"
    ></path>
    <path d="M256,216a8,8,0,0,1-8,8H104a8,8,0,0,1,0-16H248A8,8,0,0,1,256,216Zm-26.16-24.3L53.21,142.24A40.12,40.12,0,0,1,24,103.72V48A16,16,0,0,1,45.06,32.82l5.47,1.82a8,8,0,0,1,5,4.87L66.13,68.88,96,77.39V48a16,16,0,0,1,21.06-15.18l5.47,1.82a8,8,0,0,1,4.85,4.5l22.5,53.63,60.84,17A40.13,40.13,0,0,1,240,148.32V184a8,8,0,0,1-10.16,7.7ZM224,148.32a24.09,24.09,0,0,0-17.58-23.13l-64.57-18a8,8,0,0,1-5.23-4.61L114,48.67,112,48V88a8,8,0,0,1-10.19,7.7l-44-12.54a8,8,0,0,1-5.33-5L41.79,48.59,40,48v55.72a24.09,24.09,0,0,0,17.53,23.12L224,173.45Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M252,216a4,4,0,0,1-4,4H104a4,4,0,0,1,0-8H248A4,4,0,0,1,252,216Zm-21.08-28.15L54.29,138.4A36.12,36.12,0,0,1,28,103.73V48A12,12,0,0,1,43.79,36.63l5.48,1.82a4,4,0,0,1,2.49,2.44L63.07,72.18,100,82.71V48a12,12,0,0,1,15.79-11.38l5.48,1.82a4,4,0,0,1,2.42,2.25l23.25,55.42,62.7,17.52A36.1,36.1,0,0,1,236,148.33V184a4,4,0,0,1-5.08,3.85ZM228,148.33a28.07,28.07,0,0,0-20.51-27l-64.57-18a4,4,0,0,1-2.61-2.31L117,45.47l-3.75-1.25A4,4,0,0,0,108,48V88a4,4,0,0,1-5.1,3.85l-44-12.54a4,4,0,0,1-2.66-2.49L44.9,45.43l-3.64-1.21a3.95,3.95,0,0,0-3.6.55A4,4,0,0,0,36,48v55.72a28.1,28.1,0,0,0,20.45,27l171.55,48Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M256,216a12,12,0,0,1-12,12H104a12,12,0,0,1,0-24H244A12,12,0,0,1,256,216Zm-27.24-24.45L52.14,142.09A44.13,44.13,0,0,1,20,99.72V48A20,20,0,0,1,46.32,29l5.48,1.83a12,12,0,0,1,7.49,7.3L69.2,65.59,92,72.09V48a20,20,0,0,1,26.32-19l5.48,1.83a12,12,0,0,1,7.27,6.74l21.75,51.85,59,16.49A44.12,44.12,0,0,1,244,148.32V180a12,12,0,0,1-15.24,11.55ZM220,148.32a20.05,20.05,0,0,0-14.65-19.27L140.77,111a12,12,0,0,1-7.84-6.91L116,63.71V88a12,12,0,0,1-15.29,11.54L56.71,87a12,12,0,0,1-8-7.46L44,66.48V99.72A20.07,20.07,0,0,0,58.61,119L220,164.18Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M254,216a6,6,0,0,1-6,6H104a6,6,0,0,1,0-12H248A6,6,0,0,1,254,216Zm-23.62-26.22L53.75,140.32A38.14,38.14,0,0,1,26,103.72V48A14,14,0,0,1,44.43,34.71l5.47,1.83a6,6,0,0,1,3.74,3.65l11,30.33L98,80V48a14,14,0,0,1,18.43-13.29l5.47,1.83a6,6,0,0,1,3.63,3.37l22.88,54.53,61.77,17.27A38.09,38.09,0,0,1,238,148.32V184a6,6,0,0,1-7.62,5.78ZM226,148.32a26.07,26.07,0,0,0-19-25l-64.58-18a6,6,0,0,1-3.91-3.46l-23-54.7-2.89-1A2,2,0,0,0,110,48V88a6,6,0,0,1-7.64,5.77l-44-12.54a6,6,0,0,1-4-3.73L43.34,47l-2.71-.9A1.91,1.91,0,0,0,40,46a2,2,0,0,0-1.16.38A2,2,0,0,0,38,48v55.72a26.09,26.09,0,0,0,19,25l169,47.33Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M256,216a8,8,0,0,1-8,8H104a8,8,0,0,1,0-16H248A8,8,0,0,1,256,216Zm-26.16-24.3L53.21,142.24A40.12,40.12,0,0,1,24,103.72V48A16,16,0,0,1,45.06,32.82l5.47,1.82a8,8,0,0,1,5,4.87L66.13,68.88,96,77.39V48a16,16,0,0,1,21.06-15.18l5.47,1.82a8,8,0,0,1,4.85,4.5l22.5,53.63,60.84,17A40.13,40.13,0,0,1,240,148.32V184a8,8,0,0,1-10.16,7.7ZM224,148.32a24.09,24.09,0,0,0-17.58-23.13l-64.57-18a8,8,0,0,1-5.23-4.61L114,48.67,112,48V88a8,8,0,0,1-10.19,7.7l-44-12.54a8,8,0,0,1-5.33-5L41.79,48.59,40,48v55.72a24.09,24.09,0,0,0,17.53,23.12L224,173.45Z"></path>
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
