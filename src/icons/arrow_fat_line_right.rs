//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn ArrowFatLineRight(
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
                <path d="M240.49,119.51l-96-96A12,12,0,0,0,124,32V68H80A12,12,0,0,0,68,80v96a12,12,0,0,0,12,12h44v36a12,12,0,0,0,20.49,8.49l96-96A12,12,0,0,0,240.49,119.51ZM148,195V176a12,12,0,0,0-12-12H92V92h44a12,12,0,0,0,12-12V61l67,67ZM52,80v96a12,12,0,0,1-24,0V80a12,12,0,0,1,24,0Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M136,224V176H72V80h64V32l96,96Z" opacity="0.2"></path>
    <path d="M237.66,122.34l-96-96A8,8,0,0,0,128,32V72H72a8,8,0,0,0-8,8v96a8,8,0,0,0,8,8h56v40a8,8,0,0,0,13.66,5.66l96-96A8,8,0,0,0,237.66,122.34ZM144,204.69V176a8,8,0,0,0-8-8H80V88h56a8,8,0,0,0,8-8V51.31L220.69,128ZM48,80v96a8,8,0,0,1-16,0V80a8,8,0,0,1,16,0Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M48,80v96a8,8,0,0,1-16,0V80a8,8,0,0,1,16,0Zm189.66,42.34-96-96A8,8,0,0,0,128,32V72H72a8,8,0,0,0-8,8v96a8,8,0,0,0,8,8h56v40a8,8,0,0,0,13.66,5.66l96-96A8,8,0,0,0,237.66,122.34Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M236.24,123.76l-96-96A6,6,0,0,0,130,32V74H72a6,6,0,0,0-6,6v96a6,6,0,0,0,6,6h58v42a6,6,0,0,0,10.24,4.24l96-96A6,6,0,0,0,236.24,123.76ZM142,209.51V176a6,6,0,0,0-6-6H78V86h58a6,6,0,0,0,6-6V46.49L223.51,128ZM46,80v96a6,6,0,0,1-12,0V80a6,6,0,0,1,12,0Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M237.66,122.34l-96-96A8,8,0,0,0,128,32V72H72a8,8,0,0,0-8,8v96a8,8,0,0,0,8,8h56v40a8,8,0,0,0,13.66,5.66l96-96A8,8,0,0,0,237.66,122.34ZM144,204.69V176a8,8,0,0,0-8-8H80V88h56a8,8,0,0,0,8-8V51.31L220.69,128ZM48,80v96a8,8,0,0,1-16,0V80a8,8,0,0,1,16,0Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M234.83,125.17l-96-96A4,4,0,0,0,132,32V76H72a4,4,0,0,0-4,4v96a4,4,0,0,0,4,4h60v44a4,4,0,0,0,2.47,3.7,4,4,0,0,0,4.36-.87l96-96A4,4,0,0,0,234.83,125.17ZM140,214.34V176a4,4,0,0,0-4-4H76V84h60a4,4,0,0,0,4-4V41.66L226.34,128ZM44,80v96a4,4,0,0,1-8,0V80a4,4,0,0,1,8,0Z"></path>
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
