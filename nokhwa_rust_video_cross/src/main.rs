// use byteorder::{NetworkEndian, WriteBytesExt};
// use std::net::UdpSocket;
// use std::sync::mpsc::{sync_channel};
// use std::time::{SystemTime};
// use std::thread;
use nokhwa::{self, Camera, CameraFormat, FrameFormat, query_devices, backends};

fn main() {
    // set up the Camera
    let mut camera = Camera::new(
        0, // index
        Some(CameraFormat::new_from(640, 480, FrameFormat::MJPEG, 30)), // format
    )
    .unwrap();
    // open stream
    camera.open_stream().unwrap();
    loop {
        let frame = camera.frame().unwrap();
        println!("{}, {}", frame.width(), frame.height());
    }
    // let (sender, receiver) = sync_channel(1);

    //Thread to capture images while my main thread does networking.
    // thread::spawn(move || {
    //     loop {
    //         let mut frame = Mat::default();
    //         cam.read(&mut frame).unwrap();
    //         sender.send(frame).unwrap();
    //     }
    // });

    // let mut count = 0;
    // loop {
    //     let frame = receiver.recv().unwrap();
        
    //     let width = frame.size()?.width;
    //     let height = frame.size()?.height;
        
    //     let socket = UdpSocket::bind("0.0.0.0:8888").unwrap();

    //     let transmission_start_time = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs() as u32;
    //     for row in 0..height {
    //         let buff_size = ((width*3) + 28) as usize;
    //         let mut message: Vec<u8> = vec![0; buff_size];
    //         let mut header: Vec<u8> = Vec::new();
            
    //         //Setting headers
    //         header.write_u32::<NetworkEndian>(3).unwrap();
    //         header.write_u32::<NetworkEndian>(transmission_start_time as u32).unwrap();
    //         header.write_u32::<NetworkEndian>(SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs() as u32).unwrap();
    //         header.write_u32::<NetworkEndian>(count).unwrap();
    //         header.write_u32::<NetworkEndian>(height.try_into().unwrap()).unwrap();
    //         header.write_u32::<NetworkEndian>(width.try_into().unwrap()).unwrap();
    //         header.write_u32::<NetworkEndian>(row.try_into().unwrap()).unwrap();
            
    //         // This is just a memcpy
    //         message[..header.len()].copy_from_slice(&header);

    //         let mat = frame.row(row).unwrap();
    //         let bytes = mat.data_bytes().unwrap();
            
    //         //more memcpy
    //         message[28..(28 + (width*3) as usize)].copy_from_slice(&bytes);
    //         socket.send_to(&message, "127.0.0.1:9000").unwrap();
    //     }
    //     count+=1;
    // }
}
