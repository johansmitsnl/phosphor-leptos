//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="objects", feature ="finance", feature ="office"))]
#[component]
pub fn Lectern(
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
                <path d="M246.3,120.84l-40-80A15.92,15.92,0,0,0,192,32H64A15.92,15.92,0,0,0,49.7,40.84l-40,80A16,16,0,0,0,24,144h96v64H96a8,8,0,0,0,0,16h64a8,8,0,0,0,0-16H136V144h96a16,16,0,0,0,14.31-23.16ZM192,120H64a8,8,0,0,1,0-16H192a8,8,0,0,1,0,16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,136H24a8,8,0,0,1-7.16-11.58l40-80A8,8,0,0,1,64,40H192a8,8,0,0,1,7.16,4.42l40,80A8,8,0,0,1,232,136Z"
        opacity="0.2"
    ></path>
    <path d="M246.3,120.84l-40-80A15.92,15.92,0,0,0,192,32H64A15.92,15.92,0,0,0,49.7,40.84l-40,80A16,16,0,0,0,24,144h96v64H96a8,8,0,0,0,0,16h64a8,8,0,0,0,0-16H136V144h96a16,16,0,0,0,14.31-23.16ZM24,128,64,48H192l40,80Zm168-24a8,8,0,0,1-8,8H72a8,8,0,0,1,0-16H184A8,8,0,0,1,192,104Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M242.72,122.63l-40-80A11.93,11.93,0,0,0,192,36H64a11.93,11.93,0,0,0-10.73,6.63l-40,80A12,12,0,0,0,24,140H124v72H96a4,4,0,0,0,0,8h64a4,4,0,0,0,0-8H132V140H232a12,12,0,0,0,10.73-17.37Zm-7.33,7.47A3.94,3.94,0,0,1,232,132H24a4,4,0,0,1-3.58-5.79l40-80A4,4,0,0,1,64,44H192a4,4,0,0,1,3.58,2.21l40,80A3.94,3.94,0,0,1,235.39,130.1ZM188,104a4,4,0,0,1-4,4H72a4,4,0,0,1,0-8H184A4,4,0,0,1,188,104Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M249.87,119.06l-40-80A19.89,19.89,0,0,0,192,28H64A19.89,19.89,0,0,0,46.13,39.06l-40,80A20,20,0,0,0,24,148h92v56H96a12,12,0,0,0,0,24h64a12,12,0,0,0,0-24H140V148h92a20,20,0,0,0,17.89-28.94ZM30.49,124l36-72h123l36,72ZM192,100a12,12,0,0,1-12,12H76a12,12,0,0,1,0-24H180A12,12,0,0,1,192,100Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M244.51,121.74l-40-80A13.92,13.92,0,0,0,192,34H64a13.92,13.92,0,0,0-12.52,7.74l-40,80A14,14,0,0,0,24,142h98v68H96a6,6,0,0,0,0,12h64a6,6,0,0,0,0-12H134V142h98a14,14,0,0,0,12.52-20.26Zm-10.82,7.31a1.93,1.93,0,0,1-1.7.95H24a2,2,0,0,1-1.79-2.89l40-80A2,2,0,0,1,64,46H192a2,2,0,0,1,1.79,1.11l40,80A2,2,0,0,1,233.69,129.05ZM190,104a6,6,0,0,1-6,6H72a6,6,0,0,1,0-12H184A6,6,0,0,1,190,104Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M246.3,120.84l-40-80A15.92,15.92,0,0,0,192,32H64A15.92,15.92,0,0,0,49.7,40.84l-40,80A16,16,0,0,0,24,144h96v64H96a8,8,0,0,0,0,16h64a8,8,0,0,0,0-16H136V144h96a16,16,0,0,0,14.31-23.16ZM24,128,64,48H192l40,80Zm168-24a8,8,0,0,1-8,8H72a8,8,0,0,1,0-16H184A8,8,0,0,1,192,104Z"></path>
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