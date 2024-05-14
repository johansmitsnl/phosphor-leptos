//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "system"))]
#[component]
pub fn TreeView(
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
                <path d="M160,136v-8H88v64a8,8,0,0,0,8,8h64v-8a16,16,0,0,1,16-16h32a16,16,0,0,1,16,16v32a16,16,0,0,1-16,16H176a16,16,0,0,1-16-16v-8H96a24,24,0,0,1-24-24V80H64A16,16,0,0,1,48,64V32A16,16,0,0,1,64,16H96a16,16,0,0,1,16,16V64A16,16,0,0,1,96,80H88v32h72v-8a16,16,0,0,1,16-16h32a16,16,0,0,1,16,16v32a16,16,0,0,1-16,16H176A16,16,0,0,1,160,136Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M104,32V64a8,8,0,0,1-8,8H64a8,8,0,0,1-8-8V32a8,8,0,0,1,8-8H96A8,8,0,0,1,104,32ZM208,96H176a8,8,0,0,0-8,8v32a8,8,0,0,0,8,8h32a8,8,0,0,0,8-8V104A8,8,0,0,0,208,96Zm0,88H176a8,8,0,0,0-8,8v32a8,8,0,0,0,8,8h32a8,8,0,0,0,8-8V192A8,8,0,0,0,208,184Z"
        opacity="0.2"
    ></path>
    <path d="M176,152h32a16,16,0,0,0,16-16V104a16,16,0,0,0-16-16H176a16,16,0,0,0-16,16v8H88V80h8a16,16,0,0,0,16-16V32A16,16,0,0,0,96,16H64A16,16,0,0,0,48,32V64A16,16,0,0,0,64,80h8V192a24,24,0,0,0,24,24h64v8a16,16,0,0,0,16,16h32a16,16,0,0,0,16-16V192a16,16,0,0,0-16-16H176a16,16,0,0,0-16,16v8H96a8,8,0,0,1-8-8V128h72v8A16,16,0,0,0,176,152ZM64,32H96V64H64ZM176,192h32v32H176Zm0-88h32v32H176Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M176,148h32a12,12,0,0,0,12-12V104a12,12,0,0,0-12-12H176a12,12,0,0,0-12,12v12H84V76H96a12,12,0,0,0,12-12V32A12,12,0,0,0,96,20H64A12,12,0,0,0,52,32V64A12,12,0,0,0,64,76H76V192a20,20,0,0,0,20,20h68v12a12,12,0,0,0,12,12h32a12,12,0,0,0,12-12V192a12,12,0,0,0-12-12H176a12,12,0,0,0-12,12v12H96a12,12,0,0,1-12-12V124h80v12A12,12,0,0,0,176,148ZM60,64V32a4,4,0,0,1,4-4H96a4,4,0,0,1,4,4V64a4,4,0,0,1-4,4H64A4,4,0,0,1,60,64ZM172,192a4,4,0,0,1,4-4h32a4,4,0,0,1,4,4v32a4,4,0,0,1-4,4H176a4,4,0,0,1-4-4Zm0-88a4,4,0,0,1,4-4h32a4,4,0,0,1,4,4v32a4,4,0,0,1-4,4H176a4,4,0,0,1-4-4Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M176,156h32a20,20,0,0,0,20-20V104a20,20,0,0,0-20-20H176a20,20,0,0,0-20,20v4H92V84h4a20,20,0,0,0,20-20V32A20,20,0,0,0,96,12H64A20,20,0,0,0,44,32V64A20,20,0,0,0,64,84h4V192a28,28,0,0,0,28,28h60v4a20,20,0,0,0,20,20h32a20,20,0,0,0,20-20V192a20,20,0,0,0-20-20H176a20,20,0,0,0-20,20v4H96a4,4,0,0,1-4-4V132h64v4A20,20,0,0,0,176,156ZM68,36H92V60H68ZM180,196h24v24H180Zm0-88h24v24H180Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M176,150h32a14,14,0,0,0,14-14V104a14,14,0,0,0-14-14H176a14,14,0,0,0-14,14v10H86V78H96a14,14,0,0,0,14-14V32A14,14,0,0,0,96,18H64A14,14,0,0,0,50,32V64A14,14,0,0,0,64,78H74V192a22,22,0,0,0,22,22h66v10a14,14,0,0,0,14,14h32a14,14,0,0,0,14-14V192a14,14,0,0,0-14-14H176a14,14,0,0,0-14,14v10H96a10,10,0,0,1-10-10V126h76v10A14,14,0,0,0,176,150ZM62,64V32a2,2,0,0,1,2-2H96a2,2,0,0,1,2,2V64a2,2,0,0,1-2,2H64A2,2,0,0,1,62,64ZM174,192a2,2,0,0,1,2-2h32a2,2,0,0,1,2,2v32a2,2,0,0,1-2,2H176a2,2,0,0,1-2-2Zm0-88a2,2,0,0,1,2-2h32a2,2,0,0,1,2,2v32a2,2,0,0,1-2,2H176a2,2,0,0,1-2-2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M176,152h32a16,16,0,0,0,16-16V104a16,16,0,0,0-16-16H176a16,16,0,0,0-16,16v8H88V80h8a16,16,0,0,0,16-16V32A16,16,0,0,0,96,16H64A16,16,0,0,0,48,32V64A16,16,0,0,0,64,80h8V192a24,24,0,0,0,24,24h64v8a16,16,0,0,0,16,16h32a16,16,0,0,0,16-16V192a16,16,0,0,0-16-16H176a16,16,0,0,0-16,16v8H96a8,8,0,0,1-8-8V128h72v8A16,16,0,0,0,176,152ZM64,32H96V64H64ZM176,192h32v32H176Zm0-88h32v32H176Z"></path>
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
