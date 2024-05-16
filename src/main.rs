use svg::node::element::{Circle, Group};

fn main() {
    let group = Group::new()
        .add(Circle::new().set("cx", 1.0))
        .add(Circle::new().set("cx", 2.0));

    for node in group.get_children() {
        // I want to print the cx attribute of each child node which is a Circle.

        if let Some(c) = node.downcast_ref::<Circle>() {
            println!("Its a circle.");
            let attrs = c.get_attributes();
            println!("{:?}", attrs.get("cx"));
        }
    }
}
