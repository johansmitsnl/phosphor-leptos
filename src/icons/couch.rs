//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="objects", feature ="commerce"))]
#[component]
pub fn Couch(
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
                <path d="M16,100V72A16,16,0,0,1,32,56h84a4,4,0,0,1,4,4v76H64a32,32,0,0,0-32-32H20A4,4,0,0,1,16,100Zm208,4h12a4,4,0,0,0,4-4V72a16,16,0,0,0-16-16H140a4,4,0,0,0-4,4v76h56A32,32,0,0,1,224,104Zm8,16h-8a16,16,0,0,0-16,16v8a8,8,0,0,1-8,8H56a8,8,0,0,1-8-8v-8a16,16,0,0,0-16-16H24A16,16,0,0,0,8,136v32a16,16,0,0,0,16,16h8v15.73A8.18,8.18,0,0,0,39.47,208,8,8,0,0,0,48,200V184H208v15.73a8.17,8.17,0,0,0,7.47,8.25,8,8,0,0,0,8.53-8V184h8a16,16,0,0,0,16-16V136A16,16,0,0,0,232,120Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M240,120v48a8,8,0,0,1-8,8H24a8,8,0,0,1-8-8V120a8,8,0,0,1,8-8V72a8,8,0,0,1,8-8H224a8,8,0,0,1,8,8v40A8,8,0,0,1,240,120Z"
        opacity="0.2"
    ></path>
    <path d="M240,106.17V72a16,16,0,0,0-16-16H32A16,16,0,0,0,16,72v34.17A16,16,0,0,0,8,120v48a16,16,0,0,0,16,16h8v16a8,8,0,0,0,16,0V184H208v16a8,8,0,0,0,16,0V184h8a16,16,0,0,0,16-16V120A16,16,0,0,0,240,106.17ZM224,104h-8a16,16,0,0,0-16,16v16H136V72h88ZM120,72v64H56V120a16,16,0,0,0-16-16H32V72Zm112,96H24V120H40v24a8,8,0,0,0,8,8H208a8,8,0,0,0,8-8V120h16Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M236,108.7V72a12,12,0,0,0-12-12H32A12,12,0,0,0,20,72v36.7A12,12,0,0,0,12,120v48a12,12,0,0,0,12,12H36v20a4,4,0,0,0,8,0V180H212v20a4,4,0,0,0,8,0V180h12a12,12,0,0,0,12-12V120A12,12,0,0,0,236,108.7ZM228,72v36H216a12,12,0,0,0-12,12v20H132V68h92A4,4,0,0,1,228,72ZM32,68h92v72H52V120a12,12,0,0,0-12-12H28V72A4,4,0,0,1,32,68ZM236,168a4,4,0,0,1-4,4H24a4,4,0,0,1-4-4V120a4,4,0,0,1,4-4H40a4,4,0,0,1,4,4v24a4,4,0,0,0,4,4H208a4,4,0,0,0,4-4V120a4,4,0,0,1,4-4h16a4,4,0,0,1,4,4Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M244,104V72a20,20,0,0,0-20-20H32A20,20,0,0,0,12,72v32a20,20,0,0,0-8,16v48a20,20,0,0,0,20,20h4v12a12,12,0,0,0,24,0V188H204v12a12,12,0,0,0,24,0V188h4a20,20,0,0,0,20-20V120A20,20,0,0,0,244,104Zm-24-4H208a20,20,0,0,0-20,20v4H140V76h80ZM116,76v48H68v-4a20,20,0,0,0-20-20H36V76Zm112,88H28V124H44v12a12,12,0,0,0,12,12H200a12,12,0,0,0,12-12V124h16Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M238,107.37V72a14,14,0,0,0-14-14H32A14,14,0,0,0,18,72v35.37A14,14,0,0,0,10,120v48a14,14,0,0,0,14,14H34v18a6,6,0,0,0,12,0V182H210v18a6,6,0,0,0,12,0V182h10a14,14,0,0,0,14-14V120A14,14,0,0,0,238,107.37ZM226,72v34H216a14,14,0,0,0-14,14v18H134V70h90A2,2,0,0,1,226,72ZM32,70h90v68H54V120a14,14,0,0,0-14-14H30V72A2,2,0,0,1,32,70Zm202,98a2,2,0,0,1-2,2H24a2,2,0,0,1-2-2V120a2,2,0,0,1,2-2H40a2,2,0,0,1,2,2v24a6,6,0,0,0,6,6H208a6,6,0,0,0,6-6V120a2,2,0,0,1,2-2h16a2,2,0,0,1,2,2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M240,106.17V72a16,16,0,0,0-16-16H32A16,16,0,0,0,16,72v34.17A16,16,0,0,0,8,120v48a16,16,0,0,0,16,16h8v16a8,8,0,0,0,16,0V184H208v16a8,8,0,0,0,16,0V184h8a16,16,0,0,0,16-16V120A16,16,0,0,0,240,106.17ZM224,104h-8a16,16,0,0,0-16,16v16H136V72h88ZM120,72v64H56V120a16,16,0,0,0-16-16H32V72Zm112,96H24V120H40v24a8,8,0,0,0,8,8H208a8,8,0,0,0,8-8V120h16Z"></path>
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