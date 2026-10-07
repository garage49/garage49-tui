use iocraft::prelude::*;

#[derive(Default, Props)]
struct PaneProps<'a> { title: String, focused: bool, children: Vec<AnyElement<'a>> }

#[component]
fn Pane<'a>(props: &mut PaneProps<'a>) -> impl Into<AnyElement<'a>> {
    let (l, h, r, style) = if props.focused { ("┏", "━", "┓", BorderStyle::Bold) } else { ("┌", "─", "┐", BorderStyle::Single) };
    element! {
        View(flex_direction: FlexDirection::Column, width: 100pct) {
            View(flex_direction: FlexDirection::Row, height: 1) {
                View(flex_shrink: 0.0_f32) { Text(content: format!("{l}{h} ")) Text(content: props.title.clone(), weight: Weight::Bold) Text(content: " ") }
                View(flex_grow: 1.0_f32, flex_basis: FlexBasis::Length(0), overflow: Overflow::Hidden, height: 1) { Text(content: h.repeat(300), wrap: TextWrap::NoWrap) }
                View(flex_shrink: 0.0_f32) { Text(content: r) }
            }
            View(border_style: style, border_edges: Edges::Left | Edges::Right | Edges::Bottom, flex_direction: FlexDirection::Column) {
                #(props.children.iter_mut())
            }
        }
    }
}

fn main() {
    let mut e = element! {
        View(width: 40, flex_direction: FlexDirection::Row) {
            View(width: 20) { Pane(title: "Projects", focused: true) { Text(content: "▸ skills") Text(content: "  한글 aterm") } }
            View(width: 20) { Pane(title: "Detail", focused: false) { Text(content: "name skills") } }
        }
    };
    print!("{}", e.to_string());
}
