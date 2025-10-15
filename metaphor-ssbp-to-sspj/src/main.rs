use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use image::ImageReader;
use image::codecs::png::PngEncoder;
use metaphor_apk_rs::read::ApkReader;
use walkdir::WalkDir;
use ssbp6_lib::cell::Cell;
use ssbp6_lib::project::ProjectHeader;

#[derive(Debug)]
pub enum AppError {
    PrintUsage,
    InputDoesNotExist(String),
    WrongFileType(String),
    UnknownMetadata(String),
    NotInMetaphorCpk(String),
    DuplicateIndex(usize),
    FailedApkRead,
    FailedApkGetFile,
}

impl Error for AppError {}
impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PrintUsage => {
                write!(f,"Usage instructions:\n\
./metaphor-ssbp-to-sspj [input] [output] [locale]\n\
Input: Either a file to a single SSBP or a folder containing one or more SSBPs. \n\
If loading from an extracted Metaphor CPK, this should be within COMMON/ui/ss to allow locale parameter to work \n\
Output: A folder where the sprite's output files are exported to\n\
Locale (optional): A language ID supported by Metaphor. Default is EN")
            },
            _ => <Self as Debug>::fmt(self, f)
        }

    }
}

fn main() {
    if let Err(e) = app() {
        println!("{}", e);
    }
}
fn is_sprite(ext: Option<&OsStr>) -> bool {
    ext.map_or(false, |ext| ext.to_str().unwrap() == "ssbp")
}

fn app() -> Result<(), Box<dyn Error>> {
    // handle CLI arguments
    let args: Vec<String> = std::env::args().enumerate()
        .filter_map(|(i, a)| if i > 0 { Some(a) } else { None }).collect();
    if args.len() < 2 {
        return Err(Box::new(AppError::PrintUsage));
    }
    if !std::fs::exists(&args[0])? {
        return Err(Box::new(AppError::InputDoesNotExist(args[0].clone())));
    }
    let is_file = std::fs::metadata(&args[0])?.is_file();
    if is_file && !is_sprite(Path::new(&args[0]).extension()) {
        return Err(Box::new(AppError::WrongFileType(args[0].clone())));
    }
    let locale = match args.len() < 3 {
        true => "EN".to_string(), false => args[2].clone()
    };

    // get file directories
    let mut path_parent = PathBuf::new();
    let mut path_tex = PathBuf::new();
    let mut path_locale = PathBuf::new();
    let mut in_common_cpk = false;
    let comp_list = match is_file {
        true => Path::new(&args[0]).parent().unwrap(),
        false => Path::new(&args[0])
    };
    for comp in comp_list.components() {
        path_parent.push(comp);
        if let std::path::Component::Normal(n) = &comp {
            let part = n.to_str().unwrap();
            if part == "COMMON" && !in_common_cpk {
                path_locale.push(&locale);
                path_tex.push(comp);
                path_tex.push("4K");
                path_locale.push("4K");
                in_common_cpk = true;
                continue;
            }
        }
        path_tex.push(comp);
        path_locale.push(comp);
    }
    if !in_common_cpk {
        return Err(Box::new(AppError::NotInMetaphorCpk(args[0].clone())));
    }
    std::fs::create_dir_all(&args[1])?;
    if is_file {
        let filename = Path::new(&args[0]).file_name().unwrap().to_str().unwrap();
        read_file(path_parent.as_path(), Path::new(filename), path_locale.as_path(),
                  path_tex.as_path(), Path::new(&args[1]))
    } else {
        for file in WalkDir::new(path_parent.as_path()).into_iter()
            // .filter_entry(|e| filter_sprites(e)) {
            .filter_entry(|e| e.file_type().is_dir() || is_sprite(e.path().extension())) {
            let file = file?;
            if file.file_type().is_dir() {
                continue;
            }
            let file_stem = file.path().file_stem().unwrap().to_str().unwrap();
            let filename = file.path().file_name().unwrap().to_str().unwrap();
            let folder = PathBuf::from(&args[1]).join(file_stem);
            if !std::fs::exists(folder.as_path())? {
                std::fs::create_dir(folder.as_path())?;
            }
            read_file(path_parent.as_path(), Path::new(filename), path_locale.as_path(),
                      path_tex.as_path(), folder.as_path())?;
        }
        Ok(())
    }
}

