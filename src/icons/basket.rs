//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Basket(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view! {
                <path d="M140,128v40a12,12,0,0,1-24,0V128a12,12,0,0,1,24,0ZM243.82,98.64,230,202.64A20.06,20.06,0,0,1,210.13,220H45.87A20.07,20.07,0,0,1,26,202.65l-13.86-104A20,20,0,0,1,32,76H66.55L119,16.1a12,12,0,0,1,18.06,0L189.45,76H224a20,20,0,0,1,19.81,22.64ZM98.45,76h59.11L128,42.22Zm121,24H36.57l12.8,96H206.63Zm-51.37,26.81-4,40a12,12,0,0,0,10.75,13.13c.4,0,.81.06,1.21.06a12,12,0,0,0,11.92-10.81l4-40a12,12,0,1,0-23.88-2.38Zm-80.12,0a12,12,0,0,0-23.88,2.38l4,40A12,12,0,0,0,80,180c.39,0,.8,0,1.2-.06a12,12,0,0,0,10.75-13.13Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M231.93,97.06l-13.87,104a8,8,0,0,1-7.93,6.94H45.87a8,8,0,0,1-7.93-6.94l-13.87-104A8,8,0,0,1,32,88H224A8,8,0,0,1,231.93,97.06Z"
        opacity="0.2"
    ></path>
    <path d="M136,120v56a8,8,0,0,1-16,0V120a8,8,0,0,1,16,0ZM239.86,98.11,226,202.12A16,16,0,0,1,210.13,216H45.87A16,16,0,0,1,30,202.12l-13.87-104A16,16,0,0,1,32,80H68.37L122,18.73a8,8,0,0,1,12,0L187.63,80H224a16,16,0,0,1,15.85,18.11ZM89.63,80h76.74L128,36.15ZM224,96H32L45.87,200H210.13Zm-51.16,23.2-5.6,56A8,8,0,0,0,174.4,184a7.44,7.44,0,0,0,.81,0,8,8,0,0,0,7.95-7.2l5.6-56a8,8,0,0,0-15.92-1.6Zm-89.68,0a8,8,0,0,0-15.92,1.6l5.6,56a8,8,0,0,0,8,7.2,7.44,7.44,0,0,0,.81,0,8,8,0,0,0,7.16-8.76Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M236,85.46A16,16,0,0,0,224,80H187.63L134,18.73a8,8,0,0,0-12,0L68.37,80H32A16,16,0,0,0,16.14,98.11L30,202.12A16,16,0,0,0,45.87,216H210.13A16,16,0,0,0,226,202.12l13.87-104A16,16,0,0,0,236,85.46ZM81.6,184a7.44,7.44,0,0,1-.81,0,8,8,0,0,1-8-7.2l-5.6-56a8,8,0,0,1,15.92-1.6l5.6,56A8,8,0,0,1,81.6,184Zm54.4-8a8,8,0,0,1-16,0V120a8,8,0,0,1,16,0ZM89.63,80,128,36.15,166.37,80Zm99.13,40.8-5.6,56a8,8,0,0,1-7.95,7.2,7.44,7.44,0,0,1-.81,0,8,8,0,0,1-7.16-8.76l5.6-56a8,8,0,0,1,15.92,1.6Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M134,120v56a6,6,0,0,1-12,0V120a6,6,0,0,1,12,0ZM237.88,97.85,224,201.85A14,14,0,0,1,210.13,214H45.87A14,14,0,0,1,32,201.85l-13.87-104A14,14,0,0,1,32,82H69.28l54.2-61.95a6,6,0,0,1,9,0l54.2,62H224a14,14,0,0,1,13.87,15.85ZM85.22,82h85.56L128,33.11ZM225.5,94.68A2,2,0,0,0,224,94H32a2,2,0,0,0-1.51.68A2,2,0,0,0,30,96.26l13.86,104a2,2,0,0,0,2,1.73H210.13a2,2,0,0,0,2-1.73L226,96.26A1.93,1.93,0,0,0,225.5,94.68ZM181.4,114a6,6,0,0,0-6.57,5.37l-5.6,56A6,6,0,0,0,174.6,182l.61,0a6,6,0,0,0,6-5.4l5.6-56A6,6,0,0,0,181.4,114ZM81.17,119.4a6,6,0,0,0-11.94,1.2l5.6,56a6,6,0,0,0,6,5.4l.61,0a6,6,0,0,0,5.37-6.57Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M136,120v56a8,8,0,0,1-16,0V120a8,8,0,0,1,16,0ZM239.86,98.11,226,202.12A16,16,0,0,1,210.13,216H45.87A16,16,0,0,1,30,202.12l-13.87-104A16,16,0,0,1,32,80H68.37L122,18.73a8,8,0,0,1,12,0L187.63,80H224a16,16,0,0,1,15.85,18.11ZM89.63,80h76.74L128,36.15ZM224,96H32L45.87,200H210.13Zm-51.16,23.2-5.6,56A8,8,0,0,0,174.4,184a7.44,7.44,0,0,0,.81,0,8,8,0,0,0,7.95-7.2l5.6-56a8,8,0,0,0-15.92-1.6Zm-89.68,0a8,8,0,0,0-15.92,1.6l5.6,56a8,8,0,0,0,8,7.2,7.44,7.44,0,0,0,.81,0,8,8,0,0,0,7.16-8.76Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M132,120v56a4,4,0,0,1-8,0V120a4,4,0,0,1,8,0ZM235.89,97.59,222,201.59A12,12,0,0,1,210.13,212H45.87A12,12,0,0,1,34,201.59l-13.86-104A12,12,0,0,1,32,84H70.18L125,21.37a4,4,0,0,1,6,0L185.82,84H224a12,12,0,0,1,11.89,13.59ZM80.81,84h94.38L128,30.07ZM227,93.36A3.94,3.94,0,0,0,224,92H32a4,4,0,0,0-4,4.52l13.86,104a4,4,0,0,0,4,3.47H210.13a4,4,0,0,0,4-3.47L228,96.53A3.94,3.94,0,0,0,227,93.36ZM181.2,116a4,4,0,0,0-4.38,3.58l-5.6,56A4,4,0,0,0,174.8,180l.41,0a4,4,0,0,0,4-3.6l5.6-56A4,4,0,0,0,181.2,116ZM74.8,116a4,4,0,0,0-3.58,4.38l5.6,56a4,4,0,0,0,4,3.6l.41,0a4,4,0,0,0,3.58-4.38l-5.6-56A4,4,0,0,0,74.8,116Z"></path>
}.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size.get()
            height=size.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}
