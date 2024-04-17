use std::fmt::Display;

pub enum Pages {
    PrintCircuit,
    GetFromJson,
    EditComponent,
    SaveAsJson,
    Exit,
}

impl Display for Pages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pages::PrintCircuit => write!(f, "Print Circuit"),
            Pages::GetFromJson => write!(f, "Get From JSON"),
            Pages::EditComponent => write!(f, "Edit Component"),
            Pages::SaveAsJson => write!(f, "Save As JSON"),
            Pages::Exit => write!(f, "Exit"),
        }
    }
}
