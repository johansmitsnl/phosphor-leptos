//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "finance"))]
#[component]
pub fn CurrencyDollar(
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
                <path d="M160,152a16,16,0,0,1-16,16h-8V136h8A16,16,0,0,1,160,152Zm72-24A104,104,0,1,1,128,24,104.11,104.11,0,0,1,232,128Zm-56,24a32,32,0,0,0-32-32h-8V88h4a16,16,0,0,1,16,16,8,8,0,0,0,16,0,32,32,0,0,0-32-32h-4V64a8,8,0,0,0-16,0v8h-4a32,32,0,0,0,0,64h4v32h-8a16,16,0,0,1-16-16,8,8,0,0,0-16,0,32,32,0,0,0,32,32h8v8a8,8,0,0,0,16,0v-8h8A32,32,0,0,0,176,152Zm-76-48a16,16,0,0,0,16,16h4V88h-4A16,16,0,0,0,100,104Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M192,168a40,40,0,0,1-40,40H128V128h24A40,40,0,0,1,192,168ZM112,48a40,40,0,0,0,0,80h16V48Z"
        opacity="0.2"
    ></path>
    <path d="M152,120H136V56h8a32,32,0,0,1,32,32,8,8,0,0,0,16,0,48.05,48.05,0,0,0-48-48h-8V24a8,8,0,0,0-16,0V40h-8a48,48,0,0,0,0,96h8v64H104a32,32,0,0,1-32-32,8,8,0,0,0-16,0,48.05,48.05,0,0,0,48,48h16v16a8,8,0,0,0,16,0V216h16a48,48,0,0,0,0-96Zm-40,0a32,32,0,0,1,0-64h8v64Zm40,80H136V136h16a32,32,0,0,1,0,64Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M152,124H132V52h12a36,36,0,0,1,36,36,4,4,0,0,0,8,0,44.05,44.05,0,0,0-44-44H132V24a4,4,0,0,0-8,0V44H112a44,44,0,0,0,0,88h12v72H104a36,36,0,0,1-36-36,4,4,0,0,0-8,0,44.05,44.05,0,0,0,44,44h20v20a4,4,0,0,0,8,0V212h20a44,44,0,0,0,0-88Zm-40,0a36,36,0,0,1,0-72h12v72Zm40,80H132V132h20a36,36,0,0,1,0,72Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M152,116H140V60h4a28,28,0,0,1,28,28,12,12,0,0,0,24,0,52.06,52.06,0,0,0-52-52h-4V24a12,12,0,0,0-24,0V36h-4a52,52,0,0,0,0,104h4v56H104a28,28,0,0,1-28-28,12,12,0,0,0-24,0,52.06,52.06,0,0,0,52,52h12v12a12,12,0,0,0,24,0V220h12a52,52,0,0,0,0-104Zm-40,0a28,28,0,0,1,0-56h4v56Zm40,80H140V140h12a28,28,0,0,1,0,56Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M152,122H134V54h10a34,34,0,0,1,34,34,6,6,0,0,0,12,0,46.06,46.06,0,0,0-46-46H134V24a6,6,0,0,0-12,0V42H112a46,46,0,0,0,0,92h10v68H104a34,34,0,0,1-34-34,6,6,0,0,0-12,0,46.06,46.06,0,0,0,46,46h18v18a6,6,0,0,0,12,0V214h18a46,46,0,0,0,0-92Zm-40,0a34,34,0,0,1,0-68h10v68Zm40,80H134V134h18a34,34,0,0,1,0,68Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M152,120H136V56h8a32,32,0,0,1,32,32,8,8,0,0,0,16,0,48.05,48.05,0,0,0-48-48h-8V24a8,8,0,0,0-16,0V40h-8a48,48,0,0,0,0,96h8v64H104a32,32,0,0,1-32-32,8,8,0,0,0-16,0,48.05,48.05,0,0,0,48,48h16v16a8,8,0,0,0,16,0V216h16a48,48,0,0,0,0-96Zm-40,0a32,32,0,0,1,0-64h8v64Zm40,80H136V136h16a32,32,0,0,1,0,64Z"></path>
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
