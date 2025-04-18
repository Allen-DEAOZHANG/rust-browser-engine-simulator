use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Node {
    tag: String,
    content: String,
    style: String,
    children: Vec<Node>,
}

fn parse_html(input: &str) -> Node {
    Node {
        tag: "div".to_string(),
        content: "".to_string(),
        style: "width:100;height:50;background:red;".to_string(),
        children: vec![Node {
            tag: "p".to_string(),
            content: "Hello World".to_string(),
            style: "font-size:16px;".to_string(),
            children: vec![],
        }],
    }
}

fn render_layout(node: &Node, indent: usize) {
    let padding = " ".repeat(indent);
    if node.tag == "div" {
        println!("{}Div({})", padding, node.style);
    } else if node.tag == "p" {
        println!("{}Paragraph({}): {}", padding, node.style, node.content);
    }
    for child in &node.children {
        render_layout(child, indent + 2);
    }
}

fn main() {
    let html = "<div style='...'><p>Hello World</p></div>";
    let dom = Arc::new(Mutex::new(parse_html(html)));

    let dom_clone = Arc::clone(&dom);
    let handle = thread::spawn(move || {
        let tree = dom_clone.lock().unwrap();
        println!("Layout Box Tree:");
        render_layout(&tree, 0);
    });

    handle.join().unwrap();
}
