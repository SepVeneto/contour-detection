mod utils;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     fn test(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     test("Hello, core!");
// }

#[wasm_bindgen]
pub fn process_img(img: &[u8], width: u32, height: u32) -> Vec<u8> {

  let output = grayscale(img);

  edge_detection(&output, width, height)

}

fn grayscale(img: &[u8]) -> Vec<u8> {
  let mut output = img.to_vec();
  for i in (0..output.len()).step_by(4) {
    let r = output[i] as f32 * 0.299;
    let g = output[i + 1] as f32 * 0.587;
    let b = output[i + 2] as f32 * 0.114;
    let gray = (r + g + b) as u8;
    output[i] = gray;
    output[i + 1] = gray;
    output[i + 2] = gray;
  }
  output
}

fn edge_detection(data: &[u8], width: u32, height: u32) -> Vec<u8> {
  let mut output = vec![0; data.len()];

  let kx: [[i32; 3]; 3] = [
    [-1, 0, 1],
    [-2, 0, 2],
    [-1, 0, 1],
  ];
  let ky: [[i32; 3]; 3] = [
    [-1, -2, -1],
    [0, 0, 0],
    [1, 2, 1],
  ];

  for y in 1..height-1 {
    for x in 1..width-1 {
      let mut gx = 0;
      let mut gy = 0;
      for ky_idx in 0..3 {
        for kx_idx in 0..3 {
          let pixel_idx = ((y as i32 + ky_idx - 1) * width as i32 + (x as i32 + kx_idx - 1)) * 4;
          let pixel_value = data[pixel_idx as usize] as i32;
          gx += pixel_value * kx[ky_idx as usize][kx_idx as usize];
          gy += pixel_value * ky[ky_idx as usize][kx_idx as usize];
        }
      }

      let gradient = ((gx.pow(2) + gy.pow(2)) as f64).sqrt() as u8;
      let idx = (y * width + x) * 4;
      output[idx as usize] = gradient;
      output[(idx + 1) as usize] = gradient;
      output[(idx + 2) as usize] = gradient;
      output[(idx + 3) as usize] = 255;
    }
  }
  output
}
