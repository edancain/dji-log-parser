use binrw::binread;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::io::SeekFrom;

#[binread]
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[br(little, import(version: u8))]
pub struct Details {
    #[br(count=20, map = |s: Vec<u8>| String::from_utf8_lossy(&s).trim_end_matches('\0').to_string())]
    pub sub_street: String,
    #[br(count=20, map = |s: Vec<u8>| String::from_utf8_lossy(&s).trim_end_matches('\0').to_string())]
    pub street: String,
    #[br(count=20, map = |s: Vec<u8>| String::from_utf8_lossy(&s).trim_end_matches('\0').to_string())]
    pub city: String,
    #[br(count=20, map = |s: Vec<u8>| String::from_utf8_lossy(&s).trim_end_matches('\0').to_string())]
    pub area: String,
    pub is_favorite: u8,
    pub is_new: u8,
    pub needs_upload: u8,
    pub record_line_count: i32,
    pub detail_info_checksum: i32,
    #[br(map = |x: i64| DateTime::from_timestamp(x / 1000, (x % 1000 * 1000000) as u32).unwrap_or_default())]
    pub start_time: DateTime<Utc>,
    /// degrees
    pub longitude: f64,
    /// degrees
    pub latitude: f64,
    /// meters
    pub total_distance: f32,
    /// seconds
    #[br(map = |x: i32| x as f64 / 1000.0)]
    pub total_time: f64,
    /// meters
    pub max_height: f32,
    /// meters / seconds
    pub max_horizontal_speed: f32,
    /// meters / seconds
    pub max_vertical_speed: f32,
    pub capture_num: i32,
    pub video_time: i64,
    pub moment_pic_image_buffer_len: [i32; 4],
    pub moment_pic_shrink_image_buffer_len: [i32; 4],
    /// degrees
    #[br(map = |v: [f64; 4]| v.map(|rad: f64| rad.to_degrees()) )]
    pub moment_pic_longitude: [f64; 4],
    /// degrees
    #[br(map = |v: [f64; 4]| v.map(|rad: f64| rad.to_degrees()) )]
    pub moment_pic_latitude: [f64; 4],
    #[br(temp)]
    _analysis_offset: i64,
    #[br(temp)]
    _user_api_center_id_md5: [u8; 16],
    #[br(seek_before = if version <= 5 { SeekFrom::Start(352) } else { SeekFrom::Current(0) })]
    pub take_off_altitude: f32,
    #[br(
        seek_before = if version <= 5 { SeekFrom::Start(277) } else { SeekFrom::Current(0) },
        map = |x: u8| ProductType::from(x))
    ]
    pub product_type: ProductType,
    #[br(temp)]
    _activation_timestamp: i64,
    #[br(
        seek_before = if version <= 5 { SeekFrom::Start(278) } else { SeekFrom::Current(0) },
        count = if version <= 5 { 24 } else { 32 }, map = |s: Vec<u8>| String::from_utf8_lossy(&s).trim_end_matches('\0').to_string()
    )]
    pub aircraft_name: String,
    #[br(
        seek_before = if version <= 5 { SeekFrom::Start(267) } else { SeekFrom::Current(0) },
        count = if version <= 5 { 10 } else { 16 }, map = |s: Vec<u8>| String::from_utf8_lossy(&s).trim_end_matches('\0').to_string()
    )]
    pub aircraft_sn: String,
    #[br(
        seek_before = if version <= 5 { SeekFrom::Start(318) } else { SeekFrom::Current(0) },
        count = if version <= 5 { 10 } else { 16 }, map = |s: Vec<u8>| String::from_utf8_lossy(&s).trim_end_matches('\0').to_string()
    )]
    pub camera_sn: String,
    #[br(count = if version <= 5 { 10 } else { 16 }, map = |s: Vec<u8>| String::from_utf8_lossy(&s).trim_end_matches('\0').to_string())]
    pub rc_sn: String,
    #[br(count = if version <= 5 { 10 } else { 16 }, map = |s: Vec<u8>| String::from_utf8_lossy(&s).trim_end_matches('\0').to_string())]
    pub battery_sn: String,
    #[br(map = |x: u8| Platform::from(x))]
    pub app_platform: Platform,
    #[br(map = |x: [u8; 3]| format!("{}.{}.{}", x[0], x[1], x[2]))]
    pub app_version: String,
}

