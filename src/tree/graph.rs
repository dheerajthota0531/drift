use crate::tree::canvas::Canvas;
use crate::tree::node::Node;

pub fn render_graph(
    canvas: &mut Canvas,
    node: &Node,
    x: usize,
    y: usize,
    spacing: usize,
) {

    canvas.write_text(x, y, &node.name);

    let child_count = node.children.len();

    if child_count == 0 {
        return;
    }

    let start_x =
        x.saturating_sub(
            (child_count * spacing) / 2
        );

    for (index, child) in
        node.children.iter().enumerate()
    {

        let child_x =
            start_x + (index * spacing);

        let child_y = y + 4;

        // branch
        canvas.draw_char(
            (x + child_x) / 2,
            y + 1,
            '/',
        );

        render_graph(
            canvas,
            child,
            child_x,
            child_y,
            spacing / 2 + 2,
        );
    }
}
