# Project Name
Project Description

### Todo
- [X] Add error handling for `exposed::window::event::Event::create`. Using `Option<T>`. Rust does not support returning unit structs.
  
- [ ] Remove `opengl32.lib`. We are already linking it with dynamic linking.

- [ ] Add `#Safety` to docs. 

- [ ] Remove types from `exposed::window::mod.rs`.

### Win32 Todo

- [ ] Add support for reciving surrogates from `WM_CHAR` amd `WM_SYSCHAR`.

- [ ] Add support for reciving `VK_LSHIFT` like virtual key codes.
 
### X11 Todo

- [ ] X11 cleanup.