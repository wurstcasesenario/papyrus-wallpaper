// src/main.rs

// This is a simple Rust program that retrieves monitor information, allows the user to select an image file, crops the image to fit all the monitors, and sets it as the desktop wallpaper on Windows.
// Made by wurstcasesenario


// Include necessary crates
use winit::event_loop::EventLoop;
use image::{RgbImage, ImageFormat};
use rfd::FileDialog;
use std::path::PathBuf;
use image::imageops::FilterType; 
use std::env;


// #[derive(Debug)]
struct MonitorInfo {
    width: u32,
    height: u32,
    x: i32,
    y: i32,
}



// Function to get monitor information
fn get_monitors_info() -> Vec<MonitorInfo> {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    //let event_loop = EventLoop::new();
    let mut monitors = Vec::new();

    for monitor in event_loop.available_monitors() {
        let size = monitor.size();
        let position = monitor.position();

        monitors.push(MonitorInfo {
            width: size.width,     // Width of the monitor
            height: size.height,   // Height of the monitor
            x: position.x,         // X position of the monitor
            y: position.y,         // Y position of the monitor
        });
    }

    monitors
}

// Function to crop the image based on monitor information
fn crop_wallpaper_image(monitors: &[MonitorInfo], input_img: &RgbImage) -> RgbImage {
    // Calculate the bounding box that encompasses all monitors
    let min_x = monitors.iter().map(|m| m.x).min().unwrap_or(0);
    let min_y = monitors.iter().map(|m| m.y).min().unwrap_or(0);
    let max_x = monitors.iter().map(|m| m.x + m.width as i32).max().unwrap_or(800);
    let max_y = monitors.iter().map(|m| m.y + m.height as i32).max().unwrap_or(600);

    // Calculate the target width and height based on the bounding box
    let target_width = (max_x - min_x) as u32;
    let target_height = (max_y - min_y) as u32;

    // Calculate the scale factor to resize the input image
    let scale_w = target_width as f32 / input_img.width() as f32;
    let scale_h = target_height as f32 / input_img.height() as f32;
    let scale = scale_w.max(scale_h);

    // Resize the input image to fit the target dimensions
    let resized_width = (input_img.width() as f32 * scale).ceil() as u32;
    let resized_height = (input_img.height() as f32 * scale).ceil() as u32;

    let resized_img = image::imageops::resize(input_img, resized_width, resized_height, FilterType::Lanczos3);

    // Calculate the crop position to center the image
    let crop_x = if resized_width > target_width {
        (resized_width - target_width) / 2
    } else {
        0
    };
    let crop_y = if resized_height > target_height {
        (resized_height - target_height) / 2
    } else {
        0
    };

    // Crop the resized image to the target dimensions
    let cropped_img = image::imageops::crop_imm(&resized_img, crop_x, crop_y, target_width, target_height).to_image();

    cropped_img
}


// Function to set the wallpaper over all monitors in tiling mode
fn set_wallpaper_stretched(image_path: &PathBuf) -> Result<(), String> {
    // Use the winapi and winreg crates to set the wallpaper on Windows
    use std::os::windows::ffi::OsStrExt;
    use winreg::enums::*;
    use winreg::RegKey;
    use winapi::um::winuser::{SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_UPDATEINIFILE, SPIF_SENDCHANGE};


    // Convert the image path to a wide string (UTF-16) for Windows API
    let wide: Vec<u16> = image_path.as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    // Open the registry key for the current user to set the wallpaper style
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey_with_flags("Control Panel\\Desktop", KEY_WRITE)
        .map_err(|e| e.to_string())?;
    key.set_value("WallpaperStyle", &"0").map_err(|e| e.to_string())?; // 0 for tiled wallpaper
    key.set_value("TileWallpaper", &"1").map_err(|e| e.to_string())?; // 1 for tiled wallpaper

    // Call the SystemParametersInfoW function to set the wallpaper
    let result = unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            wide.as_ptr() as *mut _,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        )
    };
    if result == 0 {
        Err("Failed to set wallpaper".to_string())
    } else {
        Ok(())
    }
}


fn set_wallpaper_from_image(image: &RgbImage) -> Result<(), Box<dyn std::error::Error>> {
    // Convert the image to BMP format and save it to a temporary file	
    let mut bmp_path = env::temp_dir();
    bmp_path.push("rust_wallpaper.bmp");

    image.save_with_format(&bmp_path, ImageFormat::Bmp)?;

    // Set the wallpaper using the saved BMP file
    set_wallpaper_stretched(&bmp_path)?;

    Ok(())
}


// Main function to run the program
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get monitor information
    let monitors = get_monitors_info();
    if monitors.is_empty() {
        eprintln!("No monitors found. Exiting program.");
        return Ok(()); 
    }

    // Prompt the user to select an image file
    let image_path = match FileDialog::new()
        .add_filter("Image Files", &["png", "jpg", "jpeg", "bmp"])
        .pick_file() {
        Some(path) => path,
        None => {
            eprintln!("No image file selected. Exiting program.");
            return Ok(());
        }
    };

    // Open the selected image file
    let input_img: RgbImage = match image::open(&image_path) {
        Ok(img) => img.to_rgb8(),
        Err(err) => {
            eprintln!(
                "Failed to open '{}': {:?}\nPlease select a valid PNG/JPG/BMP file.",
                image_path.display(),
                err
            );
            return Ok(());
        }
    };

    // Crop image and set it as wallpaper
    let cropped_img = crop_wallpaper_image(&monitors, &input_img);
    set_wallpaper_from_image(&cropped_img)?;

    Ok(())
}

