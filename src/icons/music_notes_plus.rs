//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="media"))]
#[component]
pub fn MusicNotesPlus(
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
                <path d="M232,48a8,8,0,0,1-8,8H208V72a8,8,0,0,1-16,0V56H176a8,8,0,0,1,0-16h16V24a8,8,0,0,1,16,0V40h16A8,8,0,0,1,232,48ZM160.6,77.86l-6.76-6.76A32.85,32.85,0,0,1,144,49.33a31.87,31.87,0,0,1,1.67-11.66,4,4,0,0,0-4.76-5.14L78.06,48.25A8,8,0,0,0,72,56V166.1A36,36,0,1,0,52.42,232C72.25,231.77,88,215.13,88,195.3V102.25l70.74-17.69A4,4,0,0,0,160.6,77.86Zm50.11,24.31a31.91,31.91,0,0,1-7.14,1.63,4,4,0,0,0-3.57,4V134.1A36,36,0,1,0,180.42,200c19.83-.23,35.58-16.86,35.58-36.7V106A4,4,0,0,0,210.71,102.17Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,164a28,28,0,1,1-28-28A28,28,0,0,1,208,164ZM52,168a28,28,0,1,0,28,28A28,28,0,0,0,52,168Z"
        opacity="0.2"
    ></path>
    <path d="M232,48a8,8,0,0,1-8,8H208V72a8,8,0,0,1-16,0V56H176a8,8,0,0,1,0-16h16V24a8,8,0,0,1,16,0V40h16A8,8,0,0,1,232,48Zm-16,64v52a36,36,0,1,1-16-29.92V112a8,8,0,0,1,16,0Zm-16,52a20,20,0,1,0-20,20A20,20,0,0,0,200,164ZM88,110.25V196a36,36,0,1,1-16-29.92V56a8,8,0,0,1,6.06-7.76l56-14a8,8,0,0,1,3.88,15.52L88,62.25v31.5l70.06-17.51a8,8,0,0,1,3.88,15.52ZM72,196a20,20,0,1,0-20,20A20,20,0,0,0,72,196Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M228,48a4,4,0,0,1-4,4H204V72a4,4,0,0,1-8,0V52H176a4,4,0,0,1,0-8h20V24a4,4,0,0,1,8,0V44h20A4,4,0,0,1,228,48Zm-16,64v52a32.06,32.06,0,1,1-8-21.13V112a4,4,0,0,1,8,0Zm-8,52a24,24,0,1,0-24,24A24,24,0,0,0,204,164ZM84,107.12V196a32.06,32.06,0,1,1-8-21.13V56a4,4,0,0,1,3-3.88l56-14A4,4,0,0,1,137,45.88L84,59.12V98.88l75-18.76A4,4,0,0,1,161,87.88ZM76,196a24,24,0,1,0-24,24A24,24,0,0,0,76,196Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M236,48a12,12,0,0,1-12,12H212V72a12,12,0,0,1-24,0V60H176a12,12,0,0,1,0-24h12V24a12,12,0,0,1,24,0V36h12A12,12,0,0,1,236,48Zm-16,64v52a40,40,0,1,1-24-36.65V112a12,12,0,0,1,24,0Zm-24,52a16,16,0,1,0-16,16A16,16,0,0,0,196,164ZM92,113.37V196a40,40,0,1,1-24-36.65V56a12,12,0,0,1,9.09-11.64l56-14a12,12,0,0,1,5.82,23.28L92,65.37V88.63l65.09-16.27a12,12,0,0,1,5.82,23.28ZM68,196a16,16,0,1,0-16,16A16,16,0,0,0,68,196Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M230,48a6,6,0,0,1-6,6H206V72a6,6,0,0,1-12,0V54H176a6,6,0,0,1,0-12h18V24a6,6,0,0,1,12,0V42h18A6,6,0,0,1,230,48Zm-16,64v52a34.06,34.06,0,1,1-12-25.89V112a6,6,0,0,1,12,0Zm-12,52a22,22,0,1,0-22,22A22,22,0,0,0,202,164ZM86,108.68V196a34.06,34.06,0,1,1-12-25.89V56a6,6,0,0,1,4.54-5.82l56-14a6,6,0,1,1,2.92,11.64L86,60.68V96.32l72.54-18.14a6,6,0,1,1,2.92,11.64ZM74,196a22,22,0,1,0-22,22A22,22,0,0,0,74,196Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M232,48a8,8,0,0,1-8,8H208V72a8,8,0,0,1-16,0V56H176a8,8,0,0,1,0-16h16V24a8,8,0,0,1,16,0V40h16A8,8,0,0,1,232,48Zm-16,64v52a36,36,0,1,1-16-29.92V112a8,8,0,0,1,16,0Zm-16,52a20,20,0,1,0-20,20A20,20,0,0,0,200,164ZM88,110.25V196a36,36,0,1,1-16-29.92V56a8,8,0,0,1,6.06-7.76l56-14a8,8,0,0,1,3.88,15.52L88,62.25v31.5l70.06-17.51a8,8,0,0,1,3.88,15.52ZM72,196a20,20,0,1,0-20,20A20,20,0,0,0,72,196Z"></path>
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