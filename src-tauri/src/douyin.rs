
use std::{time::Duration, path::Path, sync::Arc};
use futures::future::join_all;
use serde_json::Value;
use tauri::{regex::Regex, Window};
use serde::{Serialize, Deserialize};
use tokio::time::sleep;
use crate::downloader::Downloader;
use thiserror::Error;
use anyhow::Result;

const USER_AGNET: &'static str = "Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.3 Mobile/15E148 Safari/604.1";

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub nickname: String,
    pub uid: String,
    pub avatar_url: String,
    pub video_count: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoInfoItem {
    pub video_id: String,   // 视频ID
    pub video_title: String, // 视频标题
    pub video_url: String,  // 视频链接
    pub cover_url: String, // 视频封面URL
   // pub music_url: String, // 视频音频URL
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoInfo {
    pub max_cursor: u64,
    pub has_more: bool,
    pub items: Vec<VideoInfoItem>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserVideoInfo {
    user_info: UserInfo,
    video_info: VideoInfo,
}


#[derive(Error, Debug)]
enum DouyinError {
    
    #[error("系统错误")]
    SystemError,

    #[error("未找到视频")]
    VideoInfoNotFoundError,
    
    #[error("网络错误")]
    NetworkError,

    #[error("获取数据失败")]
    GetDataError,

    #[error("下载视频失败")]
    DownloadVideoError,

    #[error("获取用户信息失败")]
    GetUserInfoFailureError,

    #[error("未找到用户")]
    UserInfoNotFoundError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElProgress {
    pub percentage: u8
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DouyinMuplitDownloadProgress {
    pub video_id: String,
    pub video_title: String,
    pub save_path: String,
    pub is_success: bool,
}

fn get_id_from_url(url: &mut String) -> String {
    if !url.contains("?") {
        url.push('?');
    }
    let regex = Regex::new(r"/[video|user]+/(?P<aweme_id>\S+)[/|\?]+").unwrap();
    let aweme_id = match regex.captures(&url) {
        Some(cap) => {
            cap.name("aweme_id").unwrap().as_str().replace('/', "").to_string()
        },
        None => "".to_string()
    };
    aweme_id
}


/// 解析视频,音频,封面URL
#[tauri::command]
pub async fn douyin_single_search(url: String) -> Result<UserVideoInfo, String> {

    let client = reqwest::Client::builder()
        .user_agent(USER_AGNET)
        .build().unwrap();
    let mut real_url = client.get(&url)
        .send()
        .await
        .map_err(|_| DouyinError::NetworkError.to_string())?
        .url()
        .to_string();

    let aweme_id = get_id_from_url(&mut real_url);
    if aweme_id.is_empty() {
        return Err(DouyinError::VideoInfoNotFoundError.to_string());
    }
    let api_url = format!("https://www.iesdouyin.com/web/api/v2/aweme/iteminfo/?item_ids={}", aweme_id);
    
    let data:Value = client.get(&api_url)
        .send()
        .await
        .map_err(|_| DouyinError::NetworkError.to_string())?
        .json::<Value>()
        .await
        .map_err(|_| DouyinError::GetDataError.to_string())?
        ["item_list"][0].clone();

    let video_id = data["aweme_id"]
        .to_string()
        .replace('"', "");

    let mut video_title = data["share_info"]["share_title"]
        .to_string()
        .replace('"', "")
        .split("@")
        .collect::<Vec<&str>>()[0]
        .to_string();

    if video_title.is_empty() {
        video_title = format!("无标题: {}", video_id);
    }

    let video_url = data["video"]["play_addr"]["url_list"][0]
        .to_string()
        .replace('"', "")
        .replace("playwm", "play")
        .replace("ratio=720p", "ratio=1080p").replace('"', "");
    
    let video_cover_url:String = data["video"]["origin_cover"]["url_list"][0]
        .to_string()
        .replace('"', "");

    // let video_music_url = data["music"]["play_url"]["url_list"][0]
    //     .to_string()
    //     .replace('"', "");

    let author_uid = data["author"]["uid"]
        .to_string()
        .replace('"', "");

    let author_name = data["author"]["nickname"]
        .to_string()
        .replace('"', "");

    let author_avatar = data["author"]["avatar_thumb"]["url_list"][0]
        .to_string()
        .replace('"', "");

    let user_info =  UserInfo { 
        nickname: author_name, 
        uid: author_uid, 
        avatar_url: author_avatar,
        video_count: 1
    };

    let video_list = vec![
        VideoInfoItem {
            video_id,
            video_title,
            video_url,
            cover_url: video_cover_url,
        }
    ];
    let video_info = VideoInfo { 
        max_cursor: 0,
        has_more: false, 
        items: video_list
    };

    let res = UserVideoInfo { user_info, video_info };

    Ok(res)

}


#[tauri::command]
pub async fn douyin_single_download(save_path: String, video_url: String, window: Window) -> Result<(), String> {
    let downloader = Downloader::new(video_url, save_path, Some(8))
        .await
        .map_err(|_|DouyinError::SystemError.to_string())?;

    let downloader_clone = downloader.clone();
    tokio::spawn(async move {
        let total_size = downloader_clone.total_size();
        loop {
            let cur_size  = downloader_clone.downloaded_size().await;
            if cur_size >= total_size {
                let _ = window.emit("douyin_single_download", ElProgress{ percentage: 100 });
                break;
            }
            let percentage = (cur_size as f64 * 100.0 / total_size as f64 ).round() as u8;
            let _ = window.emit("douyin_single_download", ElProgress{ percentage });
            sleep(Duration::from_millis(100)).await;
        }
    });
    match downloader.download().await {
        Ok(_) => {
            sleep(Duration::from_millis(100)).await
        },
        Err(_) => return Err(DouyinError::DownloadVideoError.to_string()),
    };
    Ok(())
}


// 获取用户信息
async fn get_user_info(uid: &String) -> Result<UserInfo> {
    
    let api_url = format!("https://www.iesdouyin.com/web/api/v2/user/info/?sec_uid={}", uid);

    let data = reqwest::Client::builder()
        .user_agent(USER_AGNET)
        .build()?
        .get(&api_url)
        .send()
        .await?
        .json::<Value>()
        .await?;

    
    let nickname = data["user_info"]["nickname"]
        .to_string()
        .replace('"', "");
        
    let video_count = data["user_info"]["aweme_count"]
        .as_u64()
        .map_or(0, |i|i) as u16;

    let avatar_url = data["user_info"]["avatar_thumb"]["url_list"][0]
        .to_string()
        .replace('"', "");

    Ok(UserInfo{
        nickname,
        uid: uid.to_string(),
        avatar_url,
        video_count,
    })
}  

async fn get_user_video_list(uid: String, count: u16, max_cursor: u64) -> Result<VideoInfo> {
    let api_url = format!("https://www.iesdouyin.com/web/api/v2/aweme/post/?sec_uid={uid}&count={count}&max_cursor={max_cursor}");

    let data = reqwest::Client::builder()
        .user_agent(USER_AGNET)
        .build()?
        .get(&api_url)
        .send()
        .await?
        .json::<Value>()
        .await?;

    let max_cursor = data["max_cursor"].as_u64().unwrap_or(0);
    let has_more = data["has_more"].as_bool().unwrap_or(false);

    if !data["aweme_list"].is_array() || data["aweme_list"].as_array().unwrap().len() == 0 {
        return Ok(VideoInfo {
            max_cursor,
            has_more,
            items: Vec::<VideoInfoItem>::new(),
        });
    }

    let video_items = data["aweme_list"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| {
            let video_id = item["aweme_id"].to_string().replace('"', "");
            let mut video_title = item["desc"]
                .to_string()
                .replace('"', "")
                .split("#")
                .collect::<Vec<&str>>()[0]
                .to_string()
                .split("@")
                .collect::<Vec<&str>>()[0]
                .to_string();
            if video_title.is_empty() {
                video_title = format!("无标题{}", video_id);
            }
            let video_url = item["video"]["play_addr"]["url_list"][0].to_string().replace('"', "");
            let cover_url = item["video"]["cover"]["url_list"][0].to_string().replace('"', "");
            VideoInfoItem {
                video_id,
                video_title,
                video_url,
                cover_url,
            }
        }).collect::<Vec<VideoInfoItem>>();
    
    Ok(VideoInfo {
        max_cursor,
        has_more,
        items: video_items,
    })
}


#[tauri::command]
pub async fn douyin_muplit_search(home_url: String) -> Result<UserVideoInfo, String>  {

    let client = reqwest::Client::builder()
        .user_agent(USER_AGNET)
        .build()
        .map_err(|_|{ DouyinError::SystemError.to_string()})?;
    
    let mut real_url = client.get(&home_url)
        .send()
        .await
        .map_err(|_| DouyinError::NetworkError.to_string())?
        .url()
        .to_string();

    let uid = get_id_from_url(&mut real_url);

    if uid.is_empty() {
        return Err(DouyinError::UserInfoNotFoundError.to_string()); 
    }
    
    let user_info = get_user_info(&uid)
        .await
        .map_err(|_|{ DouyinError::GetUserInfoFailureError
        .to_string()})?;

    let video_info = get_user_video_list(uid, user_info.video_count, 0)
        .await
        .map_err(|_| {DouyinError::VideoInfoNotFoundError.to_string()})?;
    
    Ok(UserVideoInfo { user_info, video_info })
}


// 获取所有的视频信息
#[tauri::command]
pub async fn douyin_get_all_video_info(uid: String, video_count: u16, max_cursor: u64, window: tauri::Window) -> Result<(), String> {
 
    let mut cursor = max_cursor;
    let mut retry_num = 3;
   
    loop {
        let result = get_user_video_list(uid.clone(), video_count, cursor)
        .await
        .map_err(|_| {DouyinError::NetworkError.to_string()});
        if let Ok(v_info) = result {
            cursor = v_info.max_cursor;

            let has_more = v_info.has_more;
            
            let _ = window.emit("douyin_get_all_video_info", v_info );

            if !has_more { 
                break;
            }
        }else {
            if retry_num < 0 {
                break;
            }
            retry_num -= 1;
        }
    }
    Ok(())
}

pub fn get_save_path(save_dir: &String, video_title: &String) -> String {

    let items: Vec<&str> = video_title.split("#").collect();
    let save_path = Path::new(&save_dir).join(&items[0].trim()).to_str().unwrap().to_string() + ".mp4"; 

    save_path
}

#[tauri::command]
pub async fn douyin_muplit_download(items: Vec<VideoInfoItem>, save_dir: String, window: Window) -> Result<(), String>{

    let window = Arc::new(window);
    let mut handler_list = Vec::new();
    for item in items.iter() {
        let video_title = Arc::new(item.video_title.clone());
        let video_title_clone = video_title.clone();
        let save_path = Arc::new(get_save_path(&save_dir, &video_title.clone())); 
        let save_path_clone = save_path.clone();
        let downloader = Downloader::new(item.video_url.clone(), save_path.clone().to_string(), Some(8)).await.unwrap();
        let downloader_clone = downloader.clone();
        let video_id = Arc::new(item.video_id.clone());
        let video_id_clone = video_id.clone();
        let window_progress = window.clone();
        let window_download = window.clone();
        tokio::spawn(async move {
            if let Err(_) = downloader.download().await {
                let _ = window_download.emit("douyin_muplit_download", DouyinMuplitDownloadProgress { 
                    video_id: video_id.to_string(),
                    is_success: false,
                    video_title: video_title.clone().to_string(),
                    save_path: save_path.clone().to_string(),
                });
            }
            
        });
        let handler = tokio::spawn(async move {
            let filesize = downloader_clone.total_size();
            loop {
                let download_size = downloader_clone.downloaded_size().await;
                if download_size >= filesize {
                    let _ = window_progress.emit("douyin_muplit_download", DouyinMuplitDownloadProgress { 
                        video_id: video_id_clone.to_string(), 
                        is_success: true,
                        video_title: video_title_clone.clone().to_string(),
                        save_path: save_path_clone.to_string()
                    });
                    break;
                }
                sleep(Duration::from_secs(1)).await;
            }
        });
        handler_list.push(handler);
    }
    join_all(handler_list).await;
    Ok(())
}