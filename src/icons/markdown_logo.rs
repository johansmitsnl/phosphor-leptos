//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="development", feature ="office", feature ="media", feature ="brand"))]
#[component]
pub fn MarkdownLogo(
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
                <path d="M232,48H24A16,16,0,0,0,8,64V192a16,16,0,0,0,16,16H232a16,16,0,0,0,16-16V64A16,16,0,0,0,232,48ZM128,152a8,8,0,0,1-16,0V123.31L93.66,141.66a8,8,0,0,1-11.32,0L64,123.31V152a8,8,0,0,1-16,0V104a8,8,0,0,1,13.66-5.66L88,124.69l26.34-26.35A8,8,0,0,1,128,104Zm77.66-18.34-24,24a8,8,0,0,1-11.32,0l-24-24a8,8,0,0,1,11.32-11.32L168,132.69V104a8,8,0,0,1,16,0v28.69l10.34-10.35a8,8,0,0,1,11.32,11.32Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M240,64V192a8,8,0,0,1-8,8H24a8,8,0,0,1-8-8V64a8,8,0,0,1,8-8H232A8,8,0,0,1,240,64Z"
        opacity="0.2"
    ></path>
    <path d="M232,48H24A16,16,0,0,0,8,64V192a16,16,0,0,0,16,16H232a16,16,0,0,0,16-16V64A16,16,0,0,0,232,48Zm0,144H24V64H232V192ZM128,104v48a8,8,0,0,1-16,0V123.31L93.66,141.66a8,8,0,0,1-11.32,0L64,123.31V152a8,8,0,0,1-16,0V104a8,8,0,0,1,13.66-5.66L88,124.69l26.34-26.35A8,8,0,0,1,128,104Zm77.66,18.34a8,8,0,0,1,0,11.32l-24,24a8,8,0,0,1-11.32,0l-24-24a8,8,0,0,1,11.32-11.32L168,132.69V104a8,8,0,0,1,16,0v28.69l10.34-10.35A8,8,0,0,1,205.66,122.34Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M232,52H24A12,12,0,0,0,12,64V192a12,12,0,0,0,12,12H232a12,12,0,0,0,12-12V64A12,12,0,0,0,232,52Zm4,140a4,4,0,0,1-4,4H24a4,4,0,0,1-4-4V64a4,4,0,0,1,4-4H232a4,4,0,0,1,4,4ZM124,104v48a4,4,0,0,1-8,0V113.66L90.83,138.83a4,4,0,0,1-5.66,0L60,113.66V152a4,4,0,0,1-8,0V104a4,4,0,0,1,6.83-2.83L88,130.34l29.17-29.17A4,4,0,0,1,124,104Zm78.83,21.17a4,4,0,0,1,0,5.66l-24,24a4,4,0,0,1-5.66,0l-24-24a4,4,0,1,1,5.66-5.66L172,142.34V104a4,4,0,0,1,8,0v38.34l17.17-17.17A4,4,0,0,1,202.83,125.17Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M232,44H24A20,20,0,0,0,4,64V192a20,20,0,0,0,20,20H232a20,20,0,0,0,20-20V64A20,20,0,0,0,232,44Zm-4,144H28V68H228ZM48,148V108a12,12,0,0,1,20.49-8.49L88,119l19.51-19.52A12,12,0,0,1,128,108v40a12,12,0,0,1-24,0V137l-7.51,7.52a12,12,0,0,1-17,0L72,137v11a12,12,0,0,1-24,0Zm98.63-23.5a12,12,0,0,1,16.87-1.87l.5.4V108a12,12,0,0,1,24,0v15l.5-.4a12,12,0,0,1,15,18.74l-20,16a12,12,0,0,1-15,0l-20-16A12,12,0,0,1,146.63,124.5Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M232,50H24A14,14,0,0,0,10,64V192a14,14,0,0,0,14,14H232a14,14,0,0,0,14-14V64A14,14,0,0,0,232,50Zm2,142a2,2,0,0,1-2,2H24a2,2,0,0,1-2-2V64a2,2,0,0,1,2-2H232a2,2,0,0,1,2,2ZM126,104v48a6,6,0,0,1-12,0V118.49L92.24,140.24a6,6,0,0,1-8.48,0L62,118.49V152a6,6,0,0,1-12,0V104a6,6,0,0,1,10.24-4.24L88,127.51l27.76-27.75A6,6,0,0,1,126,104Zm78.24,19.76a6,6,0,0,1,0,8.48l-24,24a6,6,0,0,1-8.48,0l-24-24a6,6,0,1,1,8.48-8.48L170,137.51V104a6,6,0,0,1,12,0v33.51l13.76-13.75A6,6,0,0,1,204.24,123.76Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M232,48H24A16,16,0,0,0,8,64V192a16,16,0,0,0,16,16H232a16,16,0,0,0,16-16V64A16,16,0,0,0,232,48Zm0,144H24V64H232V192ZM128,104v48a8,8,0,0,1-16,0V123.31L93.66,141.66a8,8,0,0,1-11.32,0L64,123.31V152a8,8,0,0,1-16,0V104a8,8,0,0,1,13.66-5.66L88,124.69l26.34-26.35A8,8,0,0,1,128,104Zm77.66,18.34a8,8,0,0,1,0,11.32l-24,24a8,8,0,0,1-11.32,0l-24-24a8,8,0,0,1,11.32-11.32L168,132.69V104a8,8,0,0,1,16,0v28.69l10.34-10.35A8,8,0,0,1,205.66,122.34Z"></path>
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