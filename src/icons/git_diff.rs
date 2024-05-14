//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="development"))]
#[component]
pub fn GitDiff(
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
                <path d="M118.18,213.08c-.11.14-.24.27-.36.4l-.16.18-.17.15a4.83,4.83,0,0,1-.42.37,3.92,3.92,0,0,1-.32.25l-.3.22-.38.23a2.91,2.91,0,0,1-.3.17l-.37.19-.34.15-.36.13a2.84,2.84,0,0,1-.38.13l-.36.1c-.14,0-.26.07-.4.09l-.42.07-.35.05a7,7,0,0,1-.79,0H64a8,8,0,0,1,0-16H92.69L55,162.34a23.85,23.85,0,0,1-7-17V95a32,32,0,1,1,16,0v50.38A8,8,0,0,0,66.34,151L104,188.69V160a8,8,0,0,1,16,0v48a7,7,0,0,1,0,.8c0,.11,0,.21,0,.32s0,.3-.07.46a2.83,2.83,0,0,1-.09.37c0,.13-.06.26-.1.39s-.08.23-.12.35l-.14.39-.15.31c-.06.13-.12.27-.19.4s-.11.18-.16.28l-.24.39-.21.28ZM208,161V110.63a23.85,23.85,0,0,0-7-17L163.31,56H192a8,8,0,0,0,0-16H143.82l-.6,0c-.14,0-.28,0-.41.06l-.37,0-.43.11-.33.08-.4.14-.34.13-.35.16-.36.18a3.14,3.14,0,0,0-.31.18c-.12.07-.25.14-.36.22a3.55,3.55,0,0,0-.31.23,3.81,3.81,0,0,0-.32.24c-.15.12-.28.24-.42.37l-.17.15-.16.18c-.12.13-.25.26-.36.4l-.26.35-.21.28-.24.39c-.05.1-.11.19-.16.28s-.13.27-.19.4l-.15.31-.14.39c0,.12-.09.23-.12.35s-.07.26-.1.39a2.83,2.83,0,0,0-.09.37c0,.16,0,.31-.07.46s0,.21-.05.32a7,7,0,0,0,0,.8V96a8,8,0,0,0,16,0V67.31L189.66,105a8,8,0,0,1,2.34,5.66V161a32,32,0,1,0,16,0Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M80,64A24,24,0,1,1,56,40,24,24,0,0,1,80,64ZM200,168a24,24,0,1,0,24,24A24,24,0,0,0,200,168Z"
        opacity="0.2"
    ></path>
    <path d="M112,152a8,8,0,0,0-8,8v28.69L66.34,151A8,8,0,0,1,64,145.37V95a32,32,0,1,0-16,0v50.38a23.85,23.85,0,0,0,7,17L92.69,200H64a8,8,0,0,0,0,16h48a8,8,0,0,0,8-8V160A8,8,0,0,0,112,152ZM40,64A16,16,0,1,1,56,80,16,16,0,0,1,40,64Zm168,97V110.63a23.85,23.85,0,0,0-7-17L163.31,56H192a8,8,0,0,0,0-16H144a8,8,0,0,0-8,8V96a8,8,0,0,0,16,0V67.31L189.66,105a8,8,0,0,1,2.34,5.66V161a32,32,0,1,0,16,0Zm-8,47a16,16,0,1,1,16-16A16,16,0,0,1,200,208Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M112,156a4,4,0,0,0-4,4v38.34L63.51,153.86A12,12,0,0,1,60,145.37V91.71a28,28,0,1,0-8,0v53.66a19.85,19.85,0,0,0,5.86,14.14L102.34,204H64a4,4,0,0,0,0,8h48a4,4,0,0,0,4-4V160A4,4,0,0,0,112,156ZM36,64A20,20,0,1,1,56,84,20,20,0,0,1,36,64ZM204,164.29V110.63a19.85,19.85,0,0,0-5.86-14.14L153.66,52H192a4,4,0,0,0,0-8H144a4,4,0,0,0-4,4V96a4,4,0,0,0,8,0V57.66l44.49,44.48a12,12,0,0,1,3.51,8.49v53.66a28,28,0,1,0,8,0ZM200,212a20,20,0,1,1,20-20A20,20,0,0,1,200,212Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M112,148a12,12,0,0,0-12,12v19L69.17,148.2A4,4,0,0,1,68,145.37V97.94a36,36,0,1,0-24,0v47.43a27.81,27.81,0,0,0,8.2,19.8L83,196H64a12,12,0,0,0,0,24h48a12,12,0,0,0,12-12V160A12,12,0,0,0,112,148ZM56,52A12,12,0,1,1,44,64,12,12,0,0,1,56,52ZM212,158.06V110.63a27.81,27.81,0,0,0-8.2-19.8L173,60h19a12,12,0,0,0,0-24H144a12,12,0,0,0-12,12V96a12,12,0,0,0,24,0V77l30.83,30.83a4,4,0,0,1,1.17,2.83v47.43a36,36,0,1,0,24,0ZM200,204a12,12,0,1,1,12-12A12,12,0,0,1,200,204Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M112,154a6,6,0,0,0-6,6v33.52L64.93,152.44A9.93,9.93,0,0,1,62,145.37v-52a30,30,0,1,0-12,0v52a21.88,21.88,0,0,0,6.44,15.56L97.52,202H64a6,6,0,0,0,0,12h48a6,6,0,0,0,6-6V160A6,6,0,0,0,112,154ZM38,64A18,18,0,1,1,56,82,18,18,0,0,1,38,64Zm168,98.6v-52a21.88,21.88,0,0,0-6.44-15.56L158.48,54H192a6,6,0,0,0,0-12H144a6,6,0,0,0-6,6V96a6,6,0,0,0,12,0V62.48l41.07,41.08a9.93,9.93,0,0,1,2.93,7.07v52a30,30,0,1,0,12,0ZM200,210a18,18,0,1,1,18-18A18,18,0,0,1,200,210Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M112,152a8,8,0,0,0-8,8v28.69L66.34,151A8,8,0,0,1,64,145.37V95a32,32,0,1,0-16,0v50.38a23.85,23.85,0,0,0,7,17L92.69,200H64a8,8,0,0,0,0,16h48a8,8,0,0,0,8-8V160A8,8,0,0,0,112,152ZM40,64A16,16,0,1,1,56,80,16,16,0,0,1,40,64Zm168,97V110.63a23.85,23.85,0,0,0-7-17L163.31,56H192a8,8,0,0,0,0-16H144a8,8,0,0,0-8,8V96a8,8,0,0,0,16,0V67.31L189.66,105a8,8,0,0,1,2.34,5.66V161a32,32,0,1,0,16,0Zm-8,47a16,16,0,1,1,16-16A16,16,0,0,1,200,208Z"></path>
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