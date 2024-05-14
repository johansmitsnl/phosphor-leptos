//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="commerce", feature ="objects"))]
#[component]
pub fn Pipe(
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
                <path d="M232,104H208V56h24a8,8,0,0,0,0-16H205.83A16,16,0,0,0,192,32H176a16,16,0,0,0-13.83,8H144A104.11,104.11,0,0,0,40,144v18.16A16,16,0,0,0,32,176v16a16,16,0,0,0,8,13.84V232a8,8,0,0,0,16,0V208h48v24a8,8,0,0,0,16,0V205.84A16,16,0,0,0,128,192V176a16,16,0,0,0-8-13.84V144a24,24,0,0,1,24-24h18.17A16,16,0,0,0,176,128h16a16,16,0,0,0,13.83-8H232a8,8,0,0,0,0-16ZM112,192H48V176h64Zm64-80V48h16v63.8c0,.07,0,.13,0,.2Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M168,48v64H144a32,32,0,0,0-32,32v24H48V144a96,96,0,0,1,96-96Z" opacity="0.2"></path>
    <path d="M232,104H208V56h24a8,8,0,0,0,0-16H205.83A16,16,0,0,0,192,32H176a16,16,0,0,0-13.83,8H144A104.11,104.11,0,0,0,40,144v18.16A16,16,0,0,0,32,176v16a16,16,0,0,0,8,13.84V232a8,8,0,0,0,16,0V208h48v24a8,8,0,0,0,16,0V205.84A16,16,0,0,0,128,192V176a16,16,0,0,0-8-13.84V144a24,24,0,0,1,24-24h18.17A16,16,0,0,0,176,128h16a16,16,0,0,0,13.83-8H232a8,8,0,0,0,0-16ZM112,176v16H48V176Zm-8-32v16H56V144a88.1,88.1,0,0,1,88-88h16v48H144A40,40,0,0,0,104,144Zm72-32V48h16v63.8c0,.07,0,.13,0,.2Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M232,108H204V52h28a4,4,0,0,0,0-8H203.3A12,12,0,0,0,192,36H176a12,12,0,0,0-11.3,8H144A100.11,100.11,0,0,0,44,144v20.7A12,12,0,0,0,36,176v16a12,12,0,0,0,8,11.3V232a4,4,0,0,0,8,0V204h56v28a4,4,0,0,0,8,0V203.3a12,12,0,0,0,8-11.3V176a12,12,0,0,0-8-11.3V144a28,28,0,0,1,28-28h20.7a12,12,0,0,0,11.3,8h16a12,12,0,0,0,11.3-8H232a4,4,0,0,0,0-8ZM116,192a4,4,0,0,1-4,4H48a4,4,0,0,1-4-4V176a4,4,0,0,1,4-4h64a4,4,0,0,1,4,4Zm-8-48v20H52V144a92.1,92.1,0,0,1,92-92h20v56H144A36,36,0,0,0,108,144Zm84-28H176a4,4,0,0,1-4-4V48a4,4,0,0,1,4-4h16a4,4,0,0,1,4,4v64A4,4,0,0,1,192,116Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M232,100H216V60h16a12,12,0,0,0,0-24H212a20,20,0,0,0-16-8H172a20,20,0,0,0-16,8H144A108.12,108.12,0,0,0,36,144v12a20,20,0,0,0-8,16v24a20,20,0,0,0,8,16v20a12,12,0,0,0,24,0V216h40v16a12,12,0,0,0,24,0V212a20,20,0,0,0,8-16V172a20,20,0,0,0-8-16V144a20,20,0,0,1,20-20h12a20,20,0,0,0,16,8h24a20,20,0,0,0,16-8h20a12,12,0,0,0,0-24ZM108,176v16H52V176Zm-8-32v8H60v-8a84.09,84.09,0,0,1,84-84h8v40h-8A44.05,44.05,0,0,0,100,144Zm76-92h16v56H176Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M232,106H206V54h26a6,6,0,0,0,0-12H204.63A14,14,0,0,0,192,34H176a14,14,0,0,0-12.63,8H144A102.12,102.12,0,0,0,42,144v19.37A14,14,0,0,0,34,176v16a14,14,0,0,0,8,12.63V232a6,6,0,0,0,12,0V206h52v26a6,6,0,0,0,12,0V204.63A14,14,0,0,0,126,192V176a14,14,0,0,0-8-12.63V144a26,26,0,0,1,26-26h19.37A14,14,0,0,0,176,126h16a14,14,0,0,0,12.63-8H232a6,6,0,0,0,0-12ZM112,174a2,2,0,0,1,2,2v16a2,2,0,0,1-2,2H48a2,2,0,0,1-2-2V176a2,2,0,0,1,2-2Zm-6-30v18H54V144a90.1,90.1,0,0,1,90-90h18v52H144A38,38,0,0,0,106,144Zm86-30H176a2,2,0,0,1-2-2V48a2,2,0,0,1,2-2h16a2,2,0,0,1,2,2v64A2,2,0,0,1,192,114Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M232,104H208V56h24a8,8,0,0,0,0-16H205.83A16,16,0,0,0,192,32H176a16,16,0,0,0-13.83,8H144A104.11,104.11,0,0,0,40,144v18.16A16,16,0,0,0,32,176v16a16,16,0,0,0,8,13.84V232a8,8,0,0,0,16,0V208h48v24a8,8,0,0,0,16,0V205.84A16,16,0,0,0,128,192V176a16,16,0,0,0-8-13.84V144a24,24,0,0,1,24-24h18.17A16,16,0,0,0,176,128h16a16,16,0,0,0,13.83-8H232a8,8,0,0,0,0-16ZM112,176v16H48V176Zm-8-32v16H56V144a88.1,88.1,0,0,1,88-88h16v48H144A40,40,0,0,0,104,144Zm72-32V48h16v63.8c0,.07,0,.13,0,.2Z"></path>
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