#[derive(Serialize, Debug, Clone, PartialEq, Default, Copy)]
pub enum ProductType {
    #[default]
    None,
    Inspire1,
    Phantom3Standard,
    Phantom3Advanced,
    Phantom3Pro,
    OSMO,
    Matrice100,
    Phantom4,
    LB2,
    Inspire1Pro,
    A3,
    Matrice600,
    Phantom34K,
    MavicPro,
    ZenmuseXT,
    Inspire1RAW,
    A2,
    Inspire2,
    OSMOPro,
    OSMORaw,
    OSMOPlus,
    Mavic,
    OSMOMobile,
    OrangeCV600,
    Phantom4Pro,
    N3FC,
    Spark,
    Matrice600Pro,
    Phantom4Advanced,
    Phantom3SE,
    AG405,
    Matrice200,
    Matrice210,
    Matrice210RTK,
    MavicAir,
    Mavic2,
    Phantom4ProV2,
    Phantom4RTK,
    P4Multispectral,
    Mavic2Enterprise,
    MavicMini,
    Matrice200V2,
    Matrice210V2,
    Matrice210RTKV2,
    MavicAir2,
    Matrice300RTK,
    FPV,
    MavicAir2S,
    Mini2,
    Mavic3,
    MiniSE,
    Mini3Pro,
    Mavic3Pro,
    Matrice30,
    Mavic3Enterprise,
    Avata,
    Mini4Pro,
    Avata2,
    Matrice350RTK,
    #[serde(untagged)]
    Unknown(u8),
}

impl From<u8> for ProductType {
    fn from(num: u8) -> Self {
        match num {
            0 => ProductType::None,
            1 => ProductType::Inspire1,
            2 => ProductType::Phantom3Standard,
            3 => ProductType::Phantom3Advanced,
            4 => ProductType::Phantom3Pro,
            5 => ProductType::OSMO,
            6 => ProductType::Matrice100,
            7 => ProductType::Phantom4,
            8 => ProductType::LB2,
            9 => ProductType::Inspire1Pro,
            10 => ProductType::A3,
            11 => ProductType::Matrice600,
            12 => ProductType::Phantom34K,
            13 => ProductType::MavicPro,
            14 => ProductType::ZenmuseXT,
            15 => ProductType::Inspire1RAW,
            16 => ProductType::A2,
            17 => ProductType::Inspire2,
            18 => ProductType::OSMOPro,
            19 => ProductType::OSMORaw,
            20 => ProductType::OSMOPlus,
            21 => ProductType::Mavic,
            22 => ProductType::OSMOMobile,
            23 => ProductType::OrangeCV600,
            24 => ProductType::Phantom4Pro,
            25 => ProductType::N3FC,
            26 => ProductType::Spark,
            27 => ProductType::Matrice600Pro,
            28 => ProductType::Phantom4Advanced,
            29 => ProductType::Phantom3SE,
            30 => ProductType::AG405,
            31 => ProductType::Matrice200,
            33 => ProductType::Matrice210,
            34 => ProductType::Matrice210RTK,
            38 => ProductType::MavicAir,
            42 => ProductType::Mavic2,
            44 => ProductType::Phantom4ProV2,
            46 => ProductType::Phantom4RTK,
            57 => ProductType::P4Multispectral,
            58 => ProductType::Mavic2Enterprise,
            59 => ProductType::MavicMini,
            60 => ProductType::Matrice200V2,
            61 => ProductType::Matrice210V2,
            62 => ProductType::Matrice210RTKV2,
            67 => ProductType::MavicAir2,
            70 => ProductType::Matrice300RTK,
            73 => ProductType::FPV,
            75 => ProductType::MavicAir2S,
            76 => ProductType::Mini2,
            77 => ProductType::Mavic3,
            96 => ProductType::MiniSE,
            103 => ProductType::Mini3Pro,
            111 => ProductType::Mavic3Pro,
            116 => ProductType::Matrice30,
            118 => ProductType::Mavic3Enterprise,
            121 => ProductType::Avata,
            126 => ProductType::Mini4Pro,
            152 => ProductType::Avata2,
            170 => ProductType::Matrice350RTK,
            _ => ProductType::Unknown(num),
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Platform {
    IOS,
    Android,
    DJIFly,
    Window,
    Mac,
    Linux,
    #[serde(untagged)]
    Unknown(u8),
}

impl From<u8> for Platform {
    fn from(num: u8) -> Self {
        match num {
            1 => Platform::IOS,
            2 => Platform::Android,
            6 => Platform::DJIFly,
            10 => Platform::Window,
            11 => Platform::Mac,
            12 => Platform::Linux,
            _ => Platform::Unknown(num),
        }
    }
}
