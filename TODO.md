# Game
* [ ] Use a single vector for the mine grid
* [ ] Add a retry option
* [ ] Allow specifying random seed

# rustbox
* Suggest creating a Cell implementation
  * Builder pattern for creating Cells
* Instead of set_cursor taking isize, have it take usize and add a hide_cursor
  method.

# Elm/Web UI
* Use a websocket to communicate between Rust backend and Elm(?) frontend
* Clicking buttons sends an event via websocket
* Backend sends events back via websocket
* Communicate using JSON?
