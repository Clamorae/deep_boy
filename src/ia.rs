use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use inline_python::python;


pub struct Ia{}

impl Ia {
    pub fn hellow_world(){
        python! {
            print("t'es cringe")
        }
    }
}