fn read_file<P: AsRef<Path>>(parent: P, filename: P, locale: P, tex: P, output: P) -> Result<(), Box<dyn Error>> {
    println!("{:?}", parent.as_ref().join(filename.as_ref()));
    let binary = std::fs::read(parent.as_ref().join(filename.as_ref()))?;
    // println!("{} bytes", binary.len());
    let header = unsafe { &*(binary.as_ptr().add(0) as *const ProjectHeader) };
    // println!("{:?}", header);
    let mut cells: HashMap<u16, Cell> = HashMap::new();
    // cell array index -> (map index, cell name) (for getting reference in ssae)
    let mut cell_resolver_anime: HashMap<usize, (u16, &str)> = HashMap::new();
    for (i, entry) in header.get_cells(&binary).iter().enumerate() {
        let map = entry.get_cell_map(&binary);
        match cells.get_mut(&map.get_index()) {
            Some(cell) => cell.add(entry),
            None => { let _ = cells.insert(map.get_index(), Cell::new(map, entry)); }
        };
        cell_resolver_anime.insert(i, (map.get_index(), entry.get_name(&binary)));
    }
    let mut cell_names = Vec::with_capacity(cells.len());
    for i in 0..cells.len() {
        cell_names.push(String::new());
    }
    for (i, cell) in &cells {
        let val = cell.to_xml(&binary, |image_path|  {
                let (base, ext) = image_path.rsplit_once(".").unwrap();
                let (name, is_apk) = match ext {
                    "apk" => (format!("{}.png", base), true),
                    _ => (image_path.to_string(), false)
                };
                let dims = match is_apk {
                    true => {
                        if !std::fs::exists(output.as_ref().join(&name))? {
                            let mut apk_path = tex.as_ref().join(image_path).to_str().unwrap().to_string();
                            // check locale for APK
                            if !std::fs::exists(&apk_path)? {
                                apk_path = locale.as_ref().join(image_path).to_str().unwrap().to_string();
                            }
                            let mut reader = ApkReader::read(&apk_path)
                                .map_err(|_| std::io::Error::other(AppError::FailedApkRead))?;
                            // load the target DDS file
                            let dds = reader.get_file(&format!("{}.dds", base))
                                .map_err(|_| std::io::Error::other(AppError::FailedApkGetFile))?;
                            // convert into image::DynamicImage
                            let dds_fmt = ddsfile::Dds::read(dds.as_slice())
                                .map_err(|e| std::io::Error::other(e))?;
                            let new_img = image_dds::image_from_dds(&dds_fmt, 0)
                                .map_err(|e| std::io::Error::other(e))?;
                            let mut as_png = File::create(output.as_ref().join(&name))?;
                            // re-encode as PNG
                            new_img.write_with_encoder(PngEncoder::new(&mut as_png))
                                .map_err(|e| std::io::Error::other(e))?;
                            (dds_fmt.get_width(), dds_fmt.get_height())
                        } else {
                            // use existing re-encoded PNG
                            let bytes = std::fs::read(output.as_ref().join(&name))?;
                            ImageReader::new(Cursor::new(bytes.as_slice())).with_guessed_format()?.into_dimensions()
                                .map_err(|e| std::io::Error::other(e))?
                        }
                    },
                    false => {
                        let bytes = std::fs::read(parent.as_ref().join(&name))?;
                        ImageReader::new(Cursor::new(bytes.as_slice())).with_guessed_format()?.into_dimensions()
                            .map_err(|e| std::io::Error::other(e))?
                    }
                };
                Ok((name, dims.into()))
        })?;
        cell_names[*i as usize] = format!("{}.ssce", cell.get_name(&binary));
        // cell_names.push(format!("{}.ssce", cell.get_name(&binary)));
        std::fs::write(output.as_ref().join(&cell_names[*i as usize]), &val)?;
    }
    let mut anime_names = Vec::with_capacity(header.get_num_anime() as usize);
    for anime in header.get_anime(&binary) {
        // let val = anime.to_xml(&cell_names, &binary, header.get_cells(&binary))?;
        // let val = anime.to_xml(&cell_names, &binary, &cells)?;
        let val = anime.to_xml(&cell_names, &binary, &cell_resolver_anime)?;
        anime_names.push(format!("{}.ssae", anime.get_name(&binary)));
        std::fs::write(output.as_ref().join(anime_names.last().unwrap()), &val)?;
    }
    let name = filename.as_ref().file_stem().unwrap().to_str().unwrap();
    let proj_xml = header.to_xml(name, &cell_names, &anime_names)?;
    std::fs::write(output.as_ref().join(format!("{}.sspj", name)), proj_xml.as_slice())?;
    Ok(())
}