impl From<&SharedMesh> for ConnectedMesh {
    fn from(shared_mesh: &SharedMesh) -> Self {
        let triangles = &shared_mesh.triangles;
        let mut nodes = vec![Node{ position: 0, attribute: 0, relative: 0, sibling: 0 }; triangles.len()];
        let mut vertex_to_nodes = HashMap::<i32, Vec<i32>, _>::with_hasher(
            BuildHasherDefault::<SimpleHasher>::default()
        );
        let mut face_count = 0;
        let mut i: usize = 0;
        loop {
            {
                let mut A = &mut nodes[i];
                A.position = triangles[i];
                A.attribute = triangles[i];
                A.relative = (i as i32) + 1; // B
                if !vertex_to_nodes.contains_key(&A.position) {
                    vertex_to_nodes.insert(A.position, Vec::new());
                }
                vertex_to_nodes.get_mut(&A.position).unwrap().push(i as i32);
            }
            {
                let mut B = &mut nodes[i + 1];
                B.position = triangles[i + 1];
                B.attribute = triangles[i + 1];
                B.relative = (i as i32) + 2; // C
                if !vertex_to_nodes.contains_key(&B.position) {
                    vertex_to_nodes.insert(B.position, Vec::new());
                }  
                vertex_to_nodes.get_mut(&B.position).unwrap().push((i as i32) + 1);
            }
            {
                let mut C = &mut nodes[i + 2];
                C.position = triangles[i + 2];
                C.attribute = triangles[i + 2];
                C.relative = i as i32; // A
                if !vertex_to_nodes.contains_key(&C.position) {
                    vertex_to_nodes.insert(C.position, Vec::new());
                }
                vertex_to_nodes.get_mut(&C.position).unwrap().push((i as i32) + 2);
            }
            face_count = face_count + 1;

            i = i + 3;
            if i >= triangles.len() {
                break;
            }
        }

        for x in vertex_to_nodes.values() {
            let mut previous_sibling: i32 = -1;
            let mut first_sibling: i32 = -1;
            for node in x.iter() {
                if first_sibling != -1 {
                    nodes[*node as usize].sibling = previous_sibling;
                }
                else {
                    first_sibling = *node;
                }
                previous_sibling = *node;
            }
            nodes[first_sibling as usize].sibling = previous_sibling;
        }

        return ConnectedMesh { positions: shared_mesh.positions.clone(), nodes: nodes, face_count: face_count };
    }
}

impl Into<ConnectedMesh> for SharedMesh {
    fn into(self) -> ConnectedMesh {
        return ConnectedMesh::from(&self);
    }
}

impl From<&ConnectedMesh> for SharedMesh {
    fn from(connected_mesh: &ConnectedMesh) -> Self {
        return SharedMesh { positions: Vec::new(), triangles: Vec::new(), groups: Vec::new() };
    }
}

impl Into<SharedMesh> for ConnectedMesh {
    fn into(self) -> SharedMesh {
        return SharedMesh::from(&self);
    }
}

impl From<&SharedMesh> for UnsafeMesh {
    fn from(shared_mesh: &SharedMesh) -> Self {
        unsafe {
            return UnsafeMesh {
                positions_ptr: vec_to_ptr(&shared_mesh.positions),
                positions_len: shared_mesh.positions.len() as i32,
    
                triangles_ptr: vec_to_ptr(&shared_mesh.triangles),
                triangles_len: shared_mesh.triangles.len() as i32,
    
                groups_ptr: vec_to_ptr(&shared_mesh.groups),
                groups_len: shared_mesh.positions.len() as i32,
            };
        }
    }
}

impl Into<SharedMesh> for UnsafeMesh {
    fn into(self) -> SharedMesh {
        return SharedMesh::from(&self);
    }
}

impl From<&UnsafeMesh> for SharedMesh {
    fn from(unsafe_mesh: &UnsafeMesh) -> Self {
        unsafe {
            return SharedMesh {
                positions: ptr_to_vec(unsafe_mesh.positions_ptr, unsafe_mesh.positions_len as usize),
                triangles: ptr_to_vec(unsafe_mesh.triangles_ptr, unsafe_mesh.triangles_len as usize),
                groups: ptr_to_vec(unsafe_mesh.groups_ptr, unsafe_mesh.groups_len as usize),
            };
        }
    }
}

impl Into<UnsafeMesh> for SharedMesh {
    fn into(self) -> UnsafeMesh {
        return UnsafeMesh::from(&self);
    }
}

#[cfg(test)]
mod tests {

    use crate::base::*;
    use crate::mesh::*;

    #[test]
    fn shared_mesh_to_connected_mesh() {
        
        let mut positions = Vec::new();
        // Build a square
        positions.push(Vector3::new(0., 0., 0.));
        positions.push(Vector3::new(1., 0., 0.));
        positions.push(Vector3::new(1., 1., 0.));
        positions.push(Vector3::new(0., 1., 0.));

        let mut triangles = Vec::new();
        // First triangle
        triangles.push(0);
        triangles.push(1);
        triangles.push(2);
        // Second triangle
        triangles.push(0);
        triangles.push(2);
        triangles.push(3);

        let shared_mesh = SharedMesh { positions: positions, triangles: triangles, groups: Vec::new() };
        let connected_mesh = ConnectedMesh::from(&shared_mesh);

        assert_eq!(connected_mesh.face_count, 2);
        assert_eq!(connected_mesh.positions.len(), 4);
        assert_eq!(connected_mesh.nodes.len(), 6);
    }
}
