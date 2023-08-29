use std::collections::HashMap;
use std::fs::{self,File};
use std::io::{self, BufRead,Write,BufReader};
use regex::Regex;
use image;
use reqwest;
use webbrowser;
use thirtyfour::common::capabilities::firefox::FirefoxPreferences;
use thirtyfour::{FirefoxCapabilities, WebDriver};


fn read_file(filepath: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

fn dict_processing(file:BufReader<File>) -> Result<HashMap<String,String>,Box<dyn std::error::Error>>{
    let mut dictionary: HashMap<String, String> = HashMap::new();
    let mut file_w = File::create("streetlinks.txt")?;
    for line in file.lines(){
        let street = line?.trim().to_string();
        if !dictionary.contains_key(&street){
            let linka = format!("https://www.google.com/maps/place/{} 23, Lisboa", &street).replace(" ","+");
            dictionary.insert(street.clone(), linka.clone());
            file_w.write(format!("{}\n",linka).as_bytes())?;
        }
        }
    Ok(dictionary)
}

fn browser_look()  -> Result<(),Box<dyn std::error::Error>>{
    let file = read_file("streetlinks.txt").expect("E,FNF");
    let mut link = file.lines();

    // for iter in link.{
    //     webbrowser::open(&link.next().expect("Returned None").expect("Returned Error")).unwrap();
    //   //    println!("{:?}",link.next()) ;
    //   }

    for iter in 0..50{
      webbrowser::open(&link.next().expect("Returned None").expect("Returned Error")).unwrap();
    //    println!("{:?}",link.next()) ;
    }
    Ok(())
}

#[tokio::main]
async fn image_scarp(hash_map: &HashMap<String,String>) -> color_eyre::Result<()>{
    color_eyre::install()?;

    let user_agent: &str = "Custom";

    let mut prefs = FirefoxPreferences::new();
    prefs.set_user_agent(user_agent.to_string())?;

    let mut caps = FirefoxCapabilities::new();
    caps.set_preferences(prefs)?;

    for iter in hash_map.keys(){
        println!("{}",&iter);
        let driver = WebDriver::new("http://localhost:4444", caps.clone()).await?;
        driver.goto(&hash_map[iter]).await?;
        std::thread::sleep(std::time::Duration::from_secs(2));//Прогрузка страницы
        let html = driver.source().await?;
        let re = Regex::new(r"https://streetviewpixels-pa\.googleapis\.com/v1/thumbnail\?panoid=.+?100").unwrap();
    
        let dates: Vec<String> = re.find_iter(&html).map(|m| m.as_str().to_string()).collect();
        for s in dates {
            let sn = s.replace("amp;", "");
         //   println!("{sn}{iter}");
            let response = reqwest::get(sn).await.unwrap()
            .bytes().await?;
            let image = image::load_from_memory(&response)?;
            let name = format!("images/{iter}.jpg");
            let output  =  File::create(&name)?;
            image.save(name).unwrap();
            
        }
        
    }

    Ok(())

}

fn main() {
    // The use of color_eyre gives much nicer error reports, including making
    // it much easier to locate where the error occurred.
    let filepath: &str = r"C:\Users\Gurman\Documents\Rust\geoint_76\media\streets.txt";
    let reader = read_file(filepath).expect("Error occured");
    let mapa = dict_processing(reader).unwrap();
    // let steetlinks = "streetlinks.txt";
    println!("{}",mapa.len());
    image_scarp(&mapa).expect("Error");

}