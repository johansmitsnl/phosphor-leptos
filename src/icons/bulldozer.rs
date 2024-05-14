//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="commerce", feature ="objects"))]
#[component]
pub fn Bulldozer(
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
                <path d="M248,200h-8a8,8,0,0,1-8-8V160a8,8,0,0,1,8-8h8a8,8,0,0,0,0-16h-8a24,24,0,0,0-24,24v8H199.2a40.1,40.1,0,0,0-33.71-31.61L129.44,49.85A16,16,0,0,0,114.67,40H24A16,16,0,0,0,8,56v96a40,40,0,0,0,32,64H160a40.07,40.07,0,0,0,39.2-32H216v8a24,24,0,0,0,24,24h8a8,8,0,0,0,0-16ZM64,56h50.67L148,136H64ZM24,56H48v80H40a39.72,39.72,0,0,0-16,3.35ZM160,184H40a8,8,0,0,1,0-16H160a8,8,0,0,1,0,16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M192,176h0a32,32,0,0,1-32,32H40A32,32,0,0,1,8,176H8a32,32,0,0,1,32-32H160A32,32,0,0,1,192,176Z"
        opacity="0.2"
    ></path>
    <path d="M248,200h-8a8,8,0,0,1-8-8V160a8,8,0,0,1,8-8h8a8,8,0,0,0,0-16h-8a24,24,0,0,0-24,24v8H199.2a40.09,40.09,0,0,0-33.71-31.61L129.44,49.85A16,16,0,0,0,114.67,40H24A16,16,0,0,0,8,56v96a40,40,0,0,0,32,64H160a40.07,40.07,0,0,0,39.2-32H216v8a24,24,0,0,0,24,24h8a8,8,0,0,0,0-16ZM148,136H64V56h50.67ZM48,56v80H40a39.72,39.72,0,0,0-16,3.35V56ZM160,200H40a24,24,0,0,1,0-48H160a24,24,0,0,1,0,48Zm8-24a8,8,0,0,1-8,8H40a8,8,0,0,1,0-16H160A8,8,0,0,1,168,176Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M248,204h-8a12,12,0,0,1-12-12V160a12,12,0,0,1,12-12h8a4,4,0,0,0,0-8h-8a20,20,0,0,0-20,20v12H195.77a36.06,36.06,0,0,0-33.06-31.89l-37-88.73A12,12,0,0,0,114.67,44H24A12,12,0,0,0,12,56v97.41A36,36,0,0,0,40,212H160a36,36,0,0,0,35.77-32H220v12a20,20,0,0,0,20,20h8a4,4,0,0,0,0-8ZM118.36,54.46,154,140H60V52h54.67A4,4,0,0,1,118.36,54.46ZM24,52H52v88H40a35.76,35.76,0,0,0-20,6.08V56A4,4,0,0,1,24,52ZM160,204H40a28,28,0,0,1,0-56H160a28,28,0,0,1,0,56Zm4-28a4,4,0,0,1-4,4H40a4,4,0,0,1,0-8H160A4,4,0,0,1,164,176Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M244,196h-8a4,4,0,0,1-4-4V160a4,4,0,0,1,4-4h8a12,12,0,0,0,0-24h-8a28,28,0,0,0-28,28v4h-9.68a44.13,44.13,0,0,0-34-31.2l-35.2-84.49A20,20,0,0,0,110.67,36H24A20,20,0,0,0,4,56V157.7A44,44,0,0,0,44,220H156a44.06,44.06,0,0,0,42.32-32H208v4a28,28,0,0,0,28,28h8a12,12,0,0,0,0-24ZM138,132H68V60h40ZM44,60v72a43.85,43.85,0,0,0-16,3V60ZM156,196H44a20,20,0,0,1,0-40H156a20,20,0,0,1,0,40Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M248,202h-8a10,10,0,0,1-10-10V160a10,10,0,0,1,10-10h8a6,6,0,0,0,0-12h-8a22,22,0,0,0-22,22v10H197.52a38.08,38.08,0,0,0-33.43-31.78l-36.5-87.61A14,14,0,0,0,114.67,42H24A14,14,0,0,0,10,56v96.72A38,38,0,0,0,40,214H160a38.05,38.05,0,0,0,37.52-32H218v10a22,22,0,0,0,22,22h8a6,6,0,0,0,0-12ZM116.51,55.23,151,138H62V54h52.67A2,2,0,0,1,116.51,55.23ZM24,54H50v84H40a37.82,37.82,0,0,0-18,4.54V56A2,2,0,0,1,24,54ZM160,202H40a26,26,0,0,1,0-52H160a26,26,0,0,1,0,52Zm6-26a6,6,0,0,1-6,6H40a6,6,0,0,1,0-12H160A6,6,0,0,1,166,176Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M248,200h-8a8,8,0,0,1-8-8V160a8,8,0,0,1,8-8h8a8,8,0,0,0,0-16h-8a24,24,0,0,0-24,24v8H199.2a40.09,40.09,0,0,0-33.71-31.61L129.44,49.85A16,16,0,0,0,114.67,40H24A16,16,0,0,0,8,56v96a40,40,0,0,0,32,64H160a40.07,40.07,0,0,0,39.2-32H216v8a24,24,0,0,0,24,24h8a8,8,0,0,0,0-16ZM148,136H64V56h50.67ZM48,56v80H40a39.72,39.72,0,0,0-16,3.35V56ZM160,200H40a24,24,0,0,1,0-48H160a24,24,0,0,1,0,48Zm8-24a8,8,0,0,1-8,8H40a8,8,0,0,1,0-16H160A8,8,0,0,1,168,176Z"></path>
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