/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Siren(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M120,16V8a8,8,0,0,1,16,0v8a8,8,0,0,1-16,0Zm80,32a8,8,0,0,0,5.66-2.34l8-8a8,8,0,0,0-11.32-11.32l-8,8A8,8,0,0,0,200,48ZM50.34,45.66A8,8,0,0,0,61.66,34.34l-8-8A8,8,0,0,0,42.34,37.66ZM232,176v24a16,16,0,0,1-16,16H40a16,16,0,0,1-16-16V176a16,16,0,0,1,16-16V128a88,88,0,0,1,88.67-88c48.15.36,87.33,40.29,87.33,89v31A16,16,0,0,1,232,176ZM134.68,87.89C153.67,91.08,168,108.32,168,128a8,8,0,0,0,16,0c0-27.4-20.07-51.43-46.68-55.89a8,8,0,1,0-2.64,15.78ZM216,200V176H40v24H216Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M208,129v39H48V128a80,80,0,0,1,80.61-80C172.72,48.33,208,84.89,208,129Z" opacity="0.2"/><path d="M120,16V8a8,8,0,0,1,16,0v8a8,8,0,0,1-16,0Zm80,32a8,8,0,0,0,5.66-2.34l8-8a8,8,0,0,0-11.32-11.32l-8,8A8,8,0,0,0,200,48ZM50.34,45.66A8,8,0,0,0,61.66,34.34l-8-8A8,8,0,0,0,42.34,37.66Zm87,26.45a8,8,0,1,0-2.64,15.78C153.67,91.08,168,108.32,168,128a8,8,0,0,0,16,0C184,100.6,163.93,76.57,137.32,72.11ZM232,176v24a16,16,0,0,1-16,16H40a16,16,0,0,1-16-16V176a16,16,0,0,1,16-16V128a88,88,0,0,1,88-88h.68c48.15.36,87.33,40.29,87.33,89v31A16,16,0,0,1,232,176ZM56,160H200V129c0-40-32.05-72.71-71.45-73H128a72,72,0,0,0-72,72Zm160,40V176H40v24H216Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M124,16V8a4,4,0,0,1,8,0v8a4,4,0,0,1-8,0Zm76,28a4,4,0,0,0,2.83-1.17l8-8a4,4,0,1,0-5.66-5.66l-8,8A4,4,0,0,0,200,44ZM53.17,42.83a4,4,0,0,0,5.66-5.66l-8-8a4,4,0,0,0-5.66,5.66Zm83.49,33.22a4,4,0,0,0-1.32,7.9C156.24,87.45,172,106.39,172,128a4,4,0,0,0,8,0C180,102.53,161.37,80.2,136.66,76.05ZM228,176v24a12,12,0,0,1-12,12H40a12,12,0,0,1-12-12V176a12,12,0,0,1,12-12h4V128a84,84,0,0,1,84-84h.64c46,.34,83.36,38.47,83.36,85v35h4A12,12,0,0,1,228,176ZM52,164H204V129c0-42.15-33.83-76.69-75.42-77A76,76,0,0,0,52,128Zm168,12a4,4,0,0,0-4-4H40a4,4,0,0,0-4,4v24a4,4,0,0,0,4,4H216a4,4,0,0,0,4-4Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M116,20V12a12,12,0,0,1,24,0v8a12,12,0,0,1-24,0Zm84,36a12,12,0,0,0,8.49-3.51l8-8a12,12,0,1,0-17-17l-8,8A12,12,0,0,0,200,56ZM47.51,52.49a12,12,0,0,0,17-17l-8-8a12,12,0,0,0-17,17ZM236,176v24a20,20,0,0,1-20,20H40a20,20,0,0,1-20-20V176a20,20,0,0,1,16-19.6V140a92,92,0,0,1,92-92h.71C179,48.38,220,90.1,220,141v15.4A20,20,0,0,1,236,176ZM60,140v16H196V141c0-37.77-30.27-68.72-67.48-69H128a68,68,0,0,0-68,68Zm152,40H44v16H212Zm-75.6-66.72a28,28,0,0,1,18.32,18.32,12,12,0,0,0,22.9-7.2,52,52,0,0,0-34-34,12,12,0,0,0-7.2,22.9Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M122,16V8a6,6,0,0,1,12,0v8a6,6,0,0,1-12,0Zm78,30a6,6,0,0,0,4.24-1.76l8-8a6,6,0,1,0-8.48-8.48l-8,8A6,6,0,0,0,200,46ZM51.76,44.24a6,6,0,0,0,8.48-8.48l-8-8a6,6,0,0,0-8.48,8.48ZM137,74.08a6,6,0,1,0-2,11.84c20,3.34,35,21.44,35,42.08a6,6,0,0,0,12,0C182,101.57,162.65,78.39,137,74.08ZM230,176v24a14,14,0,0,1-14,14H40a14,14,0,0,1-14-14V176a14,14,0,0,1,14-14h2V128a86,86,0,0,1,86-86h.65c47.06.35,85.35,39.38,85.35,87v33h2A14,14,0,0,1,230,176ZM54,162H202V129c0-41-32.94-74.7-73.44-75H128a74,74,0,0,0-74,74Zm164,14a2,2,0,0,0-2-2H40a2,2,0,0,0-2,2v24a2,2,0,0,0,2,2H216a2,2,0,0,0,2-2Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M120,16V8a8,8,0,0,1,16,0v8a8,8,0,0,1-16,0Zm80,32a8,8,0,0,0,5.66-2.34l8-8a8,8,0,0,0-11.32-11.32l-8,8A8,8,0,0,0,200,48ZM50.34,45.66A8,8,0,0,0,61.66,34.34l-8-8A8,8,0,0,0,42.34,37.66Zm87,26.45a8,8,0,1,0-2.64,15.78C153.67,91.08,168,108.32,168,128a8,8,0,0,0,16,0C184,100.6,163.93,76.57,137.32,72.11ZM232,176v24a16,16,0,0,1-16,16H40a16,16,0,0,1-16-16V176a16,16,0,0,1,16-16V128a88,88,0,0,1,88.67-88c48.15.36,87.33,40.29,87.33,89v31A16,16,0,0,1,232,176ZM56,160H200V129c0-40-32.05-72.71-71.45-73H128a72,72,0,0,0-72,72Zm160,40V176H40v24H216Z"/> }.into_view()
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