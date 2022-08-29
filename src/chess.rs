use anyhow::Context;
use std::{
    error::Error as SError,
    fmt::{Debug, Formatter},
};
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::error_ext::{ErrorExt, ToAnyhowNotErr};

///Enum with all of the chess piece kinds
#[derive(EnumIter, Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ChessPieceKind {
    ///Bishop Piece - move on diagonals
    Bishop = 2,
    ///Knight piece - dx=1,dy=1
    Knight = 1,
    ///Pawn - move 2 towards enemy dir on turn 1, 1 on subsequent, take diagonally
    Pawn = 0,
    ///Queen - [`Bishop`] | [`Rook`]
    Queen = 4,
    ///King - [`Queen`] but one tile
    King = 5,
    ///Rook - up,down,left,right
    Rook = 3,
}

///Enum to hold errors for chess piece kinds
#[derive(Debug, Display)]
pub enum ChessPieceKindParseError {
    ///Failed to find a match
    FailedMatch(String),
}

impl SError for ChessPieceKindParseError {}

impl TryFrom<String> for ChessPieceKind {
    type Error = ChessPieceKindParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim().to_lowercase();
        match value.as_str() {
            "bishop" => Ok(Self::Bishop),
            "knight" => Ok(Self::Knight),
            "pawn" => Ok(Self::Pawn),
            "queen" => Ok(Self::Queen),
            "king" => Ok(Self::King),
            "rook" => Ok(Self::Rook),
            _ => Err(ChessPieceKindParseError::FailedMatch(value)),
        }
    }
}

///Struct to hold a chess piece
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ChessPiece {
    ///The kind of the chess piece
    pub kind: ChessPieceKind,
    ///Whether or not this is a white piece.
    pub is_white: bool,
}
impl ChessPiece {
    ///Gets all of the variants of a [`ChessPiece`] - each of the variants of [`ChessPieceKind`] with one black and one white
    #[must_use]
    pub fn all_variants() -> Vec<Self> {
        let mut v = Vec::with_capacity(12);

        for el in ChessPieceKind::iter() {
            v.push(Self {
                kind: el,
                is_white: false,
            });
            v.push(Self {
                kind: el,
                is_white: true,
            });
        }

        v
    }

    ///Converts a [`ChessPiece`] to a file name
    #[must_use]
    pub fn to_file_name(self) -> String {
        format!(
            "{}_{}.png",
            if self.is_white { "white" } else { "black" },
            self.kind.to_string().to_lowercase()
        )
    }
}

impl Debug for ChessPiece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChessPiece")
            .field("kind", &self.kind.to_string())
            .field("is_white", &self.is_white)
            .finish()
    }
}

impl PartialOrd for ChessPiece {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.is_white.partial_cmp(&other.is_white) {
            Some(core::cmp::Ordering::Equal) => self.kind.partial_cmp(&other.kind),
            ord => ord,
        }
    }
}
impl Ord for ChessPiece {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .ae()
            .with_context(|| format!("comparing {self:?} to {other:?}"))
            .unwrap_log_error()
    }
}
