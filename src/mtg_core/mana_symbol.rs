use std::rc::Rc;
use dioxus::prelude::*;

/// Excellent mtg symbols created by (Goblin Hero) https://www.slightlymagic.net/forum/viewtopic.php?t=4430.


#[derive(PartialEq, Eq, Hash, Clone)]
pub enum ManaType {
    Plains,
    Island,
    Mountain,
    Forest,
    Swamp,
}

const VERSION: &str = "1.1";
const XMLNS: &str = "http://www.w3.org/2000/svg";
const X: &str = "0px";
const Y: &str = "0px";
// const WIDTH: &str = "1045px";
// const HEIGHT: &str = "730.002px";
const WIDTH: &str = "50px";
const HEIGHT: &str = "50px";
const VIEW_BOX: &str = "0 0 100 100";
//const VIEW_BOX: &str = "-945 -210.002 1045 730.002";
const ENABLE_BACKGROUND: &str = "new -945 -210.002 1045 730.002";

#[inline_props]
pub fn ManaSymbol<'a>(cx: Scope<'a>, mana_type: ManaType, on_clicked: EventHandler<'a, bool>) -> Element<'a> {

    // Indicate if mana symbol is selected.
    let selected = use_state(&cx, || false);

    // PLAINS
    if *mana_type == ManaType::Plains {
        cx.render(rsx!(
                svg { version: VERSION , xmlns: XMLNS, x: X, y: Y, width: WIDTH, height: HEIGHT, view_box: VIEW_BOX, enable_background: ENABLE_BACKGROUND,
                    // onclick: move |event| { selected.set(!*selected.get()); on_clicked.call(*selected.get()); if *selected.get() {println!("Selected");} else { println!("Not selected");}},
                    onclick: move |event| { selected.set(!*selected.get()); on_clicked.call(*selected.get()); },
                    g {
                        transform: "translate(525,0)",
                        circle { fill: "#F8F6D8", cx: "-475", cy: "50", r: "50", } 
                    path {
                        fill: if *selected.get() {"#0D0F0F"} else {"#AAAAAA"}, d: "M-427.309,57.064c-6.561-3.699-10.768-5.551-12.617-5.551c-1.344,0-2.395,1.032-3.154,3.092
                    	c-0.758,2.063-2.27,3.09-4.541,3.09c-0.926,0-2.818-0.336-5.678-1.008c-1.598,2.44-2.398,3.996-2.398,4.668
                    	c0,0.926,0.689,2.016,2.064,3.281c1.375,1.262,2.535,1.891,3.482,1.891c0.602,0,1.416-0.125,2.449-0.379
                    	c1.031-0.25,1.721-0.377,2.064-0.377c1.033,0,1.547,1.893,1.547,5.678c0,3.617-0.84,9.168-2.523,16.654
                    	c-2.188-8.58-4.5-12.871-6.938-12.871c-0.338,0-1.031,0.252-2.082,0.76c-1.053,0.502-1.83,0.754-2.334,0.754
                    	c-2.438,0-4.625-2.227-6.561-6.688c-3.869,0.59-5.805,2.567-5.805,5.934c0,1.684,0.777,3.027,2.336,4.035
                    	c1.553,1.008,2.334,1.727,2.334,2.145c0,2.273-3.324,5.764-9.969,10.473c-3.531,2.523-5.973,4.289-7.316,5.297
                    	c1.174-1.512,2.352-3.487,3.533-5.928c1.344-2.775,2.018-4.92,2.018-6.436c0-0.84-0.967-2.02-2.902-3.533
                    	c-1.936-1.512-2.9-3.111-2.9-4.793c0-1.428,0.502-3.193,1.512-5.299c-1.094-1.262-2.395-1.895-3.91-1.895
                    	c-3.365,0-5.045,1.096-5.045,3.28c0-1.514,0-0.379,0,3.406c0.082,2.776-2.02,4.164-6.311,4.164c-3.279,0-8.791-0.759-16.527-2.271
                    	c8.748-2.188,13.121-4.711,13.121-7.57c0,0.336-0.168-0.672-0.504-3.028c-0.338-2.604,1.514-4.961,5.551-7.063
                    	c-0.758-3.867-2.773-5.806-6.057-5.806c-0.504,0-1.432,0.884-2.775,2.647c-1.346,1.771-2.607,2.652-3.783,2.652
                    	c-2.02,0-4.629-2.186-7.822-6.563c-1.516-2.184-3.83-5.424-6.941-9.715c1.934,1.012,3.869,2.02,5.805,3.031
                    	c2.523,1.176,4.541,1.766,6.057,1.766c1.178,0,2.334-1.031,3.469-3.092c1.135-2.061,2.629-3.092,4.479-3.092
                    	c0.254,0,1.936,0.504,5.047,1.516c1.596-2.439,2.398-4.248,2.398-5.426c0-1.01-0.611-2.166-1.83-3.471
                    	c-1.221-1.303-2.334-1.955-3.344-1.955c-0.422,0-1.072,0.125-1.957,0.379c-0.881,0.252-1.533,0.379-1.953,0.379
                    	c-1.516,0-2.273-1.893-2.273-5.678c0-1.01,0.969-6.77,2.904-17.285c-0.086,1.26,0.461,3.617,1.639,7.064
                    	c1.43,4.207,3.111,6.309,5.049,6.309c0.334,0,1.008-0.252,2.018-0.758c1.008-0.504,1.807-0.754,2.396-0.754
                    	c1.934,0,3.531,1.094,4.795,3.277l1.893,3.406c1.766,0,3.238-0.629,4.414-1.891c1.178-1.262,1.768-2.777,1.768-4.543
                    	c0-1.85-0.777-3.26-2.334-4.227c-1.559-0.967-2.336-1.703-2.336-2.207c0-1.768,2.777-4.752,8.328-8.958
                    	c4.457-3.363,7.359-5.34,8.707-5.93c-3.617,4.879-5.426,8.451-5.426,10.724c0,1.178,0.713,2.441,2.145,3.785
                    	c1.766,1.598,2.775,2.734,3.027,3.406c0.84,1.938,0.756,4.586-0.252,7.949c2.271,1.6,3.994,2.396,5.174,2.396
                    	c2.436,0,3.658-1.264,3.658-3.785c0-0.252-0.105-1.051-0.314-2.396c-0.213-1.344-0.273-2.102-0.191-2.271
                    	c0.336-1.178,2.65-1.768,6.939-1.768c2.691,0,8.283,0.758,16.781,2.273c-1.852,0.504-4.627,1.26-8.326,2.27
                    	c-3.365,1.01-5.049,2.145-5.049,3.406c0,0.59,0.209,1.598,0.631,3.027c0.42,1.432,0.633,2.48,0.633,3.156
                    	c0,1.176-0.758,2.27-2.271,3.277l-4.291,3.031c1.01,1.852,1.682,2.945,2.02,3.279c0.84,1.008,1.975,1.514,3.406,1.514
                    	c1.01,0,1.934-0.883,2.775-2.648c0.84-1.768,2.188-2.65,4.037-2.65c2.27,0,4.838,2.104,7.697,6.311
                    	C-433.156,48.697-430.674,52.27-427.309,57.064z M-455.316,49.748c0-5.381-1.979-10.051-5.932-14.006
                    	c-3.953-3.953-8.621-5.93-14.004-5.93c-5.469,0-10.18,1.957-14.131,5.869c-3.953,3.91-5.973,8.6-6.055,14.066
                    	c-0.086,5.383,1.912,10.03,5.992,13.938c4.08,3.912,8.811,5.869,14.193,5.869c5.719,0,10.492-1.873,14.318-5.615
                    	C-457.105,60.199-455.234,55.469-455.316,49.748z M-457.209,49.748c0,5.131-1.725,9.381-5.174,12.74
                    	c-3.451,3.367-7.74,5.049-12.869,5.049c-4.963,0-9.211-1.723-12.742-5.174c-3.531-3.445-5.299-7.652-5.299-12.615
                    	c0-4.877,1.785-9.064,5.359-12.553c3.578-3.49,7.803-5.238,12.682-5.238c4.877,0,9.104,1.766,12.68,5.301
                    	C-458.998,40.791-457.209,44.953-457.209,49.748z",
                     }}
                }))
    }
                // ISLAND
    else if *mana_type == ManaType::Island {
        cx.render(rsx!(
                svg { version: VERSION , xmlns: XMLNS, x: X, y: Y, width: WIDTH, height: HEIGHT, view_box: VIEW_BOX, enable_background: ENABLE_BACKGROUND,
                    onclick: move |event| { selected.set(!*selected.get()); on_clicked.call(*selected.get()); },
                    g {
                        transform: "translate(420,0)",
                        circle { fill: "#C1D7E9", cx: "-370", cy: "50", r: "50", }
                        path {
                            fill: if *selected.get() {"#0D0F0F"} else {"#AAAAAA"},
                            d: "M-352.512,83.719c-4.787,4.871-10.684,7.307-17.688,7.307c-7.861,0-14.098-2.69-18.711-8.073
                                    c-4.359-5.127-6.537-11.662-6.537-19.606c0-8.543,3.717-18.286,11.15-29.224c6.064-8.969,13.199-16.83,21.402-23.58
                                    c-1.197,5.469-1.793,9.355-1.793,11.662c0,5.299,1.664,10.467,4.996,15.508c4.102,5.98,7.219,10.426,9.357,13.328
                                    c3.332,5.043,4.998,9.955,4.998,14.737C-345.336,72.871-347.729,78.852-352.512,83.719z M-352.641,56.357
                                    c-1.281-2.861-2.777-4.762-4.486-5.703c0.256,0.514,0.385,1.24,0.385,2.18c0,1.795-0.512,4.357-1.539,7.689l-1.664,5.127
                                    c0,2.99,1.492,4.486,4.484,4.486c3.16,0,4.742-2.095,4.742-6.281C-350.719,61.721-351.359,59.223-352.641,56.357z",
                        }
                    }
                }))
    }
    else if *mana_type == ManaType::Swamp {
        cx.render(rsx!(
                svg { version: VERSION , xmlns: XMLNS, x: X, y: Y, width: WIDTH, height: HEIGHT, view_box: VIEW_BOX, enable_background: ENABLE_BACKGROUND,
                    onclick: move |event| { selected.set(!*selected.get()); on_clicked.call(*selected.get()); },
                    g {
                        transform: "translate(315,0)",
                        circle { fill: "#BAB1AB", cx: "-265", cy: "49.998", r: "50", }
                        path {
                            fill: if *selected.get() {"#0D0F0F"} else {"#AAAAAA"},
                            d: "M-224.305,48.619c0,5.518-2.008,9.281-6.02,11.287c-1.172,0.586-4.85,1.379-11.037,2.383
                                    c-4.012,0.67-6.018,2.217-6.018,4.639v10.158c0,0.422,0.125,1.715,0.375,3.889l0.377,4.014c0,1.255-0.293,3.306-0.879,6.146
                                    c-1.588,0.334-3.428,0.709-5.518,1.132c-0.67-2.511-1.004-4.224-1.004-5.146c0-0.416,0.105-1.045,0.313-1.882
                                    c0.207-0.834,0.316-1.461,0.316-1.883c0-0.58-0.52-2.213-1.559-4.887h-1.945c-0.258,0.418-0.344,0.961-0.26,1.629
                                    c0.334,1.422,0.459,2.633,0.377,3.637c-1.422,1.004-3.387,2.341-5.895,4.013c-0.586-0.166-0.793-0.25-0.629-0.25v-8.904
                                    c-0.164-0.416-0.584-0.581-1.254-0.502h-1.504l-1.504,11.787c-1.174,0.084-2.592,0.084-4.264,0
                                    c-0.588-2.758-1.631-6.853-3.135-12.289h-1.004c-0.922,2.929-1.422,4.519-1.506,4.769c0,0.334,0.104,0.981,0.314,1.942
                                    c0.207,0.962,0.313,1.609,0.313,1.943c0,0.25-0.084,0.877-0.25,1.881l-0.377,3.01c-0.168,0.166-0.377,0.25-0.627,0.25
                                    c-2.508,0-4.182-0.627-5.016-1.879c-0.836-1.256-1.172-3.012-1.004-5.271l1.004-15.047c0-0.252,0.082-0.586,0.25-1.004
                                    c0.164-0.418,0.25-0.711,0.25-0.877c0-0.67-0.711-2.008-2.131-4.014c-0.248-0.082-1.549-0.377-3.887-0.879
                                    c-1.424-0.334-4.225-0.918-8.402-1.756c-5.771-1.084-8.654-5.725-8.654-13.92c0-12.207,5.018-22.365,15.051-30.475
                                    c0.414,2.258,1.127,5.266,2.129,9.029c0.754,0.17,2.385,0.545,4.891,1.129c0.504,0.168,3.053,1.088,7.652,2.76
                                    c-2.344-1.422-5.393-3.719-9.156-6.898c-1.422-1.672-2.133-4.471-2.133-8.4c0-0.92,1.59-2.008,4.768-3.264
                                    c2.84-1.17,4.975-1.836,6.396-2.006c4.514-0.582,7.984-0.879,10.41-0.879c10.449,0,18.891,2.678,25.328,8.029
                                    c-2.088,2.426-5.684,5.014-10.783,7.773c2.008,0.084,4.934-0.707,8.779-2.383c3.844-1.67,5.475-2.508,4.891-2.508
                                    c0.668,0,2.008,1.34,4.014,4.014c1.504,2.006,2.715,3.807,3.637,5.391c2.674,4.768,4.471,9.908,5.393,15.426
                                    c0,1.926,0.041,3.305,0.125,4.139v1.004H-224.305z M-272.336,50.877c0-3.594-1.568-7.002-4.703-10.223
                                    c-3.137-3.219-6.502-4.826-10.096-4.826c-3.178,0-5.977,1.348-8.402,4.039c-2.426,2.693-3.637,5.682-3.637,8.963
                                    c0,2.859,1.379,4.713,4.139,5.553c1.756,0.506,4.219,0.801,7.398,0.883h6.898C-275.141,55.35-272.336,53.887-272.336,50.877z
                                     M-258.668,66.43v-3.889c-0.584-1.086-1.17-2.215-1.754-3.387c-0.502-1.674-1.422-4.014-2.76-7.025l-1.381,14.674
                                    c0,1.172-0.25,1.756-0.752,1.756c-0.334,0-0.584-0.082-0.752-0.248c-0.586-8.863-0.879-12.709-0.879-11.541v-4.387
                                    c-0.168-0.254-0.375-0.379-0.625-0.379c-2.844,2.93-4.264,7.652-4.264,14.172c0,3.596,0.33,5.811,1.002,6.648
                                    c0.67-0.166,1.422-0.459,2.258-0.877c0.334-0.168,1.295-0.252,2.887-0.252c1.584,0,3.51,0.502,5.766,1.504
                                    C-259.086,73.199-258.668,70.943-258.668,66.43z M-230.324,48.955c0-3.367-1.254-6.375-3.762-9.025
                                    c-2.51-2.648-5.395-3.975-8.652-3.975c-3.512,0-6.795,1.607-9.846,4.826c-3.053,3.219-4.578,6.584-4.578,10.096
                                    c0,2.928,1.42,4.389,4.264,4.389h14.422C-233.043,55.184-230.324,53.08-230.324,48.955z",
                        }
                    }
                }))
    }
    else if *mana_type == ManaType::Mountain {
        cx.render(rsx!(
                svg { version: VERSION , xmlns: XMLNS, x: X, y: Y, width: WIDTH, height: HEIGHT, view_box: VIEW_BOX, enable_background: ENABLE_BACKGROUND,
                    onclick: move |event| { selected.set(!*selected.get()); on_clicked.call(*selected.get()); },
                    g {
                        transform: "translate(210,0)",
                        circle { fill: "#E49977", cx: "-160", cy: "50", r: "50", }
                                 path {
                                 fill: if *selected.get() {"#0D0F0F"} else {"#AAAAAA"},
                                 d: "M-118.035,66.617c-3.736,8.912-11.16,13.367-22.275,13.367c-2.037,0-4.246,0.254-6.621,0.762
                                                c-3.564,0.764-5.346,1.828-5.346,3.186c0,0.424,0.295,0.91,0.891,1.463c0.592,0.553,1.104,0.826,1.527,0.826
                                                c-2.123,0-0.68,0.064,4.326,0.191c5.008,0.127,8.148,0.191,9.422,0.191c-7.383,4.326-19.732,6.319-37.043,5.981
                                                c-5.688-0.084-10.566-2.588-14.639-7.51c-3.992-4.669-5.984-9.888-5.984-15.658c0-6.108,2.057-11.308,6.176-15.595
                                                c4.113-4.282,9.229-6.427,15.338-6.427c1.357,0,3.16,0.297,5.41,0.891c2.248,0.594,3.756,0.891,4.518,0.891
                                                c3.139,0,7.045-1.293,11.713-3.883c4.666-2.588,6.875-3.883,6.621-3.883c-0.85,8.912-3.82,14.896-8.914,17.948
                                                c-3.648,2.123-5.473,4.201-5.473,6.236c0,1.273,0.764,2.293,2.291,3.057c1.188,0.595,2.502,0.892,3.945,0.892
                                                c2.207,0,4.371-1.356,6.494-4.071c2.119-2.718,3.055-5.177,2.801-7.386c-0.254-2.545-0.084-5.603,0.51-9.164
                                                c0.168-1.02,0.783-2.27,1.844-3.754c1.061-1.486,2.016-2.398,2.865-2.738c0,0.762-0.275,2.037-0.828,3.818
                                                c-0.553,1.781-0.826,3.1-0.826,3.947c0,1.867,0.508,3.309,1.527,4.326c1.525-0.592,2.883-2.502,4.074-5.729
                                                c1.016-2.459,1.609-4.836,1.781-7.127c-3.566-0.17-6.982-1.781-10.248-4.838c-3.268-3.057-4.9-6.365-4.9-9.928
                                                c0-0.594,0.082-1.188,0.256-1.783c0.508,0.764,1.271,1.953,2.289,3.564c1.443,2.121,2.547,3.182,3.313,3.182
                                                c1.016,0,1.525-1.061,1.525-3.182c0-2.715-0.723-5.176-2.164-7.383c-1.613-2.631-3.693-3.947-6.238-3.947
                                                c-1.189,0-2.971,0.637-5.344,1.91c-2.379,1.271-4.543,1.91-6.492,1.91c-0.596,0-3.229-0.766-7.895-2.293
                                                c8.23-1.355,12.348-2.586,12.348-3.691c0-2.885-5.645-4.838-16.93-5.855c-1.105-0.084-3.141-0.254-6.111-0.51
                                                c0.338-0.424,2.758-0.891,7.258-1.4c3.818-0.422,6.492-0.637,8.018-0.637c20.197,0,33.012,9.805,38.443,29.408
                                                c0.934-0.773,1.402-2.066,1.402-3.871c0-2.324-0.68-5.25-2.037-8.777c-0.512-1.375-1.318-3.441-2.42-6.193
                                                c6.957,8.867,10.439,17.27,10.439,25.199c0,4.178-0.979,7.973-2.93,11.381c-1.27,2.303-3.65,5.244-7.127,8.826
                                                c-3.48,3.58-5.857,6.352-7.131,8.313c4.668-1.271,7.725-2.248,9.168-2.928c3.223-1.44,6.15-3.606,8.783-6.492
                                                C-116.635,62.756-117.102,64.412-118.035,66.617z M-173.537,16.592c0,1.525-0.85,2.502-2.545,2.926l-3.311,0.51
                                                c-1.189,0.594-2.928,2.928-5.219,7c-0.256-1.271-0.637-3.053-1.146-5.346c-0.764,0.086-2.035,0.764-3.818,2.037
                                                c-0.764,0.594-1.996,1.484-3.693,2.672c0.512-3.055,2.207-6.148,5.094-9.293c3.055-3.477,6.025-5.217,8.91-5.217
                                                C-175.447,11.881-173.537,13.453-173.537,16.592z M-151.387,28.301c0,1.443-0.785,2.654-2.355,3.629
                                                c-1.57,0.977-3.119,1.465-4.646,1.465c-2.037,0-3.863-1.146-5.473-3.438c-1.955-2.801-3.947-4.625-5.984-5.477
                                                c0.424-0.422,0.934-0.635,1.529-0.635c0.764,0,2.055,0.594,3.881,1.781c1.824,1.189,2.99,1.783,3.502,1.783
                                                c0.424,0,1.123-0.594,2.1-1.783c0.975-1.188,2.057-1.781,3.246-1.781C-152.787,23.846-151.387,25.332-151.387,28.301z",
                                 }
                     }
                }))
    }
    else {
        cx.render(rsx!(
                svg { version: VERSION , xmlns: XMLNS, x: X, y: Y, width: WIDTH, height: HEIGHT, view_box: VIEW_BOX, enable_background: ENABLE_BACKGROUND,
                    onclick: move |event| { selected.set(!*selected.get()); on_clicked.call(*selected.get()); },
                    g {
                        transform: "translate(105,0)",
                        path { fill: "#A3C095", d: "M-5,49.998C-5,77.613-27.385,100-55.002,100C-82.615,100-105,77.613-105,49.998
                                                    C-105,22.385-82.615,0-55.002,0C-27.385,0-5,22.385-5,49.998z",
                        }
                        path {
                            fill: if *selected.get() {"#0D0F0F"} else {"#AAAAAA"},
                            d: "M-11.238,56.225c0,1.668-0.645,3.164-1.936,4.498c-1.289,1.332-2.77,1.998-4.436,1.998
                                        c-2.662,0-4.623-1.25-5.869-3.748l-5.871-0.25c-1.252,0-3.709,0.543-7.371,1.625c-3.914,1.082-6.164,1.957-6.746,2.623
                                        c-0.916,0.998-1.664,3.332-2.248,6.996c-0.502,2.998-0.748,5.205-0.748,6.621c0,2.246,0.352,3.893,1.061,4.934
                                        s2.166,1.916,4.371,2.623c2.205,0.707,3.561,1.104,4.061,1.187c0.332,0,0.873-0.041,1.625-0.125h1.498c1.08,0,2.205,0.17,3.373,0.5
                                        c1.666,0.5,2.375,1.166,2.125,2c-1.168-0.166-3.207,0.084-6.121,0.75l3.496,1.748c0,1-1.416,1.498-4.246,1.498
                                        c-0.752,0-1.771-0.166-3.063-0.498c-1.291-0.336-2.145-0.5-2.559-0.5h-1.625c-0.082,0.832-0.334,2.08-0.75,3.746
                                        c-1.418-0.084-3.08-0.918-4.996-2.498c-1.918-1.58-3.123-2.373-3.621-2.373c-0.502,0-1.211,0.793-2.125,2.373
                                        c-0.918,1.58-1.375,2.664-1.375,3.248c-1.082-0.584-1.996-1.668-2.75-3.248c-0.332-1.084-0.707-2.166-1.121-3.248
                                        c-0.832,0.084-2.375,1.834-4.621,5.248h-0.627c-0.166-0.252-0.795-2-1.873-5.248c-2.582-0.832-4.996-1.248-7.246-1.248
                                        c-1.082,0-2.748,0.25-4.996,0.748l-3.496-0.248c0.498-0.5,1.955-1.457,4.371-2.873c2.83-1.666,4.996-2.5,6.496-2.5
                                        c0.246,0,0.578,0.043,1,0.125c0.414,0.086,0.75,0.125,1,0.125c0.578,0,1.518-0.312,2.809-0.938c1.291-0.623,2.039-1.186,2.246-1.684
                                        c0.211-0.504,0.316-1.793,0.316-3.875c0-4.746-1.25-8.285-3.75-10.617c-2.168-2.082-5.746-3.58-10.744-4.498
                                        c-1.332,4.746-5.08,7.123-11.24,7.123c-2,0-3.998-1.207-5.996-3.623c-1.996-2.416-2.996-4.623-2.996-6.621
                                        c0-3.082,1.287-5.621,3.869-7.623c-2.08-2.162-3.121-4.369-3.121-6.617c0-2.084,0.643-3.914,1.936-5.5
                                        c1.291-1.578,2.977-2.496,5.059-2.748c-0.166-2.662,0.707-4.496,2.623-5.496c-0.916-0.914-1.373-2.537-1.373-4.869
                                        c0-2.748,0.916-5.039,2.748-6.871c1.83-1.832,4.121-2.75,6.869-2.75c3,0,5.457,1.045,7.371,3.125
                                        c2.416-8.244,7.621-12.367,15.613-12.367c4.164,0,7.828,1.666,10.994,4.998c1.166,1.248,1.748,1.916,1.748,1.996
                                        c-1,0-0.498-0.188,1.5-0.561c1.996-0.375,3.453-0.563,4.373-0.563c3.246,0,6.119,1.207,8.619,3.623
                                        c2.164,2.166,3.664,4.912,4.498,8.244c0.58,0.084,1.498,0.332,2.748,0.748c1.83,0.92,2.748,2.498,2.748,4.748
                                        c0,0.418-0.336,1.209-1,2.373c5.328,2.998,7.994,7.162,7.994,12.492c0,1.498-0.582,3.584-1.748,6.247
                                        C-12.318,51.977-11.238,53.811-11.238,56.225z M-62.705,61.721v-1.623c0-1.914-0.936-3.664-2.809-5.246
                                        c-1.875-1.582-3.77-2.373-5.684-2.373c-2.334,0-4.496,0.541-6.496,1.621C-73.281,53.852-68.283,56.393-62.705,61.721z
                                        M-64.951,46.232c-1.25-1.418-2.332-2.875-3.25-4.373c-3.498,0.916-5.246,1.957-5.246,3.121c1-0.08,2.457,0.105,4.371,0.564
                                        C-67.162,46.003-65.785,46.232-64.951,46.232z M-57.33,42.359v-5.496c-2-0.332-3.211-0.5-3.623-0.5v1.873L-57.33,42.359z
                                        M-41.092,38.861c-1-0.416-2.875-1.25-5.621-2.498v10.742C-42.801,44.855-40.928,42.107-41.092,38.861z M-34.225,53.602
                                        l-2.746-3.373c-1.664,1.167-3.352,2.354-5.061,3.561c-1.709,1.207-3.186,2.563-4.432,4.06
                                        C-42.717,55.848-38.635,54.436-34.225,53.602z", 
                        }
                    }
                }))
    }
}
