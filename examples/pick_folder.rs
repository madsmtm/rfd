fn main() {
    let path = rfd::pick_folder(None);

    println!(
        "{}",
        path.map_or_else(
            || "The user did not choose any folder, or an error occured!".to_owned(),
            |path| format!("The user choose this folder: {}", path.to_string_lossy())
        )
    );
}
