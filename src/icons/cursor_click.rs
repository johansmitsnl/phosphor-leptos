//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design", feature = "system"))]
#[component]
pub fn CursorClick(
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
                <path d="M220.49,190.83a12,12,0,0,1,0,17L207.8,220.49a12,12,0,0,1-17,0l-56.56-56.57L115,214.09c0,.1-.08.21-.13.32a15.83,15.83,0,0,1-14.6,9.59l-.79,0a15.83,15.83,0,0,1-14.41-11L32.8,52.92A16,16,0,0,1,52.92,32.8L213,85.07a16,16,0,0,1,1.41,29.8l-.32.13-50.17,19.27ZM96,32a8,8,0,0,0,8-8V16a8,8,0,0,0-16,0v8A8,8,0,0,0,96,32ZM16,104h8a8,8,0,0,0,0-16H16a8,8,0,0,0,0,16ZM124.42,39.16a8,8,0,0,0,10.74-3.58l8-16a8,8,0,0,0-14.31-7.16l-8,16A8,8,0,0,0,124.42,39.16Zm-96,81.69-16,8a8,8,0,0,0,7.16,14.31l16-8a8,8,0,1,0-7.16-14.31Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M213.66,201,201,213.66a8,8,0,0,1-11.31,0l-51.31-51.31a8,8,0,0,0-13,2.46l-17.82,46.41a8,8,0,0,1-14.85-.71L40.41,50.44a8,8,0,0,1,10-10L210.51,92.68a8,8,0,0,1,.71,14.85l-46.41,17.82a8,8,0,0,0-2.46,13l51.31,51.31A8,8,0,0,1,213.66,201Z"
        opacity="0.2"
    ></path>
    <path d="M88,24V16a8,8,0,0,1,16,0v8a8,8,0,0,1-16,0ZM16,104h8a8,8,0,0,0,0-16H16a8,8,0,0,0,0,16ZM124.42,39.16a8,8,0,0,0,10.74-3.58l8-16a8,8,0,0,0-14.31-7.16l-8,16A8,8,0,0,0,124.42,39.16Zm-96,81.69-16,8a8,8,0,0,0,7.16,14.31l16-8a8,8,0,1,0-7.16-14.31ZM219.31,184a16,16,0,0,1,0,22.63l-12.68,12.68a16,16,0,0,1-22.63,0L132.7,168,115,214.09c0,.1-.08.21-.13.32a15.83,15.83,0,0,1-14.6,9.59l-.79,0a15.83,15.83,0,0,1-14.41-11L32.8,52.92A16,16,0,0,1,52.92,32.8L213,85.07a16,16,0,0,1,1.41,29.8l-.32.13L168,132.69ZM208,195.31,156.69,144h0a16,16,0,0,1,4.93-26l.32-.14,45.95-17.64L48,48l52.2,159.86,17.65-46c0-.11.08-.22.13-.33a16,16,0,0,1,11.69-9.34,16.72,16.72,0,0,1,3-.28,16,16,0,0,1,11.3,4.69L195.31,208Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M92,24V16a4,4,0,0,1,8,0v8a4,4,0,0,1-8,0ZM16,100h8a4,4,0,0,0,0-8H16a4,4,0,0,0,0,8ZM126.21,35.58a4,4,0,0,0,5.37-1.79l8-16a4,4,0,0,0-7.16-3.58l-8,16A4,4,0,0,0,126.21,35.58Zm-96,88.84-16,8a4,4,0,0,0,3.58,7.16l16-8a4,4,0,1,0-3.58-7.16Zm186.28,62.41a12,12,0,0,1,0,17L203.8,216.49a12,12,0,0,1-17,0l-51.31-51.31a3.93,3.93,0,0,0-3.58-1.11,4,4,0,0,0-2.89,2.27l-17.78,46.31a.77.77,0,0,1-.07.16A11.85,11.85,0,0,1,100.26,220h-.59a11.88,11.88,0,0,1-10.8-8.23L36.6,51.68A12,12,0,0,1,51.68,36.6L211.76,88.87a12,12,0,0,1,1.05,22.33l-.16.07-46.31,17.78a4,4,0,0,0-1.17,6.47Zm-5.66,5.66-51.31-51.32a12,12,0,0,1,3.7-19.49l.16-.06,46.31-17.79a3.95,3.95,0,0,0-.42-7.35L49.2,44.21a4,4,0,0,0-5,5L96.48,209.27a4,4,0,0,0,7.36.42l17.78-46.31a1.11,1.11,0,0,1,.07-.16,12,12,0,0,1,8.76-7,12.21,12.21,0,0,1,2.24-.21,12,12,0,0,1,8.49,3.52l51.31,51.31a4,4,0,0,0,5.65,0l12.69-12.69A4,4,0,0,0,210.83,192.49Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M224.15,179.17l-46.82-46.82,37.92-13.51c.26-.09.51-.19.76-.3a20,20,0,0,0-1.76-37.27L54.16,29A20,20,0,0,0,29,54.16L81.27,214.24A20,20,0,0,0,118.54,216c.11-.25.21-.5.3-.76l13.51-37.92,46.83,46.82a20,20,0,0,0,28.28,0l16.69-16.68A20,20,0,0,0,224.15,179.17Zm-30.83,25.17-48.48-48.48A20,20,0,0,0,130.7,150a20.47,20.47,0,0,0-3.73.35A20,20,0,0,0,112.35,162c-.11.25-.2.5-.3.76L100.4,195.5,54.29,54.29,195.5,100.4l-32.71,11.65c-.25.09-.51.19-.76.3a20,20,0,0,0-6.16,32.48h0l48.48,48.48ZM84,16V12a12,12,0,0,1,24,0v4a12,12,0,0,1-24,0ZM12,108a12,12,0,0,1,0-24h4a12,12,0,0,1,0,24ZM120.62,24.21l4-12a12,12,0,0,1,22.77,7.58l-4,12a12,12,0,0,1-22.77-7.58Zm-81.23,104a12,12,0,0,1-7.59,15.17l-12,4a12,12,0,1,1-7.59-22.76l12-4A12,12,0,0,1,39.39,128.21Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M90,24V16a6,6,0,0,1,12,0v8a6,6,0,0,1-12,0ZM16,102h8a6,6,0,0,0,0-12H16a6,6,0,0,0,0,12ZM125.32,37.37a6,6,0,0,0,8.05-2.69l8-16a6,6,0,0,0-10.74-5.37l-8,16A6,6,0,0,0,125.32,37.37Zm-96,85.26-16,8a6,6,0,0,0,5.36,10.74l16-8a6,6,0,1,0-5.36-10.74ZM217.9,185.41a14,14,0,0,1,0,19.8L205.21,217.9a14,14,0,0,1-19.8,0L134.1,166.59a2,2,0,0,0-3.21.54l-17.75,46.24a2.44,2.44,0,0,0-.1.24A13.85,13.85,0,0,1,100.26,222c-.23,0-.45,0-.68,0A13.85,13.85,0,0,1,87,212.38L34.7,52.3A14,14,0,0,1,52.3,34.7L212.38,87A14,14,0,0,1,213.61,113l-.24.09-46.25,17.76a2,2,0,0,0-.53,3.21Zm-8.49,8.49L158.1,142.59h0a14,14,0,0,1,4.32-22.74l.24-.1L208.91,102a2,2,0,0,0-.26-3.61L48.58,46.11a2.33,2.33,0,0,0-.65-.11,2,2,0,0,0-1.82,2.58L98.38,208.65a1.84,1.84,0,0,0,1.77,1.35,1.81,1.81,0,0,0,1.84-1.09l17.76-46.25.1-.24a14,14,0,0,1,22.74-4.32l51.31,51.31a2,2,0,0,0,2.83,0l12.68-12.68A2,2,0,0,0,209.41,193.9Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M88,24V16a8,8,0,0,1,16,0v8a8,8,0,0,1-16,0ZM16,104h8a8,8,0,0,0,0-16H16a8,8,0,0,0,0,16ZM124.42,39.16a8,8,0,0,0,10.74-3.58l8-16a8,8,0,0,0-14.31-7.16l-8,16A8,8,0,0,0,124.42,39.16Zm-96,81.69-16,8a8,8,0,0,0,7.16,14.31l16-8a8,8,0,1,0-7.16-14.31ZM219.31,184a16,16,0,0,1,0,22.63l-12.68,12.68a16,16,0,0,1-22.63,0L132.7,168,115,214.09c0,.1-.08.21-.13.32a15.83,15.83,0,0,1-14.6,9.59l-.79,0a15.83,15.83,0,0,1-14.41-11L32.8,52.92A16,16,0,0,1,52.92,32.8L213,85.07a16,16,0,0,1,1.41,29.8l-.32.13L168,132.69ZM208,195.31,156.69,144h0a16,16,0,0,1,4.93-26l.32-.14,45.95-17.64L48,48l52.2,159.86,17.65-46c0-.11.08-.22.13-.33a16,16,0,0,1,11.69-9.34,16.72,16.72,0,0,1,3-.28,16,16,0,0,1,11.3,4.69L195.31,208Z"></path>
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
