use opencv::{
    core::{Vec3b, Mat, Size},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc::{resize, cvt_color, INTER_LINEAR, COLOR_BGR2GRAY, COLOR_BGR2HSV},
    prelude::*
};

use clap::Parser;
#[derive(Parser)]
// #[clap(
//     name    = "My Application",
//     author  = "Author's name",
//     version = "v1.0.0",
//     about   = "Application short description."
// )]

// ###################################################################
// パラメータ
// ###################################################################

pub struct Arg
{
    #[clap(short='c', long="col")]
    width:  i32,

    #[clap(short='r', long="row")]
    height: i32,

    #[clap(short='g', long="gray", default_value="false")]
    isgray: bool,

    #[clap(short='u', long="hue", default_value="false")]
    ishue:  bool,

    #[clap(short='s', long="hsv", default_value="false")]
    ishsv:  bool,
    
    imgname: String,
}

// ###################################################################
// ユーティリティ
// ###################################################################

fn eprint_exit(                                                                                                                                                   
    emsg: &str,                                                                                                                                                   
    en:   i32,                                                                                                                                                    
) -> !                                                                                                                                                            
{                                                                                                                                                                 
    // プログラム名とメッセージを表示                                                                                                                             
    let args = std::env::args().collect::<Vec<String>>();                                                                                                         
    let procname = &args[0];                                                                                                                                      
    eprintln!("{}: {}", procname, emsg);                                                                                                                          
                                                                                                                                                                  
    // 指定のコードで異常終了                                                                                                                                     
    std::process::exit(en);                                                                                                                                       
}

fn load_img(imgname: &str, size: &Size) -> Mat
{
    // 画像をカラーで読み込む
    let res = imread(imgname, IMREAD_COLOR);
    if let Err(_) = res { eprint_exit("cannot open image", 10); }
    let orgimg = res.unwrap();
    
    // 画像をリサイズする
    let mut rszimg = Mat::default();
    let res = resize(&orgimg, &mut rszimg, *size, 0.0, 0.0, INTER_LINEAR);
    if let Err(_) = res { eprint_exit("resize image failed", 10); }
    
    rszimg
}

// ###################################################################
// 出力画像に関するトレイト
// ###################################################################

trait ColorSpace {   
    fn pixelstr(&self, x: i32, y: i32) -> String;
    fn width(&self)  -> i32;
    fn height(&self) -> i32;
    
    fn output(&self) {
        let width  = self.width();
        let height = self.height();
        
        for i in 0..height {
            for j in 0..width {
                print!("{}", self.pixelstr(i, j));
            }
            
            println!("");
        }
    }
}

fn imgfactory(
    img:    &Mat,
    isgray: bool,
    ishue:  bool,
    ishsv:  bool,
) -> Box::<dyn ColorSpace> {
    match (isgray, ishue, ishsv) {
        (true,  _,     _   ) => { Box::new(GrayImg::new(&img)) }
        (false, true,  _   ) => { Box::new(HueImg::new(&img))  }
        (false, false, true) => { Box::new(HsvImg::new(&img))  }
        (_,     _,     _   ) => { Box::new(RgbImg::new(&img))  }
    }    
}

// ###################################################################
// RGB画像に関する実装
// ###################################################################

struct RgbImg {
    rgbimg: Mat,
}

impl RgbImg {
    fn new(orgimg: &Mat) -> Self {
        let rgbimg = orgimg.clone();

        Self { rgbimg, }
    }
}

impl ColorSpace for RgbImg {    
    fn width(&self)  -> i32 { self.rgbimg.cols() }
    fn height(&self) -> i32 { self.rgbimg.rows() }
    
    fn pixelstr(&self, x:i32, y:i32) -> String {
        let pixel = self.rgbimg.at_2d::<Vec3b>(x, y).unwrap();
        format!("{} {} {} ", pixel[0], pixel[1], pixel[2])
    }
}

// ###################################################################
// グレースケール画像に関する実装
// ###################################################################

struct GrayImg {
    grayimg: Mat,
}

impl GrayImg {
    fn new(orgimg: &Mat) -> Self {
        let mut grayimg = Mat::default();
        let res = cvt_color(&orgimg, &mut grayimg, COLOR_BGR2GRAY, 0);
        if let Err(_) = res { eprint_exit("convert color failed", 12); }

        Self { grayimg, }
    }
}

impl ColorSpace for GrayImg {
    fn width(&self)  -> i32 { self.grayimg.cols() }
    fn height(&self) -> i32 { self.grayimg.rows() }
    
    fn pixelstr(&self, x: i32, y: i32) -> String {
        let pixel = self.grayimg.at_2d::<u8>(x, y).unwrap();
        format!("{} ", pixel)
    }
}

// ###################################################################
// 色相画像に関する実装
// ###################################################################

struct HueImg {
    hueimg: Mat,
}

impl HueImg {
    fn new(orgimg: &Mat) -> Self {
        let mut hueimg = Mat::default();
        let res = cvt_color(&orgimg, &mut hueimg, COLOR_BGR2HSV, 0);
        if let Err(_) = res { eprint_exit("convert color failed", 13); }
        
        Self { hueimg, }
    }
}

impl ColorSpace for HueImg {
    fn width(&self)  -> i32 { self.hueimg.cols() }
    fn height(&self) -> i32 { self.hueimg.rows() }
    
    fn pixelstr(&self, x: i32, y: i32) -> String {
        let pixel = self.hueimg.at_2d::<Vec3b>(x, y).unwrap();
        format!("{} ", pixel[0])
    }
}

// ###################################################################
// HSV画像に関する実装
// ###################################################################

struct HsvImg {
    hsvimg: Mat,
}

impl HsvImg {
    fn new(orgimg: &Mat) -> Self {
        let mut hsvimg = Mat::default();
        let res = cvt_color(&orgimg, &mut hsvimg, COLOR_BGR2HSV, 0);
        if let Err(_) = res { eprint_exit("convert color failed", 13); }
        
        Self { hsvimg, }
    }
}

impl ColorSpace for HsvImg {
    fn width(&self)  -> i32 { self.hsvimg.cols() }
    fn height(&self) -> i32 { self.hsvimg.rows() }
    
    fn pixelstr(&self, x: i32, y: i32) -> String {
        let pixel = self.hsvimg.at_2d::<Vec3b>(x, y).unwrap();
        format!("{} {} {} ", pixel[0], pixel[1], pixel[2])
    }
}

// ###################################################################
// 本体処理
// ###################################################################

fn main()
{
    // 引数をパース
    let arg = Arg::parse();

    // パラメータを読み込む
    let isgray  = arg.isgray;
    let ishue   = arg.ishue;
    let ishsv   = arg.ishsv;
    let width   = arg.width;
    let height  = arg.height;
    let outsize = Size::new(width, height);

    // 入力画像を読み込む（＋リサイズ）
    let img = load_img(&arg.imgname, &outsize);

    // 出力画像を生成
    let outimg = imgfactory(&img, isgray, ishue, ishsv);

    // 出力
    outimg.output();
}
