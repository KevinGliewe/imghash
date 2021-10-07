extern crate image;

extern crate img_hash;

extern crate clap;
use clap::{Arg, App};

#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

custom_derive! {
    #[derive(Debug, EnumDisplay, EnumFromStr, IterVariantNames(ClapFilterTypeVariantNames))]
    enum ClapFilterType {
        Nearest,
        Triangle,
        CatmullRom,
        Gaussian,
        Lanczos3,
    }
}

impl From<ClapFilterType> for img_hash::FilterType {
    fn from(filter_type: ClapFilterType) -> Self {
        match filter_type {
            ClapFilterType::Nearest => img_hash::FilterType::Nearest,
            ClapFilterType::Triangle => img_hash::FilterType::Triangle,
            ClapFilterType::CatmullRom => img_hash::FilterType::CatmullRom,
            ClapFilterType::Gaussian => img_hash::FilterType::Gaussian,
            ClapFilterType::Lanczos3 => img_hash::FilterType::Lanczos3,
        }
    }
}


custom_derive! {
    #[derive(Debug, EnumDisplay, EnumFromStr, IterVariantNames(ClapHashAlgVariantNames))]
    enum ClapHashAlg {
        Mean,
        Gradient,
        VertGradient,
        DoubleGradient,
        Blockhash,
    }
}

impl From<ClapHashAlg> for img_hash::HashAlg {
    fn from(hash_alg: ClapHashAlg) -> Self {
        match hash_alg {
            ClapHashAlg::Mean => img_hash::HashAlg::Mean,
            ClapHashAlg::Gradient => img_hash::HashAlg::Gradient,
            ClapHashAlg::VertGradient => img_hash::HashAlg::VertGradient,
            ClapHashAlg::DoubleGradient => img_hash::HashAlg::DoubleGradient,
            ClapHashAlg::Blockhash => img_hash::HashAlg::Blockhash,
        }
    }
}


mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}


fn real_main() -> i32 {

    let about = format!("Creates or compares hash values from images.\nhttps://github.com/KevinGliewe/imghash\nBuild time: {}\nGit commit hash: {}",
        build_info::BUILT_TIME_UTC,
        match build_info::GIT_COMMIT_HASH {
            Some(hash) => hash,
            None =>  ""
        }
    );

    let matches = 
        App::new("imghash")
            .version(build_info::PKG_VERSION)
            .author("Kevin Gliewe <kevingliewe@gmail.com>")
            .about(about.as_str())
            .arg(Arg::with_name("width")
                .short("w")
                .long("width")
                .help("Hash width: Default = 8")
                .takes_value(true)
                .use_delimiter(false))
            .arg(Arg::with_name("height")
                .short("h")
                .long("height")
                .help("Hash height: Default = 8")
                .takes_value(true)
                .use_delimiter(false))
            .arg(Arg::with_name("resize_filter")
                .short("f")
                .long("resize_filter")
                .help("Resize Filter: Default = Lanczos3")
                .takes_value(true)
                .case_insensitive(true)
                .possible_values(&ClapFilterType::iter_variant_names().collect::<Vec<&'static str>>()))
            .arg(Arg::with_name("hash_alg")
                .short("a")
                .long("hash_alg")
                .help("Hash algorithm: Default = Gradient")
                .takes_value(true)
                .case_insensitive(true)
                .possible_values(&ClapFilterType::iter_variant_names().collect::<Vec<&'static str>>()))

            .arg(Arg::with_name("IMAGE")
                .help("Sets the input image path")
                .required(true)
                .index(1))
            .arg(Arg::with_name("IMAGE_CMP")
                .help("Sets the compare image path (If not set, the hash of IMAGE will get printet out)")
                .required(false)
                .index(2))
            .get_matches();
    
    let mut config = img_hash::HasherConfig::new();


    config = config.hash_size(
            match matches.value_of("width") {
            Some(w) => match w.parse::<u32>() {
                Ok(w) => w,
                Err(_) => {
                    println!("Invalid width value: {}", w);
                    return 1;
                }
            },
            None => 8
        }, 
            match matches.value_of("height") {
            Some(h) => match h.parse::<u32>() {
                Ok(h) => h,
                Err(_) => {
                    println!("Invalid height value: {}", h);
                    return 2;
                }
            },
            None => 8
        }
    );

    config = config.resize_filter(
        img_hash::FilterType::from(
            match matches.value_of("resize_filter") {
                Some(f) => match f.parse::<ClapFilterType>() {
                    Ok(f) => f,
                    Err(_) => {
                        println!("Invalid resize filter: {}", f);
                        return 3;
                    }
                },
                None => ClapFilterType::Lanczos3
            }
        )
    );

    config = config.hash_alg(
        img_hash::HashAlg::from(
            match matches.value_of("hash_alg") {
                Some(f) => match f.parse::<ClapHashAlg>() {
                    Ok(f) => f,
                    Err(_) => {
                        println!("Invalid hash alg: {}", f);
                        return 4;
                    }
                },
                None => ClapHashAlg::Gradient
            }
        )
    );

    let hasher = config.to_hasher();


    let hash1 = hasher.hash_image(&match image::open(match matches.value_of("IMAGE") {
        Some(f) => f,
        None => {
            println!("No image specified");
            return 5;
        }
    }) {
        Ok(img) => img,
        Err(e) => {
            println!("Error opening image: {}", e);
            return 6;
        }
    });


    match matches.value_of("IMAGE_CMP") {
        Some(image2_path) => {
            let hash2 = hasher.hash_image(&match image::open(image2_path) {
                Ok(img) => img,
                Err(e) => {
                    println!("Error opening image: {}", e);
                    return 7;
                }
            });
            
            println!("{}", hash1.dist(&hash2));
            
        },
        None => {
            let hash_str = hash1.as_bytes().iter().map(|b| format!("{:02x}", b)).collect::<String>();
            println!("{}", hash_str);
        }
    }

    return 0;
}

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code);
}