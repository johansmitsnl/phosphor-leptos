//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "map", feature = "objects"))]
#[component]
pub fn Jeep(
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
                <path d="M248,103.47A8.17,8.17,0,0,0,239.73,96H232a8,8,0,0,0-.18-1.68L221.18,44.65A16.08,16.08,0,0,0,205.53,32H50.47A16.08,16.08,0,0,0,34.82,44.65L24.18,94.32A8,8,0,0,0,24,96H16.27A8.17,8.17,0,0,0,8,103.47,8,8,0,0,0,16,112h8v88a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V184h20a4,4,0,0,0,4-4V128.27a8.17,8.17,0,0,1,7.47-8.25,8,8,0,0,1,8.53,8v52a4,4,0,0,0,4,4h8a4,4,0,0,0,4-4V128.27a8.17,8.17,0,0,1,7.47-8.25,8,8,0,0,1,8.53,8v52a4,4,0,0,0,4,4h20v16a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V112h8A8,8,0,0,0,248,103.47ZM68,144a12,12,0,1,1,12-12A12,12,0,0,1,68,144Zm120,0a12,12,0,1,1,12-12A12,12,0,0,1,188,144ZM40.18,96,50.47,48H205.53l10.29,48Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,96H32L42.65,46.32A8,8,0,0,1,50.47,40H205.53a8,8,0,0,1,7.82,6.32Z"
        opacity="0.2"
    ></path>
    <path d="M240,88h-9.53l-9.29-43.35A16.08,16.08,0,0,0,205.53,32H50.47A16.08,16.08,0,0,0,34.82,44.65L25.53,88H16a8,8,0,0,0,0,16h8v96a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V176h96v24a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V104h8a8,8,0,0,0,0-16ZM50.47,48H205.53l8.57,40H41.9ZM64,200H40V176H64Zm128,0V176h24v24Zm24-40H152V128a8,8,0,0,0-16,0v32H120V128a8,8,0,0,0-16,0v32H40V104H216ZM56,132a12,12,0,1,1,12,12A12,12,0,0,1,56,132Zm120,0a12,12,0,1,1,12,12A12,12,0,0,1,176,132Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M240,92H227.23l-10-46.51A12.07,12.07,0,0,0,205.53,36H50.47a12.07,12.07,0,0,0-11.74,9.49L28.77,92H16a4,4,0,0,0,0,8H28V200a12,12,0,0,0,12,12H64a12,12,0,0,0,12-12V172H180v28a12,12,0,0,0,12,12h24a12,12,0,0,0,12-12V100h12a4,4,0,0,0,0-8ZM46.56,47.16A4,4,0,0,1,50.47,44H205.53a4,4,0,0,1,3.91,3.16L219.05,92H37ZM68,200a4,4,0,0,1-4,4H40a4,4,0,0,1-4-4V172H68Zm148,4H192a4,4,0,0,1-4-4V172h32v28A4,4,0,0,1,216,204Zm4-40H148V128a4,4,0,0,0-8,0v36H116V128a4,4,0,0,0-8,0v36H36V100H220ZM60,132a8,8,0,1,1,8,8A8,8,0,0,1,60,132Zm120,0a8,8,0,1,1,8,8A8,8,0,0,1,180,132Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M240,84h-6.3l-8.61-40.19A20.11,20.11,0,0,0,205.53,28H50.47A20.11,20.11,0,0,0,30.91,43.81L22.3,84H16a12,12,0,0,0,0,24h4v92a20,20,0,0,0,20,20H68a20,20,0,0,0,20-20V180h80v20a20,20,0,0,0,20,20h28a20,20,0,0,0,20-20V108h4a12,12,0,0,0,0-24ZM53.7,52H202.3l6.86,32H46.84ZM64,196H44V180H64Zm128,0V180h20v16Zm20-40H180V132a12,12,0,0,0-24,0v24H140V132a12,12,0,0,0-24,0v24H100V132a12,12,0,0,0-24,0v24H44V108H212Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M240,90H228.85l-9.63-44.93A14.06,14.06,0,0,0,205.53,34H50.47A14.06,14.06,0,0,0,36.78,45.07L27.15,90H16a6,6,0,0,0,0,12H26v98a14,14,0,0,0,14,14H64a14,14,0,0,0,14-14V174H178v26a14,14,0,0,0,14,14h24a14,14,0,0,0,14-14V102h10a6,6,0,0,0,0-12ZM48.51,47.58a2,2,0,0,1,2-1.58H205.53a2,2,0,0,1,2,1.58L216.58,90H39.42ZM66,200a2,2,0,0,1-2,2H40a2,2,0,0,1-2-2V174H66Zm150,2H192a2,2,0,0,1-2-2V174h28v26A2,2,0,0,1,216,202Zm2-40H150V128a6,6,0,0,0-12,0v34H118V128a6,6,0,0,0-12,0v34H38V102H218ZM58,132a10,10,0,1,1,10,10A10,10,0,0,1,58,132Zm120,0a10,10,0,1,1,10,10A10,10,0,0,1,178,132Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M240,88h-9.53l-9.29-43.35A16.08,16.08,0,0,0,205.53,32H50.47A16.08,16.08,0,0,0,34.82,44.65L25.53,88H16a8,8,0,0,0,0,16h8v96a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V176h96v24a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V104h8a8,8,0,0,0,0-16ZM50.47,48H205.53l8.57,40H41.9ZM64,200H40V176H64Zm128,0V176h24v24Zm24-40H152V128a8,8,0,0,0-16,0v32H120V128a8,8,0,0,0-16,0v32H40V104H216ZM56,132a12,12,0,1,1,12,12A12,12,0,0,1,56,132Zm120,0a12,12,0,1,1,12,12A12,12,0,0,1,176,132Z"></path>
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
