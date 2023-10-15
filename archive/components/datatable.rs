use ratatui::widgets::TableState;

fn data() -> Vec<Vec<&'static str>>{
    vec![
        vec!["1", "Home Name", "Understat", "Salled", "Dkeedls", "wsksddlw"],
        vec!["2", "Frenddd", "dienddl", "Sdeindd", "einddle", "endiendd"],
        vec!["3", "Home Name", "Understat", "Salled", "Dkeedls", "wsksddlw"],
        vec!["4", "Frenddd", "dienddl", "Sdeindd", "einddle", "endiendd"],
        vec!["5", "Home Name", "Understat", "Salled", "Dkeedls", "wsksddlw"],
        vec!["6", "Frenddd", "dienddl", "Sdeindd", "einddle", "endiendd"],
        vec!["7", "Home Name", "Understat", "Salled", "Dkeedls", "wsksddlw"],
        vec!["8", "Frenddd", "dienddl", "Sdeindd", "einddle", "endiendd"],
        vec!["9", "Home Name", "Understat", "Salled", "Dkeedls", "wsksddlw"],
        vec!["10", "Frenddd", "dienddl", "Sdeindd", "einddle", "endiendd"],
        vec!["11", "Home Name", "Understat", "Salled", "Dkeedls", "wsksddlw"],
        vec!["12", "Frenddd", "dienddl", "Sdeindd", "einddle", "endiendd"]
    ]
}
pub struct DataTable<'a> {
    pub state: TableState,
    pub items: Vec<Vec<&'a str>>
}

impl<'a> DataTable<'a> {
    pub fn new() -> DataTable<'a> {
        DataTable { state: TableState::default(), items: data() }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            },
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            },
            None => 0,
        };
        self.state.select(Some(i));
    }

}