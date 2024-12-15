use std::fs;
use std::path::PathBuf;

pub mod compiler;

pub struct GameMakerProject<'a> {
    pub objects: Vec<GameMakerObject<'a>>,
    pub scripts: Vec<GameMakerScript<'a>>,
}

impl GameMakerProject<'_> {
    pub fn write_in_fs<T: Into<PathBuf>>(&self, path: T) -> Result<(), std::io::Error> {
        let path = path.into();
        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        }
        fs::create_dir(&path)?;

        let objects_path = path.join("objects");
        fs::create_dir(&objects_path)?;
        for object in &self.objects {
            object.write_into_fs(&objects_path)?;
        }

        let scripts_path = path.join("scripts");
        fs::create_dir(&scripts_path)?;
        for script in &self.scripts {
            let script_path = scripts_path.join(&script.name);
            fs::create_dir(&script_path)?;
            fs::write(
                script_path.join(format!("{}.gml", &script.name)),
                &script.src,
            )?;
            fs::write(script_path.join(format!("{}.yy", &script.name)), "")?;
        }

        fs::write(path.join("project.yyp"), "{}")?;

        Ok(())
    }
}

pub struct GameMakerObject<'a> {
    pub name: &'a String,
    pub info: String,
    pub create: Option<&'a String>,
    pub step: Option<&'a String>,
    pub clean_up: Option<&'a String>,
    pub draw: Option<&'a String>,
    pub draw_gui: Option<&'a String>,
}

impl GameMakerObject<'_> {
    pub fn new<'a, T: Into<String>>(
        name: &'a String,
        info: T,
        create: Option<&'a String>,
        step: Option<&'a String>,
        clean_up: Option<&'a String>,
        draw: Option<&'a String>,
        draw_gui: Option<&'a String>,
    ) -> GameMakerObject<'a> {
        GameMakerObject {
            name,
            info: info.into(),
            create,
            step,
            clean_up,
            draw,
            draw_gui,
        }
    }

    fn create_event_file(
        &self,
        path: &PathBuf,
        src: &Option<&String>,
        name: &str,
    ) -> Result<(), std::io::Error> {
        if let Some(src) = src {
            fs::write(path.join(name), src)?;
        }

        Ok(())
    }
    pub fn write_into_fs(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        let path = path.join(&self.name);

        fs::create_dir(&path)?;
        self.create_event_file(&path, &self.create, "Create_0.gml")?;
        self.create_event_file(&path, &self.step, "Step_1.gml")?;
        self.create_event_file(&path, &self.draw, "Draw_0.gml")?;
        self.create_event_file(&path, &self.draw_gui, "Draw_64.gml")?;
        self.create_event_file(&path, &self.clean_up, "CleanUp_0.gml")?;

        Ok(())
    }
}

pub struct GameMakerScript<'a> {
    pub name: &'a String,
    pub src: &'a String,
    pub info: String,
}

impl<'a> GameMakerScript<'a> {
    pub fn new(name: &'a String, src: &'a String, info: String) -> GameMakerScript<'a> {
        GameMakerScript { name, src, info }
    }
}
