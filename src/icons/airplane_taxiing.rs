//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="map", feature ="objects"))]
#[component]
pub fn AirplaneTaxiing(
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
                <path d="M248,136v24a8,8,0,0,1-8,8H61.07a39.75,39.75,0,0,1-38.31-28.51L8.69,92.6A16,16,0,0,1,24,72h8a8,8,0,0,1,5.65,2.34L59.32,96H81.81l-9-26.94A16,16,0,0,1,88,48h8a8,8,0,0,1,5.66,2.34L147.32,96H208A40,40,0,0,1,248,136Zm-40,48a16,16,0,1,0,16,16A16,16,0,0,0,208,184Zm-96,0a16,16,0,1,0,16,16A16,16,0,0,0,112,184Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M240,136v24H61.06a32,32,0,0,1-30.65-22.8L16.34,90.3A8,8,0,0,1,24,80h8l24,24H92.91L80.42,66.53A8,8,0,0,1,88,56h8l48,48h64A32,32,0,0,1,240,136Z"
        opacity="0.2"
    ></path>
    <path d="M208,96H147.32L101.66,50.34A8,8,0,0,0,96,48H88A16,16,0,0,0,72.83,69.06l9,26.94H59.32L37.66,74.34A8,8,0,0,0,32,72H24A16,16,0,0,0,8.69,92.6l14.07,46.89A39.75,39.75,0,0,0,61.07,168H240a8,8,0,0,0,8-8V136A40,40,0,0,0,208,96Zm24,56H61.07a23.85,23.85,0,0,1-23-17.1L24,88h4.68l21.66,21.66A8,8,0,0,0,56,112h36.9a8,8,0,0,0,7.59-10.53L88,64h4.68l45.66,45.66A8,8,0,0,0,144,112h64a24,24,0,0,1,24,24Zm-8,48a16,16,0,1,1-16-16A16,16,0,0,1,224,200Zm-96,0a16,16,0,1,1-16-16A16,16,0,0,1,128,200Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M208,100H145.66L98.83,53.17A4,4,0,0,0,96,52H88A12,12,0,0,0,76.63,67.79L87.36,100H57.66L34.83,77.17A4,4,0,0,0,32,76H24A12,12,0,0,0,12.52,91.45l14.06,46.89A35.79,35.79,0,0,0,61.06,164H240a4,4,0,0,0,4-4V136A36,36,0,0,0,208,100Zm28,56H61.06a27.83,27.83,0,0,1-26.81-19.95L20.18,89.15A4,4,0,0,1,24,84h6.34l22.83,22.83A4,4,0,0,0,56,108H92.91a4,4,0,0,0,3.79-5.26L84.21,65.26A4,4,0,0,1,88,60h6.34l46.82,46.83A4,4,0,0,0,144,108h64a28,28,0,0,1,28,28Zm-16,44a12,12,0,1,1-12-12A12,12,0,0,1,220,200Zm-96,0a12,12,0,1,1-12-12A12,12,0,0,1,124,200Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M208,88H149L104.49,43.51A12,12,0,0,0,96,40H88A20,20,0,0,0,69,66.33L76.27,88H61L40.49,67.51A12,12,0,0,0,32,64H24A20,20,0,0,0,4.86,89.75l14.07,46.89A43.72,43.72,0,0,0,61.07,168H240a12,12,0,0,0,12-12V132A44.05,44.05,0,0,0,208,88Zm20,56H61.07a19.89,19.89,0,0,1-19.16-14.25L30.4,91.36l17.12,17.13A12,12,0,0,0,56,112h36.9A12,12,0,0,0,104.3,96.21L94.83,67.79l40.69,40.7A12,12,0,0,0,144,112h64a20,20,0,0,1,20,20Zm0,60a20,20,0,1,1-20-20A20,20,0,0,1,228,204Zm-96,0a20,20,0,1,1-20-20A20,20,0,0,1,132,204Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M208,98H146.49L100.25,51.76A6,6,0,0,0,96,50H88A14,14,0,0,0,74.73,68.43L84.59,98H58.49L36.25,75.76A6,6,0,0,0,32,74H24A14,14,0,0,0,10.6,92l14.07,46.9A37.77,37.77,0,0,0,61.07,166H240a6,6,0,0,0,6-6V136A38,38,0,0,0,208,98Zm26,56H61.07a25.86,25.86,0,0,1-24.91-18.53L22.1,88.57a1.91,1.91,0,0,1,.31-1.76A1.93,1.93,0,0,1,24,86h5.51l22.24,22.24A6,6,0,0,0,56,110H92.91a6,6,0,0,0,5.69-7.9L86.11,64.63A2,2,0,0,1,88,62h5.51l46.24,46.24A6,6,0,0,0,144,110h64a26,26,0,0,1,26,26Zm-12,46a14,14,0,1,1-14-14A14,14,0,0,1,222,200Zm-96,0a14,14,0,1,1-14-14A14,14,0,0,1,126,200Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M208,96H147.32L101.66,50.34A8,8,0,0,0,96,48H88A16,16,0,0,0,72.83,69.06l9,26.94H59.32L37.66,74.34A8,8,0,0,0,32,72H24A16,16,0,0,0,8.69,92.6l14.07,46.89A39.75,39.75,0,0,0,61.07,168H240a8,8,0,0,0,8-8V136A40,40,0,0,0,208,96Zm24,56H61.07a23.85,23.85,0,0,1-23-17.1L24,88h4.68l21.66,21.66A8,8,0,0,0,56,112h36.9a8,8,0,0,0,7.59-10.53L88,64h4.68l45.66,45.66A8,8,0,0,0,144,112h64a24,24,0,0,1,24,24Zm-8,48a16,16,0,1,1-16-16A16,16,0,0,1,224,200Zm-96,0a16,16,0,1,1-16-16A16,16,0,0,1,128,200Z"></path>
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