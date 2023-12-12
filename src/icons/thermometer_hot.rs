//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn ThermometerHot(
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
                <path d="M174.12,82.81a12,12,0,0,1,3.07-16.69c11.86-8.18,29.76-8.18,41.62,0,3.63,2.5,10.75,2.5,14.38,0a12,12,0,0,1,13.62,19.76,38.34,38.34,0,0,1-41.62,0c-3.63-2.5-10.75-2.5-14.38,0A12,12,0,0,1,174.12,82.81Zm59.07,23.31c-3.63,2.5-10.75,2.5-14.38,0-11.86-8.18-29.76-8.18-41.62,0a12,12,0,1,0,13.62,19.76c3.63-2.5,10.75-2.5,14.38,0a38.34,38.34,0,0,0,41.62,0,12,12,0,0,0-13.62-19.76ZM160,150.69a64,64,0,1,1-104,0V56a52,52,0,0,1,104,0ZM148,188a40,40,0,0,0-9.23-25.55,12,12,0,0,1-2.77-7.68V56a28,28,0,0,0-56,0v98.78a12,12,0,0,1-2.91,7.83A40,40,0,1,0,148,188Zm-28-20.78V56a12,12,0,0,0-24,0V167.22a24,24,0,1,0,24,0Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M152,138V48a32,32,0,0,0-64,0v90a56,56,0,1,0,64,0Zm-32,70a24,24,0,1,1,24-24A24,24,0,0,1,120,208Z"
        opacity="0.2"
    ></path>
    <path d="M177.41,80.54a8,8,0,0,1,2.05-11.12c10.4-7.18,26.68-7.18,37.08,0,5,3.47,13.88,3.47,18.92,0a8,8,0,0,1,9.08,13.16,34.64,34.64,0,0,1-37.08,0c-5-3.47-13.88-3.47-18.92,0A8,8,0,0,1,177.41,80.54Zm58.05,20.88c-5,3.47-13.88,3.47-18.92,0-10.4-7.18-26.68-7.18-37.08,0a8,8,0,1,0,9.08,13.16c5-3.47,13.88-3.47,18.92,0a34.64,34.64,0,0,0,37.08,0,8,8,0,0,0-9.08-13.16ZM152,184a32,32,0,1,1-40-31V48a8,8,0,0,1,16,0V153A32.06,32.06,0,0,1,152,184Zm-16,0a16,16,0,1,0-16,16A16,16,0,0,0,136,184Zm48,0A64,64,0,1,1,80,134V48a40,40,0,0,1,80,0v86A64.08,64.08,0,0,1,184,184Zm-16,0a48.08,48.08,0,0,0-20.58-39.4A8,8,0,0,1,144,138V48a24,24,0,0,0-48,0v90a8,8,0,0,1-3.42,6.56A48,48,0,1,0,168,184Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M177.41,80.54a8,8,0,0,1,2.05-11.12c10.4-7.18,26.68-7.18,37.08,0,5,3.47,13.88,3.47,18.92,0a8,8,0,0,1,9.08,13.16,34.64,34.64,0,0,1-37.08,0c-5-3.47-13.88-3.47-18.92,0A8,8,0,0,1,177.41,80.54Zm58.05,20.88c-5,3.47-13.88,3.47-18.92,0-10.4-7.18-26.68-7.18-37.08,0a8,8,0,1,0,9.08,13.16c5-3.47,13.88-3.47,18.92,0a34.64,34.64,0,0,0,37.08,0,8,8,0,0,0-9.08-13.16ZM176,192a56,56,0,1,1-88-45.92V40a32,32,0,0,1,64,0V146.08A56,56,0,0,1,176,192ZM136,40a16,16,0,0,0-32,0V80h32Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M179.06,79.41a6,6,0,0,1,1.53-8.35c9.76-6.73,25.06-6.73,34.82,0,5.64,3.89,15.54,3.89,21.18,0a6,6,0,1,1,6.82,9.88,32.54,32.54,0,0,1-34.82,0c-5.64-3.89-15.54-3.89-21.18,0A6,6,0,0,1,179.06,79.41Zm57.53,23.65c-5.64,3.89-15.54,3.89-21.18,0-9.76-6.73-25.06-6.73-34.82,0a6,6,0,1,0,6.82,9.88c5.64-3.89,15.54-3.89,21.18,0a32.54,32.54,0,0,0,34.82,0,6,6,0,0,0-6.82-9.88ZM150,184a30,30,0,1,1-36-29.4V48a6,6,0,0,1,12,0V154.6A30.05,30.05,0,0,1,150,184Zm-12,0a18,18,0,1,0-18,18A18,18,0,0,0,138,184Zm44,0A62,62,0,1,1,82,135V48a38,38,0,0,1,76,0v87A62.06,62.06,0,0,1,182,184Zm-12,0a50.07,50.07,0,0,0-21.43-41A6,6,0,0,1,146,138V48a26,26,0,0,0-52,0v90A6,6,0,0,1,91.43,143,50,50,0,1,0,170,184Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M177.41,80.54a8,8,0,0,1,2.05-11.12c10.4-7.18,26.68-7.18,37.08,0,5,3.47,13.88,3.47,18.92,0a8,8,0,0,1,9.08,13.16,34.64,34.64,0,0,1-37.08,0c-5-3.47-13.88-3.47-18.92,0A8,8,0,0,1,177.41,80.54Zm58.05,20.88c-5,3.47-13.88,3.47-18.92,0-10.4-7.18-26.68-7.18-37.08,0a8,8,0,1,0,9.08,13.16c5-3.47,13.88-3.47,18.92,0a34.64,34.64,0,0,0,37.08,0,8,8,0,0,0-9.08-13.16ZM152,184a32,32,0,1,1-40-31V48a8,8,0,0,1,16,0V153A32.06,32.06,0,0,1,152,184Zm-16,0a16,16,0,1,0-16,16A16,16,0,0,0,136,184Zm48,0A64,64,0,1,1,80,134V48a40,40,0,0,1,80,0v86A64.08,64.08,0,0,1,184,184Zm-16,0a48.08,48.08,0,0,0-20.58-39.4A8,8,0,0,1,144,138V48a24,24,0,0,0-48,0v90a8,8,0,0,1-3.42,6.56A48,48,0,1,0,168,184Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M180.71,78.27a4,4,0,0,1,1-5.56c9.12-6.3,23.42-6.3,32.54,0,6.36,4.38,17.1,4.38,23.46,0a4,4,0,0,1,4.54,6.58,30.4,30.4,0,0,1-32.54,0c-6.36-4.38-17.1-4.38-23.46,0A4,4,0,0,1,180.71,78.27Zm57,26.44c-6.36,4.38-17.1,4.38-23.46,0-9.12-6.3-23.42-6.3-32.54,0a4,4,0,1,0,4.54,6.58c6.36-4.38,17.1-4.38,23.46,0a30.4,30.4,0,0,0,32.54,0,4,4,0,0,0-4.54-6.58ZM148,184a28,28,0,1,1-32-27.71V48a4,4,0,0,1,8,0V156.29A28,28,0,0,1,148,184Zm-8,0a20,20,0,1,0-20,20A20,20,0,0,0,140,184Zm40,0a60,60,0,1,1-96-48V48a36,36,0,0,1,72,0v88A60.06,60.06,0,0,1,180,184Zm-8,0a52.06,52.06,0,0,0-22.29-42.68A4,4,0,0,1,148,138V48a28,28,0,0,0-56,0v90a4,4,0,0,1-1.71,3.28A52,52,0,1,0,172,184Z"></path>
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
