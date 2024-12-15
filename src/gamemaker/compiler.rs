use crate::organizer::YoyogayProject;

use super::{GameMakerObject, GameMakerProject, GameMakerScript};

impl GameMakerProject<'_> {
    pub fn new_from_yoyogay_project<'a>(
        yoyogay_project: &'a YoyogayProject,
    ) -> GameMakerProject<'a> {
        let objects: Vec<GameMakerObject<'a>> = yoyogay_project
            .objects
            .iter()
            .map(|obj| {
                GameMakerObject::new(
                    &obj.id,
                    "{}",
                    obj.create.as_ref(),
                    obj.step.as_ref(),
                    obj.draw.as_ref(),
                    obj.draw_gui.as_ref(),
                    obj.clean_up.as_ref(),
                )
            })
            .collect();
        let scripts: Vec<GameMakerScript<'a>> = Vec::new();

        GameMakerProject { objects, scripts }
    }
}
