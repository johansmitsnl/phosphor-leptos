//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="objects"))]
#[component]
pub fn DeviceTablet(
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
                <path d="M192,24H64A24,24,0,0,0,40,48V208a24,24,0,0,0,24,24H192a24,24,0,0,0,24-24V48A24,24,0,0,0,192,24ZM64,40H192a8,8,0,0,1,8,8v8H56V48A8,8,0,0,1,64,40ZM192,216H64a8,8,0,0,1-8-8v-8H200v8A8,8,0,0,1,192,216Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M208,64V192H48V64Z" opacity="0.2"></path>
    <path d="M192,24H64A24,24,0,0,0,40,48V208a24,24,0,0,0,24,24H192a24,24,0,0,0,24-24V48A24,24,0,0,0,192,24ZM56,72H200V184H56Zm8-32H192a8,8,0,0,1,8,8v8H56V48A8,8,0,0,1,64,40ZM192,216H64a8,8,0,0,1-8-8v-8H200v8A8,8,0,0,1,192,216Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M192,28H64A20,20,0,0,0,44,48V208a20,20,0,0,0,20,20H192a20,20,0,0,0,20-20V48A20,20,0,0,0,192,28ZM52,68H204V188H52ZM64,36H192a12,12,0,0,1,12,12V60H52V48A12,12,0,0,1,64,36ZM192,220H64a12,12,0,0,1-12-12V196H204v12A12,12,0,0,1,192,220Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M192,20H64A28,28,0,0,0,36,48V208a28,28,0,0,0,28,28H192a28,28,0,0,0,28-28V48A28,28,0,0,0,192,20ZM60,84H196v88H60Zm4-40H192a4,4,0,0,1,4,4V60H60V48A4,4,0,0,1,64,44ZM192,212H64a4,4,0,0,1-4-4V196H196v12A4,4,0,0,1,192,212Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M192,26H64A22,22,0,0,0,42,48V208a22,22,0,0,0,22,22H192a22,22,0,0,0,22-22V48A22,22,0,0,0,192,26ZM54,70H202V186H54ZM64,38H192a10,10,0,0,1,10,10V58H54V48A10,10,0,0,1,64,38ZM192,218H64a10,10,0,0,1-10-10V198H202v10A10,10,0,0,1,192,218Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M192,24H64A24,24,0,0,0,40,48V208a24,24,0,0,0,24,24H192a24,24,0,0,0,24-24V48A24,24,0,0,0,192,24ZM56,72H200V184H56Zm8-32H192a8,8,0,0,1,8,8v8H56V48A8,8,0,0,1,64,40ZM192,216H64a8,8,0,0,1-8-8v-8H200v8A8,8,0,0,1,192,216Z"></path>
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