//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "communication", feature = "objects"))]
#[component]
pub fn Megaphone(
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
                <path d="M200,72H160.2c-2.91-.17-53.62-3.74-101.91-44.24A16,16,0,0,0,32,40V200a16,16,0,0,0,26.29,12.25c37.77-31.68,77-40.76,93.71-43.3v31.72A16,16,0,0,0,159.12,214l11,7.33A16,16,0,0,0,194.5,212l11.77-44.36A48,48,0,0,0,200,72ZM179,207.89l0,.11-11-7.33V168h21.6ZM200,152H168V88h32a32,32,0,1,1,0,64Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M240,120a40,40,0,0,1-40,40H160V80h40A40,40,0,0,1,240,120Z" opacity="0.2"></path>
    <path d="M248,120a48.05,48.05,0,0,0-48-48H160.2c-2.91-.17-53.62-3.74-101.91-44.24A16,16,0,0,0,32,40V200a16,16,0,0,0,26.29,12.25c37.77-31.68,77-40.76,93.71-43.3v31.72A16,16,0,0,0,159.12,214l11,7.33A16,16,0,0,0,194.5,212l11.77-44.36A48.07,48.07,0,0,0,248,120ZM48,199.93V40h0c42.81,35.91,86.63,45,104,47.24v65.48C134.65,155,90.84,164.07,48,199.93Zm131,8,0,.11-11-7.33V168h21.6ZM200,152H168V88h32a32,32,0,1,1,0,64Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M200,76H160.1c-2.19-.11-54.55-3.38-104.38-45.17A12,12,0,0,0,36,40V200a11.88,11.88,0,0,0,6.92,10.87A12.18,12.18,0,0,0,48,212a11.93,11.93,0,0,0,7.69-2.83c42.46-35.62,86.76-43.25,100.28-44.8v36.29a12,12,0,0,0,5.34,10l11,7.33a12,12,0,0,0,18.28-7l12.49-47.09A44,44,0,0,0,200,76Zm-44,80.3c-13.53,1.35-60,8.62-105.42,46.75A4,4,0,0,1,44,200V40a4,4,0,0,1,2.3-3.62A4.32,4.32,0,0,1,48.05,36a3.88,3.88,0,0,1,2.53,1C96,75.09,142.47,82.36,156,83.71ZM182.87,209a4,4,0,0,1-6.1,2.36l-11-7.33a4,4,0,0,1-1.78-3.33V164h30.8ZM200,156H164V84h36a36,36,0,0,1,0,72Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M252,120a52.06,52.06,0,0,0-52-52H160.32c-3.44-.21-52.6-4-99.46-43.3A20,20,0,0,0,28,40V200a19.8,19.8,0,0,0,11.54,18.12,19.86,19.86,0,0,0,21.32-2.81A192.92,192.92,0,0,1,144,174.47v26.2a20,20,0,0,0,8.9,16.64,11.35,11.35,0,0,0,1.39.8l14.44,7.06A20,20,0,0,0,198.37,213l11.09-41.82A52.07,52.07,0,0,0,252,120ZM52,191.63V48.4c36.17,28.07,72.17,38.1,92,41.66V150C124.17,153.52,88.17,163.55,52,191.63ZM176.39,202.2,168,198.1V172h16.4ZM200,148H168V92h32a28,28,0,1,1,0,56Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M246,120a46.05,46.05,0,0,0-46-46H160.15c-2.58-.15-54.1-3.57-103.15-44.71A14,14,0,0,0,34,40V200a13.85,13.85,0,0,0,8.07,12.68A14.16,14.16,0,0,0,48,214a13.9,13.9,0,0,0,9-3.3c40-33.52,81.57-42,97-44.07v34a14,14,0,0,0,6.23,11.65l11,7.33a14,14,0,0,0,21.32-8.17l12.13-45.71A46.07,46.07,0,0,0,246,120ZM49.29,201.52A2,2,0,0,1,46,200V40a1.9,1.9,0,0,1,1.15-1.8A2.08,2.08,0,0,1,48,38a1.91,1.91,0,0,1,1.26.48c44,36.92,89,45.19,104.71,47v69C138.29,156.33,93.3,164.61,49.29,201.52Zm131.64,7a2,2,0,0,1-3.05,1.18l-11-7.33a2,2,0,0,1-.89-1.67V166h26.2ZM200,154H166V86h34a34,34,0,1,1,0,68Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M248,120a48.05,48.05,0,0,0-48-48H160.2c-2.91-.17-53.62-3.74-101.91-44.24A16,16,0,0,0,32,40V200a16,16,0,0,0,26.29,12.25c37.77-31.68,77-40.76,93.71-43.3v31.72A16,16,0,0,0,159.12,214l11,7.33A16,16,0,0,0,194.5,212l11.77-44.36A48.07,48.07,0,0,0,248,120ZM48,199.93V40h0c42.81,35.91,86.63,45,104,47.24v65.48C134.65,155,90.84,164.07,48,199.93Zm131,8,0,.11-11-7.33V168h21.6ZM200,152H168V88h32a32,32,0,1,1,0,64Z"></path>
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
