//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="communication"))]
#[component]
pub fn ChatDots(
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
                <path d="M216,48H40A16,16,0,0,0,24,64V224a15.84,15.84,0,0,0,9.25,14.5A16.05,16.05,0,0,0,40,240a15.89,15.89,0,0,0,10.25-3.78l.09-.07L83,208H216a16,16,0,0,0,16-16V64A16,16,0,0,0,216,48ZM84,140a12,12,0,1,1,12-12A12,12,0,0,1,84,140Zm44,0a12,12,0,1,1,12-12A12,12,0,0,1,128,140Zm44,0a12,12,0,1,1,12-12A12,12,0,0,1,172,140Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,64V192a8,8,0,0,1-8,8H80L45.15,230.11A8,8,0,0,1,32,224V64a8,8,0,0,1,8-8H216A8,8,0,0,1,224,64Z"
        opacity="0.2"
    ></path>
    <path d="M116,128a12,12,0,1,1,12,12A12,12,0,0,1,116,128ZM84,140a12,12,0,1,0-12-12A12,12,0,0,0,84,140Zm88,0a12,12,0,1,0-12-12A12,12,0,0,0,172,140Zm60-76V192a16,16,0,0,1-16,16H83l-32.6,28.16-.09.07A15.89,15.89,0,0,1,40,240a16.13,16.13,0,0,1-6.8-1.52A15.85,15.85,0,0,1,24,224V64A16,16,0,0,1,40,48H216A16,16,0,0,1,232,64ZM40,224h0ZM216,64H40V224l34.77-30A8,8,0,0,1,80,192H216Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M136,128a8,8,0,1,1-8-8A8,8,0,0,1,136,128Zm-52-8a8,8,0,1,0,8,8A8,8,0,0,0,84,120Zm88,0a8,8,0,1,0,8,8A8,8,0,0,0,172,120Zm56-56V192a12,12,0,0,1-12,12H81.49L47.76,233.13l0,0A11.89,11.89,0,0,1,40,236a12.17,12.17,0,0,1-5.1-1.14A11.89,11.89,0,0,1,28,224V64A12,12,0,0,1,40,52H216A12,12,0,0,1,228,64Zm-8,0a4,4,0,0,0-4-4H40a4,4,0,0,0-4,4V224a4,4,0,0,0,6.56,3.08L77.38,197A4,4,0,0,1,80,196H216a4,4,0,0,0,4-4Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M88,128a16,16,0,1,1,16,16A16,16,0,0,1,88,128Zm64,16a16,16,0,1,0-16-16A16,16,0,0,0,152,144Zm84-80V192a20,20,0,0,1-20,20H84.47L53,239.17l-.12.11A19.91,19.91,0,0,1,40.05,244a20.14,20.14,0,0,1-8.49-1.9A19.82,19.82,0,0,1,20,224V64A20,20,0,0,1,40,44H216A20,20,0,0,1,236,64Zm-24,4H44V215.23l28.16-24.32A11.93,11.93,0,0,1,80,188H212Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M138,128a10,10,0,1,1-10-10A10,10,0,0,1,138,128ZM84,118a10,10,0,1,0,10,10A10,10,0,0,0,84,118Zm88,0a10,10,0,1,0,10,10A10,10,0,0,0,172,118Zm58-54V192a14,14,0,0,1-14,14H82.23L49.07,234.64l-.06.05A13.87,13.87,0,0,1,40,238a14.11,14.11,0,0,1-5.95-1.33A13.88,13.88,0,0,1,26,224V64A14,14,0,0,1,40,50H216A14,14,0,0,1,230,64Zm-12,0a2,2,0,0,0-2-2H40a2,2,0,0,0-2,2V224a2,2,0,0,0,3.26,1.55l34.82-30.08A6,6,0,0,1,80,194H216a2,2,0,0,0,2-2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M116,128a12,12,0,1,1,12,12A12,12,0,0,1,116,128ZM84,140a12,12,0,1,0-12-12A12,12,0,0,0,84,140Zm88,0a12,12,0,1,0-12-12A12,12,0,0,0,172,140Zm60-76V192a16,16,0,0,1-16,16H83l-32.6,28.16-.09.07A15.89,15.89,0,0,1,40,240a16.13,16.13,0,0,1-6.8-1.52A15.85,15.85,0,0,1,24,224V64A16,16,0,0,1,40,48H216A16,16,0,0,1,232,64ZM40,224h0ZM216,64H40V224l34.77-30A8,8,0,0,1,80,192H216Z"></path>
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