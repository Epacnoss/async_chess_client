use crate::chess::ChessPiece;
use color_eyre::Report;
use find_folder::Search::ParentsThenKids;
use piston_window::{Filter, Flip, G2dTexture, PistonWindow, Texture, TextureSettings};
use std::{collections::HashMap, path::PathBuf};
use crate::eyre;

pub const TILE_S: f64 = 20.0;
pub const BOARD_S: f64 = 256.0;

#[derive(Debug)]pub struct Cacher {
    path: PathBuf,
    assets: HashMap<String, G2dTexture>,
}

impl Cacher {
    pub fn new() -> Result<Self, find_folder::Error> {
        let path = ParentsThenKids(3, 3).for_folder("assets")?;
        Ok(Self {
            path,
            assets: HashMap::new(),
        })
    }
    pub fn new_and_populate(win: &mut PistonWindow) -> Result<Self, Report> {
        let mut s = Self::new()?;
        s.populate(win)?;
        Ok(s)
    }

    pub fn get(&self, p: &str) -> Option<&G2dTexture> {
        self.assets.get(p)
    }

    fn insert(&mut self, p: &str, win: &mut PistonWindow) -> Result<(), Report> {
        let path = self.path.join(p);
        let ts = TextureSettings::new().filter(Filter::Nearest);

        match Texture::from_path(&mut win.create_texture_context(), path, Flip::None, &ts) {
            Ok(tex) => {
                self.assets.insert(p.to_string(), tex);
                Ok(())
            }
            Err(e) => {
                Err(eyre!("Unable to find texture: {e}"))
            }
        }
    }

    #[tracing::instrument(skip(self, win), fields(s_len=self.assets.len(), path=?self.path))]
    pub fn populate(&mut self, win: &mut PistonWindow) -> Result<(), Report>{
        for variant in ChessPiece::all_variants() {
            self.insert(&variant.to_file_name(), win).map_err(|e| eyre!("Unable to insert {variant:?}: {e}"))?;
        }

        for extra in &["board_alt.png", "highlight.png", "selected.png"] {
            self.insert(extra, win).map_err(|e| eyre!("Unable to insert {extra:?}: {e}"))?;
        }

        Ok(())
    }
}
