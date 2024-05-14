//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "objects", feature = "development"))]
#[component]
pub fn HardHat(
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
                <path d="M152,152H104V40a16,16,0,0,1,16-16h16a16,16,0,0,1,16,16Zm72,16H32a16,16,0,0,0-16,16v8a16,16,0,0,0,16,16H224a16,16,0,0,0,16-16v-8A16,16,0,0,0,224,168Zm0-20V136a96.44,96.44,0,0,0-50.11-84.31A4,4,0,0,0,168,55.22V152h52A4,4,0,0,0,224,148ZM36,152H88V55.22a4,4,0,0,0-5.89-3.53A96.44,96.44,0,0,0,32,136v12A4,4,0,0,0,36,152Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M104,51.31V160H40V136A88,88,0,0,1,104,51.31Zm48,0V160h64V136A88,88,0,0,0,152,51.31Z"
        opacity="0.2"
    ></path>
    <path d="M224,152V136a96.37,96.37,0,0,0-64-90.51V40a16,16,0,0,0-16-16H112A16,16,0,0,0,96,40v5.49A96.37,96.37,0,0,0,32,136v16a16,16,0,0,0-16,16v24a16,16,0,0,0,16,16H224a16,16,0,0,0,16-16V168A16,16,0,0,0,224,152Zm-16-16v16H160V62.67A80.36,80.36,0,0,1,208,136ZM144,40V152H112V40ZM48,136A80.36,80.36,0,0,1,96,62.67V152H48Zm176,56H32V168H224v24Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M224,156h-4V136a92.35,92.35,0,0,0-64-87.65V40a12,12,0,0,0-12-12H112a12,12,0,0,0-12,12v8.35A92.35,92.35,0,0,0,36,136v20H32a12,12,0,0,0-12,12v24a12,12,0,0,0,12,12H224a12,12,0,0,0,12-12V168A12,12,0,0,0,224,156Zm-12-20v20H156V56.8A84.33,84.33,0,0,1,212,136ZM112,36h32a4,4,0,0,1,4,4V156H108V40A4,4,0,0,1,112,36ZM44,136a84.33,84.33,0,0,1,56-79.2V156H44Zm184,56a4,4,0,0,1-4,4H32a4,4,0,0,1-4-4V168a4,4,0,0,1,4-4H224a4,4,0,0,1,4,4Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M228,148.4V136a100.41,100.41,0,0,0-64-93.3V40a20,20,0,0,0-20-20H112A20,20,0,0,0,92,40v2.7A100.41,100.41,0,0,0,28,136v12.4A20,20,0,0,0,12,168v24a20,20,0,0,0,20,20H224a20,20,0,0,0,20-20V168A20,20,0,0,0,228,148.4ZM204,136v12H164V69.07A76.35,76.35,0,0,1,204,136ZM140,44V148H116V44ZM92,69.07V148H52V136A76.35,76.35,0,0,1,92,69.07ZM220,188H36V172H220Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M224,154h-2V136a94.37,94.37,0,0,0-64-89.1V40a14,14,0,0,0-14-14H112A14,14,0,0,0,98,40v6.9A94.37,94.37,0,0,0,34,136v18H32a14,14,0,0,0-14,14v24a14,14,0,0,0,14,14H224a14,14,0,0,0,14-14V168A14,14,0,0,0,224,154Zm-14-18v18H158V59.68A82.33,82.33,0,0,1,210,136ZM112,38h32a2,2,0,0,1,2,2V154H110V40A2,2,0,0,1,112,38ZM46,136A82.33,82.33,0,0,1,98,59.68V154H46Zm180,56a2,2,0,0,1-2,2H32a2,2,0,0,1-2-2V168a2,2,0,0,1,2-2H224a2,2,0,0,1,2,2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M224,152V136a96.37,96.37,0,0,0-64-90.51V40a16,16,0,0,0-16-16H112A16,16,0,0,0,96,40v5.49A96.37,96.37,0,0,0,32,136v16a16,16,0,0,0-16,16v24a16,16,0,0,0,16,16H224a16,16,0,0,0,16-16V168A16,16,0,0,0,224,152Zm-16-16v16H160V62.67A80.36,80.36,0,0,1,208,136ZM144,40V152H112V40ZM48,136A80.36,80.36,0,0,1,96,62.67V152H48Zm176,56H32V168H224v24Z"></path>
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
