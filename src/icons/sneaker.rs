//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="commerce", feature ="objects", feature ="health"))]
#[component]
pub fn Sneaker(
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
                <path d="M228.65,129.11l-28.06-9.35a4,4,0,0,0-2.63,0l-43.23,15.72A8.14,8.14,0,0,1,152,136a8,8,0,0,1-7.71-5.88,8.17,8.17,0,0,1,5.22-9.73L168,113.67a2.54,2.54,0,0,0-.06-4.8,23.93,23.93,0,0,1-8.8-5.25,4,4,0,0,0-4.17-.91l-24.22,8.8a8,8,0,0,1-10.44-5.39,8.17,8.17,0,0,1,5.22-9.73L146,88.93a4,4,0,0,0,2.31-5.34l-3.06-7.16a4,4,0,0,0-5.05-2.19l-25.5,9.27a8,8,0,0,1-10.44-5.39,8.17,8.17,0,0,1,5.22-9.73l24-8.73a4,4,0,0,0,2.31-5.33L130.39,41.6s0-.07,0-.1A16,16,0,0,0,110.25,33L34.53,60.49A16.05,16.05,0,0,0,24,75.53V192a16,16,0,0,0,16,16H240a16,16,0,0,0,16-16V167.06A40,40,0,0,0,228.65,129.11ZM240,192H40V176H240Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M248,167.06V168H32V75.54A8,8,0,0,1,37.27,68L113,40.48a8,8,0,0,1,10,4.27L146.27,99.1a32,32,0,0,0,19.12,17.36l60.73,20.25A32,32,0,0,1,248,167.06Z"
        opacity="0.2"
    ></path>
    <path d="M228.65,129.11l-60.73-20.24a24,24,0,0,1-14.32-13L130.39,41.6s0-.07,0-.1A16,16,0,0,0,110.25,33L34.53,60.49A16.05,16.05,0,0,0,24,75.53V192a16,16,0,0,0,16,16H240a16,16,0,0,0,16-16V167.06A40,40,0,0,0,228.65,129.11ZM40,75.53,115.72,48l7.11,16.63-21.56,7.85A8,8,0,0,0,104,88a7.91,7.91,0,0,0,2.73-.49l22.4-8.14,4.74,11.07-16.6,6A8,8,0,0,0,120,112a7.91,7.91,0,0,0,2.73-.49l17.6-6.4a40.06,40.06,0,0,0,7.68,10l-14.74,5.36A8,8,0,0,0,136,136a8.14,8.14,0,0,0,2.73-.48l28-10.18,56.87,18.95A24,24,0,0,1,238.93,160H40ZM240,192H40V176H240Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M227.38,132.91l-60.72-20.24A28,28,0,0,1,150,97.53L126.69,43.12a12,12,0,0,0-15.07-6.4L35.9,64.25A12,12,0,0,0,28,75.53V192a12,12,0,0,0,12,12H240a12,12,0,0,0,12-12V167.06A36,36,0,0,0,227.38,132.91ZM38.63,71.77l75.72-27.53a3.84,3.84,0,0,1,1.37-.24,4,4,0,0,1,3.63,2.32L128.17,67l-25.54,9.29A4,4,0,0,0,104,84a4.12,4.12,0,0,0,1.37-.24l25.95-9.44,7.89,18.44-20.58,7.48A4,4,0,0,0,120,108a4.12,4.12,0,0,0,1.37-.24l21-7.64.25.6a36.11,36.11,0,0,0,13.52,15.7l-21.5,7.82A4,4,0,0,0,136,132a4.12,4.12,0,0,0,1.37-.24l29.3-10.66,58.18,19.4a28,28,0,0,1,19,23.5H36V75.53A4,4,0,0,1,38.63,71.77ZM240,196H40a4,4,0,0,1-4-4V172H244v20A4,4,0,0,1,240,196Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M255.8,162.93l0-.31A43.94,43.94,0,0,0,226,125.36l-56.62-20.2-.24-.09a20,20,0,0,1-11.92-10.78L134.06,40l-.06-.16A20.06,20.06,0,0,0,108.89,29.2L33.17,56.73A20.07,20.07,0,0,0,20,75.53V192a20,20,0,0,0,20,20H236a20,20,0,0,0,20-20V167.06C256,165.67,255.93,164.3,255.8,162.93ZM113.53,53.05l6,14L103.9,72.72a12,12,0,1,0,8.2,22.55L129,89.14l4.54,10.63-13.6,5A12,12,0,0,0,124,128a11.79,11.79,0,0,0,4.1-.73l19.57-7.11a43.86,43.86,0,0,0,13.8,7.64L218.09,148l.23.08a19.89,19.89,0,0,1,6.84,3.91H44V78.33ZM44,188V176H232v12Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M228,131l-60.73-20.24a26,26,0,0,1-15.51-14L128.51,42.31a14,14,0,0,0-17.57-7.47L35.22,62.37A14.05,14.05,0,0,0,26,75.53V192a14,14,0,0,0,14,14H240a14,14,0,0,0,14-14V167.06A38,38,0,0,0,228,131ZM39.32,73.65,115,46.12a1.81,1.81,0,0,1,.68-.12,2,2,0,0,1,1.79,1.11l8,18.68L102,74.36A6,6,0,0,0,104,86a5.92,5.92,0,0,0,2-.37l24.18-8.79,6.31,14.76L118,98.36A6,6,0,0,0,120,110a6.15,6.15,0,0,0,2-.36l19.26-7a38,38,0,0,0,10.57,13.21L134,122.36A6,6,0,0,0,136,134a6.15,6.15,0,0,0,2.05-.36l28.64-10.42,57.53,19.18A25.94,25.94,0,0,1,241.49,162H38V75.53A2,2,0,0,1,39.32,73.65ZM240,194H40a2,2,0,0,1-2-2V174H242v18A2,2,0,0,1,240,194Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M228.65,129.11l-60.73-20.24a24,24,0,0,1-14.32-13L130.39,41.6s0-.07,0-.1A16,16,0,0,0,110.25,33L34.53,60.49A16.05,16.05,0,0,0,24,75.53V192a16,16,0,0,0,16,16H240a16,16,0,0,0,16-16V167.06A40,40,0,0,0,228.65,129.11ZM115.72,48l7.11,16.63-21.56,7.85A8,8,0,0,0,104,88a7.91,7.91,0,0,0,2.73-.49l22.4-8.14,4.74,11.07-16.6,6A8,8,0,0,0,120,112a7.91,7.91,0,0,0,2.73-.49l17.6-6.4a40.24,40.24,0,0,0,7.68,10l-14.74,5.36A8,8,0,0,0,136,136a8.14,8.14,0,0,0,2.73-.48l28-10.18,56.87,18.95A24,24,0,0,1,238.93,160H40V75.53ZM40,192h0V176H240v16Z"></path>
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