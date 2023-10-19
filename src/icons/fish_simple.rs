/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn FishSimple(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M168,76a12,12,0,1,1-12-12A12,12,0,0,1,168,76Zm31.54,90.77c-27.06,27-69.42,38.35-126.32,33.95q2.39,17.84,6.7,37.57a8,8,0,0,1-6.11,9.52,7.81,7.81,0,0,1-1.72.19,8,8,0,0,1-7.8-6.29q-4.91-22.43-7.39-42.64-20.2-2.49-42.61-7.39a8,8,0,0,1,3.42-15.63q19.71,4.3,37.54,6.7c-4.39-56.89,7-99.24,34-126.29C133,12.58,202.43,24.9,215.9,27.82A16.07,16.07,0,0,1,228.18,40.1C231.1,53.57,243.43,123,199.54,166.77Zm13-123.28a167.84,167.84,0,0,0-49.25-2.78c-20.24,2-37.4,7.83-51.25,17.46A88,88,0,0,0,197.83,144c9.62-13.85,15.49-31,17.46-51.25A169,169,0,0,0,212.54,43.49Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M201.9,152c-21.4,28-62.18,47.89-137.78,39.87C56.11,116.26,76,75.49,104,54.1c0,.63,0,1.26,0,1.9a96,96,0,0,0,96,96Z" opacity="0.2"/><path d="M168,76a12,12,0,1,1-12-12A12,12,0,0,1,168,76Zm31.54,90.77c-27.06,27-69.42,38.35-126.32,33.95q2.39,17.84,6.7,37.57a8,8,0,0,1-6.11,9.52,7.81,7.81,0,0,1-1.72.19,8,8,0,0,1-7.8-6.29q-4.91-22.43-7.39-42.64-20.2-2.49-42.61-7.39a8,8,0,0,1,3.42-15.63q19.71,4.3,37.54,6.7c-4.39-56.89,7-99.24,34-126.29C133,12.58,202.43,24.9,215.9,27.82A16.07,16.07,0,0,1,228.18,40.1C231.1,53.57,243.43,123,199.54,166.77Zm-15-7.89A104.12,104.12,0,0,1,97.13,71.41C75.56,95.76,67,133.67,71.42,184.55,122.31,189,160.22,180.44,184.57,158.88Zm28-115.39a167.84,167.84,0,0,0-49.25-2.78c-20.24,2-37.4,7.83-51.25,17.46A88,88,0,0,0,197.83,144c9.62-13.85,15.49-31,17.46-51.25A169,169,0,0,0,212.54,43.49Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M164,76a8,8,0,1,1-8-8A8,8,0,0,1,164,76ZM196.71,164c-27,26.91-69.93,37.74-128.08,32.37q2.43,20.2,7.37,42.82a4,4,0,0,1-3,4.77,4.19,4.19,0,0,1-.86.09,4,4,0,0,1-3.9-3.15,453.52,453.52,0,0,1-7.7-45.37,456,456,0,0,1-45.34-7.71A4,4,0,1,1,16.86,180q22.59,4.95,42.78,7.38C54.3,129.21,65.13,86.26,92,59.3c42.31-42.41,109.88-30.4,123-27.56A12.09,12.09,0,0,1,224.26,41C227.1,54.08,239.11,121.64,196.71,164Zm-5.65-5.66c.81-.82,1.6-1.66,2.38-2.5a100,100,0,0,1-93.23-93.24c-.85.79-1.69,1.58-2.51,2.4C72.27,90.43,62.25,131.78,67.75,188.22,124.2,193.73,165.56,183.72,191.06,158.29ZM216.44,42.65a4,4,0,0,0-3.09-3.09,173.25,173.25,0,0,0-50.46-2.82C141.1,38.86,122.7,45.38,108,56.14A92,92,0,0,0,199.86,148c10.76-14.7,17.28-33.1,19.4-54.9A173.21,173.21,0,0,0,216.44,42.65Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M172,76a16,16,0,1,1-16-16A16,16,0,0,1,172,76Zm30.37,93.61C175.27,196.65,133.52,208.5,77.86,205q2.27,15.48,6,32.39a12,12,0,1,1-23.45,5.13q-4.56-20.9-7.05-39.92-19-2.49-39.89-7.06a12,12,0,0,1,5.13-23.45q16.89,3.7,32.36,6c-3.44-55.64,8.41-97.37,35.46-124.48C131.65,8.26,202.93,20.9,216.76,23.89a20.1,20.1,0,0,1,15.35,15.35C235.1,53.07,247.74,124.35,202.38,169.6Zm-27.09-8.43a107.09,107.09,0,0,1-51.65-28.81A107.23,107.23,0,0,1,94.82,80.71c-16.49,23-23.08,56.51-19.7,100.14C118.75,184.24,152.27,177.66,175.29,161.17ZM209.16,46.84a165.65,165.65,0,0,0-45.47-2.16c-18.67,1.81-34.58,7-47.55,15.54a83.92,83.92,0,0,0,79.64,79.64c8.51-13,13.73-28.88,15.54-47.55A165.65,165.65,0,0,0,209.16,46.84Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M166,76a10,10,0,1,1-10-10A10,10,0,0,1,166,76Zm32.13,89.36c-27,27-69.69,38-127.22,33.17q2.41,19,7,40.19a6,6,0,0,1-4.58,7.14,6.29,6.29,0,0,1-1.29.14,6,6,0,0,1-5.85-4.72q-5.08-23.21-7.55-44-20.79-2.49-44-7.55A6,6,0,0,1,17.28,178q21.15,4.63,40.16,7.05C52.57,127.54,63.67,84.89,90.63,57.88c43.05-43.15,111.54-31,124.84-28.1a14.06,14.06,0,0,1,10.75,10.75C229.1,53.82,241.27,122.32,198.13,165.36Zm-9.06-7.93a102.05,102.05,0,0,1-90.5-90.5c-24.34,24.85-34,64.91-29,119.46C124.15,191.45,164.21,181.77,189.07,157.43ZM214.49,43.07a2,2,0,0,0-1.56-1.56,171,171,0,0,0-49.84-2.78c-21,2-38.8,8.24-53.08,18.43A90,90,0,0,0,198.84,146c10.19-14.28,16.39-32.06,18.43-53.08A171,171,0,0,0,214.49,43.07Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M168,76a12,12,0,1,1-12-12A12,12,0,0,1,168,76Zm31.54,90.77c-27.06,27-69.42,38.35-126.32,33.95q2.39,17.84,6.7,37.57a8,8,0,0,1-6.11,9.52,7.81,7.81,0,0,1-1.72.19,8,8,0,0,1-7.8-6.29q-4.91-22.43-7.39-42.64-20.2-2.49-42.61-7.39a8,8,0,0,1,3.42-15.63q19.71,4.3,37.54,6.7c-4.39-56.89,7-99.24,34-126.29C133,12.58,202.43,24.9,215.9,27.82A16.07,16.07,0,0,1,228.18,40.1C231.1,53.57,243.43,123,199.54,166.77Zm-15-7.89A104.12,104.12,0,0,1,97.13,71.41C75.56,95.76,67,133.67,71.42,184.55,122.31,189,160.22,180.44,184.57,158.88Zm28-115.39a167.84,167.84,0,0,0-49.25-2.78c-20.24,2-37.4,7.83-51.25,17.46A88,88,0,0,0,197.83,144c9.62-13.85,15.49-31,17.46-51.25A169,169,0,0,0,212.54,43.49Z"/> }.into_view()
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