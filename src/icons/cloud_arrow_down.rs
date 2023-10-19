/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn CloudArrowDown(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M247.93,124.52C246.11,77.54,207.07,40,160.06,40A88.1,88.1,0,0,0,81.29,88.67h0A87.48,87.48,0,0,0,72,127.73,8.18,8.18,0,0,1,64.57,136,8,8,0,0,1,56,128a103.66,103.66,0,0,1,5.34-32.92,4,4,0,0,0-4.75-5.18A64.09,64.09,0,0,0,8,152c0,35.19,29.75,64,65,64H160A88.09,88.09,0,0,0,247.93,124.52Zm-58.27,41.14-32,32a8,8,0,0,1-11.32,0l-32-32a8,8,0,0,1,11.32-11.32L144,172.69V112a8,8,0,0,1,16,0v60.69l18.34-18.35a8,8,0,0,1,11.32,11.32Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M240,128a80,80,0,0,1-80,80H72A56,56,0,1,1,85.92,97.74l0,.1A80,80,0,0,1,240,128Z" opacity="0.2"/><path d="M248,128a87.34,87.34,0,0,1-17.6,52.81,8,8,0,1,1-12.8-9.62A71.34,71.34,0,0,0,232,128a72,72,0,0,0-144,0,8,8,0,0,1-16,0,88,88,0,0,1,3.29-23.88C74.2,104,73.1,104,72,104a48,48,0,0,0,0,96H96a8,8,0,0,1,0,16H72A64,64,0,1,1,81.29,88.68,88,88,0,0,1,248,128Zm-69.66,42.34L160,188.69V128a8,8,0,0,0-16,0v60.69l-18.34-18.35a8,8,0,0,0-11.32,11.32l32,32a8,8,0,0,0,11.32,0l32-32a8,8,0,0,0-11.32-11.32Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M244,128a83.28,83.28,0,0,1-16.8,50.4,4,4,0,1,1-6.4-4.8A76,76,0,1,0,84,128a4,4,0,0,1-8,0,83.45,83.45,0,0,1,4.57-27.27A52,52,0,1,0,72,204H96a4,4,0,0,1,0,8H72A60,60,0,1,1,83.61,93.13,84,84,0,0,1,244,128Zm-62.83,45.17L156,198.34V128a4,4,0,0,0-8,0v70.34l-25.17-25.17a4,4,0,0,0-5.66,5.66l32,32a4,4,0,0,0,5.66,0l32-32a4,4,0,0,0-5.66-5.66Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M252,128a91.18,91.18,0,0,1-18.41,55.21,12,12,0,0,1-19.18-14.42A68,68,0,1,0,92,128a12,12,0,0,1-24,0,91.7,91.7,0,0,1,2.19-20A44,44,0,0,0,72,196H96a12,12,0,0,1,0,24H72A68,68,0,1,1,79,84.37,92,92,0,0,1,252,128Zm-76.49,39.51L164,179V128a12,12,0,0,0-24,0v51l-11.51-11.52a12,12,0,1,0-17,17l32,32a12,12,0,0,0,17,0l32-32a12,12,0,0,0-17-17Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M246,128a85.27,85.27,0,0,1-17.2,51.6,6,6,0,1,1-9.6-7.2A74,74,0,1,0,86,128a6,6,0,0,1-12,0,85.54,85.54,0,0,1,3.91-25.64A50.68,50.68,0,0,0,72,102a50,50,0,0,0,0,100H96a6,6,0,0,1,0,12H72A62,62,0,1,1,82.43,90.88,86,86,0,0,1,246,128Zm-66.24,43.76L158,193.51V128a6,6,0,0,0-12,0v65.51l-21.76-21.75a6,6,0,0,0-8.48,8.48l32,32a6,6,0,0,0,8.48,0l32-32a6,6,0,0,0-8.48-8.48Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M248,128a87.34,87.34,0,0,1-17.6,52.81,8,8,0,1,1-12.8-9.62A71.34,71.34,0,0,0,232,128a72,72,0,0,0-144,0,8,8,0,0,1-16,0,88,88,0,0,1,3.29-23.88C74.2,104,73.1,104,72,104a48,48,0,0,0,0,96H96a8,8,0,0,1,0,16H72A64,64,0,1,1,81.29,88.68,88,88,0,0,1,248,128Zm-69.66,42.34L160,188.69V128a8,8,0,0,0-16,0v60.69l-18.34-18.35a8,8,0,0,0-11.32,11.32l32,32a8,8,0,0,0,11.32,0l32-32a8,8,0,0,0-11.32-11.32Z"/> }.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg 
            xmlns="http://www.w3.org/2000/svg" 
            width=size.clone()
            height=size
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}