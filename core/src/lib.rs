mod utils;
use image::{imageops::{blur, grayscale}, load_from_memory, GrayImage, ImageBuffer, Rgb, Rgba};
use imageproc::gradients::sobel_gradients;

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
pub fn process_img(img: &[u8], width: u32, height: u32) -> Vec<u16> {

  let image_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, img.to_vec()).unwrap();


  // let output = grayscale(img);
  let gray_res = grayscale(&image_buffer);

  let blurred_res = blur(&gray_res, 1.5);
  
  let sobel_res = sobel_gradients(&blurred_res);


  let mut rgba_img = ImageBuffer::new(width, height);
  for (x, y, gray_pixel) in sobel_res.enumerate_pixels() {
    let gray = gray_pixel[0];

    let norm;
    if gray > 10 {
      norm = 0;
    } else {
      norm = 255;
    }
    let rgba = Rgba([norm, norm, norm, 255]);
    rgba_img.put_pixel(x, y, rgba)
  }


  // Gaussian
  // edge_detection(&output, width, height)
  // blurred_res.to_vec()
  // vec![]
  rgba_img.to_vec()
}

// fn grayscale(img: &[u8]) -> Vec<u8> {
//   let mut output = img.to_vec();
//   for i in (0..output.len()).step_by(4) {
//     let r = output[i] as f32 * 0.299;
//     let g = output[i + 1] as f32 * 0.587;
//     let b = output[i + 2] as f32 * 0.114;
//     let gray = (r + g + b) as u8;
//     output[i] = gray;
//     output[i + 1] = gray;
//     output[i + 2] = gray;
//   }
//   output
// }

// fn edge_detection(data: &[u8], width: u32, height: u32) -> Vec<u8> {
//   let mut output = vec![0; data.len()];

//   let kx: [[i32; 3]; 3] = [
//     [-1, 0, 1],
//     [-2, 0, 2],
//     [-1, 0, 1],
//   ];
//   let ky: [[i32; 3]; 3] = [
//     [-1, -2, -1],
//     [0, 0, 0],
//     [1, 2, 1],
//   ];

//   for y in 1..height-1 {
//     for x in 1..width-1 {
//       let mut gx = 0;
//       let mut gy = 0;
//       for ky_idx in 0..3 {
//         for kx_idx in 0..3 {
//           let pixel_idx = ((y as i32 + ky_idx - 1) * width as i32 + (x as i32 + kx_idx - 1)) * 4;
//           let pixel_value = data[pixel_idx as usize] as i32;
//           gx += pixel_value * kx[ky_idx as usize][kx_idx as usize];
//           gy += pixel_value * ky[ky_idx as usize][kx_idx as usize];
//         }
//       }

//       let gradient = ((gx.pow(2) + gy.pow(2)) as f64).sqrt() as u8;
//       let idx = (y * width + x) * 4;
//       output[idx as usize] = gradient;
//       output[(idx + 1) as usize] = gradient;
//       output[(idx + 2) as usize] = gradient;
//       output[(idx + 3) as usize] = 255;
//     }
//   }
//   output
// }

// fn gaussain_blur(data: &[u8], width: u32, height: u32, sigma: f32) -> Vec<f32> {
//   let kernel_size = (2.0 * 3.0 * sigma).ceil() as usize | 1;
//   let kernel= guassain_kernel(kernel_size, sigma);
//   apply_kernel(data, width, height, &kernel)
// }

// fn gaussain_kernel(size: usize, sigma: f32) -> Vec<u8> {
//   let mut kernel = vec![0.0; size];
//   let half = (size / 2) as isize;
//   let mut sum = 0.0;

//   for i in 0..size {
//     let x = (i as isize) - half;
//     let value = (-(x * x) as f32 / (2.0 * sigma * sigma)).exp();
//     kernel[i]= value;
//     sum += value
//   }

//   for i in 0..size {
//     kernel[i] /= sum;
//   }

//   kernel
// }

// fn apply_kernel(data: &[u8], width: usize, height: usize, kernel: &[f32]) -> Vec<f32> {
//   let mut result = vec![0.0; width * height];
//   let k_size = kernel.len();
//   let k_half = k_size / 2;

//   for y in 0..height {
//     for x in 0..width {
//       let mut sum = 0.0;
//       for k in 0..k_size {
//         let ix = (x as isize + (k as isize - k_half) as isize).clamp(0, (width - 1) as isize) as usize;
//         sum += data[y * width + ix] * kernel[k];
//       }
//       result[y * width + x] = sum;
//     }
//   }

//   let mut temp = vec![0.0; width * height];
//   for x in 0..width {
//     for y in 0..height {
//       let mut sum = 0.0;
//       for k in 0..k_size {
//         let iy = (y as isize + (k as isize - k_half) as isize).clamp(0, (height - 1) as isize) as usize;
//         sum += result[iy * width + x] * kernel[k];
//       }
//       temp[y * width + x] = sum;
//     }
//   }

//   temp
// }
