//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="commerce", feature ="finance", feature ="objects"))]
#[component]
pub fn TipJar(
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
                <path d="M184,48.81V32a16,16,0,0,0-16-16H88A16,16,0,0,0,72,32V48.81A40.05,40.05,0,0,0,40,88V200a40,40,0,0,0,40,40h96a40,40,0,0,0,40-40V88A40.05,40.05,0,0,0,184,48.81ZM120,32h16V48H120ZM88,32h16V48H88Zm48,152v8a8,8,0,0,1-16,0v-8h-8a8,8,0,0,1,0-16h24a8,8,0,0,0,0-16H120a24,24,0,0,1,0-48V96a8,8,0,0,1,16,0v8h8a8,8,0,0,1,0,16H120a8,8,0,0,0,0,16h16a24,24,0,0,1,0,48ZM168,48H152V32h16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,88V200a32,32,0,0,1-32,32H80a32,32,0,0,1-32-32V88A32,32,0,0,1,80,56h96A32,32,0,0,1,208,88Z"
        opacity="0.2"
    ></path>
    <path d="M184,48.81V32a16,16,0,0,0-16-16H88A16,16,0,0,0,72,32V48.81A40.05,40.05,0,0,0,40,88V200a40,40,0,0,0,40,40h96a40,40,0,0,0,40-40V88A40.05,40.05,0,0,0,184,48.81ZM168,48H152V32h16Zm-48,0V32h16V48ZM104,32V48H88V32Zm96,168a24,24,0,0,1-24,24H80a24,24,0,0,1-24-24V88A24,24,0,0,1,80,64h96a24,24,0,0,1,24,24Zm-40-40a24,24,0,0,1-24,24v8a8,8,0,0,1-16,0v-8h-8a8,8,0,0,1,0-16h24a8,8,0,0,0,0-16H120a24,24,0,0,1,0-48V96a8,8,0,0,1,16,0v8h8a8,8,0,0,1,0,16H120a8,8,0,0,0,0,16h16A24,24,0,0,1,160,160Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M180,52.23V32a12,12,0,0,0-12-12H88A12,12,0,0,0,76,32V52.23A36,36,0,0,0,44,88V200a36,36,0,0,0,36,36h96a36,36,0,0,0,36-36V88A36,36,0,0,0,180,52.23ZM148,28h20a4,4,0,0,1,4,4V52H148Zm-32,0h24V52H116ZM84,32a4,4,0,0,1,4-4h20V52H84ZM204,200a28,28,0,0,1-28,28H80a28,28,0,0,1-28-28V88A28,28,0,0,1,80,60h96a28,28,0,0,1,28,28Zm-48-40a20,20,0,0,1-20,20h-4v12a4,4,0,0,1-8,0V180H112a4,4,0,0,1,0-8h24a12,12,0,0,0,0-24H120a20,20,0,0,1,0-40h4V96a4,4,0,0,1,8,0v12h12a4,4,0,0,1,0,8H120a12,12,0,0,0,0,24h16A20,20,0,0,1,156,160Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M188,49.68V32a20,20,0,0,0-20-20H88A20,20,0,0,0,68,32V49.68A44.06,44.06,0,0,0,36,92V200a44.05,44.05,0,0,0,44,44h96a44.05,44.05,0,0,0,44-44V92A44.06,44.06,0,0,0,188,49.68ZM164,48H140V36h24ZM116,36V48H92V36Zm80,164a20,20,0,0,1-20,20H80a20,20,0,0,1-20-20V92A20,20,0,0,1,80,72h96a20,20,0,0,1,20,20Zm-28-38a30,30,0,0,1-28,29.93V196a12,12,0,0,1-24,0v-4h-8a12,12,0,0,1,0-24h30a6,6,0,0,0,0-12H118a30,30,0,0,1-2-59.93V96a12,12,0,0,1,24,0h8a12,12,0,0,1,0,24H118a6,6,0,0,0,0,12h20A30,30,0,0,1,168,162Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M182,50.48V32a14,14,0,0,0-14-14H88A14,14,0,0,0,74,32V50.48A38.05,38.05,0,0,0,42,88V200a38,38,0,0,0,38,38h96a38,38,0,0,0,38-38V88A38.05,38.05,0,0,0,182,50.48ZM170,32V50H150V30h18A2,2,0,0,1,170,32ZM118,50V30h20V50ZM88,30h18V50H86V32A2,2,0,0,1,88,30ZM202,200a26,26,0,0,1-26,26H80a26,26,0,0,1-26-26V88A26,26,0,0,1,80,62h96a26,26,0,0,1,26,26Zm-44-40a22,22,0,0,1-22,22h-2v10a6,6,0,0,1-12,0V182H112a6,6,0,0,1,0-12h24a10,10,0,0,0,0-20H120a22,22,0,0,1,0-44h2V96a6,6,0,0,1,12,0v10h10a6,6,0,0,1,0,12H120a10,10,0,0,0,0,20h16A22,22,0,0,1,158,160Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M184,48.81V32a16,16,0,0,0-16-16H88A16,16,0,0,0,72,32V48.81A40.05,40.05,0,0,0,40,88V200a40,40,0,0,0,40,40h96a40,40,0,0,0,40-40V88A40.05,40.05,0,0,0,184,48.81ZM168,48H152V32h16Zm-48,0V32h16V48ZM104,32V48H88V32Zm96,168a24,24,0,0,1-24,24H80a24,24,0,0,1-24-24V88A24,24,0,0,1,80,64h96a24,24,0,0,1,24,24Zm-40-40a24,24,0,0,1-24,24v8a8,8,0,0,1-16,0v-8h-8a8,8,0,0,1,0-16h24a8,8,0,0,0,0-16H120a24,24,0,0,1,0-48V96a8,8,0,0,1,16,0v8h8a8,8,0,0,1,0,16H120a8,8,0,0,0,0,16h16A24,24,0,0,1,160,160Z"></path>
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