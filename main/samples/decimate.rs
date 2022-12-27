#![allow(dead_code)]
#![allow(incomplete_include)]

#[path = "../src/lib.rs"]
mod nanomesh;

use std::time::Instant;
use std::io::{BufWriter, BufReader};
use std::fs::File;

fn main()
{
    let now = Instant::now();
    let shared_mesh_file = File::open("./sphere_flat_hp.obj").unwrap();
    let mut shared_mesh_buffer = BufReader::new(shared_mesh_file);
    let shared_mesh = nanomesh::io::obj::read(&mut shared_mesh_buffer);
    println!("read obj done in {} ms", now.elapsed().as_millis());

    let now = Instant::now();
    let mut mesh = nanomesh::mesh::ConnectedMesh::from(&shared_mesh);
    println!("to connected mesh done in {} ms", now.elapsed().as_millis());

    let now = Instant::now();
    mesh.decimate_to_ratio(0.5);
    println!("decimation done in {} ms", now.elapsed().as_millis());

    let now = Instant::now();
    let shared_mesh = nanomesh::mesh::SharedMesh::from(&mesh);
    println!("to shared mesh done in {} ms", now.elapsed().as_millis());

    let now = Instant::now();
    let output_file = File::open("./output.obj").unwrap();
    let mut output_buffer = BufWriter::new(output_file);
    nanomesh::io::obj::write(&shared_mesh, &mut output_buffer);
    println!("write obj done in {} ms", now.elapsed().as_millis());
}