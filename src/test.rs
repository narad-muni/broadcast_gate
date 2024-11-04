use web_view::*;

fn main() -> WVResult {
    let webview = web_view::builder()
        .title("Dialog example")
        .content(Content::Html(HTML))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match arg {
                _ => println!("Callback {}", arg),
            };
            Ok(())
        })
        .build()?;

    webview.run()
}

const HTML: &str = r#"
<!doctype html>
<html>
    <body>
        <button onclick="external.invoke('open')">Open</button>
        <button onclick="external.invoke('save')">Save</button>
        <button onclick="external.invoke('info')">Info</button>
        <button onclick="external.invoke('warning')">Warning</button>
        <button onclick="external.invoke('error')">Error</button>
        <button onclick="external.invoke('exit')">Exit</button>
    </body>
</html>
"#;
