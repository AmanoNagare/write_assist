use nannou::prelude::*;
use nannou::text::*;

fn main() {
    nannou::app(model)
        .event(event) // イベント処理関数を追加
        .update(update)
        .run();
}

struct Model{
    font: Font,
    page: usize,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1600, 900)
        .title("Enter Key Example")
        .view(view)
        .build()
        .unwrap();
    let font_path = std::path::Path::new("assets/UDDigiKyokashoN-R001.ttf");
    let font = font::from_file(font_path)
        .expect(&format!("Failed to load font from path: {:?}", font_path));
    Model{
        font: font,
        page: 0,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // 特に処理なし
}

// イベント処理
fn event(_app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent { simple, .. } = event {
        if let Some(WindowEvent::KeyPressed(Key::Return)) = simple {
            model.page += 1;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let test: Vec<String> =
        insert_separator("漢字とひらがなとカタカナの試験もしくは南無阿弥陀仏テスト");

    let draw = app.draw();
    draw.background().color(WHITE);
    draw.text(&test[model.page])
        .x_y(0.0, 0.0) // 中央に配置
        .font(model.font.clone())
        .font_size(50) // フォントを指定
        .color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}

fn insert_separator(text: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut start = 0;

    for (i, c) in text.char_indices() {
        if is_kanji(&c) {
            if start < i {
                result.push(text[start..i].to_string());
            }
            result.push(c.to_string());
            start = i + c.len_utf8(); // 次の文字の開始位置を更新
        }
    }
    
    if start < text.len() {
        result.push(text[start..].to_string());
    }

    result
}


fn is_kanji(c: &char) -> bool {
    // 漢字のUnicode範囲をチェック
    (*c >= '\u{4E00}' && *c <= '\u{9FFF}') || // CJK統合漢字
    (*c >= '\u{3400}' && *c <= '\u{4DBF}') || // CJK統合漢字拡張A
    (*c >= '\u{20000}' && *c <= '\u{2A6DF}') || // CJK統合漢字拡張B
    (*c >= '\u{2A700}' && *c <= '\u{2B73F}') || // CJK統合漢字拡張C
    (*c >= '\u{2B740}' && *c <= '\u{2B81F}') || // CJK統合漢字拡張D
    (*c >= '\u{2B820}' && *c <= '\u{2CEAF}') || // CJK統合漢字拡張E
    (*c >= '\u{2CEB0}' && *c <= '\u{2EBEF}') // CJK統合漢字拡張F
}