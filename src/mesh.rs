use serde::{Deserialize, Serialize};

/// Represents a mesh object in three-dimensional space.
#[derive(Deserialize, Serialize)]
pub struct Mesh {
    /// The identifier for the mesh. The value of MeshId should be greater than or equal to 0.
    pub mesh_id: i32,
    /// The list of coordinates for the mesh vertices.
    pub coordinates: Vec<f64>,
    /// The list of indices for the mesh triangles.
    pub indices: Vec<i32>,
}

impl PartialEq for Mesh {
    fn eq(&self, other: &Self) -> bool {
        if self.mesh_id != other.mesh_id {
            return false;
        }
        if self.coordinates.len() != other.coordinates.len() {
            return false;
        }
        for i in 0..self.coordinates.len() {
            if self.coordinates[i] != other.coordinates[i] {
                return false;
            }
        }
        if self.indices.len() != other.indices.len() {
            return false;
        }
        for i in 0..self.indices.len() {
            if self.indices[i] != other.indices[i] {
                return false;
            }
        }

        true
    }
}

impl Mesh {
    /// Returns a new Mesh
    pub fn new(mesh_id: i32, coordinates: Vec<f64>, indices: Vec<i32>) -> Mesh {Mesh {mesh_id, coordinates, indices}}
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;
    use serde_json::to_string;
    use super::*;

    #[test]
    fn test_new() {
        let result = Mesh::new(12,
                               vec![0.0, 0.0, 0.0,
                                              10.0, 0.0, 0.0,
                                              10.0, -15.0, 0.0],
                               vec![0, 1, 2]);
        assert_eq!(result.mesh_id, 12);
        assert_eq!(result.coordinates, vec![0.0, 0.0, 0.0,
                                            10.0, 0.0, 0.0,
                                            10.0, -15.0, 0.0]);
        assert_eq!(result.indices, vec![0, 1, 2]);
    }

    #[test]
    fn test_partialeq_true() {
        let a = Mesh::new(12,
                          vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        let b = Mesh::new(12,
                          vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_different_id_false() {
        let a = Mesh::new(11,
                          vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        let b = Mesh::new(12,
                          vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_coordinates_count_false() {
        let a = Mesh::new(12,
                          vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0,
                                5.0, 1.0, 0.0],
                          vec![0, 1, 2]);
        let b = Mesh::new(12,
                          vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_different_coordinates_false() {
        let a = Mesh::new(12,
                          vec![0.0, 0.0, 0.0,
                               10.0, 2.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        let b = Mesh::new(12,
                          vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_indices_count_false() {
        let a = Mesh::new(12,
                          vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2, 2, 1, 0]);
        let b = Mesh::new(12,
                          vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_different_indices_false() {
        let a = Mesh::new(12,
                          vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 2, 1]);
        let b = Mesh::new(12,
                          vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_to_json() {
        let input = Mesh::new(12,
                              vec![0.0, 0.0, 0.0,
                                   10.0, 0.0, 0.0,
                                   10.0, -15.0, 0.0],
                              vec![0, 1, 2]);
        let input_serialized = to_string(&input);
        assert_eq!(input_serialized.is_ok(), true);
        let input_serialized_string = input_serialized.ok().unwrap();
        assert_eq!(input_serialized_string, "{\"mesh_id\":12,\"coordinates\":[0.0,0.0,0.0,10.0,0.0,0.0,10.0,-15.0,0.0],\"indices\":[0,1,2]}");
    }

    #[test]
    fn test_from_json() {
        let json = "{\"mesh_id\":12,\"coordinates\":[0.0,0.0,0.0,10.0,0.0,0.0,10.0,-15.0,0.0],\"indices\":[0,1,2]}";
        let actual_result = from_str::<Mesh>(json);
        assert_eq!(actual_result.is_ok(), true);
        let actual = actual_result.ok().unwrap();
        let expected = Mesh::new(12,
                                 vec![0.0, 0.0, 0.0,
                                      10.0, 0.0, 0.0,
                                      10.0, -15.0, 0.0],
                                 vec![0, 1, 2]);
        assert_eq!(expected.eq(&actual), true);
    }
}