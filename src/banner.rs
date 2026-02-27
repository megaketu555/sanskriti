pub fn display_banner() {
    let banner_text = r#"
   _____                  __        _ __  _ 
  / ___/____ _____  _____/ /_______(_) /_(_)
  \__ \/ __ `/ __ \/ ___/ //_/ ___/ / __/ /    
 ___/ / /_/ / / / (__  ) ,< / /  / / /_/ /  
/____/\__,_/_/ /_/____/_/|_/_/  /_/\__/_/   
"#;
   let author_tag ="\x1b[32mVersion 1.1.0 For The Lord!\x1b[0m";
   let version_tag = "\x1b[1;31mCrafted by megaketu555\x1b[0m";
    println!("{}",banner_text);
    println!("{}",author_tag);
    println!("{}",version_tag)
}
