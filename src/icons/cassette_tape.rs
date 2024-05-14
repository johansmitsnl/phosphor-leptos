//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "media", feature = "objects"))]
#[component]
pub fn CassetteTape(
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
                <path d="M156.3,96a31.92,31.92,0,0,0,0,32H99.7a31.92,31.92,0,0,0,0-32ZM72,96a16,16,0,1,0,16,16A16,16,0,0,0,72,96ZM240,64V192a16,16,0,0,1-16,16H32a16,16,0,0,1-16-16V64A16,16,0,0,1,32,48H224A16,16,0,0,1,240,64ZM186,192l-15.6-20.8A8,8,0,0,0,164,168H92a8,8,0,0,0-6.4,3.2L70,192Zm30-80a32,32,0,0,0-32-32H72a32,32,0,0,0,0,64H184A32,32,0,0,0,216,112ZM184,96a16,16,0,1,0,16,16A16,16,0,0,0,184,96Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M168,168l24,32H64l24-32Zm8-80a24,24,0,1,0,24,24A24,24,0,0,0,176,88Zm-72,24a24,24,0,1,0-24,24A24,24,0,0,0,104,112Z"
        opacity="0.2"
    ></path>
    <path d="M224,48H32A16,16,0,0,0,16,64V192a16,16,0,0,0,16,16H224a16,16,0,0,0,16-16V64A16,16,0,0,0,224,48ZM80,192l12-16h72l12,16Zm144,0H196l-21.6-28.8A8,8,0,0,0,168,160H88a8,8,0,0,0-6.4,3.2L60,192H32V64H224V192ZM176,80H80a32,32,0,0,0,0,64h96a32,32,0,0,0,0-64ZM148.3,96a31.92,31.92,0,0,0,0,32H107.7a31.92,31.92,0,0,0,0-32ZM64,112a16,16,0,1,1,16,16A16,16,0,0,1,64,112Zm112,16a16,16,0,1,1,16-16A16,16,0,0,1,176,128Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M224,52H32A12,12,0,0,0,20,64V192a12,12,0,0,0,12,12H224a12,12,0,0,0,12-12V64A12,12,0,0,0,224,52ZM72,196l18-24h76l18,24Zm156-4a4,4,0,0,1-4,4H194l-22.8-30.4A4,4,0,0,0,168,164H88a4,4,0,0,0-3.2,1.6L62,196H32a4,4,0,0,1-4-4V64a4,4,0,0,1,4-4H224a4,4,0,0,1,4,4ZM176,84H80a28,28,0,0,0,0,56h96a28,28,0,0,0,0-56ZM60,112a20,20,0,1,1,20,20A20,20,0,0,1,60,112Zm39.57,20a27.94,27.94,0,0,0,0-40h56.86a27.94,27.94,0,0,0,0,40ZM176,132a20,20,0,1,1,20-20A20,20,0,0,1,176,132Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M224,44H32A20,20,0,0,0,12,64V192a20,20,0,0,0,20,20H224a20,20,0,0,0,20-20V64A20,20,0,0,0,224,44Zm-4,144H183l-12.6-16.8A8,8,0,0,0,164,168H92a8,8,0,0,0-6.4,3.2L73,188H36V68H220ZM82,152h92a34,34,0,0,0,0-68H82a34,34,0,0,0,0,68Zm0-44a10,10,0,1,1-10,10A10,10,0,0,1,82,108Zm102,10a10,10,0,1,1-10-10A10,10,0,0,1,184,118Zm-42.5,10h-27a34.08,34.08,0,0,0,0-20h27a34.08,34.08,0,0,0,0,20Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M224,50H32A14,14,0,0,0,18,64V192a14,14,0,0,0,14,14H224a14,14,0,0,0,14-14V64A14,14,0,0,0,224,50ZM76,194l15-20h74l15,20Zm150-2a2,2,0,0,1-2,2H195l-22.2-29.6A6,6,0,0,0,168,162H88a6,6,0,0,0-4.8,2.4L61,194H32a2,2,0,0,1-2-2V64a2,2,0,0,1,2-2H224a2,2,0,0,1,2,2ZM176,82H80a30,30,0,0,0,0,60h96a30,30,0,0,0,0-60ZM152,94a29.92,29.92,0,0,0,0,36H104a29.92,29.92,0,0,0,0-36ZM62,112a18,18,0,1,1,18,18A18,18,0,0,1,62,112Zm114,18a18,18,0,1,1,18-18A18,18,0,0,1,176,130Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M224,48H32A16,16,0,0,0,16,64V192a16,16,0,0,0,16,16H224a16,16,0,0,0,16-16V64A16,16,0,0,0,224,48ZM80,192l12-16h72l12,16Zm144,0H196l-21.6-28.8A8,8,0,0,0,168,160H88a8,8,0,0,0-6.4,3.2L60,192H32V64H224V192ZM176,80H80a32,32,0,0,0,0,64h96a32,32,0,0,0,0-64ZM148.3,96a31.92,31.92,0,0,0,0,32H107.7a31.92,31.92,0,0,0,0-32ZM64,112a16,16,0,1,1,16,16A16,16,0,0,1,64,112Zm112,16a16,16,0,1,1,16-16A16,16,0,0,1,176,128Z"></path>
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
