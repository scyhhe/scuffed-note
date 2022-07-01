use serde::Deserialize;

#[derive(Deserialize)]
pub struct NoteDto {
    pub contents: String,
    pub uses: Option<u8>,
}

#[derive(Clone)]
pub struct Note {
    pub contents: String,
    pub uses: u8,
}

impl From<NoteDto> for Note {
    fn from(dto: NoteDto) -> Self {
        Note {
            contents: dto.contents,
            uses: dto.uses.unwrap_or(1),
        }
    }
}
