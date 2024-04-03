#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    pub data: Vec<(i32, String, i64)>,
}

impl Default for Data {
    fn default() -> Self {
        Self { data: vec![
            (0, String::from("Big Brown Fox"), 0),
            (1, String::from("Yellow Brick Road"), 5),
            (2, String::from("Lorem Ipsum"), 6)
        ] }
    }
}

pub enum DataActions {
    #[allow(dead_code)]
    RemoveData(i32),
}

impl yew::Reducible for Data {
    type Action = DataActions;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut new = (*self).clone();
        match action {
            DataActions::RemoveData(id) => {
                new.data.retain(|(i, _, _)| i != &id);
            }
        }
        std::rc::Rc::new(new)
    }
}
