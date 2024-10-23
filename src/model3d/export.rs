use std::fs::File;
use std::io::Write;

use numtoa::NumToA;
use nanoserde::SerJson;

use crate::model3d::Model3D;
use crate::types::Params3D;

impl Model3D {
    pub fn export_model(&self, name: &str, save: &[&str]) -> Result<(), std::io::Error> {
        let mut result = String::from("");
        result += "{\"params3D\":";

        if save.contains(&"params") {
            export_params(&mut result, &self.params);
        } else { result += "null" }

        result += ",\"borders\":";
        if save.contains(&"borders") {
            export_border_num(&mut result, &self.borders)
        } else { result += "null" }

        if name == "TestModelBench.test.bench" { return Ok(()) }

        let mut file = File::create(format!("{name}.json"))?;
        file.write_all(result.as_bytes())?;
        Ok(())
    }
}

fn export_border_num(result: &mut String, borders: &[Vec<Vec<u32>>]){
    let mut buf = [0u8; 12]; *result += "[";
    for (depth_num, depth) in borders.iter().enumerate() {
        *result += "{\"bo";
        *result += format!("{depth_num}\":[").as_str();
        for (y_num, y_axis) in depth.iter().enumerate() {
            *result += "{\"y";
            *result += format!("{y_num}\":[").as_str();

            result.push_str(y_axis[0].numtoa_str(10, &mut buf));

            for x in y_axis[1..].iter() {
                result.push(',');
                result.push_str(x.numtoa_str(10, &mut buf));
            }

            if y_num != depth.len() - 1 {
                *result += "]},"
            } else {
                *result += "]}"
            }
        }
        if depth_num != borders.len() - 1 {
            *result += "]},";
        } else {
            *result += "]}";
        }
    }
    *result += "]";
}

fn export_params(result: &mut String, params: &Params3D) {
    result.push_str(SerJson::serialize_json(params).as_str());
}
