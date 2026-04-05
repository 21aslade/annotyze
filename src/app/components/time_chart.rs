use std::rc::Rc;

use annotyze::analytics::progress::BoMProgress;
use chrono::{DateTime, Days, Local};
use leptos::{ev::Event, prelude::*};
use leptos_chartistry::{
    AspectRatio, AxisMarker, Chart, Colour, IntoInner, Line, Marker, MarkerShape, Period,
    RotatedLabel, Series, TickLabels, Timestamps, Tooltip, TooltipPlacement,
};
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};

pub type Item = (DateTime<Local>, BoMProgress);

#[component]
pub fn TimeChart(annotations: Vec<Item>) -> impl IntoView {
    let min_start = annotations[0].0;
    let max_end = annotations[annotations.len() - 1].0;
    let (start, set_start) = signal(min_start);
    let (end, set_end) = signal(max_end);

    let data = move || {
        let start_n = annotations
            .binary_search_by_key(&start.get(), |&(t, _)| t)
            .unwrap_or_else(|e| e);
        let end_n = annotations
            .binary_search_by_key(&end.get(), |&(t, _)| t)
            .unwrap_or_else(|e| e);
        annotations[start_n..end_n].to_vec()
    };

    let series = Series::new(|&(time, _): &Item| time.with_timezone(&Local))
        .with_y_range(0.0, 1.0)
        .line(
            Line::new(|&(_, progress): &Item| progress.percentage())
                .with_width(0.0)
                .with_marker(
                    Marker::from_shape(MarkerShape::Square)
                        .with_border(Colour::from_rgb(0x45, 0x85, 0x88))
                        .with_border_width(2.3),
                ),
        );
    view! {
        <div class="p-5 flex flex-col space-y-5">
            <Chart
                aspect_ratio=AspectRatio::from_env_width_apply_ratio(2.0)
                series=series
                data=data
                top=RotatedLabel::middle("Book of Mormon Annotations")
                left=vec![
                    RotatedLabel::middle("Book Progress").into(),
                    TickLabels::aligned_floats()
                        .with_format(|t, _| { format!("{}%", (*t * 100.0) as usize) })
                        .into(),
                ]
                bottom=vec![
                    TickLabels::from_generator(
                            Timestamps::from_periods([Period::Month, Period::Year]),
                        )
                        .into(),
                    RotatedLabel::middle("Annotation Date").into(),
                ]
                inner=[AxisMarker::left_edge().into_inner(), AxisMarker::bottom_edge().into_inner()]
                tooltip=Tooltip::new(
                    TooltipPlacement::LeftCursor,
                    TickLabels::from_generator(
                            Timestamps::from_periods([Period::Month, Period::Year]),
                        )
                        .with_format(|t, _| t.date_naive().format("%b %-d, %Y").to_string()),
                    TickLabels::aligned_floats()
                        .with_format(|t, _| format!("{}", BoMProgress::from_percentage(*t))),
                )
            />
            <DateSliders
                start=min_start
                end=max_end
                on_change=move |s, e| {
                    set_start.set(s);
                    set_end.set(e);
                }
            />
        </div>
    }
}

#[component]
fn DateSliders(
    start: impl Into<Signal<DateTime<Local>>>,
    end: impl Into<Signal<DateTime<Local>>>,
    on_change: impl Fn(DateTime<Local>, DateTime<Local>) + 'static,
) -> impl IntoView {
    let min_start = start.into();
    let max_end = end.into();

    let min_num = 0usize;
    let max_num = move || {
        max_end
            .get()
            .signed_duration_since(min_start.get())
            .num_weeks()
            .max(1) as usize
    };

    let (min, set_min) = signal(min_num);
    let (max, set_max) = signal(max_num());

    let min_slide = move |e: Event| {
        let input: HtmlInputElement = e.target().unwrap().unchecked_into();
        let val = input
            .value()
            .parse::<usize>()
            .expect("slider wasn't a number");
        set_max.maybe_update(|m| {
            if *m <= val {
                *m = val + 1;
                true
            } else {
                false
            }
        });
        set_min.set(val);
    };

    let max_slide = move |e: Event| {
        let input: HtmlInputElement = e.target().unwrap().unchecked_into();
        let val = input
            .value()
            .parse::<usize>()
            .expect("slider wasn't a number");
        set_min.maybe_update(|m| {
            if *m >= val {
                *m = val - 1;
                true
            } else {
                false
            }
        });
        set_max.set(val);
    };

    let num_date = move |n: usize| {
        min_start
            .get()
            .checked_add_days(Days::new(n as u64 * 7))
            .unwrap_or_else(|| max_end.get())
    };

    let on_change = Rc::new(on_change);
    let commit = move |_: Event| {
        let start = num_date(min.get());
        let end = num_date(max.get());
        on_change(start, end);
    };

    view! {
        <div>
            <div class="flex items-center w-full space-x-3 justify-between">
                <input
                    type="range"
                    class="w-3/4"
                    min=min_num
                    max=move || max_num() - 1
                    prop:value=min
                    on:input=min_slide
                    on:change=commit.clone()
                />
                <label>{move || num_date(min.get()).format("%b %d, %Y").to_string()}</label>
            </div>
            <div class="flex items-center w-full space-x-3 justify-between">
                <input
                    type="range"
                    class="w-3/4"
                    min=min_num + 1
                    max=move || max_num()
                    prop:value=max
                    on:input=max_slide
                    on:change=commit
                />
                <label>{move || num_date(max.get()).format("%b %d, %Y").to_string()}</label>
            </div>
        </div>
    }
}
