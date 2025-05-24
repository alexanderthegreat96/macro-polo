# 🛠️ Macro Polo: Development Requirements & Features

## ✅ Core Requirements

- [ ] Mouse and keyboard simulation (movement, clicks, typing)
- [ ] Lua scripting support with function bindings
- [ ] Ability to register native Rust functions callable from Lua
- [ ] GUI for managing, editing, and running Lua scripts
- [ ] Keyboard and mouse shortcut bindings to trigger scripts

---

## 🖱️ Mouse Functions

| Lua Function            | Description                                | Rust (`enigo`) Equivalent                          |
|-------------------------|--------------------------------------------|----------------------------------------------------|
| `move_mouse(x, y)`      | Move cursor to screen position              | `move_mouse(x, y, Coordinate::Abs)`                |
| `move_rel(dx, dy)`      | Move cursor by delta                        | `mouse_move_relative(dx, dy)`                      |
| `click(button)`         | Click a mouse button                        | `button(Button::X, Action::Click)`                 |
| `press_mouse(button)`   | Hold a mouse button                         | `button(Button::X, Action::Press)`                 |
| `release_mouse(button)` | Release a held mouse button                 | `button(Button::X, Action::Release)`               |
| `scroll(x, y)`          | Scroll the mouse wheel                      | `mouse_scroll_x(value)` / `mouse_scroll_y(value)`  |

**➕ Bonus:**  
`drag_mouse(x, y)` — Composite: Press → Move → Release

---

## ⌨️ Keyboard Functions

| Lua Function       | Description               | Rust (`enigo`) Equivalent         |
|--------------------|---------------------------|-----------------------------------|
| `key_press(key)`   | Simulate key press        | `key_click(Key::Layout(...))`     |
| `key_down(key)`    | Hold a key down           | `key_down(Key::Layout(...))`      |
| `key_up(key)`      | Release a key             | `key_up(Key::Layout(...))`        |
| `type_text("...")` | Type a string of text     | `text("...")`                      |

**Tip:** Add support for string → key mappings: `"ctrl"`, `"shift"`, `"enter"`, etc.

---

## ⏱️ Timing & Delays

| Lua Function     | Description                      | Rust Equivalent                 |
|------------------|----------------------------------|---------------------------------|
| `sleep(ms)`      | Pause execution in milliseconds  | `std::thread::sleep()`          |
| `wait_for(cond)` | Wait until a Lua condition       | Polling logic inside Lua        |

---

## 🧠 Macro Logic & Structure

Lua supports standard control flow, but helper functions enhance scripting experience:

| Lua Helper         | Description                              |
|--------------------|------------------------------------------|
| `macro(name, fn)`  | Register a reusable named macro          |
| `run_macro(name)`  | Execute a registered macro by name       |
| `bind(key, fn)`    | Bind a macro or function to a hotkey     |

---

## 🔥 Advanced Features (Planned / Optional)

| Feature                | Description                                  | Notes                                 |
|------------------------|----------------------------------------------|----------------------------------------|
| `bind_hotkey(key, fn)` | Trigger Lua function via global hotkey       | Requires `hotkey` crate or native API |
| `listen_mouse(fn)`     | Hook and respond to global mouse events      | Requires global mouse hook             |
| `set_clipboard(text)`  | Set clipboard contents                       | Use `copypasta` crate                  |
| `get_color(x, y)`      | Read pixel color from screen at coordinates  | Use `screenshot` / `image` crate       |

---

## 💡 Next Steps

- [ ] Implement core input bindings (mouse + keyboard)
- [ ] Integrate Lua VM and bind base functions
- [ ] Set up simple GUI (e.g., `egui`, `iced`)
- [ ] Implement script loader and runtime
- [ ] Add hotkey system to trigger macros
- [ ] Design script editor with syntax highlighting (future)
- [ ] Ability to do screenshot / pixel detection

---
