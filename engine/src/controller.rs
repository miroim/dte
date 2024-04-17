use crate::debug;
use crate::model::Model;
use crate::region::Region;

pub struct Controller {
  /// Edited textual content.
  model: Model,
  /// Visible content viewport.
  viewport: Region,
}

impl Controller {
  pub fn new(content: String, width: usize, height: usize) -> Self {
    let model = Model::new(content);
    let viewport = Region::new(0, 0, width, height);
    Self { model, viewport }
  }

  pub fn viewport(&self) -> &Region {
    &self.viewport
  }

  pub fn resize(&mut self, width: usize, height: usize) -> Vec<Region> {
    let mut dirties = vec![];
    if width > self.viewport.width() {
      dirties.push(Region::new(self.viewport.width(), 0, width.saturating_sub(self.viewport.width()), height));
    }
    if height > self.viewport.height() {
      dirties.push(Region::new(0, self.viewport.height(), width, height.saturating_sub(self.viewport.height())));
    }
    self.viewport.resize(width, height);
    dirties
  }

  /// Returns the cursor position (model coordinates).
  pub fn cursor_position(&self) -> (usize, usize) {
    self.model.cursor_position()
  }

  pub fn content(&self) -> &[Vec<char>] {
    self.model.content()
  }

  pub fn content_region(&mut self) -> Region {
    self.model.content_region()
  }

  pub fn cursor_move_left(&mut self) -> Option<bool> {
    if self.model.cursor_move_left() {
      let (x, _) = self.cursor_position();
      if self.viewport.left() > self.content_region().left() {
        if self.viewport.adjust_left(x, 1) {
          return Some(true);
        }
      }
      return Some(false);
    }
    None
  }

  pub fn cursor_move_right(&mut self) -> Option<bool> {
    if self.model.cursor_move_right() {
      let (x, _) = self.cursor_position();
      if self.viewport.right() < self.content_region().right() {
        if self.viewport.adjust_right(x, 1) {
          return Some(true);
        }
      }
      return Some(false);
    }
    None
  }

  pub fn cursor_move_up(&mut self) -> Option<bool> {
    if self.model.cursor_move_up() {
      let (_, y) = self.cursor_position();
      if self.viewport.top() > self.content_region().top() {
        if self.viewport.adjust_up(y, 1) {
          return Some(true);
        }
      }
      return Some(false);
    }
    None
  }

  pub fn cursor_move_down(&mut self) -> Option<bool> {
    if self.model.cursor_move_down() {
      let (_, y) = self.cursor_position();
      if self.viewport.bottom() < self.content_region().bottom() {
        if self.viewport.adjust_down(y, 1) {
          return Some(true);
        }
      }
      return Some(false);
    }
    None
  }

  pub fn cursor_move_cell_start(&mut self) -> Option<bool> {
    if self.model.cursor_move_cell_start() {
      return Some(false);
    }
    None
  }

  pub fn cursor_move_cell_end(&mut self) -> Option<bool> {
    if self.model.cursor_move_cell_end() {
      return Some(false);
    }
    None
  }

  pub fn cursor_move_cell_next(&mut self) -> Option<bool> {
    if self.model.cursor_move_cell_next() {
      return Some(false);
    }
    None
  }

  pub fn cursor_move_cell_prev(&mut self) -> Option<bool> {
    if self.model.cursor_move_cell_prev() {
      return Some(false);
    }
    None
  }

  pub fn cursor_move_row_start(&mut self) -> Option<bool> {
    if self.model.cursor_move_row_start() {
      return Some(false);
    }
    None
  }

  pub fn cursor_move_row_end(&mut self) -> Option<bool> {
    if self.model.cursor_move_row_end() {
      return Some(false);
    }
    None
  }

  pub fn cursor_toggle(&mut self) {
    self.model.cursor_toggle();
  }

  pub fn cursor_toggle_bar_block(&mut self) {
    self.model.cursor_toggle_bar_block();
  }

  pub fn cursor_is_bar(&self) -> bool {
    self.model.cursor_is_bar()
  }

  pub fn cursor_is_block(&self) -> bool {
    self.model.cursor_is_block()
  }

  pub fn cursor_is_underscore(&self) -> bool {
    self.model.cursor_is_underscore()
  }

  /// Returns the character pointed by cursor.
  pub fn cursor_char(&self) -> Option<char> {
    self.model.cursor_char()
  }
}
