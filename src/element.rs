use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use crate::color::Color;
use crate::rotation::Rotation;
use crate::vector::Vector;

/// Represents a mesh object in three-dimensional space.
#[derive(Deserialize, Serialize)]
pub struct Element {
    /// The identifier of the mesh associated with the element.
    mesh_id: i32,
    /// The position of the element.
    vector: Vector,
    /// The rotation of the element.
    rotation: Rotation,
    /// The globally unique identifier for the element.
    guid: String,
    /// The type of the element.
    #[serde(rename(serialize = "type", deserialize = "type"))]
    element_type: String,
    /// The color of the element.
    color: Color,
    /// The list of integers, that determine face colors of a mesh.
    /// They should be organized like this: [r1, g1, b1, a1, r2, g2, b2, a2, r3, g3, b3, a3, ... rn, gn, bn, an]
    /// E.g. list like: [255, 0, 0, 255, 135, 206, 235, 255, 255, 255, 255, 255]
    /// means first triangle should be colored as red (255,0,0,255),
    /// second as skyblue (135,206,235,255),
    /// third as white (255,255,255,255).
    #[serde(skip_serializing_if = "Option::is_none")]
    face_colors: Option<Vec<i32>>,
    /// Additional information about the element.
    info: HashMap<String, String>
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        if self.mesh_id != other.mesh_id {
            return false;
        }
        if self.vector != other.vector {
            return false;
        }
        if self.rotation != other.rotation {
            return false;
        }
        if self.guid != other.guid {
            return false;
        }
        if self.element_type != other.element_type {
            return false;
        }
        if self.color != other.color {
            return false;
        }
        if self.face_colors.is_none() && other.face_colors.is_none() {

        } else {
            let self_face_colors_unpacked = self.face_colors.as_ref().unwrap();
            let other_face_colors_unpacked = other.face_colors.as_ref().unwrap();
            if self_face_colors_unpacked.len() != other_face_colors_unpacked.len() {
                return false;
            }
            for i in 0..self_face_colors_unpacked.len() {
                if self_face_colors_unpacked[i] != other_face_colors_unpacked[i] {
                    return false;
                }
            }
        }

        if !self.info.eq(&other.info) {
            return false;
        }

        true
    }
}

impl Element {
    /// Returns a new Element
    pub fn new(mesh_id: i32, vector: Vector, rotation: Rotation, guid: String, element_type: String,
               color: Color, face_colors: Option<Vec<i32>>, info: HashMap<String, String>)
        -> Element { Element { mesh_id, vector, rotation, guid, element_type, color, face_colors, info } }
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;
    use super::*;

    fn get_blue_test_element() -> Element {
        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Rotation::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        )
    }

    #[test]
    fn test_new_without_face_colors() {
        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let result = get_blue_test_element();

        assert_eq!(result.mesh_id, 4);
        assert_eq!(Vector::new(0.2, 0.3, 0.4).eq(&result.vector), true);
        assert_eq!(Rotation::new(1.0, 1.5, 2.0, 2.5).eq(&result.rotation), true);
        assert_eq!(String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353").eq(&result.guid), true);
        assert_eq!(String::from("Plate").eq(&result.element_type), true);
        assert_eq!(Color::new(0,0,255,0).eq(&result.color), true);
        assert_eq!(result.face_colors, None);
        assert_eq!(info.eq(&result.info), true);
    }
    #[test]
    fn test_to_json() {
        let input = get_blue_test_element();
        let input_serialized = to_string(&input);
        assert_eq!(input_serialized.is_ok(), true);
        let input_serialized_string = input_serialized.ok().unwrap();
        assert_eq!(input_serialized_string, "{\"mesh_id\":4,\"vector\":{\"x\":0.2,\"y\":0.3,\"z\":0.4},\"rotation\":{\"qx\":1.0,\"qy\":1.5,\"qz\":2.0,\"qw\":2.5},\"guid\":\"b8a7a2ed-0c30-4c20-867e-baa1ef7b8353\",\"type\":\"Plate\",\"color\":{\"r\":0,\"g\":0,\"b\":255,\"a\":0},\"info\":{\"Key\":\"Value\"}}");
    }

    #[test]
    fn test_from_json() {
        let json = "{\"mesh_id\":4,\"vector\":{\"x\":0.2,\"y\":0.3,\"z\":0.4},\"rotation\":{\"qx\":1.0,\"qy\":1.5,\"qz\":2.0,\"qw\":2.5},\"guid\":\"b8a7a2ed-0c30-4c20-867e-baa1ef7b8353\",\"type\":\"Plate\",\"color\":{\"r\":0,\"g\":0,\"b\":255,\"a\":0},\"info\":{\"Key\":\"Value\"}}";
        let actual_result = from_str::<Element>(json);
        // println!("{}", actual_result.err().unwrap());
        assert_eq!(actual_result.is_ok(), true);
        let actual = actual_result.ok().unwrap();
        let expected = get_blue_test_element();
        assert_eq!(expected.eq(&actual), true);
    }
}