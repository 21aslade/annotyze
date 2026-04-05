use annotyze::analytics::LibraryItem;
use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn Summary<'a>(item: &'a dyn LibraryItem) -> impl IntoView + use<> {
    let name = item.name();
    let count = item.count();
    let children = item
        .children()
        .map(|c| view! { <Summary item=c /> })
        .collect::<Vec<_>>();
    let has_children = !children.is_empty();

    let (expanded, set_expanded) = signal::<bool>(false);
    let on_click = move |_| {
        if has_children {
            set_expanded.update(|v| *v = !*v);
        }
    };

    let row_class = move || {
        if expanded.get() {
            "flex justify-between w-full hover:bg-gb-bg1 active:bg-gb-bg2 border-b border-b-gb-bg1 pr-2"
        } else {
            "flex justify-between w-full hover:bg-gb-bg1 active:bg-gb-bg2 pr-2"
        }
    };

    let child_class = move || {
        if expanded.get() {
            "px-3 divide-y divide-gb-bg1"
        } else {
            "hidden"
        }
    };

    let icon_class = if has_children {
        "text-2xl"
    } else {
        "text-2xl invisible"
    };

    let right = i::BiChevronRightRegular;
    let down = i::BiChevronDownRegular;
    let icon = move || if expanded.get() { down } else { right };
    view! {
        <div class="w-full py-1">
            <div class=row_class on:click=on_click>
                <span class="flex items-center space-x-5">
                    <span class=icon_class>
                        <Icon width="1em" height="1em" icon=icon />
                    </span>
                    {name}
                </span>
                <span>{count}</span>
            </div>
            <div class=child_class>{children}</div>
        </div>
    }
}
