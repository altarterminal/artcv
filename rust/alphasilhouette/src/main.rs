use std::path::Path;

use opencv::{
    core::{Vec4b, Mat, Size},
    imgcodecs::{imread, IMREAD_UNCHANGED},
    imgproc::{resize, INTER_LINEAR},
    prelude::*
};

use clap::Parser;

//######################################################################
//パラメータ
//######################################################################

#[derive(Debug,Parser)]
pub struct Arg
{
    #[clap(short='c', long="col")]
    width:  i32,

    #[clap(short='r', long="row")]
    height: i32,
    
    iname: String,
}

fn input_param() -> Arg
{
    // 引数を入力
    let arg    = Arg::parse();
    let width  = arg.width;
    let height = arg.height;
    let iname  = &arg.iname;

    // 数値が正の数であるか判定
    if width  <= 0 { eprint_exit("negative specifed for width",  21); }
    if height <= 0 { eprint_exit("negative specifed for height", 22); }

    // 画像ファイルが存在するか判定
    let ifile = Path::new(iname);
    if ifile.is_file() == false {
        eprint_exit("input file not exist", 23);
    }

    arg
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

// ###################################################################
// 本体処理
// ###################################################################

fn main() {
    // パラメータを入力
    let param  = input_param();
    let width  = param.width;
    let height = param.height;
    let iname  = param.iname;
    let osize  = Size::new(param.width, param.height);

    // 画像をアルファチャンネル付きで読み出す
    let res = imread(&iname, IMREAD_UNCHANGED);
    if let Err(_) = res { eprint_exit("cannot open image", 11); }
    let oimg = res.unwrap();
    
    // 画像をリサイズする
    let mut rimg = Mat::default();
    let res = resize(&oimg, &mut rimg, osize, 0.0, 0.0, INTER_LINEAR);
    if let Err(_) = res { eprint_exit("resize image failed", 12); }

    // アルファチャンネルの値を判定
    for i in 0..height {
        for j in 0..width {
            let pixel = rimg.at_2d::<Vec4b>(i, j).unwrap();
            print!("{} ", if pixel[3] > 0 { 1 } else { 0 } );
        }
        
        println!("");
    }
}
