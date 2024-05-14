//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="design", feature ="editor", feature ="office"))]
#[component]
pub fn PenNib(
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
                <path d="M243.31,81.36,174.63,12.68a16,16,0,0,0-22.63,0L123.56,41.12l-58,21.76A16,16,0,0,0,55.36,75.23L34.59,199.83a4,4,0,0,0,6.77,3.49l57-57a23.85,23.85,0,0,1-2.29-12.08,24,24,0,1,1,13.6,23.4l-57,57a4,4,0,0,0,3.49,6.77l124.61-20.77a16,16,0,0,0,12.35-10.16l21.77-58.07L243.31,104a16,16,0,0,0,0-22.63ZM208,116.68,139.32,48l24-24L232,92.68Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M128,48,68.32,70.38a8,8,0,0,0-5.08,6.17L40,216l139.45-23.24a8,8,0,0,0,6.17-5.08L208,128Zm-4,104a20,20,0,1,1,20-20A20,20,0,0,1,124,152Z"
        opacity="0.2"
    ></path>
    <path d="M248,92.68a15.86,15.86,0,0,0-4.69-11.31L174.63,12.68a16,16,0,0,0-22.63,0L123.57,41.11l-58,21.77A16.06,16.06,0,0,0,55.35,75.23L32.11,214.68A8,8,0,0,0,40,224a8.4,8.4,0,0,0,1.32-.11l139.44-23.24a16,16,0,0,0,12.35-10.17l21.77-58L243.31,104A15.87,15.87,0,0,0,248,92.68Zm-69.87,92.19L63.32,204l47.37-47.37a28,28,0,1,0-11.32-11.32L52,192.7,71.13,77.86,126,57.29,198.7,130ZM112,132a12,12,0,1,1,12,12A12,12,0,0,1,112,132Zm96-15.32L139.31,48l24-24L232,92.68Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M244,92.68a11.93,11.93,0,0,0-3.51-8.48L171.8,15.51a12,12,0,0,0-17,0L125.78,44.56,66.92,66.63a12.06,12.06,0,0,0-7.63,9.26L36.05,215.34A4,4,0,0,0,40,220a4.89,4.89,0,0,0,.66-.05L180.1,196.7a12,12,0,0,0,9.27-7.62l22.07-58.86,29-29A11.92,11.92,0,0,0,244,92.68Zm-62.12,93.59a4,4,0,0,1-3.09,2.54L51.66,210l58.45-58.45a24,24,0,1,0-5.66-5.66L46,204.35,67.19,77.21a4,4,0,0,1,2.54-3.09L127,52.64,203.35,129ZM108,132a16,16,0,1,1,16,16A16,16,0,0,1,108,132ZM234.83,95.51,208,122.34,133.66,48l26.82-26.82a4,4,0,0,1,5.66,0l68.69,68.69a4,4,0,0,1,0,5.65Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M252,92.68a19.86,19.86,0,0,0-5.86-14.14L177.46,9.85a20,20,0,0,0-28.29,0L121.35,37.67,64.11,59.14A20,20,0,0,0,51.4,74.58L28.16,214A12,12,0,0,0,40,228a11.9,11.9,0,0,0,2-.16l139.45-23.25a20.07,20.07,0,0,0,15.44-12.7l21.46-57.25,27.82-27.82A19.85,19.85,0,0,0,252,92.68ZM175.2,181.3,75,198l33-33a34,34,0,1,0-17-17L58,181,74.7,80.8,125,61.94,194.05,131ZM112,134a10,10,0,1,1,10,10A10,10,0,0,1,112,134Zm96-23L145,48l18.34-18.34,63,63Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M246,92.68a13.94,13.94,0,0,0-4.1-9.9L173.21,14.1a14,14,0,0,0-19.8,0L124.68,42.83,66.22,64.76a14,14,0,0,0-8.9,10.8L34.08,215A6,6,0,0,0,40,222a6.61,6.61,0,0,0,1-.08l139.44-23.24a14,14,0,0,0,10.81-8.9l21.92-58.46,28.74-28.74A13.92,13.92,0,0,0,246,92.68Zm-66,92.89a2,2,0,0,1-1.54,1.27L57.49,207l52.87-52.88a26,26,0,1,0-8.48-8.48L49,198.53l20.17-121A2,2,0,0,1,70.43,76l56.06-21L201,129.51ZM110,132a14,14,0,1,1,14,14A14,14,0,0,1,110,132ZM233.41,94.1,208,119.51,136.48,48,161.9,22.58a2,2,0,0,1,2.83,0l68.68,68.69a2,2,0,0,1,0,2.83Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M248,92.68a15.86,15.86,0,0,0-4.69-11.31L174.63,12.68a16,16,0,0,0-22.63,0L123.57,41.11l-58,21.77A16.06,16.06,0,0,0,55.35,75.23L32.11,214.68A8,8,0,0,0,40,224a8.4,8.4,0,0,0,1.32-.11l139.44-23.24a16,16,0,0,0,12.35-10.17l21.77-58L243.31,104A15.87,15.87,0,0,0,248,92.68Zm-69.87,92.19L63.32,204l47.37-47.37a28,28,0,1,0-11.32-11.32L52,192.7,71.13,77.86,126,57.29,198.7,130ZM112,132a12,12,0,1,1,12,12A12,12,0,0,1,112,132Zm96-15.32L139.31,48l24-24L232,92.68Z"></path>
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