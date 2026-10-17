use crate::org::tiqian::protocol::layout_vertex::LayoutVertex;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum LayoutFloatShape {
    Circle { radius: f64 },
    Square { side: f64 },
    Polygon { vertices: Vec<LayoutVertex> },
}

impl LayoutFloatShape {
    pub fn to_string(&self) -> String {
        match self {
            LayoutFloatShape::Circle { radius } => format!("Circle(radius={})", (radius).to_string()),
            LayoutFloatShape::Square { side } => format!("Square(side={})", (side).to_string()),
            LayoutFloatShape::Polygon { vertices } => format!("Polygon(vertices={})", {
            let mut out = String::new();
            out.push('[');
            let mut j0 = 0usize;
            while j0 < vertices.len() {
                if j0 > 0 { out.push_str(", "); }
                let _ = write!(out, "{}", format!("{{x={}, y={}}}",
            (vertices[j0].x).to_string(),
            (vertices[j0].y).to_string()
        ));
                j0 += 1;
            }
            out.push(']');
            out
        }),
        }
    }
}
