//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="media", feature ="system"))]
#[component]
pub fn ImagesSquare(
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
                <path d="M208,32H80A16,16,0,0,0,64,48V64H48A16,16,0,0,0,32,80V208a16,16,0,0,0,16,16H176a16,16,0,0,0,16-16V192h16a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM80,48H208v69.38l-16.7-16.7a16,16,0,0,0-22.62,0L93.37,176H80Zm96,160H48V80H64v96a16,16,0,0,0,16,16h96ZM104,88a16,16,0,1,1,16,16A16,16,0,0,1,104,88Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,40H80a8,8,0,0,0-8,8V176a8,8,0,0,0,8,8H96.69l77.65-77.66a8,8,0,0,1,11.32,0L216,136.69V48A8,8,0,0,0,208,40Zm-88,64a16,16,0,1,1,16-16A16,16,0,0,1,120,104Z"
        opacity="0.2"
    ></path>
    <path d="M208,32H80A16,16,0,0,0,64,48V64H48A16,16,0,0,0,32,80V208a16,16,0,0,0,16,16H176a16,16,0,0,0,16-16V192h16a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM80,48H208v69.38l-16.7-16.7a16,16,0,0,0-22.62,0L93.37,176H80Zm96,160H48V80H64v96a16,16,0,0,0,16,16h96Zm32-32H116l64-64,28,28v36Zm-88-64A24,24,0,1,0,96,88,24,24,0,0,0,120,112Zm0-32a8,8,0,1,1-8,8A8,8,0,0,1,120,80Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M208,36H80A12,12,0,0,0,68,48V68H48A12,12,0,0,0,36,80V208a12,12,0,0,0,12,12H176a12,12,0,0,0,12-12V188h20a12,12,0,0,0,12-12V48A12,12,0,0,0,208,36ZM76,48a4,4,0,0,1,4-4H208a4,4,0,0,1,4,4v79l-23.51-23.52a12,12,0,0,0-17,0L95,180H80a4,4,0,0,1-4-4ZM180,208a4,4,0,0,1-4,4H48a4,4,0,0,1-4-4V80a4,4,0,0,1,4-4H68V176a12,12,0,0,0,12,12H180Zm28-28H106.34l70.83-70.83a4,4,0,0,1,5.66,0L212,138.34V176A4,4,0,0,1,208,180Zm-88-72a20,20,0,1,0-20-20A20,20,0,0,0,120,108Zm0-32a12,12,0,1,1-12,12A12,12,0,0,1,120,76Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M208,28H88A20,20,0,0,0,68,48V60H48A20,20,0,0,0,28,80V208a20,20,0,0,0,20,20H176a20,20,0,0,0,20-20V188h12a20,20,0,0,0,20-20V48A20,20,0,0,0,208,28ZM92,52H204v59.72l-9.86-9.86a20,20,0,0,0-28.28,0L103.72,164H92Zm80,152H52V84H68v84a20,20,0,0,0,20,20h84Zm-34.34-40L180,121.66l24,24V164ZM108,88a20,20,0,1,1,20,20A20,20,0,0,1,108,88Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M208,34H80A14,14,0,0,0,66,48V66H48A14,14,0,0,0,34,80V208a14,14,0,0,0,14,14H176a14,14,0,0,0,14-14V190h18a14,14,0,0,0,14-14V48A14,14,0,0,0,208,34ZM78,48a2,2,0,0,1,2-2H208a2,2,0,0,1,2,2v74.2l-20.1-20.1a14,14,0,0,0-19.8,0L94.2,178H80a2,2,0,0,1-2-2ZM178,208a2,2,0,0,1-2,2H48a2,2,0,0,1-2-2V80a2,2,0,0,1,2-2H66v98a14,14,0,0,0,14,14h98Zm30-30H111.17l67.41-67.41a2,2,0,0,1,2.83,0L210,139.17V176A2,2,0,0,1,208,178Zm-88-68A22,22,0,1,0,98,88,22,22,0,0,0,120,110Zm0-32a10,10,0,1,1-10,10A10,10,0,0,1,120,78Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M208,32H80A16,16,0,0,0,64,48V64H48A16,16,0,0,0,32,80V208a16,16,0,0,0,16,16H176a16,16,0,0,0,16-16V192h16a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM80,48H208v69.38l-16.7-16.7a16,16,0,0,0-22.62,0L93.37,176H80Zm96,160H48V80H64v96a16,16,0,0,0,16,16h96Zm32-32H116l64-64,28,28v36Zm-88-64A24,24,0,1,0,96,88,24,24,0,0,0,120,112Zm0-32a8,8,0,1,1-8,8A8,8,0,0,1,120,80Z"></path>
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