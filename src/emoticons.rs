//! Emoticons database - comprehensive list organized by categories
use std::collections::HashMap;
use std::sync::OnceLock;
pub static EMOTICONS: OnceLock<HashMap<&'static str, Vec<String>>> = OnceLock::new();
pub fn get_emoticons() -> &'static HashMap<&'static str, Vec<String>> {
    EMOTICONS.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("Happy", vec![
            "😀", "😃", "😄", "😁", "😆", "😊", "😇", "🙂", "🙃", "😉", "😌", "😍", "🥰", "😘"
        ].iter().map(|s| s.to_string()).collect());
        map.insert("Sad", vec![
            "😢", "😭", "😿", "😔", "😞", "😟", "😥", "😰", "😨", "😧", "😦"
        ].iter().map(|s| s.to_string()).collect());
        map.insert("Angry", vec![
            "😠", "😡", "🤬", "😤", "😾", "💢"
        ].iter().map(|s| s.to_string()).collect());
        map.insert("Surprised", vec![
            "😮", "😯", "😲", "😳", "🤯"
        ].iter().map(|s| s.to_string()).collect());
        map.insert("Love", vec![
            "❤️", "💕", "💖", "💗", "💓", "💞", "💝", "💘", "💟", "♥️"
        ].iter().map(|s| s.to_string()).collect());
        map.insert("Gestures", vec![
            "👍", "👎", "👌", "✌️", "🤞", "🤘", "🤙", "👏", "🙌", "👐", "🤲", "🤝", "🙏"
        ].iter().map(|s| s.to_string()).collect());
        map.insert("Faces", vec![
            "😐", "😑", "😶", "🙄", "😏", "😣", "😥", "😮", "🤐", "😯", "😪", "😫", "🥱", "😴"
        ].iter().map(|s| s.to_string()).collect());
        map.insert("Cool", vec![
            "😎", "🤓", "🧐", "😺", "😸", "😹", "😻", "😼", "😽", "🙀"
        ].iter().map(|s| s.to_string()).collect());
        map.insert("Symbols", vec![
            "⭐", "✨", "🌟", "💫", "🔥", "💥", "💦", "💨", "✅", "❌", "⚡", "🌈"
        ].iter().map(|s| s.to_string()).collect());
        map.insert("Objects", vec![
            "🎉", "🎊", "🎈", "🎁", "🏆", "🥇", "🥈", "🥉", "🏅", "🎖️"
        ].iter().map(|s| s.to_string()).collect());
        map.insert("Animals", vec![
            "🐶", "🐱", "🐭", "🐹", "🐰", "🦊", "🐻", "🐼", "🐨", "🐯", "🦁", "🐮", "🐷", "🐸", "🐵"
        ].iter().map(|s| s.to_string()).collect());
        map.insert("Food", vec![
            "🍕", "🍔", "🍟", "🌭", "🍿", "🧂", "🍰", "🎂", "🍩", "🍪", "🍫", "🍬", "🍭", "☕", "🍵"
        ].iter().map(|s| s.to_string()).collect());
        map.insert("Classic", vec![
            ":-)", ":)", ":(", ":-(", ";-)", ";)", ":-D", ":D", ":-P", ":P", ":-O", ":O", ":-|", ":|",
            "<3", "</3", ":*", ":-*", "^_^", "^.^", "o_o", "O_O", "T_T", "ToT", ">_<", "-_-",
            r"¯\_(ツ)_/¯", "(╯°□°）╯︵ ┻━┻", "(ಠ_ಠ)", "(◕‿◕)", "(づ｡◕‿‿◕｡)づ", "ʕ•ᴥ•ʔ"
        ].iter().map(|s| s.to_string()).collect());
        map
    })
}
