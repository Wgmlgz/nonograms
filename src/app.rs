use std::collections::HashMap;

use crate::nonogram::*;
use fancy_regex::Regex;
use gloo_console::log;
use gloo_net::http::Request;
use serde_wasm_bindgen::to_value;
use yew::prelude::*;

async fn fetch_non(url: &str) -> Board {
    let fetched = Request::get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let keys: String = [
        "catalogue",
        "title",
        "by",
        "copyright",
        "license",
        "color",
        "height",
        "width",
        "rows",
        "columns",
        "goal",
    ]
    .iter()
    .cloned()
    .collect::<Vec<_>>()
    .join("|");

    // let fetched: String = r#"catalogue "webpbn.com #1"
    // title "Demo Puzzle from Front Page"
    // by "Jan Wolter"
    // copyright "&copy; Copyright 2004 by Jan Wolter"
    // license CC-BY-3.0
    // width 5
    // height 10

    // rows
    // 2
    // 2,1
    // 1,1
    // 3
    // 1,1
    // 1,1
    // 2
    // 1,1
    // 1,2
    // 2

    // columns
    // 2,1
    // 2,1,3
    // 7
    // 1,3
    // 2,1

    // goal "01100011010010101110101001010000110010100101111000""#.into();

    let re = Regex::new(format!("({keys})([\\s\\S]+?)(?={keys}|$)").as_str()).unwrap();

    let text = fetched.clone();
    let dict = re
        .captures_iter(text.as_str())
        .filter_map(|c| {
            let c = c.unwrap();
            let key = c.get(1);
            let val = c.get(2);
            if let (Some(key), Some(val)) = (key, val) {
                Some((key.as_str(), val.as_str().trim().trim_matches('"')))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    // log!(keys.clone());
    // log!(fetched.clone());
    // log!(to_value(&dict).unwrap());
    // log!(url);

    let parse_side = |len: Option<&&str>, constraints: Option<&&str>| {
        let len = len.unwrap().parse::<usize>().unwrap();
        let constraints = constraints
            .unwrap()
            .split('\n')
            .filter_map(|s| {
                let arr = s
                    .trim()
                    .split(',')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<Vec<_>>();
                (arr.len() != 0).then(|| arr)
            })
            .collect::<Vec<_>>();
        Side { len, constraints }
    };

    let x = parse_side(dict.get("width"), dict.get("columns"));
    let y = parse_side(dict.get("height"), dict.get("rows"));

    let board = Board::by_sides(x, y);

    // let size = board.x.len;
    // if let Some(goal) = dict.get("goal") {
    //     log!(goal.clone());
    //     let chars = goal.chars().collect::<Vec<_>>();
    //     for (x, v) in board.state.iter_mut().enumerate() {
    //         for (y, cell) in v.iter_mut().enumerate() {
    //             *cell = match &chars[x + y * size] {
    //                 '0' => Cell::Set(false),
    //                 '1' => Cell::Set(true),
    //                 _ => Cell::Unset,
    //             }
    //         }
    //     }
    // }

    log!(to_value(&board.clone()).unwrap());
    board
    // Board::new(15, 20)
}

#[function_component(App)]
pub fn app() -> Html {
    let board = use_state(|| Board::new(10, 10));
    {
        let s = board.clone();
        use_effect_with_deps(
            move |_| {
                let s = s.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos = fetch_non("https://raw.githubusercontent.com/mikix/nonogram-db/master/db/qnonograms/examples/candle.non")
                        .await;
                    s.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }

    let width = "500px";
    let size = format!("calc({} / {})", width, board.x.len.max(board.y.len));

    let make_constraints = |side: &Side| {
        html! {
            <>
                {side.constraints.iter().map(|v| {
                    html!{
                        <tr class="constraints-row">
                        {v.iter().map(|item| {
                            html!{
                                <td class="cell" style={
                                    format!("width: {size}; height: {size}")
                                } >
                                    {item}
                                </td>
                            }
                        }).collect::<Html>()}
                        </tr>
                    }
                }).collect::<Html>()}
            </>
        }
    };

    html! {
        <main>
            <div class="table-container">
                <div class="left-side">
                    <table class="constraints">
                        {make_constraints(&board.y)}
                    </table>
                </div>
                <div class="right-side">
                    <table class="constraints">
                        {make_constraints(&board.x)}
                    </table>
                    <table class="main-table"> {
                        board.state.iter().enumerate().map(|(row_idx, row)| {
                            html!{<tr class="main-col" key={row_idx}>{
                                row.iter().enumerate().map(|(col_idx, &cell)| {
                                    html! {
                                        <td style={
                                            format!("width: {size}; height: {size}")
                                        } class={format!("table-border cell {} {}", match cell {
                                            Cell::Unset => "unset",
                                            Cell::Set(false) => "empty",
                                            Cell::Set(true) => "filled",
                                        }, match board.solve_state {
                                            Some(SolveState{idx, is_x, ..}) if if is_x {
                                                row_idx == idx
                                            } else {
                                                col_idx == idx
                                            } => "highlighted",
                                            _ => ""
                                        })} key={col_idx}> {match cell {
                                            Cell::Set(false) => "-",
                                            _ => "",
                                        }}
                                        </td>
                                    }
                                }).collect::<Html>()
                            }</tr>}
                        }).collect::<Html>()
                    } </table>
                </div>
            </div>
            <button class="solve-btn btn" onclick={Callback::from(move |_| {
                let board_state = board.clone();
                let mut board = (*board_state).clone();
                for _ in 0..10 {
                    let _ = board.next();
                }
                log!("done");
                board_state.set(board);
            })}>
                <p>
                    {"Solve 10 Lines"}
                </p>
            </button>
        </main>
    }
}
