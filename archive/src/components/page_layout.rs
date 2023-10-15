use std::rc::Rc;

use itertools::Itertools;
use ratatui::prelude::*;

pub struct PageLayout;

impl PageLayout {
    pub fn new(area: Rect, direction: Direction, heights: Vec<u16>) -> Rc<[Rect]> {
        let constraints = heights
            .iter()
            .map(|&h| {
                if h > 0 {
                    Constraint::Length(h)
                } else {
                    Constraint::Min(0)
                }
            }).collect_vec();

            Layout::default()
                .direction(direction)
                .constraints(constraints)
                .split(area)
    }
}