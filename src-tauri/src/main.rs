#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use clap::{Arg, App};
use tauri::Manager;
fn main() {
  
  tauri::Builder::default()
  .setup(|app| {
    let matches = App::new("Wiz")
      .version("0.1.0")
      .author("Justin Maximillian Kimlim <kimlimjustin@gmail.com>")
      .about("Open a url on Wiz")
      .arg(Arg::new("url")
          .about("Url to be opened on Wiz. http:// protocol can be ommited.")
          .index(1))
      .get_matches();
    let mut url:String = "https://github.com/kimlimjustin/wiz".to_string();
    if let Some(i) = matches.value_of("url") {
      url = if i.starts_with("http://") || i.starts_with("https://") {
        i.to_string()
      } else {
        format!("http://{}", i)
      }
    }
    let main_window = app.get_window("main").unwrap();
      
    main_window.eval(format!("window.location.replace('{url}')", url=url).as_str()).unwrap();
    Ok(())
  })
  .on_page_load(|window, _| {
    window.eval("(() => {
      const el = document.createElement('div')
      el.setAttribute('data-tauri-drag-region','')
      el.id = 'wiz-drag-area'
      const style = document.createElement('style')
      style.innerHTML='#wiz-drag-area{position:fixed;left:10px;top:10px;width:40px;height:40px;border-radius:50%;cursor:grab;-webkit-app-region:drag;z-index:99999999;}#wiz-drag-area:hover{background:#8885;}'
      document.body.appendChild(el)
      document.body.appendChild(style)
      
      const rootStyle = document.createElement('style')
      rootStyle.innerHTML='::-webkit-scrollbar {display: none;}'
      document.head.appendChild(rootStyle)
      document.addEventListener('keydown', e => {
        if (e.key === 'Escape') {
          window.__TAURI__.window.appWindow.close()
        }
      })
      })()").unwrap();
  })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
