use dioxus_core_macro::{inline_props, Props};

fn main() {}

type Element<'a> = ();

pub struct Scope<'a, T = ()> {
    props: &'a T,
}

// #[inline_props]
pub fn component<'a>(
    cx: Scope<'a>,
    chkk: String,
    chkk2: String,
    r: u32,
    cat: &'a str,
    drd: String,
    e: String,
) -> Element<'a> {
    let r = chkk.len();
}
