//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "communication", feature = "objects", feature = "system"))]
#[component]
pub fn WebcamSlash(
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
                <path d="M83.34,43.19a4,4,0,0,1,.78-6A80,80,0,0,1,190.39,154a4,4,0,0,1-6.11.22l-16.43-18.08a4,4,0,0,1-.3-5,48,48,0,0,0-62.84-69.11,4,4,0,0,1-4.94-.78ZM126.93,72a31.8,31.8,0,0,0-8.43,1.42A4,4,0,0,0,116.75,80l34.12,37.53a4,4,0,0,0,6.67-1.18A31.84,31.84,0,0,0,160,104,32.36,32.36,0,0,0,126.93,72Zm86.45,149.9a8,8,0,0,1-11.3-.54L197.19,216H32a8,8,0,0,1-8-8.53A8.17,8.17,0,0,1,32.27,200H120V183.6A79.93,79.93,0,0,1,58.86,63.84L42.08,45.38A8,8,0,1,1,53.92,34.62l160,176A8,8,0,0,1,213.38,221.92ZM128,152a48.17,48.17,0,0,0,10-1.06l-13.79-15.17A32,32,0,0,1,96,104.71L82.23,89.55A48,48,0,0,0,128,152Zm54.64,48-21.22-23.34A79.24,79.24,0,0,1,136,183.6V200Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M128,32a72,72,0,1,0,72,72A72,72,0,0,0,128,32Zm0,104a32,32,0,1,1,32-32A32,32,0,0,1,128,136Z"
        opacity="0.2"
    ></path>
    <path d="M213.92,210.62l-160-176A8,8,0,1,0,42.08,45.38L58.82,63.8A80,80,0,0,0,120,183.6V200H32a8,8,0,0,0,0,16H197.19l4.89,5.38a8,8,0,1,0,11.84-10.76ZM64,104a63.65,63.65,0,0,1,6.26-27.62L88.68,96.64A40,40,0,0,0,128,144c1.2,0,2.39-.06,3.58-.17L150,164.11A64,64,0,0,1,64,104Zm72,96V183.59a79.91,79.91,0,0,0,25.44-6.91L182.64,200ZM85.52,45.31a8,8,0,0,1,3-10.91,80,80,0,0,1,105,115.5,8,8,0,1,1-13.1-9.19,64,64,0,0,0-84-92.4A8,8,0,0,1,85.52,45.31Zm65.31,66.12A24,24,0,0,0,128,80a24.17,24.17,0,0,0-5.24.57A8,8,0,1,1,119.3,65,40,40,0,0,1,166,116.38a8,8,0,0,1-15.21-4.95Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M211,213.31,51,37.31A4,4,0,0,0,45,42.69L63.8,63.32A76,76,0,0,0,124,179.89V204H32a4,4,0,0,0,0,8H199l6.08,6.69a4,4,0,1,0,5.92-5.38ZM100,103.18l26.14,28.75A28,28,0,0,1,100,104C100,103.73,100,103.45,100,103.18ZM60,104a67.62,67.62,0,0,1,9.4-34.52L93,95.48a36,36,0,0,0,40.13,44.14l23.63,26A68,68,0,0,1,60,104Zm72,100V179.89a76.09,76.09,0,0,0,30.39-8.11L191.69,204ZM89,43.33a4,4,0,0,1,1.5-5.45A76,76,0,0,1,190.26,147.6a4,4,0,1,1-6.55-4.59A68,68,0,0,0,94.46,44.83,4,4,0,0,1,89,43.33Zm65.63,69.34a28,28,0,0,0-32.73-36,4,4,0,0,1-1.74-7.81,36,36,0,0,1,42.08,46.28,4,4,0,0,1-3.8,2.77,4.12,4.12,0,0,1-1.24-.2A4,4,0,0,1,154.63,112.67Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M216.88,207.93l-160-176A12,12,0,1,0,39.12,48.07L53.93,64.36A84,84,0,0,0,116,187.13V196H32a12,12,0,0,0,0,24H195.42l3.7,4.07a12,12,0,0,0,17.76-16.14ZM68,104a59.84,59.84,0,0,1,3.52-20.29l12.91,14.2A44,44,0,0,0,128,148c.64,0,1.28,0,1.92-.05l12.9,14.19A60,60,0,0,1,68,104Zm72,92v-8.87a84.41,84.41,0,0,0,20.42-5.63L173.6,196ZM88.89,42.35a12,12,0,0,1,6.37-15.73A84,84,0,0,1,201.91,143.94a12,12,0,0,1-10.56,6.29,11.84,11.84,0,0,1-5.7-1.45,12,12,0,0,1-4.84-16.26,60,60,0,0,0-76.19-83.8A12,12,0,0,1,88.89,42.35ZM172,103.64a12,12,0,0,1-11.9,12.1H160a12,12,0,0,1-12-11.91A20,20,0,0,0,130.06,84.1a12,12,0,1,1,2.42-23.87A43.92,43.92,0,0,1,172,103.64Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M212.44,212,52.44,36A6,6,0,0,0,43.56,44L61.31,63.56A78,78,0,0,0,122,181.75V202H32a6,6,0,0,0,0,12H198.07l5.49,6a6,6,0,0,0,8.88-8.08Zm-91.67-83a26.05,26.05,0,0,1-18.31-20.15ZM62,104a65.6,65.6,0,0,1,7.78-31.12L90.85,96.06A38,38,0,0,0,128,142a37.59,37.59,0,0,0,4.38-.26l21.06,23.17A66,66,0,0,1,62,104Zm72,98V181.76a78.27,78.27,0,0,0,27.93-7.51L187.16,202ZM87.26,44.32a6,6,0,0,1,2.26-8.18A78,78,0,0,1,191.89,148.75a6,6,0,1,1-9.82-6.89A66,66,0,0,0,95.44,46.57,6,6,0,0,1,87.26,44.32Zm65.47,67.73a26,26,0,0,0-30.4-33.43,6,6,0,1,1-2.6-11.72,38,38,0,0,1,44.41,48.86,6,6,0,0,1-11.41-3.71Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M213.92,210.62l-160-176A8,8,0,1,0,42.08,45.38L58.82,63.8A80,80,0,0,0,120,183.6V200H32a8,8,0,0,0,0,16H197.19l4.89,5.38a8,8,0,1,0,11.84-10.76ZM64,104a63.65,63.65,0,0,1,6.26-27.62L88.68,96.64A40,40,0,0,0,128,144c1.2,0,2.39-.06,3.58-.17L150,164.11A64,64,0,0,1,64,104Zm72,96V183.59a79.91,79.91,0,0,0,25.44-6.91L182.64,200ZM85.52,45.31a8,8,0,0,1,3-10.91,80,80,0,0,1,105,115.5,8,8,0,1,1-13.1-9.19,64,64,0,0,0-84-92.4A8,8,0,0,1,85.52,45.31Zm65.31,66.12A24,24,0,0,0,128,80a24.17,24.17,0,0,0-5.24.57A8,8,0,1,1,119.3,65,40,40,0,0,1,166,116.38a8,8,0,0,1-15.21-4.95Z"></path>
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
