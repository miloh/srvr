use dioxus::prelude::*;

use crate::frontend::server_fns::{
    create_device_model, delete_device_model, get_device_models,
};
use crate::models::DeviceModel;

#[component]
pub fn DeviceModels() -> Element {
    let mut models = use_resource(move || get_device_models());
    let mut model_list: Signal<Option<Vec<DeviceModel>>> = use_signal(|| None);

    // Seed from initial fetch
    use_effect(move || {
        if let Some(Ok(ms)) = models() {
            if model_list().is_none() {
                model_list.set(Some(ms));
            }
        }
    });

    let current_models = model_list();

    rsx! {
        div { class: "mb-8 flex items-center justify-between",
            div {
                h1 { class: "text-3xl font-bold text-gray-900 tracking-tight",
                    "Device Models"
                }
                p { class: "text-gray-500 mt-1",
                    "Display profiles for TRMNL-compatible hardware"
                }
            }
        }

        // Add new model form
        AddDeviceModelForm { model_list: model_list }

        // Model list
        match current_models {
            None => rsx! {
                div { class: "bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden",
                    div { class: "flex flex-col items-center justify-center py-12 gap-3",
                        div { class: "w-6 h-6 border-2 border-gray-200 border-t-gray-900 rounded-full animate-spin" }
                        p { class: "text-sm text-gray-400", "Loading..." }
                    }
                }
            },
            Some(ref ms) if ms.is_empty() => rsx! {
                div { class: "bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden",
                    div { class: "py-16 text-center",
                        p { class: "text-gray-400 text-lg",
                            "No device models defined"
                        }
                    }
                }
            },
            Some(ref ms) => rsx! {
                div { class: "bg-white rounded-xl shadow-sm border border-gray-100",
                    div { class: "divide-y divide-gray-100",
                        for m in ms {
                            DeviceModelRow {
                                key: "{m.id}",
                                model: m.clone(),
                                model_list: model_list,
                            }
                        }
                    }
                }
            },
        }
    }
}

#[component]
fn AddDeviceModelForm(mut model_list: Signal<Option<Vec<DeviceModel>>>) -> Element {
    let mut name = use_signal(|| String::new());
    let mut width = use_signal(|| String::from("800"));
    let mut height = use_signal(|| String::from("480"));
    let mut is_virtual = use_signal(|| false);
    let mut saving = use_signal(|| false);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);

    rsx! {
        div { class: "bg-white rounded-xl shadow-sm border border-gray-100 p-6 mb-6",
            h2 { class: "text-xs font-semibold text-gray-400 uppercase tracking-wider mb-4",
                "Add Device Model"
            }
            div { class: "flex items-end gap-3 flex-wrap",
                div { class: "flex-1 min-w-[200px]",
                    label { class: "block text-sm font-medium text-gray-700 mb-1",
                        "Name"
                    }
                    input {
                        r#type: "text",
                        placeholder: "e.g. TRMNL OG 7.5\"",
                        value: "{name()}",
                        oninput: move |evt| name.set(evt.value()),
                        class: "w-full text-sm border border-gray-200 rounded-lg px-3 py-1.5 focus:outline-none focus:ring-1 focus:ring-gray-300",
                    }
                }
                div { class: "w-24",
                    label { class: "block text-sm font-medium text-gray-700 mb-1",
                        "Width"
                    }
                    input {
                        r#type: "number",
                        value: "{width()}",
                        oninput: move |evt| width.set(evt.value()),
                        class: "w-full text-sm border border-gray-200 rounded-lg px-3 py-1.5 focus:outline-none focus:ring-1 focus:ring-gray-300",
                    }
                }
                div { class: "w-24",
                    label { class: "block text-sm font-medium text-gray-700 mb-1",
                        "Height"
                    }
                    input {
                        r#type: "number",
                        value: "{height()}",
                        oninput: move |evt| height.set(evt.value()),
                        class: "w-full text-sm border border-gray-200 rounded-lg px-3 py-1.5 focus:outline-none focus:ring-1 focus:ring-gray-300",
                    }
                }
                div { class: "flex items-center gap-2 pb-1",
                    label { class: "relative inline-flex items-center cursor-pointer",
                        input {
                            r#type: "checkbox",
                            class: "sr-only peer",
                            checked: is_virtual(),
                            onchange: move |evt| is_virtual.set(evt.checked()),
                        }
                        div { class: "w-9 h-5 bg-gray-200 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-gray-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-gray-900" }
                    }
                    span { class: "text-sm text-gray-500", "Virtual" }
                }
                button {
                    class: "px-4 py-1.5 bg-gray-900 text-white text-sm font-medium rounded-lg hover:bg-gray-700 transition-colors disabled:opacity-50",
                    disabled: saving() || name().trim().is_empty(),
                    onclick: move |_| {
                        let n = name().trim().to_string();
                        let w: i64 = width().parse().unwrap_or(800);
                        let h: i64 = height().parse().unwrap_or(480);
                        let v = is_virtual();
                        saving.set(true);
                        error_msg.set(None);
                        spawn(async move {
                            match create_device_model(n, w, h, v).await {
                                Ok(new_model) => {
                                    let mut list = model_list.write();
                                    let ms = list.get_or_insert_with(Vec::new);
                                    ms.push(new_model);
                                    // Reset form
                                    name.set(String::new());
                                    width.set(String::from("800"));
                                    height.set(String::from("480"));
                                    is_virtual.set(false);
                                }
                                Err(e) => {
                                    error_msg.set(Some(format!("{e}")));
                                }
                            }
                            saving.set(false);
                        });
                    },
                    if saving() { "Adding..." } else { "Add" }
                }
            }
            if let Some(ref err) = error_msg() {
                p { class: "text-sm text-red-500 mt-2", "{err}" }
            }
        }
    }
}

#[component]
fn DeviceModelRow(
    model: DeviceModel,
    mut model_list: Signal<Option<Vec<DeviceModel>>>,
) -> Element {
    let mut deleting = use_signal(|| false);
    let model_id = model.id;

    rsx! {
        div { class: "flex items-center justify-between px-6 py-4",
            div { class: "flex items-center gap-4",
                div {
                    div { class: "flex items-center gap-2",
                        p { class: "text-sm font-medium text-gray-900",
                            "{model.name}"
                        }
                        if model.is_virtual {
                            span { class: "text-xs bg-gray-100 text-gray-500 px-1.5 py-0.5 rounded",
                                "virtual"
                            }
                        }
                    }
                    p { class: "text-xs text-gray-400 font-mono",
                        "{model.width}\u{00d7}{model.height}"
                    }
                }
            }
            button {
                class: "px-3 py-1 text-xs font-medium text-red-600 hover:bg-red-50 rounded-lg transition-colors",
                disabled: deleting(),
                onclick: move |_| {
                    deleting.set(true);
                    spawn(async move {
                        if let Ok(()) = delete_device_model(model_id).await {
                            let mut list = model_list.write();
                            if let Some(ref mut ms) = *list {
                                ms.retain(|m| m.id != model_id);
                            }
                        }
                        deleting.set(false);
                    });
                },
                if deleting() { "Deleting..." } else { "Delete" }
            }
        }
    }
}
