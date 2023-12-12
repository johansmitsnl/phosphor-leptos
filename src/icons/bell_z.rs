//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn BellZ(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view! {
                <path d="M156,140a12,12,0,0,1-12,12H112a12,12,0,0,1-10-18.66L121.58,104H112a12,12,0,1,1,0-24h32a12,12,0,0,1,10,18.66L134.42,128H144A12,12,0,0,1,156,140Zm69.33,46A19.77,19.77,0,0,1,208,196H171.82a44,44,0,0,1-87.64,0H48a19.77,19.77,0,0,1-17.31-10,20.08,20.08,0,0,1,.05-20.06C39.39,151,44,129.58,44,104a84,84,0,0,1,168,0c0,25.57,4.59,47,13.27,61.93A20.08,20.08,0,0,1,225.34,186ZM147.6,196H108.4a20,20,0,0,0,39.2,0Zm53.74-24C192.49,154,188,131.13,188,104a60,60,0,0,0-120,0c0,27.14-4.48,50-13.33,68Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,192H48a8,8,0,0,1-6.88-12C47.71,168.6,56,139.81,56,104a72,72,0,0,1,144,0c0,35.82,8.3,64.6,14.9,76A8,8,0,0,1,208,192Z"
        opacity="0.2"
    ></path>
    <path d="M152,144a8,8,0,0,1-8,8H112a8,8,0,0,1-6.65-12.44L129.05,104H112a8,8,0,0,1,0-16h32a8,8,0,0,1,6.65,12.44L127,136h17A8,8,0,0,1,152,144Zm69.84,48A15.8,15.8,0,0,1,208,200H167.19a40,40,0,0,1-78.38,0H48a16,16,0,0,1-13.8-24.06C39.75,166.38,48,139.34,48,104a80,80,0,1,1,160,0c0,35.33,8.26,62.38,13.81,71.94A15.89,15.89,0,0,1,221.84,192Zm-71.22,8H105.38a24,24,0,0,0,45.24,0ZM208,184c-7.73-13.27-16-43.95-16-80a64,64,0,1,0-128,0c0,36.06-8.28,66.74-16,80Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M221.8,175.94C216.25,166.38,208,139.33,208,104a80,80,0,1,0-160,0c0,35.34-8.26,62.38-13.81,71.94A16,16,0,0,0,48,200H88.81a40,40,0,0,0,78.38,0H208a16,16,0,0,0,13.8-24.06ZM128,216a24,24,0,0,1-22.62-16h45.24A24,24,0,0,1,128,216Zm16-64H112a8,8,0,0,1-6.65-12.44L129.05,104H112a8,8,0,0,1,0-16h32a8,8,0,0,1,6.65,12.44L127,136h17a8,8,0,0,1,0,16Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M150,144a6,6,0,0,1-6,6H112a6,6,0,0,1-5-9.33L132.79,102H112a6,6,0,0,1,0-12h32a6,6,0,0,1,5,9.33L123.21,138H144A6,6,0,0,1,150,144Zm70.11,47a13.83,13.83,0,0,1-12.1,7H165.52a38,38,0,0,1-75,0H48a14,14,0,0,1-12.06-21.06C41.59,167.2,50,139.74,50,104a78,78,0,1,1,156,0c0,35.73,8.42,63.2,14.08,72.94A13.9,13.9,0,0,1,220.11,191Zm-66.82,7H102.71a26,26,0,0,0,50.58,0Zm56.41-15C202.13,170,194,139.68,194,104a66,66,0,1,0-132,0c0,35.69-8.14,66-15.71,79a2,2,0,0,0,0,2,1.9,1.9,0,0,0,1.7,1H208a1.9,1.9,0,0,0,1.7-1A2,2,0,0,0,209.7,183Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M152,144a8,8,0,0,1-8,8H112a8,8,0,0,1-6.65-12.44L129.05,104H112a8,8,0,0,1,0-16h32a8,8,0,0,1,6.65,12.44L127,136h17A8,8,0,0,1,152,144Zm69.84,48A15.8,15.8,0,0,1,208,200H167.19a40,40,0,0,1-78.38,0H48a16,16,0,0,1-13.8-24.06C39.75,166.38,48,139.34,48,104a80,80,0,1,1,160,0c0,35.33,8.26,62.38,13.81,71.94A15.89,15.89,0,0,1,221.84,192Zm-71.22,8H105.38a24,24,0,0,0,45.24,0ZM208,184c-7.73-13.27-16-43.95-16-80a64,64,0,1,0-128,0c0,36.06-8.28,66.74-16,80Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M148,144a4,4,0,0,1-4,4H112a4,4,0,0,1-3.33-6.22L136.53,100H112a4,4,0,0,1,0-8h32a4,4,0,0,1,3.33,6.22L119.47,140H144A4,4,0,0,1,148,144Zm70.38,46A11.84,11.84,0,0,1,208,196H163.77a36,36,0,0,1-71.54,0H48A12,12,0,0,1,37.65,178C43.42,168,52,140.13,52,104a76,76,0,1,1,152,0c0,36.13,8.59,64,14.36,73.95A11.92,11.92,0,0,1,218.38,190Zm-62.67,6H100.29a28,28,0,0,0,55.42,0Zm55.72-14C204,169.17,196,139.31,196,104a68,68,0,1,0-136,0c0,35.32-8,65.17-15.44,78a4,4,0,0,0,0,4A3.91,3.91,0,0,0,48,188H208a3.91,3.91,0,0,0,3.44-2A4,4,0,0,0,211.43,182Z"></path>
}.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size.get()
            height=size.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}
