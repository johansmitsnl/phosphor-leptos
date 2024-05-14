//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "health", feature = "objects"))]
#[component]
pub fn Sunglasses(
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
                <path d="M200,40a8,8,0,0,0,0,16,16,16,0,0,1,16,16v56H40V72A16,16,0,0,1,56,56a8,8,0,0,0,0-16A32,32,0,0,0,24,72v92a44,44,0,0,0,88,0V144h32v20a44,44,0,0,0,88,0V72A32,32,0,0,0,200,40ZM91.22,179.22a8,8,0,0,1-11.31,0L58.34,157.66a8,8,0,0,1,11.32-11.32l21.56,21.57A8,8,0,0,1,91.22,179.22Zm120,0a8,8,0,0,1-11.31,0l-21.57-21.56a8,8,0,0,1,11.32-11.32l21.56,21.57A8,8,0,0,1,211.22,179.22Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M32,136h72v28a36,36,0,0,1-72,0Zm120,0v28a36,36,0,0,0,72,0V136Z" opacity="0.2"></path>
    <path d="M200,40a8,8,0,0,0,0,16,16,16,0,0,1,16,16v56H40V72A16,16,0,0,1,56,56a8,8,0,0,0,0-16A32,32,0,0,0,24,72v92a44,44,0,0,0,88,0V144h32v20a44,44,0,0,0,88,0V72A32,32,0,0,0,200,40Zm12.63,137.31L179.31,144H216v20A27.8,27.8,0,0,1,212.63,177.31ZM40,164V147.31l41.31,41.32A28,28,0,0,1,40,164Zm56,0a27.8,27.8,0,0,1-3.37,13.31L59.31,144H96Zm64,0V147.31l41.31,41.32A28,28,0,0,1,160,164Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M200,44a4,4,0,0,0,0,8,20,20,0,0,1,20,20v60H36V72A20,20,0,0,1,56,52a4,4,0,0,0,0-8A28,28,0,0,0,28,72v92a40,40,0,0,0,80,0V140h40v24a40,40,0,0,0,80,0V72A28,28,0,0,0,200,44ZM36,164V140h2.34l49.27,49.26A32,32,0,0,1,36,164Zm64,0a31.83,31.83,0,0,1-6.74,19.61L49.66,140H100Zm56,0V140h2.34l49.27,49.26A32,32,0,0,1,156,164Zm57.26,19.61L169.66,140H220v24A31.83,31.83,0,0,1,213.26,183.61Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M200,36a12,12,0,0,0,0,24,12,12,0,0,1,12,12v52H44V72A12,12,0,0,1,56,60a12,12,0,0,0,0-24A36,36,0,0,0,20,72v92a48,48,0,0,0,96,0V148h24v16a48,48,0,0,0,96,0V72A36,36,0,0,0,200,36ZM68,188a24,24,0,0,1-24-24v-9l31.74,31.74A23.89,23.89,0,0,1,68,188Zm24-24a24.8,24.8,0,0,1-.44,4.59L71,148H92Zm96,24a24,24,0,0,1-24-24v-9l31.74,31.74A23.89,23.89,0,0,1,188,188Zm24-24a24.8,24.8,0,0,1-.44,4.59L191,148h21Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M200,42a6,6,0,0,0,0,12,18,18,0,0,1,18,18v58H38V72A18,18,0,0,1,56,54a6,6,0,0,0,0-12A30,30,0,0,0,26,72v92a42,42,0,0,0,84,0V142h36v22a42,42,0,0,0,84,0V72A30,30,0,0,0,200,42ZM38,164V142.48L84.53,189A30,30,0,0,1,38,164Zm60,0a29.83,29.83,0,0,1-5,16.53L54.48,142H98Zm60,0V142.48L204.53,189A30,30,0,0,1,158,164Zm55,16.53L174.48,142H218v22A29.83,29.83,0,0,1,213,180.53Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M200,40a8,8,0,0,0,0,16,16,16,0,0,1,16,16v56H40V72A16,16,0,0,1,56,56a8,8,0,0,0,0-16A32,32,0,0,0,24,72v92a44,44,0,0,0,88,0V144h32v20a44,44,0,0,0,88,0V72A32,32,0,0,0,200,40Zm12.63,137.31L179.31,144H216v20A27.8,27.8,0,0,1,212.63,177.31ZM40,164V147.31l41.31,41.32A28,28,0,0,1,40,164Zm56,0a27.8,27.8,0,0,1-3.37,13.31L59.31,144H96Zm64,0V147.31l41.31,41.32A28,28,0,0,1,160,164Z"></path>
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
