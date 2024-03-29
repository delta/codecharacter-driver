#[derive(Debug)]
pub struct GameDir {
    full_path: String,
}

impl GameDir {
    pub fn new(game_id: &str) -> Option<Self> {
        let err = std::fs::create_dir(format!("/tmp/{game_id}"));
        if let Err(_e) = err {
            return None;
        }
        Some(GameDir {
            full_path: format!("/tmp/{game_id}"),
        })
    }

    pub fn create_sub_dir(&self, sub_dir: &str) -> Option<()> {
        std::fs::create_dir_all(format!("{}/{}", self.full_path, sub_dir)).ok()
    }

    pub fn get_path(&self) -> &str {
        &self.full_path
    }
}
impl Drop for GameDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(self.get_path());
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Write, path::Path};

    use super::GameDir;

    #[test]
    fn dir_creation_and_deletion_check() {
        let game_id = "030af985-f4b5-4914-94d8-e559576449e3";
        let match_dir_handle = GameDir::new(game_id).unwrap();

        let full_path = match_dir_handle.get_path().to_owned();

        let player_code_file = format!("{full_path}/something");
        let _ = std::fs::File::create(player_code_file).and_then(|mut file| {
            file.write_all("Hello".as_bytes())
                .and_then(|_| file.sync_all())
        });
        assert!(Path::new(&full_path).exists());

        drop(match_dir_handle);

        assert!(!Path::new(&full_path).exists());
    }
}
