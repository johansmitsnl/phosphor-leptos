/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn GoogleDriveLogo(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M237.6,151.78,169.13,39.52A15.91,15.91,0,0,0,155.56,32H100.43a15.89,15.89,0,0,0-13.56,7.52l-.05.07L18.44,151.7a16,16,0,0,0-.33,16.42l27.32,47.82A16,16,0,0,0,59.32,224H196.67a16,16,0,0,0,13.89-8.06l27.32-47.82A15.91,15.91,0,0,0,237.6,151.78ZM219,152H172.52L137.33,93.33l22.75-37.92ZM92.53,168h70.94l24,40H68.53Zm9.6-16L128,108.88,153.87,152ZM95.91,55.41l22.76,37.92L83.47,152H37Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M24,160H88L55.12,214.8A7.91,7.91,0,0,1,52.38,212L25.05,164.15A8,8,0,0,1,24,160Zm144,0,32.88,54.8a7.91,7.91,0,0,0,2.74-2.83l27.32-47.82A8,8,0,0,0,232,160ZM100.43,40a8.06,8.06,0,0,0-3.84,1L128,93.33,159.41,41a8.07,8.07,0,0,0-3.85-1Z" opacity="0.2"/><path d="M237.6,151.78,169.13,39.52A15.91,15.91,0,0,0,155.56,32H100.43a15.89,15.89,0,0,0-13.56,7.52l-.05.07L18.44,151.7a16,16,0,0,0-.33,16.42l27.32,47.82A16,16,0,0,0,59.32,224H196.67a16,16,0,0,0,13.89-8.06l27.32-47.82A15.91,15.91,0,0,0,237.6,151.78ZM219,152H172.52L137.33,93.33l22.75-37.92Zm-116.87,0L128,108.88,153.87,152Zm61.34,16,24,40H68.53l24-40ZM128,77.78,110.12,48l35.78-.05ZM95.91,55.41l22.76,37.92L83.47,152H37ZM36.54,168H73.87L54.72,199.92Zm164.74,31.93L182.12,168h37.41Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M234.19,153.86,165.74,41.64A11.94,11.94,0,0,0,155.56,36H100.43a11.93,11.93,0,0,0-10.17,5.64l0,0L21.83,153.82a12,12,0,0,0-.25,12.32L48.91,214A12,12,0,0,0,59.32,220H196.67A12,12,0,0,0,207.09,214l27.32-47.81A12,12,0,0,0,234.19,153.86ZM226.13,156H170.26l-37.6-62.67,27.39-45.65ZM95.06,156,128,101.11,160.93,156Zm70.67,8,28.8,48H61.47l28.79-48ZM128,85.56,103.06,44h49.87ZM95.94,47.68l27.39,45.65L85.74,156H29.87ZM29.58,164H80.94L54.63,207.85Zm171.79,43.85L175.06,164h51.36Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M241,149.65,172.59,37.51l-.07-.11a19.85,19.85,0,0,0-17-9.4H100.44a19.85,19.85,0,0,0-17,9.4l-.07.11L15,149.65a20,20,0,0,0-.36,20.46L42,217.92A20,20,0,0,0,59.33,228H196.67A20,20,0,0,0,214,217.92l27.32-47.81A20,20,0,0,0,241,149.65ZM211.88,148H174.79L142,93.33l18.12-30.19ZM54.8,192,43.36,172H66.81Zm40-20h66.4l19.2,32H75.6Zm14.4-24L128,116.66,146.8,148Zm80,24h23.45L201.2,192ZM128,70,117.19,52h21.62ZM95.89,63.14,114,93.33,81.21,148H44.12Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M235.9,152.82,167.43,40.58A13.91,13.91,0,0,0,155.56,34H100.43a13.92,13.92,0,0,0-11.87,6.58l0,.05L20.13,152.76a14,14,0,0,0-.28,14.37L47.17,215A14,14,0,0,0,59.32,222H196.67A14,14,0,0,0,208.83,215l27.32-47.82A14,14,0,0,0,235.9,152.82ZM222.56,154H171.39L135,93.33l25.08-41.79Zm-124,0L128,105l29.4,49Zm66,12L191,210H65l26.4-44ZM128,81.67,106.6,46h42.8ZM95.93,51.54,121,93.33,84.6,154H33.43ZM33,166H77.4L54.67,203.89Zm168.3,37.89L178.59,166H223Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M237.6,151.78,169.13,39.52A15.91,15.91,0,0,0,155.56,32H100.43a15.89,15.89,0,0,0-13.56,7.52l-.05.07L18.44,151.7a16,16,0,0,0-.33,16.42l27.32,47.82A16,16,0,0,0,59.32,224H196.67a16,16,0,0,0,13.89-8.06l27.32-47.82A15.91,15.91,0,0,0,237.6,151.78ZM219,152H172.52L137.33,93.33l22.75-37.92Zm-116.87,0L128,108.88,153.87,152Zm61.34,16,24,40H68.53l24-40ZM128,77.78,110.12,48l35.78-.05ZM95.91,55.41l22.76,37.92L83.47,152H37ZM36.54,168H73.87L54.72,199.92Zm164.74,31.93L182.12,168h37.41Z"/> }.into_view()
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