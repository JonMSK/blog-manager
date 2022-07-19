use std::fs;
use std::io::Write;

/// Saves the current entered text into the specified file.
pub fn save_post(myapp: &mut super::MyApp) {
    let mut out = fs::File::create(format!("E:/Programming/Web/git_blog/website/posts/{}.html", myapp.file_name))
        .unwrap();
    write!(out, "<!DOCTYPE html>
<head>
    <link rel='stylesheet' href='../style.css' />
</head>
<body>
    <div id='wrapper'>
        <div id='sidebar'>
            <a href='../index.html'><p class='title'>Jon's<br>blog</p></a>
            <a href='https://github.com/deetonate/deetonate/blob/main/README.md' class='sidebar-content'><p class='title sidebar-content'><i>Resume (temp)</i></p></a>
        </div>
        <div id='main-content'>
        {}
        </div>
    </div>
</body>", myapp.content)
        .expect("Failed to write to post file");
}

/// Updates index.html with the newest data.
pub fn update_index() {
    let mut out = fs::File::create("E:/Programming/Web/git_blog/website/index.html").unwrap();
    let post_paths = fs::read_dir("E:/Programming/Web/git_blog/website/posts").unwrap();
    let mut stringlist = String::from("<ul>");

    for post_path in post_paths {
        let mut post_path = post_path.unwrap().file_name().into_string().unwrap();

        stringlist.push_str(format!("<li><a href='posts/{}'>",
            post_path
        ).as_str());

        for _i in 0..5 {
            post_path.pop();
        }

        stringlist.push_str(format!("{}</a></li>",
            post_path
        ).as_str());
    }

    stringlist.push_str("</ul>");

    write!(out, "<!DOCTYPE html>
<head>
    <link rel='stylesheet' href='style.css' />
</head>
<body>
    <div id='wrapper'>
        <div id='sidebar'>
            <a href='index.html'><p class='title'>Jon's<br>blog</p></a>
            <a href='https://github.com/deetonate/deetonate/blob/main/README.md' class='sidebar-content'><p class='title sidebar-content'><i>Resume (temp)</i></p></a>
        </div>
        <div id='main-content' class='index'>
        {}
        </div>
    </div>
</body>", stringlist)
    .expect("Failed to write to post file");
}