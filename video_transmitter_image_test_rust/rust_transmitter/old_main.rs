use image::io::Reader as ImageReader;
use image::{GenericImageView, Pixel};
use std::time::{SystemTime};
use std::net::UdpSocket;
use byteorder::{NetworkEndian, WriteBytesExt};

fn main() {
    let image = ImageReader::open("./test_images/cat.png").unwrap().decode().unwrap();
    let (width, height) = image.dimensions();
    let timer = SystemTime::now();
    let image_bytes = image.to_luma8().into_raw();
    let socket = UdpSocket::bind("0.0.0.0:8888").unwrap();
    
    println!("Image size: {}x{}", height, width);
    let mut count: u32 = 0;
    loop {
        let transmission_start_time = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs() as u32;
        println!("Transmission start time: {}", transmission_start_time);
        for row in 0..height{
            let buff_size = ((width) + 24) as usize;
            let mut message: Vec<u8> = vec![0; buff_size];
            let mut header: Vec<u8> = Vec::new();
            header.write_u32::<NetworkEndian>(1).unwrap();
            header.write_u32::<NetworkEndian>(transmission_start_time as u32).unwrap();
            header.write_u32::<NetworkEndian>(SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs() as u32).unwrap();
            header.write_u32::<NetworkEndian>(count).unwrap();
            header.write_u32::<NetworkEndian>(height).unwrap();
            header.write_u32::<NetworkEndian>(width).unwrap();
            header.write_u32::<NetworkEndian>(row).unwrap();
            // This is just a memcpy
            message[..header.len()].copy_from_slice(&header);
            //more memcpy
            message[24..(24 + (width) as usize)].copy_from_slice(&image_bytes[(row * width) as usize..((row * width) + (width)) as usize]);
            //println!("Bytes sent in row: {:?}", &image_bytes[(row * width) as usize..((row * width) + (width)) as usize]);
            println!("Header: {:?}", &header);
            socket.send_to(&message, "127.0.0.1:9000").unwrap();
        }
        
        if count >= 30{
            break;
        }else {
            count += 1;
        }
    }
    
    let elapsed = timer.elapsed().unwrap();
    println!("Elapsed: {:?}", elapsed.as_millis());
}
