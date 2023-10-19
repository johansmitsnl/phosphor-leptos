/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn EggCrack(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M216,152a88.11,88.11,0,0,1-87.8,88c-50.66.12-90.21-43-88.12-93.62,1.21-29.21,11.71-60.54,29.23-86.82C87.5,32.29,109.43,16,128,16c13.25,0,28.23,8.32,42.34,23a4,4,0,0,1,.09,5.44L122,98.67a8,8,0,0,0,4,13.09l24.61,6.15-6.51,32.52a8,8,0,0,0,6.28,9.41A7.7,7.7,0,0,0,152,160a8,8,0,0,0,7.83-6.43l8-40a8,8,0,0,0-5.9-9.33l-19.16-4.79,36.89-41.33a4,4,0,0,1,6.29.41c.24.34.47.68.7,1C205.3,87.54,216,121.23,216,152Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M208,152a80,80,0,0,1-160,0C48,88,96,24,128,24S208,88,208,152Z" opacity="0.2"/><path d="M186.66,59.56C168.47,32.29,146.54,16,128,16S87.53,32.29,69.34,59.56C50.7,87.54,40,121.23,40,152a88,88,0,0,0,176,0C216,121.23,205.3,87.54,186.66,59.56ZM128,224a72.08,72.08,0,0,1-72-72c0-27.69,9.72-58.15,26.66-83.56C97.19,46.64,115.41,32,128,32c9.5,0,22.2,8.33,34.1,21.78L122,98.67a8,8,0,0,0,4,13.09l24.6,6.15-6.5,32.52a8,8,0,0,0,6.27,9.41A7.77,7.77,0,0,0,152,160a8,8,0,0,0,7.83-6.43l8-40a8,8,0,0,0-5.9-9.33l-19.16-4.79L172.1,66.6c.42.61.83,1.22,1.24,1.84C190.28,93.85,200,124.31,200,152A72.08,72.08,0,0,1,128,224Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M128,20C92.87,20,44,86.52,44,152a84,84,0,0,0,168,0C212,86.52,163.13,20,128,20Zm0,208a76.08,76.08,0,0,1-76-76c0-28.46,10-59.73,27.33-85.78C94.81,43,113.91,28,128,28c11.39,0,26.05,9.8,39.42,25.82L125,101.34a4,4,0,0,0,2,6.54l28.3,7.08-7.25,36.26a4,4,0,0,0,3.14,4.7,3.44,3.44,0,0,0,.78.08,4,4,0,0,0,3.92-3.22l8-40a4,4,0,0,0-2.95-4.66l-25.58-6.4L172.44,60.2q2.16,2.91,4.23,6C194,92.27,204,123.54,204,152A76.08,76.08,0,0,1,128,228Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M190,57.34C171.06,29,147.88,12,128,12S84.94,29,66,57.34C46.94,86,36,120.46,36,152a92,92,0,0,0,184,0C220,120.46,209.06,86,190,57.34ZM128,220a68.07,68.07,0,0,1-68-68c0-61.12,46.19-116,68-116,7.59,0,18.13,6.65,28.64,17.88L119.05,96a12,12,0,0,0,6,19.63l20.9,5.23-5.76,28.78a12,12,0,0,0,9.42,14.12A11.87,11.87,0,0,0,152,164a12,12,0,0,0,11.76-9.65l8-40a12,12,0,0,0-8.86-14l-12.74-3.19,21.4-24C185.2,94.34,196,122.44,196,152A68.07,68.07,0,0,1,128,220Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M185,60.67C167.18,34,145.87,18,128,18S88.82,34,71,60.67C52.57,88.32,42,121.61,42,152a86,86,0,0,0,172,0C214,121.61,203.43,88.32,185,60.67ZM128,226a74.09,74.09,0,0,1-74-74c0-28.08,9.84-58.94,27-84.67C96.11,44.65,114.56,30,128,30c10.52,0,24.12,9,36.78,23.77L123.52,100a6,6,0,0,0,3,9.82L153,116.43l-6.87,34.39a6,6,0,0,0,4.7,7.06A6.08,6.08,0,0,0,152,158a6,6,0,0,0,5.87-4.82l8-40a6,6,0,0,0-4.42-7l-22.37-5.59,33.2-37.2q1.36,1.92,2.72,3.94c17.15,25.73,27,56.59,27,84.67A74.09,74.09,0,0,1,128,226Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M186.66,59.56C168.47,32.29,146.54,16,128,16S87.53,32.29,69.34,59.56C50.7,87.54,40,121.23,40,152a88,88,0,0,0,176,0C216,121.23,205.3,87.54,186.66,59.56ZM128,224a72.08,72.08,0,0,1-72-72c0-27.69,9.72-58.15,26.66-83.56C97.19,46.64,115.41,32,128,32c9.5,0,22.2,8.33,34.1,21.78L122,98.67a8,8,0,0,0,4,13.09l24.6,6.15-6.5,32.52a8,8,0,0,0,6.27,9.41A7.77,7.77,0,0,0,152,160a8,8,0,0,0,7.83-6.43l8-40a8,8,0,0,0-5.9-9.33l-19.16-4.79L172.1,66.6c.42.61.83,1.22,1.24,1.84C190.28,93.85,200,124.31,200,152A72.08,72.08,0,0,1,128,224Z"/> }.into_view()
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