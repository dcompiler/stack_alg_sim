extern crate csv;
extern crate rand;
// extern crate lru_vec;
// extern crate lru_stack;
extern crate test_cases;
// use test_cases::nmm;

use rusoto_core::Region;
use rusoto_s3::{S3, S3Client, PutObjectRequest};

use std::env;
use std::time::{Instant, Duration};
use std::error::Error as OError;
use csv::Writer;
use rand::Rng;
use lru_vec::LRUVec;
use lru_stack::LRUStack;

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string(),
    }
}

fn generate_data(size: usize, mode: &String, flag: u8) -> Vec<String>{
    
    match mode.as_str(){
        "Cyclic" => {
            let mut data = Vec::new();
            // for _ in 0..repeat{
                for i in 0..size{
                    // println!("{:?}", i);
                    data.push(i.to_string());
                }
            // }  
            data
        },
        "Sawtooth" => {
            let mut data = Vec::new();
            // // for r in 0..repeat{
                if flag == 0 {
                    for i in 0..size{
                        // println!("{:?}", i);
                        data.push(i.to_string());
                    }
                }else{
                    for i in (0..size).rev(){
                        // println!("{:?}", i);                                                
                        data.push(i.to_string());
                    }
                }
            // }
            data
        },
        "Random" => {
            let mut data = Vec::new();
            let mut rng = rand::thread_rng();
            // for _ in 0..repeat{
                for _ in 0..size{
                    data.push(rng.gen_range(0..size).to_string());
                }
            // }
            data
        }
        _ => {
            Vec::new()
        }
    } 
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Format: exe mode   test_mode   mem_size/a_size_row[MM]   data_size/a_size_col[MM]   repetitions/b_size_row[MM]   b_size_col[MM]");
        return;
    }

    let mode = &args[1];
    let t_mode = &args[2];
    let data = &args[3];

    //so... I need data about the matrix sizes. I could determine if the test is nmm, and if it is, I use data in 3. Otherwise, I use data in 3, 4, 5
    //ok, so what now? if 

    //
    // Fill 2 matrices with random data (100x100), add rec_acces() in NMM, return dists
    match t_mode.as_str(){
        "MM" => {
            "Vec" => {
                let res: Vec<(String, Option<u32>)> = 
                    test_cases::nmm(500, 500, 500, 500, "Vec");

                
            },
            "Stack" => {
                let res: Vec<(String, Option<u32>)> = 
                    test_cases::nmm(500, 500, 500, 500, "Stack");
            }
        },
        _ => {
            let mut total_time = Duration::new(0, 0);
            let mut flag: u8 = 0;

            let mut total_mis = 0;

            let data_objects: Vec<&str> = data.split(',').collect();
            let mem_size = data_objects[0].parse::<usize>().unwrap();
            let d_size = &data_objects[1].parse::<usize>().unwrap();
            let repeat = &data_objects[2].parse::<usize>().unwrap();
            for r in 0..*repeat{
                println!("repeating {} time.", r + 1);
                println!("Data generation start.");
                let data = generate_data(*d_size, t_mode, flag);
                flag ^= 1;
                // println!("Data generation finish.");
                let all_size = &data.len();
                let mut miss = 0;
                match mode.as_str(){
                    "Vec" =>{
                        let mut analyzer = LRUVec::<String>::new();
                        let mut dists: Vec<(String, Option<u32>)> = Vec::new();
                        for c in data {
                            
                            let cur = analyzer.rec_access(c.to_string());
                            // println!("{:?}", cur);
                            dists.push((c, cur));
                        }
                        dists
                    },
                    "Stack" =>{
                        let mut analyzer = LRUStack::<String>::new();
                        let mut dists: Vec<(&String, Option<u32>)> = Vec::new();
                        
                        for c in &data {
            
                            let cur = analyzer.rec_access(c.to_string());
                            // println!("{:?}", cur);
                            dists.push((c, cur_u32));
                        }

                        dists
                    },
                    _ => {
                        println!("Mode Stack or Mode Vec.");
                        Vec::new()
                    },
                };
            }
        }
    }
    
    let start = Instant::now();
        // let mut count = 0;

    

        
    let duration = start.elapsed();
    // total_time += duration;
    println!("{:?}", duration);
    // println!("missing rate: {:.3?}\n", miss as f32 / *all_size as f32);
    // total_mis += miss;
    let csv_path = get_current_working_dir() + "/" + mode + "_" + t_mode + "_" + data + ".csv";
    match save_csv(&csv_path, &res){
        Ok(_) => {
            println!("csv path: {:?}", csv_path);
        },
        Err(m) => {
            println!("{:?}", m);
        },
    }
    // }
    // println!("Total Time: {:?}, Avg Time: {:?}", total_time, total_time / *repeat as u32);
    // println!("Avg Miss Rate: {:.3?}", total_mis as f32 / (d_size * repeat) as f32);

}


fn save_csv(path: &String, data: &Vec<(&String, Option<u32>)>) -> Result<(), Box<dyn OError>> {
    let mut wtr = Writer::from_path(path)?;
    for i in data{
        wtr.write_record(&[i.0, &i.1.unwrap_or(0).to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}

fn save_csv_to_s3(bucket: &str, key: &str, data: &Vec<(String, Option<u32>)>) -> Result<(), Box<dyn std::error::Error>> {
    let s3_client = S3Client::new(Region::default()); // Replace with your desired region

    let mut csv_data = Vec::new();
    {
        let mut wtr = csv::Writer::from_writer(&mut csv_data);
        for i in data {
            wtr.write_record(&[i.0.as_str(), i.1.unwrap_or(0).to_string().as_str()])?;
        }
    } // csv_data is now populated with the CSV content

    let put_request = PutObjectRequest {
        bucket: bucket.to_string(),
        key: key.to_string(),
        body: Some(csv_data.into()),
        ..Default::default()
    };

    s3_client.put_object(put_request).sync()?;
    Ok(())
}

pub async fn upload_object(
    client: &Client,
    bucket_name: &str,
    file_name: &str,
    key: &str,
    ) -> Result<PutObjectOutput, SdkError<PutObjectError>> {

    let body = ByteStream::from_path(Path::new(file_name)).await;
    client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body.unwrap())
        .send()
        .await
}



fn save_csv(path: &String, data: &Vec<(String, Option<u32>)>) -> Result<(), Box<dyn OError>> {
    let mut wtr = Writer::from_path(path)?;
    for i in data{
        wtr.write_record(&[i.0.as_str(), i.1.unwrap_or(0).to_string().as_str()])?;
    }
    wtr.flush()?;
    Ok(())
}